use std::{fs, path::PathBuf, process::Command};

const PROBE_SOURCE: &str = r#"---cargo
[package]
edition = "2024"
---

#![allow(unused_features)]
#![cfg_attr(target_arch = "arm", feature(stdarch_arm_feature_detection))]
#![cfg_attr(
    any(target_arch = "aarch64", target_arch = "arm64ec"),
    feature(stdarch_aarch64_feature_detection)
)]
#![cfg_attr(
    any(target_arch = "loongarch32", target_arch = "loongarch64"),
    feature(stdarch_loongarch_feature_detection)
)]
#![cfg_attr(
    any(target_arch = "mips", target_arch = "mips64"),
    feature(stdarch_mips_feature_detection)
)]
#![cfg_attr(
    any(target_arch = "powerpc", target_arch = "powerpc64"),
    feature(stdarch_powerpc_feature_detection)
)]
#![cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    feature(stdarch_riscv_feature_detection)
)]
#![cfg_attr(target_arch = "s390x", feature(s390x_target_feature))]
#![cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature(
        apx_target_feature,
        avx10_target_feature,
        clflushopt_target_feature,
        movrs_target_feature,
        x86_amx_intrinsics,
        xop_target_feature
    )
)]

fn main() {
    let _ = std::arch::is_{arch}_feature_detected!({target_feature});
}
"#;

#[derive(Copy, Clone)]
pub(crate) enum RuntimeDetection {
    Unsupported,
    Macro(&'static str),
    MacroOn {
        arch: &'static str,
        triple: &'static str,
    },
}

fn is_expected_error(stderr: &str, arch: &str, target_feature: &str) -> bool {
    let unknown_feature = format!("error: unknown {arch} target feature: {target_feature}");
    let undetectable_feature =
        format!("error: \"{target_feature}\" feature cannot be detected at run-time");
    let compile_failure =
        "error: could not compile `detect-feature` (bin \"detect-feature\") due to 1 previous error";
    let mut errors = stderr.lines().filter(|line| line.starts_with("error"));
    matches!(errors.next(), Some(error) if error == unknown_feature || error == undetectable_feature)
        && errors.next() == Some(compile_failure)
        && errors.next().is_none()
}

fn write_probe(arch: &str, target_feature: &str) -> (PathBuf, PathBuf) {
    let probe_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("runtime-detection");
    fs::create_dir_all(&probe_dir).unwrap();

    let probe = probe_dir.join("detect-feature.rs");
    fs::write(
        &probe,
        PROBE_SOURCE
            .replace("{arch}", arch)
            .replace("{target_feature}", &format!("{target_feature:?}")),
    )
    .unwrap();

    (probe, probe_dir.join("target"))
}

pub(crate) fn detect(detection: RuntimeDetection, triple: &str, target_feature: &str) -> bool {
    let (triple, arch) = match detection {
        RuntimeDetection::Unsupported => return false,
        RuntimeDetection::Macro(arch) => (triple, arch),
        RuntimeDetection::MacroOn { arch, triple } => (triple, arch),
    };
    let (probe, target_dir) = write_probe(arch, target_feature);

    let output = Command::new("cargo")
        .args([
            "+nightly",
            "check",
            "--release",
            "--quiet",
            "--color=never",
            "--manifest-path",
            probe.to_str().unwrap(),
            "-Zscript",
            "-Zbuild-std",
            "--target",
            triple,
            "--target-dir",
            target_dir.to_str().unwrap(),
        ])
        .env("PATH", std::env::var("PATH").unwrap())
        .output()
        .unwrap();

    if output.status.success() {
        true
    } else {
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        if is_expected_error(stderr, arch, target_feature) {
            false
        } else {
            panic!(
                "unexpected runtime-detection failure for {triple} feature {target_feature}\n\nstdout:\n{}\nstderr:\n{}",
                std::str::from_utf8(&output.stdout).unwrap(),
                stderr,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_expected_error;

    #[test]
    fn recognizes_expected_errors() {
        let stderr = "error: unknown x86 target feature: example\n\
                       error: could not compile `detect-feature` (bin \"detect-feature\") due to 1 previous error\n";
        assert!(is_expected_error(stderr, "x86", "example"));

        let stderr = "error: \"example\" feature cannot be detected at run-time\n\
                       error: could not compile `detect-feature` (bin \"detect-feature\") due to 1 previous error\n";
        assert!(is_expected_error(stderr, "x86", "example"));
    }

    #[test]
    fn rejects_other_errors() {
        let stderr = "error[E0658]: use of unstable library feature\n\
                       error: could not compile `detect-feature` (bin \"detect-feature\") due to 1 previous error\n";
        assert!(!is_expected_error(stderr, "x86", "example"));

        let stderr = "error: unknown x86 target feature: example\n\
                       error: another compiler error\n\
                       error: could not compile `detect-feature` (bin \"detect-feature\") due to 2 previous errors\n";
        assert!(!is_expected_error(stderr, "x86", "example"));

        let stderr = "error: unknown x86 target feature: example\n\
                       error: could not compile `detect-feature` (lib) due to 1 previous error\n";
        assert!(!is_expected_error(stderr, "x86", "example"));
    }
}
