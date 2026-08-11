#![feature(rustc_private)]

mod architectures;
mod documentation;
mod platform;
mod runtime_detection;

use std::{
    io::Write,
    process::{Command, Stdio},
};

use architectures::ARCHITECTURES;
use platform::{Cpu, Feature};

fn rustc_version() -> String {
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

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let mut rust_version = std::fs::File::create(current_dir.join("rustc-version.txt")).unwrap();
    writeln!(rust_version, "{}", rustc_version()).unwrap();

    let mut target_features =
        std::fs::File::create(current_dir.join("target-features.txt")).unwrap();
    let mut target_cpus = std::fs::File::create(current_dir.join("target-cpus.txt")).unwrap();
    let mut target_docs = std::fs::File::create(current_dir.join("docs.rs")).unwrap();

    for architecture in ARCHITECTURES {
        println!("reading arch: {}", architecture.name);
        let features = platform::features(architecture);
        for Feature {
            feature,
            description,
            implies,
            runtime,
        } in &features
        {
            writeln!(target_features, "feature = {feature}").unwrap();
            writeln!(target_features, "arch = {}", architecture.name).unwrap();
            writeln!(target_features, "implies = {}", implies.join(" ")).unwrap();
            writeln!(target_features, "description = {description}").unwrap();
            writeln!(target_features, "runtime = {runtime}").unwrap();
            writeln!(target_features).unwrap();
        }

        println!("reading CPUs for arch: {}", architecture.name);
        let cpus = platform::cpus(architecture);
        for Cpu { cpu, features } in &cpus {
            writeln!(target_cpus, "cpu = {cpu}").unwrap();
            writeln!(target_cpus, "arch = {}", architecture.name).unwrap();
            writeln!(target_cpus, "features = {}", features.join(" ")).unwrap();
            writeln!(target_cpus).unwrap();
        }

        documentation::write(&mut target_docs, architecture.name, &features, &cpus).unwrap();
    }
}
