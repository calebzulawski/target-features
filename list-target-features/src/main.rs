use std::{
    io::Write,
    process::{Command, Stdio},
};

fn get_rustc_version() -> String {
    let output = Command::new("rustc")
        .args(["+nightly", "--version"])
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .to_string()
}

struct Feature {
    feature: String,
    description: String,
    implies: Vec<String>,
}

fn get_features(triple: &str, target_feature: &str) -> Vec<String> {
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "cfg", "--target", triple])
        .arg(format!("-Ctarget-feature={}", target_feature))
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    std::str::from_utf8(&output.stdout)
        .unwrap()
        .lines()
        .filter_map(|s| {
            s.strip_prefix("target_feature=\"")
                .and_then(|s| s.strip_suffix('"'))
                .map(ToString::to_string)
        })
        .filter(|f| f != "llvm14-builtins-abi")
        .collect()
}

fn get_all_features(triple: &str) -> Vec<Feature> {
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "target-features", "--target", triple])
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    let disable_default_features = get_features(triple, "")
        .iter()
        .map(|s| format!("-{}", s))
        .collect::<Vec<String>>()
        .join(",");

    // avoid LLVM issue
    let required_arch_features = if triple.starts_with("mips") {
        ",+fp64"
    } else {
        ""
    };

    let mut features = Vec::new();
    for line in std::str::from_utf8(&output.stdout).unwrap().lines().skip(1) {
        let mut split = line.split(" - ");
        let feature = split.next().unwrap().trim().to_string();
        if feature.is_empty() {
            break;
        } else {
            let description = split.next().unwrap().trim().to_string();

            let mut implies = get_features(
                triple,
                &format!(
                    "{},+{}{}",
                    disable_default_features, feature, required_arch_features
                ),
            );
            implies.sort();
            implies.retain(|f| f != &feature);

            features.push(Feature {
                feature,
                description,
                implies,
            });
        }
    }
    features
}

fn main() {
    let arches = [
        ("Arm", "arm-unknown-linux-gnueabihf"),
        ("AArch64", "aarch64-unknown-none"),
        ("Bpf", "bpfeb-unknown-none"),
        ("Hexagon", "hexagon-unknown-linux-musl"),
        ("Mips", "mips64-unknown-linux-gnuabi64"),
        ("PowerPC", "powerpc64-unknown-linux-gnu"),
        ("RiscV", "riscv32i-unknown-none-elf"),
        ("Wasm", "wasm32-unknown-unknown"),
        ("X86", "x86_64-unknown-none"),
    ];

    let mut file =
        std::fs::File::create(std::env::current_dir().unwrap().join("target-features.txt"))
            .unwrap();

    writeln!(file, "rustc_version = {}", get_rustc_version()).unwrap();
    for (arch, triple) in arches {
        for Feature {
            feature,
            description,
            implies,
        } in get_all_features(triple)
        {
            writeln!(file, "").unwrap();
            writeln!(file, "feature = {feature}").unwrap();
            writeln!(file, "arch = {arch}").unwrap();
            writeln!(file, "implies = {}", implies.join(" ")).unwrap();
            writeln!(file, "description = {description}").unwrap();
        }
    }
}
