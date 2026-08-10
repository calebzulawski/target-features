use crate::Architecture;

#[doc(hidden)]
pub enum SimdTypeImpl {
    Float32,
    Float64,
    Other,
}

/// Types which can be SIMD vector elements.
pub trait SimdType {
    #[doc(hidden)]
    const IMPL: SimdTypeImpl;
}

impl SimdType for u8 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for u16 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for u32 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for u64 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for usize {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for i8 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for i16 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for i32 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for i64 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for isize {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl SimdType for f32 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Float32;
}

impl SimdType for f64 {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Float64;
}

impl<T> SimdType for *const T {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl<T> SimdType for *mut T {
    const IMPL: SimdTypeImpl = SimdTypeImpl::Other;
}

impl crate::Target {
    /// Returns a suggested number of elements for a SIMD vector of the provided type.
    ///
    /// The returned value is an approximation and not necessarily indicative of the
    /// optimal vector width.  A few caveats:
    /// * Every instruction set is different, and this function doesn't take into account any
    ///   particular operations--it's just a guess, and should be accurate at least for basic arithmetic.
    /// * Variable length vector instruction sets (ARM SVE and RISC-V V) only return the minimum
    ///   vector length.
    ///
    /// The following features are accounted for:
    ///
    /// | Architecture        | Feature                                 | Width                              |
    /// | ------------------- | --------------------------------------- | ---------------------------------- |
    /// | `arm`               | `neon`                                  | 128-bit (excluding `f64`)          |
    /// | `arm`               | `mve`                                   | 128-bit (integers up to 32-bit)    |
    /// | `arm`               | `mve.fp`                                | 128-bit (`f32`)                    |
    /// | `aarch64`/`arm64ec` | `neon`                                  | 128-bit                            |
    /// | `hexagon`           | `hvx-length128b`                        | 1024-bit (only integers)           |
    /// | `hexagon`           | `hvx-length128b` and `hvx-ieee-fp`      | 1024-bit (`f32`)                   |
    /// | `hexagon`           | `hvx`                                   | 512-bit (only integers)            |
    /// | `loongarch{32,64}`  | `lasx`                                  | 256-bit                            |
    /// | `loongarch{32,64}`  | `lsx`                                   | 128-bit                            |
    /// | `mips{64}`          | `msa`                                   | 128-bit                            |
    /// | `powerpc{64}`       | `vsx`                                   | 128-bit                            |
    /// | `powerpc{64}`       | `altivec`                               | 128-bit (excluding `f64`)          |
    /// | `riscv{32,64}`      | `zve32*`                                | 32-bit minimum                     |
    /// | `riscv{32,64}`      | `zve64*`                                | 64-bit minimum                     |
    /// | `riscv{32,64}`      | `v`                                     | 128-bit minimum                    |
    /// | `riscv{32,64}`      | `zvl*b`                                 | specified minimum                  |
    /// | `s390x`             | `vector`                                | 128-bit                            |
    /// | `wasm{32,64}`       | `simd128`                               | 128-bit                            |
    /// | `x86{_64}`          | `avx512f`                               | 512-bit                            |
    /// | `x86{_64}`          | `avx2`                                  | 256-bit                            |
    /// | `x86{_64}`          | `avx`                                   | 256-bit (only floating point)      |
    /// | `x86{_64}`          | `sse2`                                  | 128-bit                            |
    /// | `x86{_64}`          | `sse`                                   | 128-bit (`f32` only)               |
    pub const fn suggested_simd_width<T: SimdType>(&self) -> Option<usize> {
        let is_f32 = T::IMPL as u8 == SimdTypeImpl::Float32 as u8;
        let is_f64 = T::IMPL as u8 == SimdTypeImpl::Float64 as u8;
        let is_integer = !is_f32 && !is_f64;
        let element_size = core::mem::size_of::<T>();

        let v128 = 16 / element_size;
        let v256 = 32 / element_size;
        let v512 = 64 / element_size;
        let v1024 = 128 / element_size;

        if let Architecture::Arm = self.architecture() {
            // Neon on arm doesn't support f64
            if (self.supports_feature_str("neon") && !is_f64)
                || (is_f32 && self.supports_feature_str("mve.fp"))
                || (is_integer && element_size <= 4 && self.supports_feature_str("mve"))
            {
                Some(v128)
            } else {
                None
            }
        } else if matches!(
            self.architecture(),
            Architecture::AArch64 | Architecture::Arm64EC
        ) {
            if self.supports_feature_str("neon") {
                Some(v128)
            } else {
                None
            }
        } else if let Architecture::Hexagon = self.architecture() {
            if is_f64 {
                None
            } else if is_f32 {
                // General SIMD arithmetic requires IEEE semantics.
                if self.supports_feature_str("hvx-length128b")
                    && self.supports_feature_str("hvxv68")
                    && self.supports_feature_str("hvx-ieee-fp")
                {
                    Some(v1024)
                } else {
                    None
                }
            } else if self.supports_feature_str("hvx-length128b") {
                Some(v1024)
            } else if self.supports_feature_str("hvx") {
                Some(v512)
            } else {
                None
            }
        } else if self.architecture().is_loongarch_family() {
            if self.supports_feature_str("lasx") {
                Some(v256)
            } else if self.supports_feature_str("lsx") {
                Some(v128)
            } else {
                None
            }
        } else if self.architecture().is_mips_family() {
            if self.supports_feature_str("msa") {
                Some(v128)
            } else {
                None
            }
        } else if self.architecture().is_powerpc_family() {
            // Altivec without VSX doesn't support f64
            if self.supports_feature_str("vsx") || (self.supports_feature_str("altivec") && !is_f64)
            {
                Some(v128)
            } else {
                None
            }
        } else if self.architecture().is_riscv_family() {
            let supports_element = if is_f64 {
                self.supports_feature_str("zve64d")
            } else if is_f32 {
                self.supports_feature_str("zve32f")
            } else if element_size <= 4 {
                self.supports_feature_str("zve32x")
            } else if element_size <= 8 {
                self.supports_feature_str("zve64x")
            } else {
                false
            };

            if supports_element {
                let vector_bytes = if self.supports_feature_str("zvl65536b") {
                    8192
                } else if self.supports_feature_str("zvl32768b") {
                    4096
                } else if self.supports_feature_str("zvl16384b") {
                    2048
                } else if self.supports_feature_str("zvl8192b") {
                    1024
                } else if self.supports_feature_str("zvl4096b") {
                    512
                } else if self.supports_feature_str("zvl2048b") {
                    256
                } else if self.supports_feature_str("zvl1024b") {
                    128
                } else if self.supports_feature_str("zvl512b") {
                    64
                } else if self.supports_feature_str("zvl256b") {
                    32
                } else if self.supports_feature_str("zvl128b") {
                    16
                } else if self.supports_feature_str("zvl64b") {
                    8
                } else if self.supports_feature_str("zvl32b") {
                    4
                } else {
                    0
                };

                if vector_bytes == 0 {
                    None
                } else {
                    Some(vector_bytes / element_size)
                }
            } else {
                None
            }
        } else if let Architecture::S390X = self.architecture() {
            if self.supports_feature_str("vector") {
                Some(v128)
            } else {
                None
            }
        } else if self.architecture().is_wasm_family() {
            if self.supports_feature_str("simd128") {
                Some(v128)
            } else {
                None
            }
        } else if self.architecture().is_x86_family() {
            if self.supports_feature_str("avx512f") {
                Some(v512)
            } else if self.supports_feature_str("avx2")
                || (is_f32 || is_f64) && self.supports_feature_str("avx")
            {
                // AVX supports f32 and f64
                Some(v256)
            } else if self.supports_feature_str("sse2")
                || is_f32 && self.supports_feature_str("sse")
            {
                // SSE supports f32
                Some(v128)
            } else {
                None
            }
        } else {
            None
        }
    }
}
