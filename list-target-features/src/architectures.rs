use crate::runtime_detection::RuntimeDetection;
use crate::runtime_detection::RuntimeDetection::{Macro, MacroOn, Unsupported};

pub(crate) struct ArchitectureSpec {
    pub(crate) name: &'static str,
    pub(crate) triple: &'static str,
    pub(crate) runtime_detection: RuntimeDetection,
}

impl ArchitectureSpec {
    const fn new(
        name: &'static str,
        triple: &'static str,
        runtime_detection: RuntimeDetection,
    ) -> Self {
        Self {
            name,
            triple,
            runtime_detection,
        }
    }
}

#[rustfmt::skip]
pub(crate) const ARCHITECTURES: &[ArchitectureSpec] = &[
    ArchitectureSpec::new("Arm",         "arm-unknown-linux-gnueabihf",         Macro("arm")),
    ArchitectureSpec::new("AArch64",     "aarch64-unknown-linux-gnu",           Macro("aarch64")),
    ArchitectureSpec::new("Arm64EC",     "arm64ec-pc-windows-msvc",             Macro("aarch64")),
    ArchitectureSpec::new("Bpf",         "bpfeb-unknown-none",                  Unsupported),
    ArchitectureSpec::new("Hexagon",     "hexagon-unknown-linux-musl",          Unsupported),
    ArchitectureSpec::new("Mips",        "mips-unknown-linux-gnu",              Macro("mips")),
    ArchitectureSpec::new("Mips64",      "mips64-unknown-linux-gnuabi64",       Macro("mips64")),
    ArchitectureSpec::new("LoongArch32", "loongarch32-unknown-none",            MacroOn { arch: "loongarch", triple: "loongarch64-unknown-linux-gnu" }),
    ArchitectureSpec::new("LoongArch64", "loongarch64-unknown-linux-gnu",       Macro("loongarch")),
    ArchitectureSpec::new("Nvptx64",     "nvptx64-nvidia-cuda",                 Unsupported),
    ArchitectureSpec::new("PowerPC",     "powerpc-unknown-linux-gnu",           Macro("powerpc")),
    ArchitectureSpec::new("PowerPC64",   "powerpc64-unknown-linux-gnu",         Macro("powerpc64")),
    ArchitectureSpec::new("RiscV32",     "riscv32gc-unknown-linux-gnu",         Macro("riscv")),
    ArchitectureSpec::new("RiscV64",     "riscv64gc-unknown-linux-gnu",         Macro("riscv")),
    ArchitectureSpec::new("S390X",       "s390x-unknown-linux-gnu",             Macro("s390x")),
    ArchitectureSpec::new("Sparc",       "sparc-unknown-linux-gnu",             Unsupported),
    ArchitectureSpec::new("Sparc64",     "sparc64-unknown-linux-gnu",           Unsupported),
    ArchitectureSpec::new("Wasm32",      "wasm32-unknown-unknown",              Unsupported),
    ArchitectureSpec::new("Wasm64",      "wasm64-unknown-unknown",              Unsupported),
    ArchitectureSpec::new("X86",         "i586-unknown-linux-gnu",              Macro("x86")),
    ArchitectureSpec::new("X86_64",      "x86_64-unknown-linux-gnu",            Macro("x86")),
];
