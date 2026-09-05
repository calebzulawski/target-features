extern crate rustc_driver;
extern crate rustc_target;

use std::process::{Command, Stdio};

use rustc_target::spec::{Target, TargetTuple};

use crate::architectures::ArchitectureSpec;

pub(crate) struct Feature {
    pub(crate) feature: String,
    pub(crate) description: String,
    pub(crate) implies: Vec<String>,
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

pub(crate) fn features(architecture: &ArchitectureSpec) -> Vec<Feature> {
    let triple = architecture.triple;
    let listed_features = listed_features(triple);
    let target = Target::expect_builtin(&TargetTuple::from_tuple(triple));
    let target_features = target.rust_target_features_map();

    let make_feature = |feature: &str, description: &str| {
        let mut implies = Vec::new();
        if target_features.contains_key(feature) {
            let mut closure = target.implied_target_features(feature, &target_features);

            // Tied features must always be enabled and disabled together. Add
            // the complete implication closure of every member in a matching
            // tied group, repeating in case groups ever overlap.
            loop {
                let old_len = closure.len();
                for tied in target.tied_target_features() {
                    if tied.iter().any(|tied| closure.contains(tied)) {
                        for tied in *tied {
                            closure.extend(target.implied_target_features(tied, &target_features));
                        }
                    }
                }
                if closure.len() == old_len {
                    break;
                }
            }

            implies = closure
                .into_iter()
                .filter(|implied| {
                    *implied != feature
                        && listed_features.iter().any(|(listed, _)| listed == *implied)
                })
                .map(ToString::to_string)
                .collect();
        }
        implies.sort();

        Feature {
            feature: feature.to_string(),
            description: description.to_string(),
            implies,
        }
    };

    listed_features
        .iter()
        .map(|(feature, description)| make_feature(feature, description))
        .collect()
}
