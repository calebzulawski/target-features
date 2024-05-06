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

struct Cpu {
    cpu: String,
    features: Vec<String>,
}

struct Feature {
    feature: String,
    description: String,
    implies: Vec<String>,
    runtime: bool,
}

fn get_features(triple: &str, target_feature: &str, target_cpu: &str) -> Vec<String> {
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "cfg", "--target", triple])
        .arg(format!("-Ctarget-feature={}", target_feature))
        .arg(format!("-Ctarget-cpu={}", target_cpu))
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

fn detect_at_runtime(triple: &str, target_feature: &str) -> bool {
    Command::new("cargo")
        .args([
            "+nightly",
            "build",
            "--manifest-path=detect-feature/Cargo.toml",
            "-Zbuild-std",
            "--target",
            triple,
        ])
        .env("PATH", std::env::var("PATH").unwrap())
        .env("FEATURE", target_feature)
        .output()
        .unwrap()
        .status
        .success()
}

fn get_all_cpus(triple: &str) -> Vec<Cpu> {
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "target-cpus", "--target", triple])
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    // Add listed CPUS
    let mut cpus = Vec::new();
    for line in std::str::from_utf8(&output.stdout).unwrap().lines().skip(1) {
        let cpu = line.trim().split(' ').next().unwrap().trim().to_string();
        if cpu.starts_with("native") {
            continue;
        }
        if cpu.is_empty() {
            break;
        }

        let features = get_features(triple, "", &cpu);

        cpus.push(Cpu { cpu, features })
    }

    cpus
}

fn get_all_features(triple: &str) -> Vec<Feature> {
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "target-features", "--target", triple])
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    let disable_default_features = get_features(triple, "", "")
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
    let make_feature = |feature: &str, description: &str| {
        let mut implies = get_features(
            triple,
            &format!(
                "{},+{}{}",
                disable_default_features, feature, required_arch_features
            ),
            "",
        );
        implies.sort();
        implies.retain(|f| f != &feature);

        let runtime = detect_at_runtime(triple, feature);

        Feature {
            feature: feature.to_string(),
            description: description.to_string(),
            implies,
            runtime,
        }
    };

    // Add listed features
    for line in std::str::from_utf8(&output.stdout).unwrap().lines().skip(1) {
        let mut split = line.split(" - ");
        let feature = split.next().unwrap().trim();
        if feature.is_empty() {
            break;
        } else {
            let description = split.next().unwrap().trim();
            features.push(make_feature(feature, description));
        }
    }

    features
}

fn main() {
    let arches = [
        ("Arm", "arm-unknown-linux-gnueabihf"),
        ("AArch64", "aarch64-unknown-linux-gnu"),
        ("Bpf", "bpfeb-unknown-none"),
        ("Hexagon", "hexagon-unknown-linux-musl"),
        ("Mips", "mips64-unknown-linux-gnuabi64"),
        ("PowerPC", "powerpc64-unknown-linux-gnu"),
        ("RiscV", "riscv64gc-unknown-linux-gnu"),
        ("Wasm", "wasm32-unknown-unknown"),
        ("X86", "x86_64-unknown-linux-gnu"),
    ];

    let mut rust_version =
        std::fs::File::create(std::env::current_dir().unwrap().join("rustc-version.txt")).unwrap();
    writeln!(rust_version, "{}", get_rustc_version()).unwrap();

    let mut features =
        std::fs::File::create(std::env::current_dir().unwrap().join("target-features.txt"))
            .unwrap();
    for (arch, triple) in arches {
        println!("reading arch: {arch}");
        for Feature {
            feature,
            description,
            implies,
            runtime,
        } in get_all_features(triple)
        {
            writeln!(features, "feature = {feature}").unwrap();
            writeln!(features, "arch = {arch}").unwrap();
            writeln!(features, "implies = {}", implies.join(" ")).unwrap();
            writeln!(features, "description = {description}").unwrap();
            writeln!(features, "runtime = {runtime}").unwrap();
            writeln!(features, "").unwrap();
        }
    }

    let mut cpus =
        std::fs::File::create(std::env::current_dir().unwrap().join("target-cpus.txt")).unwrap();
    for (arch, triple) in arches {
        println!("reading CPUs for arch: {arch}");
        for Cpu { cpu, features } in get_all_cpus(triple) {
            writeln!(cpus, "cpu = {cpu}").unwrap();
            writeln!(cpus, "arch = {arch}").unwrap();
            writeln!(cpus, "features = {}", features.join(" ")).unwrap();
            writeln!(cpus, "").unwrap();
        }
    }
}
