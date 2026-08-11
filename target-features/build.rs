use std::{error::Error, fs::File, io::Write, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let rustc_version = include_str!("rustc-version.txt").trim();
    let target_features = include_str!("target-features.txt");
    let target_cpus = include_str!("target-cpus.txt");
    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    // Parse the generated features file
    let mut lines = target_features.lines().peekable();
    let mut features = Vec::new();
    while lines.peek().is_some() {
        let feature = lines
            .next()
            .unwrap()
            .strip_prefix("feature =")
            .unwrap()
            .trim();
        let arch = lines.next().unwrap().strip_prefix("arch =").unwrap().trim();
        let implies = lines
            .next()
            .unwrap()
            .strip_prefix("implies =")
            .unwrap()
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let description = lines
            .next()
            .unwrap()
            .strip_prefix("description =")
            .unwrap()
            .trim();
        let runtime = lines
            .next()
            .unwrap()
            .strip_prefix("runtime =")
            .unwrap()
            .trim();
        let _ = lines.next();
        features.push((feature, arch, description, implies, runtime));
    }

    // Parse the generated CPUs file
    let mut lines = target_cpus.lines().peekable();
    let mut cpus = Vec::new();
    while lines.peek().is_some() {
        let cpu = lines.next().unwrap().strip_prefix("cpu =").unwrap().trim();
        let arch = lines.next().unwrap().strip_prefix("arch =").unwrap().trim();
        let features = lines
            .next()
            .unwrap()
            .strip_prefix("features =")
            .unwrap()
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let _ = lines.next();
        cpus.push((cpu, arch, features));
    }

    // Write the generated docs
    let mut rustc_docs = File::create(Path::new(&out_dir).join("generated.md"))?;
    writeln!(rustc_docs, "Generated with {rustc_version}.")?;

    // Write a module
    let mut module = File::create(Path::new(&out_dir).join("generated.rs"))?;

    // Generate the features array
    writeln!(
        module,
        "const FEATURES: &[(crate::Architecture, &str, &str, &[Feature], bool)] = &["
    )?;
    for (feature, arch, description, implies, runtime) in &features {
        let implies = implies
            .iter()
            .map(|implied_feature| {
                format!(
                    "Feature({})",
                    features
                        .iter()
                        .position(|(f, a, _, _, _)| implied_feature == f && arch == a)
                        .unwrap()
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            module,
            "    (crate::Architecture::{arch}, \"{feature}\", \"{description}\", &[{implies}], {runtime}),"
        )?;
    }
    writeln!(module, "];")?;

    // Generate the CPUs array
    writeln!(
        module,
        "const CPUS: &[(crate::Architecture, &str, &[Feature])] = &["
    )?;
    for (cpu, arch, cpu_features) in &cpus {
        let cpu_features = cpu_features
            .iter()
            .map(|feature| {
                format!(
                    "Feature({})",
                    features
                        .iter()
                        .position(|(f, a, _, _, _)| feature == f && arch == a)
                        .unwrap()
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            module,
            "    (crate::Architecture::{arch}, \"{cpu}\", &[{cpu_features}]),"
        )?;
    }
    writeln!(module, "];")?;

    let build_features = std::env::var("CARGO_CFG_TARGET_FEATURE")
        .map(|x| x.split(',').map(ToString::to_string).collect())
        .unwrap_or_else(|_| Vec::new());
    let build_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    writeln!(module, "/// The target of this build.")?;
    writeln!(module, "#[allow(clippy::let_and_return)]")?;
    writeln!(module, "pub const BUILD_TARGET: Target = {{")?;
    writeln!(
        module,
        "    let arch = Architecture::from_str({build_arch:?});"
    )?;
    writeln!(module, "    let target = Target::new(arch);")?;
    for feature in build_features {
        writeln!(module, "    let target = if let Ok(feature) = Feature::new(arch, \"{feature}\") {{ target.with_feature(feature) }} else {{ target }};")?;
    }
    writeln!(module, "    target")?;
    writeln!(module, "}};")?;

    // Rerun build if the source features changed
    println!("cargo:rerun-if-changed=rustc-version.txt");
    println!("cargo:rerun-if-changed=target-features.txt");
    println!("cargo:rerun-if-changed=target-cpus.txt");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
