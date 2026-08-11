//! Types and constants for working with target features.
//!
//! Target feature constants are provided in a module named for the target
//! architecture, such as `x86_64` or `aarch64`. Each constant includes the
//! target feature it names and any implicitly enabled target features.
//!
//! ```
//! # #[cfg(target_arch = "x86_64")] {
//! use target_features::{TargetFeatures, x86_64::{AVX, BMI2, FMA}};
//!
//! const REQUIRED: TargetFeatures = AVX.with(FMA).with(BMI2);
//! assert!(REQUIRED.contains(AVX));
//! # }
//! ```
//!
#![doc = include_str!("../rustc-version.md")]
#![no_std]

#[allow(unknown_lints, unexpected_cfgs)]
mod database;
pub use database::*;

#[allow(unknown_lints, unexpected_cfgs)]
mod simd;
pub use simd::*;

/// A set of target features.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TargetFeatures {
    bits: [u64; database::FEATURE_WORDS],
}

impl TargetFeatures {
    /// Returns a set containing no target features.
    pub const fn empty() -> Self {
        Self {
            bits: [0; database::FEATURE_WORDS],
        }
    }

    pub(crate) const fn with_bit(mut self, index: usize) -> Self {
        self.bits[index / 64] |= 1 << (index % 64);
        self
    }

    /// Returns the target features enabled at compile time.
    ///
    /// This includes features enabled by the target specification, target CPU,
    /// and `-C target-feature` compiler options.
    pub const fn enabled_for_target() -> Self {
        database::enabled_for_target()
    }

    /// Returns whether `self` contains every target feature in `required`.
    pub const fn contains(self, required: Self) -> bool {
        let mut i = 0;
        while i < self.bits.len() {
            if self.bits[i] & required.bits[i] != required.bits[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Returns `self` with all target features in `additional`.
    #[must_use]
    pub const fn with(mut self, additional: Self) -> Self {
        let mut i = 0;
        while i < self.bits.len() {
            self.bits[i] |= additional.bits[i];
            i += 1;
        }
        self
    }
}

impl core::fmt::Debug for TargetFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        struct Basis<'a>(&'a TargetFeatures);

        impl core::fmt::Debug for Basis<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let all = database::features();
                let mut list = f.debug_list();

                for (index, feature) in all.iter().enumerate() {
                    if !self.0.contains(feature.features) {
                        continue;
                    }

                    let mut redundant = false;
                    for (other_index, other) in all.iter().enumerate() {
                        if !self.0.contains(other.features)
                            || !other.features.contains(feature.features)
                        {
                            continue;
                        }

                        // Do not display a feature that another feature in the
                        // set implicitly enables. Equal sets are aliases or
                        // tied features; display the first name.
                        if other.features != feature.features || other_index < index {
                            redundant = true;
                            break;
                        }
                    }

                    if !redundant {
                        list.entry(&feature.name);
                    }
                }

                list.finish()
            }
        }

        f.debug_tuple("TargetFeatures").field(&Basis(self)).finish()
    }
}
