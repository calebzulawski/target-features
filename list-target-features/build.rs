use std::{env, path::Path, process::Command};

fn main() {
    let rustc = env::var_os("RUSTC").expect("Cargo did not set RUSTC");
    let output = Command::new(rustc)
        .args(["--print", "sysroot"])
        .output()
        .expect("failed to query the rustc sysroot");
    assert!(output.status.success(), "failed to query the rustc sysroot");

    let sysroot = std::str::from_utf8(&output.stdout)
        .expect("rustc returned a non-UTF-8 sysroot")
        .trim();
    let libdir = Path::new(sysroot).join("lib");

    println!("cargo:rustc-link-search=native={}", libdir.display());
    #[cfg(unix)]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libdir.display());
    println!("cargo:rerun-if-env-changed=RUSTC");
}
