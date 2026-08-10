extern crate rustc_driver;
extern crate rustc_target;

use std::process::{Command, Stdio};

use rustc_target::spec::{Target, TargetTuple};

use crate::{architectures::ArchitectureSpec, runtime_detection};

pub(crate) struct Cpu {
    pub(crate) cpu: String,
    pub(crate) features: Vec<String>,
}

pub(crate) struct Feature {
    pub(crate) feature: String,
    pub(crate) description: String,
    pub(crate) implies: Vec<String>,
    pub(crate) runtime: bool,
}

fn listed_features(triple: &str) -> Vec<(String, String)> {
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "target-features", "--target", triple])
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    let mut features = Vec::new();
    for line in std::str::from_utf8(&output.stdout).unwrap().lines().skip(1) {
        let mut split = line.split(" - ");
        let feature = split.next().unwrap().trim();
        if feature.is_empty() {
            break;
        }
        let description = split.next().unwrap().trim();
        features.push((feature.to_string(), description.to_string()));
    }
    features
}

fn cpu_features(
    triple: &str,
    target_cpu: &str,
    listed_features: &[(String, String)],
) -> Vec<String> {
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "cfg", "--target", triple])
        .arg(format!("-Ctarget-cpu={}", target_cpu))
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    let mut features = std::str::from_utf8(&output.stdout)
        .unwrap()
        .lines()
        .filter_map(|s| {
            s.strip_prefix("target_feature=\"")
                .and_then(|s| s.strip_suffix('"'))
                .map(ToString::to_string)
        })
        .collect::<Vec<_>>();
    features.retain(|feature| listed_features.iter().any(|(listed, _)| listed == feature));
    features
}

pub(crate) fn cpus(architecture: &ArchitectureSpec) -> Vec<Cpu> {
    let triple = architecture.triple;
    let listed_features = listed_features(triple);
    let output = Command::new("rustc")
        .args(["+nightly", "--print", "target-cpus", "--target", triple])
        .env("PATH", std::env::var("PATH").unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success());

    let mut cpus = Vec::new();
    for line in std::str::from_utf8(&output.stdout).unwrap().lines().skip(1) {
        let cpu = line.trim().split(' ').next().unwrap().trim().to_string();
        if cpu.starts_with("native") {
            continue;
        }
        if cpu.is_empty() {
            break;
        }

        if cpu == "mips5" {
            continue; // unsupported by LLVM
        }

        let features = cpu_features(triple, &cpu, &listed_features);

        cpus.push(Cpu { cpu, features })
    }

    cpus
}

pub(crate) fn features(architecture: &ArchitectureSpec) -> Vec<Feature> {
    let triple = architecture.triple;
    let listed_features = listed_features(triple);
    let target = Target::expect_builtin(&TargetTuple::from_tuple(triple));
    let target_features = target.rust_target_features_map();

    let make_feature = |feature: &str, description: &str| {
        let mut implies = if target_features.contains_key(feature) {
            target
                .implied_target_features(feature, &target_features)
                .into_iter()
                .filter(|implied| {
                    *implied != feature
                        && listed_features.iter().any(|(listed, _)| listed == *implied)
                })
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        implies.sort();

        let runtime = runtime_detection::detect(architecture.runtime_detection, triple, feature);

        Feature {
            feature: feature.to_string(),
            description: description.to_string(),
            implies,
            runtime,
        }
    };

    listed_features
        .iter()
        .map(|(feature, description)| make_feature(feature, description))
        .collect()
}
