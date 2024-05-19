//! # Target features
//! A database of target features available to the Rust compiler.
//!
#![doc = include_str!(concat!(env!("OUT_DIR"), "/generated.md"))]
#![no_std]

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// List of features available for each architecture.
pub mod docs {
    include!(concat!(env!("OUT_DIR"), "/docs.rs"));
}

mod simd;
pub use simd::*;

const fn str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();

    if a.len() != b.len() {
        return false;
    }

    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

macro_rules! architectures {
    { $($str:literal: $enum:ident,)* } => {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        #[non_exhaustive]
        pub enum Architecture {
            /// A target which is not supported by this crate, and has no features.
            Unsupported,
            $($enum),*
        }

        impl Architecture {
            /// Create a new `Architecture` from its name.
            pub const fn from_str(architecture: &str) -> Self {
                $(
                if str_eq(architecture, $str) {
                    return Self::$enum
                }
                )*
                Self::Unsupported
            }

            /// Return the name of this architecture.
            pub const fn as_str(&self) -> Option<&'static str> {
                $(
                if let Self::$enum = self {
                    return Some($str)
                }
                )*
                None
            }
        }
    }
}

impl Architecture {
    /// Is arm or aarch64
    pub const fn is_arm_family(&self) -> bool {
        matches!(self, Self::Arm | Self::AArch64)
    }

    /// Is mips or mips64
    pub const fn is_mips_family(&self) -> bool {
        matches!(self, Self::Mips | Self::Mips64)
    }

    /// Is powerpc or powerpc64
    pub const fn is_powerpc_family(&self) -> bool {
        matches!(self, Self::PowerPC | Self::PowerPC64)
    }

    /// Is riscv32 or riscv64
    pub const fn is_riscv_family(&self) -> bool {
        matches!(self, Self::RiscV32 | Self::RiscV64)
    }

    /// Is wasm32 or wasm64
    pub const fn is_wasm_family(&self) -> bool {
        matches!(self, Self::Wasm32 | Self::Wasm64)
    }

    /// Is x86 or x86_64
    pub const fn is_x86_family(&self) -> bool {
        matches!(self, Self::X86 | Self::X86_64)
    }
}

architectures! {
    "arm": Arm,
    "aarch64": AArch64,
    "bpf": Bpf,
    "hexagon": Hexagon,
    "mips": Mips,
    "mips64": Mips64,
    "loongarch64": LoongArch64,
    "nvptx": Nvptx,
    "nvptx64": Nvptx64,
    "powerpc": PowerPC,
    "powerpc64": PowerPC64,
    "riscv32": RiscV32,
    "riscv64": RiscV64,
    "s390x": S390X,
    "sparc": Sparc,
    "sparc64": Sparc64,
    "wasm32": Wasm32,
    "wasm64": Wasm64,
    "x86": X86,
    "x86_64": X86_64,
}

/// Returned by [`Feature::new`] when the requested feature can't be found.
#[derive(Copy, Clone, Debug)]
pub struct UnknownFeature;

impl core::fmt::Display for UnknownFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "unknown target feature")
    }
}

/// Returned by [`Target::from_cpu`] when the requested CPU can't be found.
#[derive(Copy, Clone, Debug)]
pub struct UnknownCpu;

impl core::fmt::Display for UnknownCpu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "unknown target CPU")
    }
}

/// A target feature.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Feature(usize);

impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        f.debug_struct("Feature")
            .field("architecture", &self.architecture())
            .field("name", &self.name())
            .finish()
    }
}

impl Feature {
    /// Look up a feature.
    pub const fn new(architecture: Architecture, feature: &str) -> Result<Self, UnknownFeature> {
        let mut i = 0;
        while i < FEATURES.len() {
            if (architecture as u8) == (FEATURES[i].0 as u8) && str_eq(feature, FEATURES[i].1) {
                return Ok(Self(i));
            }
            i += 1;
        }

        Err(UnknownFeature)
    }

    /// Get the name of the feature.
    pub const fn name(&self) -> &'static str {
        FEATURES[self.0].1
    }

    /// Get the architecture this feature is for.
    pub const fn architecture(&self) -> Architecture {
        FEATURES[self.0].0
    }

    /// Get a human-readable description of the feature.
    pub const fn description(&self) -> &'static str {
        FEATURES[self.0].2
    }

    /// Return all features which are implied by the existence of this feature.
    ///
    /// For example, "avx2" implies the existence of "avx" on x86 architectures.
    pub const fn implies(&self) -> &'static [Feature] {
        FEATURES[self.0].3
    }

    /// Whether or not this feature can be detected at runtime.
    pub const fn can_detect_at_runtime(&self) -> bool {
        FEATURES[self.0].4
    }
}

/// Iterator returned by [`Target::features`].
pub struct FeaturesIter {
    target: Target,
    index: usize,
}

impl Iterator for FeaturesIter {
    type Item = Feature;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.target.features.len() {
            let feature = if self.target.features[self.index] {
                Some(Feature(self.index))
            } else {
                None
            };
            self.index += 1;
            if feature.is_some() {
                return feature;
            }
        }
        None
    }
}

impl core::fmt::Debug for Target {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        struct FeaturesHelper(Target);
        impl core::fmt::Debug for FeaturesHelper {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
                f.debug_list().entries(self.0.features()).finish()
            }
        }

        f.debug_struct("Target")
            .field("architecture", &self.architecture())
            .field("features", &FeaturesHelper(*self))
            .finish()
    }
}

/// A target architecture with optional features.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Target {
    architecture: Architecture,
    features: [bool; FEATURES.len()],
}

impl Target {
    /// Create a target with no specified features.
    pub const fn new(architecture: Architecture) -> Self {
        Self {
            architecture,
            features: [false; FEATURES.len()],
        }
    }

    /// Create a target based on a particular CPU.
    pub const fn from_cpu(architecture: Architecture, cpu: &str) -> Result<Self, UnknownCpu> {
        let mut target = Self::new(architecture);
        let mut i = 0;
        while i < CPUS.len() {
            if architecture as u8 == CPUS[i].0 as u8 && str_eq(cpu, CPUS[i].1) {
                let mut j = 0;
                while j < CPUS[i].2.len() {
                    target = target.with_feature(CPUS[i].2[j]);
                    j += 1;
                }
                return Ok(target);
            }
            i += 1;
        }
        Err(UnknownCpu)
    }

    /// Returns the target architecture.
    pub const fn architecture(&self) -> Architecture {
        self.architecture
    }

    /// Returns an iterator over the features.
    pub const fn features(&self) -> FeaturesIter {
        FeaturesIter {
            target: *self,
            index: 0,
        }
    }

    /// Returns whether the target supports the specified feature.
    pub const fn supports_feature(&self, feature: Feature) -> bool {
        self.features[feature.0]
    }

    /// Returns whether the target supports the specified feature.
    ///
    /// # Panics
    /// Panics if the feature doesn't belong to the target architecture.
    pub const fn supports_feature_str(&self, feature: &str) -> bool {
        if let Ok(feature) = Feature::new(self.architecture, feature) {
            self.supports_feature(feature)
        } else {
            panic!("unknown feature");
        }
    }

    /// Add a feature to the target.
    ///
    /// # Panics
    /// Panics if the feature doesn't belong to the target architecture.
    pub const fn with_feature(mut self, feature: Feature) -> Self {
        assert!(feature.architecture() as u8 == self.architecture as u8);
        self.features[feature.0] = true;

        let mut i = 0;
        let implies = feature.implies();
        while i < implies.len() {
            self.features[implies[i].0] = true;
            i += 1;
        }

        self
    }

    /// Add a feature to the target.
    ///
    /// # Panics
    /// Panics if the requested feature name doesn't exist for the target architecture.
    pub const fn with_feature_str(self, feature: &str) -> Self {
        if let Ok(feature) = Feature::new(self.architecture, feature) {
            self.with_feature(feature)
        } else {
            panic!("unknown feature");
        }
    }
}
