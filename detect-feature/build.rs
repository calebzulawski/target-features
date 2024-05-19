fn main() {
    use std::io::Write;
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut lib = std::fs::File::create(&format!("{}/lib.rs", out_dir)).unwrap();

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let arch = match arch.as_str() {
        "x86_64" => "x86",
        "riscv32" => "riscv",
        "riscv64" => "riscv",
        arch => arch,
    };

    write!(
        lib,
        r#"pub fn check() -> bool {{ std::arch::is_{}_feature_detected!("{}") }}"#,
        arch,
        std::env::var("FEATURE").unwrap()
    )
    .unwrap();

    println!("cargo:rerun-if-env-changed=FEATURE");
}
