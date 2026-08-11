#[cfg(target_arch = "aarch64")]
use crate::aarch64 as features;
#[cfg(target_arch = "arm")]
use crate::arm as features;
#[cfg(target_arch = "arm64ec")]
use crate::arm64ec as features;
#[cfg(target_arch = "hexagon")]
use crate::hexagon as features;
#[cfg(target_arch = "loongarch32")]
use crate::loongarch32 as features;
#[cfg(target_arch = "loongarch64")]
use crate::loongarch64 as features;
#[cfg(target_arch = "mips")]
use crate::mips as features;
#[cfg(target_arch = "mips64")]
use crate::mips64 as features;
#[cfg(target_arch = "powerpc")]
use crate::powerpc as features;
#[cfg(target_arch = "powerpc64")]
use crate::powerpc64 as features;
#[cfg(target_arch = "riscv32")]
use crate::riscv32 as features;
#[cfg(target_arch = "riscv64")]
use crate::riscv64 as features;
#[cfg(target_arch = "s390x")]
use crate::s390x as features;
#[cfg(target_arch = "wasm32")]
use crate::wasm32 as features;
#[cfg(target_arch = "wasm64")]
use crate::wasm64 as features;
#[cfg(target_arch = "x86")]
use crate::x86 as features;
#[cfg(target_arch = "x86_64")]
use crate::x86_64 as features;

mod sealed {
    pub trait SealedSimdElement {}
}

#[doc(hidden)]
pub enum SimdElementImpl {
    Float32,
    Float64,
    Other,
}

/// Types that can be elements of SIMD vectors.
pub trait SimdElement: sealed::SealedSimdElement {
    #[doc(hidden)]
    const IMPL: SimdElementImpl;
}

macro_rules! impl_simd_element {
    ($impl:ident: $($ty:ty),* $(,)?) => {
        $(
            impl sealed::SealedSimdElement for $ty {}
            impl SimdElement for $ty {
                const IMPL: SimdElementImpl = SimdElementImpl::$impl;
            }
        )*
    };
}

impl_simd_element!(Other: u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl_simd_element!(Float32: f32);
impl_simd_element!(Float64: f64);

impl<T> sealed::SealedSimdElement for *const T {}
impl<T> SimdElement for *const T {
    const IMPL: SimdElementImpl = SimdElementImpl::Other;
}

impl<T> sealed::SealedSimdElement for *mut T {}
impl<T> SimdElement for *mut T {
    const IMPL: SimdElementImpl = SimdElementImpl::Other;
}

impl crate::TargetFeatures {
    /// Returns a suggested number of elements for a SIMD vector of type `T`.
    ///
    /// The returned value is an approximation and not necessarily indicative of the
    /// optimal vector width.  A few caveats:
    /// * Every instruction set is different, and this function doesn't take into account any
    ///   particular operations--it's just a guess, and should be accurate at least for basic arithmetic.
    /// * Variable-length vector instruction sets (ARM SVE and RISC-V V) only return the minimum
    ///   vector length.
    #[allow(unused_variables)]
    pub const fn suggested_simd_width<T: SimdElement>(&self) -> Option<usize> {
        let is_f32 = T::IMPL as u8 == SimdElementImpl::Float32 as u8;
        let is_f64 = T::IMPL as u8 == SimdElementImpl::Float64 as u8;
        let is_integer = !is_f32 && !is_f64;
        let element_size = core::mem::size_of::<T>();

        let v128 = 16 / element_size;
        let v256 = 32 / element_size;
        let v512 = 64 / element_size;
        let v1024 = 128 / element_size;

        #[cfg(target_arch = "arm")]
        {
            return if (self.contains(features::NEON) && !is_f64)
                || (is_f32 && self.contains(features::MVE_FP))
                || (is_integer && element_size <= 4 && self.contains(features::MVE))
            {
                Some(v128)
            } else {
                None
            };
        }

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        {
            return if self.contains(features::NEON) {
                Some(v128)
            } else {
                None
            };
        }

        #[cfg(target_arch = "hexagon")]
        {
            return if is_f64 {
                None
            } else if is_f32 {
                if self.contains(features::HVX_LENGTH128B)
                    && self.contains(features::HVXV68)
                    && self.contains(features::HVX_IEEE_FP)
                {
                    Some(v1024)
                } else {
                    None
                }
            } else if self.contains(features::HVX_LENGTH128B) {
                Some(v1024)
            } else if self.contains(features::HVX) {
                Some(v512)
            } else {
                None
            };
        }

        #[cfg(any(target_arch = "loongarch32", target_arch = "loongarch64"))]
        {
            return if self.contains(features::LASX) {
                Some(v256)
            } else if self.contains(features::LSX) {
                Some(v128)
            } else {
                None
            };
        }

        #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
        {
            return if self.contains(features::MSA) {
                Some(v128)
            } else {
                None
            };
        }

        #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
        {
            return if self.contains(features::VSX) || (self.contains(features::ALTIVEC) && !is_f64)
            {
                Some(v128)
            } else {
                None
            };
        }

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        {
            let supports_element = if is_f64 {
                self.contains(features::ZVE64D)
            } else if is_f32 {
                self.contains(features::ZVE32F)
            } else if element_size <= 4 {
                self.contains(features::ZVE32X)
            } else if element_size <= 8 {
                self.contains(features::ZVE64X)
            } else {
                false
            };

            let vector_bytes = if self.contains(features::ZVL65536B) {
                8192
            } else if self.contains(features::ZVL32768B) {
                4096
            } else if self.contains(features::ZVL16384B) {
                2048
            } else if self.contains(features::ZVL8192B) {
                1024
            } else if self.contains(features::ZVL4096B) {
                512
            } else if self.contains(features::ZVL2048B) {
                256
            } else if self.contains(features::ZVL1024B) {
                128
            } else if self.contains(features::ZVL512B) {
                64
            } else if self.contains(features::ZVL256B) {
                32
            } else if self.contains(features::ZVL128B) {
                16
            } else if self.contains(features::ZVL64B) {
                8
            } else if self.contains(features::ZVL32B) {
                4
            } else {
                0
            };

            return if supports_element && vector_bytes != 0 {
                Some(vector_bytes / element_size)
            } else {
                None
            };
        }

        #[cfg(target_arch = "s390x")]
        {
            return if self.contains(features::VECTOR) {
                Some(v128)
            } else {
                None
            };
        }

        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        {
            return if self.contains(features::SIMD128) {
                Some(v128)
            } else {
                None
            };
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            return if self.contains(features::AVX512F)
                && (!is_integer || element_size >= 4 || self.contains(features::AVX512BW))
            {
                Some(v512)
            } else if self.contains(features::AVX2)
                || ((is_f32 || is_f64) && self.contains(features::AVX))
            {
                Some(v256)
            } else if self.contains(features::SSE2) || (is_f32 && self.contains(features::SSE)) {
                Some(v128)
            } else {
                None
            };
        }

        #[allow(unreachable_code)]
        None
    }
}
