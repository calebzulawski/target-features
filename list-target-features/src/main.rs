#![feature(rustc_private)]

mod architectures;
mod platform;
mod templates;

use std::{
    error::Error,
    process::{Command, Stdio},
};

use architectures::ARCHITECTURES;
use platform::Feature;

pub(crate) struct ArchitectureData {
    spec: &'static architectures::ArchitectureSpec,
    features: Vec<Feature>,
}

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

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir().unwrap();
    let mut architectures = Vec::new();

    for architecture in ARCHITECTURES {
        println!("reading arch: {}", architecture.name);
        let features = platform::features(architecture);
        architectures.push(ArchitectureData {
            spec: architecture,
            features,
        });
    }

    let outputs = templates::render(&architectures)?;
    std::fs::write(
        current_dir.join("rustc-version.md"),
        format!("Generated with {}.\n", rustc_version()),
    )?;
    std::fs::write(current_dir.join("database.rs"), outputs.database)?;
    Ok(())
}
