#![feature(rustc_private)]

mod architectures;
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
    let mut rust_version =
        std::fs::File::create(std::env::current_dir().unwrap().join("rustc-version.txt")).unwrap();
    writeln!(rust_version, "{}", rustc_version()).unwrap();

    let mut features =
        std::fs::File::create(std::env::current_dir().unwrap().join("target-features.txt"))
            .unwrap();
    for architecture in ARCHITECTURES {
        println!("reading arch: {}", architecture.name);
        for Feature {
            feature,
            description,
            implies,
            runtime,
        } in platform::features(architecture)
        {
            writeln!(features, "feature = {feature}").unwrap();
            writeln!(features, "arch = {}", architecture.name).unwrap();
            writeln!(features, "implies = {}", implies.join(" ")).unwrap();
            writeln!(features, "description = {description}").unwrap();
            writeln!(features, "runtime = {runtime}").unwrap();
            writeln!(features).unwrap();
        }
    }

    let mut cpus =
        std::fs::File::create(std::env::current_dir().unwrap().join("target-cpus.txt")).unwrap();
    for architecture in ARCHITECTURES {
        println!("reading CPUs for arch: {}", architecture.name);
        for Cpu { cpu, features } in platform::cpus(architecture) {
            writeln!(cpus, "cpu = {cpu}").unwrap();
            writeln!(cpus, "arch = {}", architecture.name).unwrap();
            writeln!(cpus, "features = {}", features.join(" ")).unwrap();
            writeln!(cpus).unwrap();
        }
    }
}
