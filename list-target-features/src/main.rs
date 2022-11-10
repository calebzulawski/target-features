use std::{
    collections::hash_set::HashSet,
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

fn get_all_features(triple: &str) -> HashSet<String> {
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "target-features", "--target", triple])
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    let mut features = HashSet::new();
    for line in std::str::from_utf8(&output.stdout).unwrap().lines().skip(1) {
        let feature = line.split(" - ").next().unwrap().trim();
        if feature.is_empty() {
            break;
        } else {
            features.insert(feature.to_string());
        }
    }
    features
}

fn get_features(triple: &str, target_feature: &str) -> HashSet<String> {
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

fn get_implied_features(
    triple: &str,
    possible_features: HashSet<String>,
) -> Vec<(String, Vec<String>)> {
    let disable_default_features = get_features(triple, "")
        .iter()
        .map(|s| format!("-{}", s))
        .collect::<Vec<String>>()
        .join(",");

    // avoid LLVM issue
    let arch_features = if triple.starts_with("mips") {
        ",+fp64"
    } else {
        ""
    };

    let mut implied_features = possible_features
        .iter()
        .map(|feature| {
            let mut implied_features = get_features(
                triple,
                &format!("{},+{}{}", disable_default_features, feature, arch_features),
            )
            .intersection(&possible_features)
            .cloned()
            .collect::<Vec<_>>();
            implied_features.sort();
            (feature.to_string(), implied_features)
        })
        .collect::<Vec<_>>();
    implied_features.sort();
    implied_features
}

fn main() {
    let arches = [
        ("arm", "arm-unknown-linux-gnueabihf"),
        ("aarch64", "aarch64-unknown-none"),
        ("bpf", "bpfeb-unknown-none"),
        ("hexagon", "hexagon-unknown-linux-musl"),
        ("mips", "mips64-unknown-linux-gnuabi64"),
        ("powerpc", "powerpc64-unknown-linux-gnu"),
        ("riscv", "riscv32i-unknown-none-elf"),
        ("wasm", "wasm32-unknown-unknown"),
        ("x86", "x86_64-unknown-none"),
    ];

    let mut file =
        std::fs::File::create(std::env::current_dir().unwrap().join("target-features.txt"))
            .unwrap();

    writeln!(file, "{}", get_rustc_version()).unwrap();
    for (arch, triple) in arches {
        let features = get_all_features(triple);
        let features = get_implied_features(triple, features);
        for (feature, implied) in features {
            writeln!(file, "{} {}: {}", arch, feature, implied.join(" ")).unwrap();
        }
    }
}
