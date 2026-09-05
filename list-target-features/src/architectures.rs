pub(crate) struct ArchitectureSpec {
    pub(crate) name: &'static str,
    pub(crate) triple: &'static str,
}

impl ArchitectureSpec {
    const fn new(name: &'static str, triple: &'static str) -> Self {
        Self { name, triple }
    }
}

#[rustfmt::skip]
pub(crate) const ARCHITECTURES: &[ArchitectureSpec] = &[
    ArchitectureSpec::new("Arm",         "arm-unknown-linux-gnueabihf"),
    ArchitectureSpec::new("AArch64",     "aarch64-unknown-linux-gnu"),
    ArchitectureSpec::new("Arm64EC",     "arm64ec-pc-windows-msvc"),
    ArchitectureSpec::new("Bpf",         "bpfeb-unknown-none"),
    ArchitectureSpec::new("Hexagon",     "hexagon-unknown-linux-musl"),
    ArchitectureSpec::new("Mips",        "mips-unknown-linux-gnu"),
    ArchitectureSpec::new("Mips64",      "mips64-unknown-linux-gnuabi64"),
    ArchitectureSpec::new("LoongArch32", "loongarch32-unknown-none"),
    ArchitectureSpec::new("LoongArch64", "loongarch64-unknown-linux-gnu"),
    ArchitectureSpec::new("Nvptx64",     "nvptx64-nvidia-cuda"),
    ArchitectureSpec::new("PowerPC",     "powerpc-unknown-linux-gnu"),
    ArchitectureSpec::new("PowerPC64",   "powerpc64-unknown-linux-gnu"),
    ArchitectureSpec::new("RiscV32",     "riscv32gc-unknown-linux-gnu"),
    ArchitectureSpec::new("RiscV64",     "riscv64gc-unknown-linux-gnu"),
    ArchitectureSpec::new("S390X",       "s390x-unknown-linux-gnu"),
    ArchitectureSpec::new("Sparc",       "sparc-unknown-linux-gnu"),
    ArchitectureSpec::new("Sparc64",     "sparc64-unknown-linux-gnu"),
    ArchitectureSpec::new("Wasm32",      "wasm32-unknown-unknown"),
    ArchitectureSpec::new("Wasm64",      "wasm64-unknown-unknown"),
    ArchitectureSpec::new("X86",         "i586-unknown-linux-gnu"),
    ArchitectureSpec::new("X86_64",      "x86_64-unknown-linux-gnu"),
];
