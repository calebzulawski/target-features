use std::{collections::HashMap, error::Error, fs::File, io::Write, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let generated = include_str!("target-features.txt");
    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    let mut lines = generated.lines();

    // Parse the generated features file
    let rustc_version = lines
        .next()
        .unwrap()
        .strip_prefix("rustc_version =")
        .unwrap()
        .trim();

    let mut features = Vec::new();
    while lines.next().is_some() {
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
        features.push((feature, arch, description, implies));
    }

    // Generate the features module
    let mut module = File::create(Path::new(&out_dir).join("generated.rs"))?;
    writeln!(
        module,
        "const FEATURES: &[(crate::Architecture, &str, &str, &[Feature])] = &["
    )?;
    for (feature, arch, description, implies) in &features {
        let implies = implies
            .iter()
            .map(|implied_feature| {
                format!(
                    "Feature({})",
                    features
                        .iter()
                        .position(|(f, a, _, _)| implied_feature == f && arch == a)
                        .unwrap()
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            module,
            "    (crate::Architecture::{arch}, \"{feature}\", \"{description}\", &[{implies}]),"
        )?;
    }
    writeln!(module, "];")?;

    let build_features = std::env::var("CARGO_CFG_TARGET_FEATURE")
        .map(|x| x.split(',').map(ToString::to_string).collect())
        .unwrap_or_else(|_| Vec::new());
    let build_arch = match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "arm" => "Arm",
        "aarch64" => "AArch64",
        "bpf" => "Bpf",
        "hexagon" => "Hexagon",
        "mips" | "mips64" => "Mips",
        "powerpc" | "powerpc64" => "PowerPC",
        "riscv32" | "riscv64" => "RiscV",
        "wasm32" | "wasm64" => "Wasm",
        "x86" | "x86_64" => "X86",
        _ => "Unsupported",
    };
    writeln!(module, "/// The target of the current build.")?;
    writeln!(module, "#[allow(clippy::let_and_return)]")?;
    writeln!(module, "pub const CURRENT_TARGET: Target = {{")?;
    writeln!(module, "    let arch = Architecture::{build_arch};")?;
    writeln!(module, "    let target = Target::new(arch);")?;
    for feature in build_features {
        writeln!(module, "    let target = if let Ok(feature) = Feature::new(arch, \"{feature}\") {{ target.with_feature(feature) }} else {{ target }};")?;
    }
    writeln!(module, "    target")?;
    writeln!(module, "}};")?;

    // Generate the features docs
    let mut feature_docs = File::create(Path::new(&out_dir).join("features.md"))?;
    let mut features_by_arch = HashMap::<_, Vec<_>>::new();
    for (feature, arch, description, _) in features {
        features_by_arch
            .entry(arch)
            .or_default()
            .push((feature, description));
    }
    let mut features_by_arch = features_by_arch.drain().collect::<Vec<_>>();
    features_by_arch.sort();
    writeln!(feature_docs, "Generated with {rustc_version}")?;
    for (arch, features) in features_by_arch.drain(..) {
        writeln!(feature_docs, "## {} features", arch.to_lowercase())?;
        for (feature, description) in features {
            writeln!(feature_docs, " * `{feature}` - {description}")?
        }
    }

    // Rerun build if the source features changed
    println!("cargo:rerun-if-changed=target-features.txt");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
