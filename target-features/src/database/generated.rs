// This file is @generated.

use super::FeatureData;
use crate::TargetFeatures;

#[cfg(target_arch = "arm")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "aarch64")]
pub(crate) const FEATURE_WORDS: usize = 2;
#[cfg(target_arch = "arm64ec")]
pub(crate) const FEATURE_WORDS: usize = 2;
#[cfg(target_arch = "bpf")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "hexagon")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "mips")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "mips64")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "loongarch32")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "loongarch64")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "nvptx64")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "powerpc")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "powerpc64")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "riscv32")]
pub(crate) const FEATURE_WORDS: usize = 2;
#[cfg(target_arch = "riscv64")]
pub(crate) const FEATURE_WORDS: usize = 2;
#[cfg(target_arch = "s390x")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "sparc")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "sparc64")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "wasm32")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "wasm64")]
pub(crate) const FEATURE_WORDS: usize = 1;
#[cfg(target_arch = "x86")]
pub(crate) const FEATURE_WORDS: usize = 2;
#[cfg(target_arch = "x86_64")]
pub(crate) const FEATURE_WORDS: usize = 2;

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "bpf",
    target_arch = "hexagon",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "loongarch32",
    target_arch = "loongarch64",
    target_arch = "nvptx64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "s390x",
    target_arch = "sparc",
    target_arch = "sparc64",
    target_arch = "wasm32",
    target_arch = "wasm64",
    target_arch = "x86",
    target_arch = "x86_64",
)))]
pub(crate) const FEATURE_WORDS: usize = 0;

#[cfg(any(doc, target_arch = "arm"))]
#[rustfmt::skip]
pub mod arm {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        ACLASS,
        ACQUIRE_RELEASE,
        AES,
        CRC,
        CRT_STATIC,
        D32,
        DOTPROD,
        DSP,
        FP_ARMV8,
        FP16,
        FP64,
        FPREGS,
        I8MM,
        MCLASS,
        MVE,
        MVE_FP,
        NEON,
        RCLASS,
        SHA2,
        SOFT_FLOAT,
        THUMB_MODE,
        THUMB2,
        TRUSTZONE,
        V5TE,
        V6,
        V6K,
        V6M,
        V6T2,
        V7,
        V8,
        V8_1M_MAIN,
        V8M,
        V8M_MAIN,
        VFP2,
        VFP2SP,
        VFP3,
        VFP4,
        VIRTUALIZATION,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "arm")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "arm")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Is application profile ('A' series)."]
    pub const ACLASS: TargetFeatures = feature_set!(ACLASS);

    #[doc = "Has v8 acquire/release (lda/ldaex  etc) instructions."]
    pub const ACQUIRE_RELEASE: TargetFeatures = feature_set!(ACQUIRE_RELEASE);

    #[doc = "Enable AES support."]
    pub const AES: TargetFeatures = feature_set!(AES, D32, FP64, FPREGS, NEON, VFP2, VFP2SP, VFP3);

    #[doc = "Enable support for CRC instructions."]
    pub const CRC: TargetFeatures = feature_set!(CRC);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Extend FP to 32 double registers."]
    pub const D32: TargetFeatures = feature_set!(D32);

    #[doc = "Enable support for dot product instructions."]
    pub const DOTPROD: TargetFeatures = feature_set!(D32, DOTPROD, FP64, FPREGS, NEON, VFP2, VFP2SP, VFP3);

    #[doc = "Supports DSP instructions in ARM and/or Thumb2."]
    pub const DSP: TargetFeatures = feature_set!(DSP);

    #[doc = "Enable ARMv8 FP."]
    pub const FP_ARMV8: TargetFeatures = feature_set!(D32, FP_ARMV8, FP64, FPREGS, VFP2, VFP2SP, VFP3, VFP4);

    #[doc = "Enable full half-precision floating point."]
    pub const FP16: TargetFeatures = feature_set!(D32, FP16, FP64, FPREGS, NEON, VFP2, VFP2SP, VFP3);

    #[doc = "Floating point unit supports double precision."]
    pub const FP64: TargetFeatures = feature_set!(FP64);

    #[doc = "Enable FP registers."]
    pub const FPREGS: TargetFeatures = feature_set!(FPREGS);

    #[doc = "Enable Matrix Multiply Int8 Extension."]
    pub const I8MM: TargetFeatures = feature_set!(D32, FP64, FPREGS, I8MM, NEON, VFP2, VFP2SP, VFP3);

    #[doc = "Is microcontroller profile ('M' series)."]
    pub const MCLASS: TargetFeatures = feature_set!(MCLASS);

    #[doc = "Support M-Class Vector Extension with integer ops."]
    pub const MVE: TargetFeatures = feature_set!(DSP, FPREGS, MVE, THUMB2, V5TE, V6, V6K, V6M, V6T2, V7, V8_1M_MAIN, V8M, V8M_MAIN);

    #[doc = "Support M-Class Vector Extension with integer and floating ops."]
    pub const MVE_FP: TargetFeatures = feature_set!(DSP, FPREGS, MVE, MVE_FP, THUMB2, V5TE, V6, V6K, V6M, V6T2, V7, V8_1M_MAIN, V8M, V8M_MAIN);

    #[doc = "Enable NEON instructions."]
    pub const NEON: TargetFeatures = feature_set!(D32, FP64, FPREGS, NEON, VFP2, VFP2SP, VFP3);

    #[doc = "Is realtime profile ('R' series)."]
    pub const RCLASS: TargetFeatures = feature_set!(RCLASS);

    #[doc = "Enable SHA1 and SHA256 support."]
    pub const SHA2: TargetFeatures = feature_set!(D32, FP64, FPREGS, NEON, SHA2, VFP2, VFP2SP, VFP3);

    #[doc = "Use software floating point features.."]
    pub const SOFT_FLOAT: TargetFeatures = feature_set!(SOFT_FLOAT);

    #[doc = "Thumb mode."]
    pub const THUMB_MODE: TargetFeatures = feature_set!(THUMB_MODE);

    #[doc = "Enable Thumb2 instructions."]
    pub const THUMB2: TargetFeatures = feature_set!(THUMB2);

    #[doc = "Enable support for TrustZone security extensions."]
    pub const TRUSTZONE: TargetFeatures = feature_set!(TRUSTZONE);

    #[doc = "Support ARM v5TE, v5TEj, and v5TExp instructions."]
    pub const V5TE: TargetFeatures = feature_set!(V5TE);

    #[doc = "Support ARM v6 instructions."]
    pub const V6: TargetFeatures = feature_set!(V5TE, V6);

    #[doc = "Support ARM v6k instructions."]
    pub const V6K: TargetFeatures = feature_set!(V5TE, V6, V6K);

    #[doc = "Support ARM v6M instructions."]
    pub const V6M: TargetFeatures = feature_set!(V5TE, V6, V6M);

    #[doc = "Support ARM v6t2 instructions."]
    pub const V6T2: TargetFeatures = feature_set!(THUMB2, V5TE, V6, V6K, V6M, V6T2, V8M);

    #[doc = "Support ARM v7 instructions."]
    pub const V7: TargetFeatures = feature_set!(THUMB2, V5TE, V6, V6K, V6M, V6T2, V7, V8M);

    #[doc = "Support ARM v8 instructions."]
    pub const V8: TargetFeatures = feature_set!(ACQUIRE_RELEASE, THUMB2, V5TE, V6, V6K, V6M, V6T2, V7, V8, V8M);

    #[doc = "Support ARM v8-1M Mainline instructions."]
    pub const V8_1M_MAIN: TargetFeatures = feature_set!(THUMB2, V5TE, V6, V6K, V6M, V6T2, V7, V8_1M_MAIN, V8M, V8M_MAIN);

    #[doc = "Support ARM v8M Baseline instructions."]
    pub const V8M: TargetFeatures = feature_set!(V5TE, V6, V6M, V8M);

    #[doc = "Support ARM v8M Mainline instructions."]
    pub const V8M_MAIN: TargetFeatures = feature_set!(THUMB2, V5TE, V6, V6K, V6M, V6T2, V7, V8M, V8M_MAIN);

    #[doc = "Enable VFP2 instructions."]
    pub const VFP2: TargetFeatures = feature_set!(FP64, FPREGS, VFP2, VFP2SP);

    #[doc = "Enable VFP2 instructions with no double precision."]
    pub const VFP2SP: TargetFeatures = feature_set!(FPREGS, VFP2SP);

    #[doc = "Enable VFP3 instructions."]
    pub const VFP3: TargetFeatures = feature_set!(D32, FP64, FPREGS, VFP2, VFP2SP, VFP3);

    #[doc = "Enable VFP4 instructions."]
    pub const VFP4: TargetFeatures = feature_set!(D32, FP64, FPREGS, VFP2, VFP2SP, VFP3, VFP4);

    #[doc = "Supports Virtualization extension."]
    pub const VIRTUALIZATION: TargetFeatures = feature_set!(VIRTUALIZATION);


    #[cfg(target_arch = "arm")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("aclass", ACLASS),
        FeatureData::new("acquire-release", ACQUIRE_RELEASE),
        FeatureData::new("aes", AES),
        FeatureData::new("crc", CRC),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("d32", D32),
        FeatureData::new("dotprod", DOTPROD),
        FeatureData::new("dsp", DSP),
        FeatureData::new("fp-armv8", FP_ARMV8),
        FeatureData::new("fp16", FP16),
        FeatureData::new("fp64", FP64),
        FeatureData::new("fpregs", FPREGS),
        FeatureData::new("i8mm", I8MM),
        FeatureData::new("mclass", MCLASS),
        FeatureData::new("mve", MVE),
        FeatureData::new("mve.fp", MVE_FP),
        FeatureData::new("neon", NEON),
        FeatureData::new("rclass", RCLASS),
        FeatureData::new("sha2", SHA2),
        FeatureData::new("soft-float", SOFT_FLOAT),
        FeatureData::new("thumb-mode", THUMB_MODE),
        FeatureData::new("thumb2", THUMB2),
        FeatureData::new("trustzone", TRUSTZONE),
        FeatureData::new("v5te", V5TE),
        FeatureData::new("v6", V6),
        FeatureData::new("v6k", V6K),
        FeatureData::new("v6m", V6M),
        FeatureData::new("v6t2", V6T2),
        FeatureData::new("v7", V7),
        FeatureData::new("v8", V8),
        FeatureData::new("v8.1m.main", V8_1M_MAIN),
        FeatureData::new("v8m", V8M),
        FeatureData::new("v8m.main", V8M_MAIN),
        FeatureData::new("vfp2", VFP2),
        FeatureData::new("vfp2sp", VFP2SP),
        FeatureData::new("vfp3", VFP3),
        FeatureData::new("vfp4", VFP4),
        FeatureData::new("virtualization", VIRTUALIZATION),
    ];

    #[cfg(target_arch = "arm")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "aclass")]
        let features = features.with(ACLASS);
        #[cfg(target_feature = "acquire-release")]
        let features = features.with(ACQUIRE_RELEASE);
        #[cfg(target_feature = "aes")]
        let features = features.with(AES);
        #[cfg(target_feature = "crc")]
        let features = features.with(CRC);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "d32")]
        let features = features.with(D32);
        #[cfg(target_feature = "dotprod")]
        let features = features.with(DOTPROD);
        #[cfg(target_feature = "dsp")]
        let features = features.with(DSP);
        #[cfg(target_feature = "fp-armv8")]
        let features = features.with(FP_ARMV8);
        #[cfg(target_feature = "fp16")]
        let features = features.with(FP16);
        #[cfg(target_feature = "fp64")]
        let features = features.with(FP64);
        #[cfg(target_feature = "fpregs")]
        let features = features.with(FPREGS);
        #[cfg(target_feature = "i8mm")]
        let features = features.with(I8MM);
        #[cfg(target_feature = "mclass")]
        let features = features.with(MCLASS);
        #[cfg(target_feature = "mve")]
        let features = features.with(MVE);
        #[cfg(target_feature = "mve.fp")]
        let features = features.with(MVE_FP);
        #[cfg(target_feature = "neon")]
        let features = features.with(NEON);
        #[cfg(target_feature = "rclass")]
        let features = features.with(RCLASS);
        #[cfg(target_feature = "sha2")]
        let features = features.with(SHA2);
        #[cfg(target_feature = "soft-float")]
        let features = features.with(SOFT_FLOAT);
        #[cfg(target_feature = "thumb-mode")]
        let features = features.with(THUMB_MODE);
        #[cfg(target_feature = "thumb2")]
        let features = features.with(THUMB2);
        #[cfg(target_feature = "trustzone")]
        let features = features.with(TRUSTZONE);
        #[cfg(target_feature = "v5te")]
        let features = features.with(V5TE);
        #[cfg(target_feature = "v6")]
        let features = features.with(V6);
        #[cfg(target_feature = "v6k")]
        let features = features.with(V6K);
        #[cfg(target_feature = "v6m")]
        let features = features.with(V6M);
        #[cfg(target_feature = "v6t2")]
        let features = features.with(V6T2);
        #[cfg(target_feature = "v7")]
        let features = features.with(V7);
        #[cfg(target_feature = "v8")]
        let features = features.with(V8);
        #[cfg(target_feature = "v8.1m.main")]
        let features = features.with(V8_1M_MAIN);
        #[cfg(target_feature = "v8m")]
        let features = features.with(V8M);
        #[cfg(target_feature = "v8m.main")]
        let features = features.with(V8M_MAIN);
        #[cfg(target_feature = "vfp2")]
        let features = features.with(VFP2);
        #[cfg(target_feature = "vfp2sp")]
        let features = features.with(VFP2SP);
        #[cfg(target_feature = "vfp3")]
        let features = features.with(VFP3);
        #[cfg(target_feature = "vfp4")]
        let features = features.with(VFP4);
        #[cfg(target_feature = "virtualization")]
        let features = features.with(VIRTUALIZATION);
        features
    }

}
#[cfg(any(doc, target_arch = "aarch64"))]
#[rustfmt::skip]
pub mod aarch64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        AES,
        BF16,
        BTI,
        CRC,
        CRT_STATIC,
        CSSC,
        DIT,
        DOTPROD,
        DPB,
        DPB2,
        ECV,
        F32MM,
        F64MM,
        FAMINMAX,
        FCMA,
        FHM,
        FLAGM,
        FLAGM2,
        FP16,
        FP8,
        FP8DOT2,
        FP8DOT4,
        FP8FMA,
        FRINTTS,
        HBC,
        I8MM,
        JSCONV,
        LOR,
        LSE,
        LSE128,
        LSE2,
        LUT,
        MOPS,
        MTE,
        NEON,
        OUTLINE_ATOMICS,
        PACA,
        PACG,
        PAN,
        PAUTH_LR,
        PMUV3,
        RAND,
        RAS,
        RCPC,
        RCPC2,
        RCPC3,
        RDM,
        SB,
        SHA2,
        SHA3,
        SM4,
        SME,
        SME_B16B16,
        SME_F16F16,
        SME_F64F64,
        SME_F8F16,
        SME_F8F32,
        SME_FA64,
        SME_I16I64,
        SME_LUTV2,
        SME2,
        SME2P1,
        SPE,
        SSBS,
        SSVE_FP8DOT2,
        SSVE_FP8DOT4,
        SSVE_FP8FMA,
        SVE,
        SVE_B16B16,
        SVE2,
        SVE2_AES,
        SVE2_BITPERM,
        SVE2_SHA3,
        SVE2_SM4,
        SVE2P1,
        V8_1A,
        V8_2A,
        V8_3A,
        V8_4A,
        V8_5A,
        V8_6A,
        V8_7A,
        V8_8A,
        V8_9A,
        V9_1A,
        V9_2A,
        V9_3A,
        V9_4A,
        V9_5A,
        V9A,
        VH,
        WFXT,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "aarch64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "aarch64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enable AES support."]
    pub const AES: TargetFeatures = feature_set!(AES, NEON);

    #[doc = "Enable BFloat16 Extension."]
    pub const BF16: TargetFeatures = feature_set!(BF16);

    #[doc = "Enable Branch Target Identification."]
    pub const BTI: TargetFeatures = feature_set!(BTI);

    #[doc = "Enable Armv8.0-A CRC-32 checksum instructions."]
    pub const CRC: TargetFeatures = feature_set!(CRC);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Enable Common Short Sequence Compression (CSSC) instructions."]
    pub const CSSC: TargetFeatures = feature_set!(CSSC);

    #[doc = "Enable Armv8.4-A Data Independent Timing instructions."]
    pub const DIT: TargetFeatures = feature_set!(DIT);

    #[doc = "Enable dot product support."]
    pub const DOTPROD: TargetFeatures = feature_set!(DOTPROD, NEON);

    #[doc = "Enable Armv8.2-A data Cache Clean to Point of Persistence."]
    pub const DPB: TargetFeatures = feature_set!(DPB);

    #[doc = "Enable Armv8.5-A Cache Clean to Point of Deep Persistence."]
    pub const DPB2: TargetFeatures = feature_set!(DPB, DPB2);

    #[doc = "Enable enhanced counter virtualization extension."]
    pub const ECV: TargetFeatures = feature_set!(ECV);

    #[doc = "Enable Matrix Multiply FP32 Extension."]
    pub const F32MM: TargetFeatures = feature_set!(F32MM, FP16, NEON, SVE);

    #[doc = "Enable Matrix Multiply FP64 Extension."]
    pub const F64MM: TargetFeatures = feature_set!(F64MM, FP16, NEON, SVE);

    #[doc = "Enable FAMIN and FAMAX instructions."]
    pub const FAMINMAX: TargetFeatures = feature_set!(FAMINMAX);

    #[doc = "Enable Armv8.3-A Floating-point complex number support."]
    pub const FCMA: TargetFeatures = feature_set!(FCMA, NEON);

    #[doc = "Enable FP16 FML instructions."]
    pub const FHM: TargetFeatures = feature_set!(FHM, FP16, NEON);

    #[doc = "Enable Armv8.4-A Flag Manipulation instructions."]
    pub const FLAGM: TargetFeatures = feature_set!(FLAGM);

    #[doc = "Enable alternative NZCV format for floating point comparisons."]
    pub const FLAGM2: TargetFeatures = feature_set!(FLAGM2);

    #[doc = "Enable half-precision floating-point data processing."]
    pub const FP16: TargetFeatures = feature_set!(FP16, NEON);

    #[doc = "Enable FP8 instructions."]
    pub const FP8: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT);

    #[doc = "Enable FP8 2-way dot instructions."]
    pub const FP8DOT2: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, FP8DOT2, FP8DOT4, FP8FMA, LUT);

    #[doc = "Enable FP8 4-way dot instructions."]
    pub const FP8DOT4: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, FP8DOT4, FP8FMA, LUT);

    #[doc = "Enable Armv9.5-A FP8 multiply-add instructions."]
    pub const FP8FMA: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, FP8FMA, LUT);

    #[doc = "Enable FRInt\\[32\\|64\\]\\[Z\\|X\\] instructions that round a floating-point number to an integer (in FP format) forcing it to fit into a 32- or 64-bit int."]
    pub const FRINTTS: TargetFeatures = feature_set!(FRINTTS);

    #[doc = "Enable Armv8.8-A Hinted Conditional Branches Extension."]
    pub const HBC: TargetFeatures = feature_set!(HBC);

    #[doc = "Enable Matrix Multiply Int8 Extension."]
    pub const I8MM: TargetFeatures = feature_set!(I8MM);

    #[doc = "Enable Armv8.3-A JavaScript FP conversion instructions."]
    pub const JSCONV: TargetFeatures = feature_set!(JSCONV, NEON);

    #[doc = "Enable Armv8.1-A Limited Ordering Regions extension."]
    pub const LOR: TargetFeatures = feature_set!(LOR);

    #[doc = "Enable Armv8.1-A Large System Extension (LSE) atomic instructions."]
    pub const LSE: TargetFeatures = feature_set!(LSE);

    #[doc = "Enable Armv9.4-A 128-bit Atomic instructions."]
    pub const LSE128: TargetFeatures = feature_set!(LSE, LSE128);

    #[doc = "Enable Armv8.4-A Large System Extension 2 (LSE2) atomicity rules."]
    pub const LSE2: TargetFeatures = feature_set!(LSE2);

    #[doc = "Enable Lookup Table instructions."]
    pub const LUT: TargetFeatures = feature_set!(LUT);

    #[doc = "Enable Armv8.8-A memcpy and memset acceleration instructions."]
    pub const MOPS: TargetFeatures = feature_set!(MOPS);

    #[doc = "Enable Memory Tagging Extension."]
    pub const MTE: TargetFeatures = feature_set!(MTE);

    #[doc = "Enable Advanced SIMD instructions."]
    pub const NEON: TargetFeatures = feature_set!(NEON);

    #[doc = "Enable out of line atomics to support LSE instructions."]
    pub const OUTLINE_ATOMICS: TargetFeatures = feature_set!(OUTLINE_ATOMICS);

    #[doc = "Enable Armv8.3-A Pointer Authentication extension."]
    pub const PACA: TargetFeatures = feature_set!(PACA, PACG);

    #[doc = "Enable Armv8.3-A Pointer Authentication extension."]
    pub const PACG: TargetFeatures = feature_set!(PACA, PACG);

    #[doc = "Enable Armv8.1-A Privileged Access-Never extension."]
    pub const PAN: TargetFeatures = feature_set!(PAN);

    #[doc = "Enable Armv9.5-A PAC enhancements."]
    pub const PAUTH_LR: TargetFeatures = feature_set!(PAUTH_LR);

    #[doc = "Enable Armv8.0-A PMUv3 Performance Monitors extension."]
    pub const PMUV3: TargetFeatures = feature_set!(PMUV3);

    #[doc = "Enable Random Number generation instructions."]
    pub const RAND: TargetFeatures = feature_set!(RAND);

    #[doc = "Enable Armv8.0-A Reliability, Availability and Serviceability Extensions."]
    pub const RAS: TargetFeatures = feature_set!(RAS);

    #[doc = "Enable support for RCPC extension."]
    pub const RCPC: TargetFeatures = feature_set!(RCPC);

    #[doc = "Enable Armv8.4-A RCPC instructions with Immediate Offsets."]
    pub const RCPC2: TargetFeatures = feature_set!(RCPC, RCPC2);

    #[doc = "Enable Armv8.9-A RCPC instructions for A64 and Advanced SIMD and floating-point instruction set."]
    pub const RCPC3: TargetFeatures = feature_set!(RCPC, RCPC2, RCPC3);

    #[doc = "Enable Armv8.1-A Rounding Double Multiply Add/Subtract instructions."]
    pub const RDM: TargetFeatures = feature_set!(NEON, RDM);

    #[doc = "Enable Armv8.5-A Speculation Barrier."]
    pub const SB: TargetFeatures = feature_set!(SB);

    #[doc = "Enable SHA1 and SHA256 support."]
    pub const SHA2: TargetFeatures = feature_set!(NEON, SHA2);

    #[doc = "Enable SHA512 and SHA3 support."]
    pub const SHA3: TargetFeatures = feature_set!(NEON, SHA2, SHA3);

    #[doc = "Enable SM3 and SM4 support."]
    pub const SM4: TargetFeatures = feature_set!(NEON, SM4);

    #[doc = "Enable Scalable Matrix Extension (SME)."]
    pub const SME: TargetFeatures = feature_set!(BF16, SME);

    #[doc = "Enable SME2.1 ZA-targeting non-widening BFloat16 instructions."]
    pub const SME_B16B16: TargetFeatures = feature_set!(BF16, SME, SME_B16B16, SME2, SVE_B16B16);

    #[doc = "Enable SME non-widening Float16 instructions."]
    pub const SME_F16F16: TargetFeatures = feature_set!(BF16, SME, SME_F16F16, SME2);

    #[doc = "Enable Scalable Matrix Extension (SME) F64F64 instructions."]
    pub const SME_F64F64: TargetFeatures = feature_set!(BF16, SME, SME_F64F64);

    #[doc = "Enable Scalable Matrix Extension (SME) F8F16 instructions."]
    pub const SME_F8F16: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME_F8F16, SME_F8F32, SME2);

    #[doc = "Enable Scalable Matrix Extension (SME) F8F32 instructions."]
    pub const SME_F8F32: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME_F8F32, SME2);

    #[doc = "Enable the full A64 instruction set in streaming SVE mode."]
    pub const SME_FA64: TargetFeatures = feature_set!(BF16, FP16, NEON, SME, SME_FA64, SVE, SVE2);

    #[doc = "Enable Scalable Matrix Extension (SME) I16I64 instructions."]
    pub const SME_I16I64: TargetFeatures = feature_set!(BF16, SME, SME_I16I64);

    #[doc = "Enable Scalable Matrix Extension (SME) LUTv2 instructions."]
    pub const SME_LUTV2: TargetFeatures = feature_set!(SME_LUTV2);

    #[doc = "Enable Scalable Matrix Extension 2 (SME2) instructions."]
    pub const SME2: TargetFeatures = feature_set!(BF16, SME, SME2);

    #[doc = "Enable Scalable Matrix Extension 2.1 instructions."]
    pub const SME2P1: TargetFeatures = feature_set!(BF16, SME, SME2, SME2P1);

    #[doc = "Enable Statistical Profiling extension."]
    pub const SPE: TargetFeatures = feature_set!(SPE);

    #[doc = "Enable Speculative Store Bypass Safe bit."]
    pub const SSBS: TargetFeatures = feature_set!(SSBS);

    #[doc = "Enable SVE2 FP8 2-way dot product instructions."]
    pub const SSVE_FP8DOT2: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME2, SSVE_FP8DOT2, SSVE_FP8DOT4, SSVE_FP8FMA);

    #[doc = "Enable SVE2 FP8 4-way dot product instructions."]
    pub const SSVE_FP8DOT4: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME2, SSVE_FP8DOT4, SSVE_FP8FMA);

    #[doc = "Enable SVE2 FP8 multiply-add instructions."]
    pub const SSVE_FP8FMA: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME2, SSVE_FP8FMA);

    #[doc = "Enable Scalable Vector Extension (SVE) instructions."]
    pub const SVE: TargetFeatures = feature_set!(FP16, NEON, SVE);

    #[doc = "Enable SVE2 non-widening and SME2 Z-targeting non-widening BFloat16 instructions."]
    pub const SVE_B16B16: TargetFeatures = feature_set!(BF16, SVE_B16B16);

    #[doc = "Enable Scalable Vector Extension 2 (SVE2) instructions."]
    pub const SVE2: TargetFeatures = feature_set!(FP16, NEON, SVE, SVE2);

    #[doc = "Shorthand for +sve2+sve-aes."]
    pub const SVE2_AES: TargetFeatures = feature_set!(AES, FP16, NEON, SVE, SVE2, SVE2_AES);

    #[doc = "Shorthand for +sve2+sve-bitperm."]
    pub const SVE2_BITPERM: TargetFeatures = feature_set!(FP16, NEON, SVE, SVE2, SVE2_BITPERM);

    #[doc = "Shorthand for +sve2+sve-sha3."]
    pub const SVE2_SHA3: TargetFeatures = feature_set!(FP16, NEON, SHA2, SHA3, SVE, SVE2, SVE2_SHA3);

    #[doc = "Shorthand for +sve2+sve-sm4."]
    pub const SVE2_SM4: TargetFeatures = feature_set!(FP16, NEON, SM4, SVE, SVE2, SVE2_SM4);

    #[doc = "Enable Scalable Vector Extension 2.1 instructions."]
    pub const SVE2P1: TargetFeatures = feature_set!(FP16, NEON, SVE, SVE2, SVE2P1);

    #[doc = "Support ARM v8.1a architecture."]
    pub const V8_1A: TargetFeatures = feature_set!(CRC, LOR, LSE, NEON, PAN, RDM, V8_1A, VH);

    #[doc = "Support ARM v8.2a architecture."]
    pub const V8_2A: TargetFeatures = feature_set!(CRC, DPB, LOR, LSE, NEON, PAN, RAS, RDM, V8_1A, V8_2A, VH);

    #[doc = "Support ARM v8.3a architecture."]
    pub const V8_3A: TargetFeatures = feature_set!(CRC, DPB, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, V8_1A, V8_2A, V8_3A, VH);

    #[doc = "Support ARM v8.4a architecture."]
    pub const V8_4A: TargetFeatures = feature_set!(CRC, DIT, DOTPROD, DPB, FLAGM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, V8_1A, V8_2A, V8_3A, V8_4A, VH);

    #[doc = "Support ARM v8.5a architecture."]
    pub const V8_5A: TargetFeatures = feature_set!(BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, VH);

    #[doc = "Support ARM v8.6a architecture."]
    pub const V8_6A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, I8MM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, VH);

    #[doc = "Support ARM v8.7a architecture."]
    pub const V8_7A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, I8MM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, VH, WFXT);

    #[doc = "Support ARM v8.8a architecture."]
    pub const V8_8A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, VH, WFXT);

    #[doc = "Support ARM v8.9a architecture."]
    pub const V8_9A: TargetFeatures = feature_set!(BF16, BTI, CRC, CSSC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, V8_9A, VH, WFXT);

    #[doc = "Support ARM v9.1a architecture."]
    pub const V9_1A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, I8MM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V9_1A, V9A, VH);

    #[doc = "Support ARM v9.2a architecture."]
    pub const V9_2A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, I8MM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V9_1A, V9_2A, V9A, VH, WFXT);

    #[doc = "Support ARM v9.3a architecture."]
    pub const V9_3A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, V9_1A, V9_2A, V9_3A, V9A, VH, WFXT);

    #[doc = "Support ARM v9.4a architecture."]
    pub const V9_4A: TargetFeatures = feature_set!(BF16, BTI, CRC, CSSC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, V8_9A, V9_1A, V9_2A, V9_3A, V9_4A, V9A, VH, WFXT);

    #[doc = "Support ARM v9.5a architecture."]
    pub const V9_5A: TargetFeatures = feature_set!(BF16, BTI, CRC, CSSC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, V8_9A, V9_1A, V9_2A, V9_3A, V9_4A, V9_5A, V9A, VH, WFXT);

    #[doc = "Support ARM v9a architecture."]
    pub const V9A: TargetFeatures = feature_set!(BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V9A, VH);

    #[doc = "Enable Armv8.1-A Virtual Host extension."]
    pub const VH: TargetFeatures = feature_set!(VH);

    #[doc = "Enable Armv8.7-A WFET and WFIT instruction."]
    pub const WFXT: TargetFeatures = feature_set!(WFXT);


    #[cfg(target_arch = "aarch64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("aes", AES),
        FeatureData::new("bf16", BF16),
        FeatureData::new("bti", BTI),
        FeatureData::new("crc", CRC),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("cssc", CSSC),
        FeatureData::new("dit", DIT),
        FeatureData::new("dotprod", DOTPROD),
        FeatureData::new("dpb", DPB),
        FeatureData::new("dpb2", DPB2),
        FeatureData::new("ecv", ECV),
        FeatureData::new("f32mm", F32MM),
        FeatureData::new("f64mm", F64MM),
        FeatureData::new("faminmax", FAMINMAX),
        FeatureData::new("fcma", FCMA),
        FeatureData::new("fhm", FHM),
        FeatureData::new("flagm", FLAGM),
        FeatureData::new("flagm2", FLAGM2),
        FeatureData::new("fp16", FP16),
        FeatureData::new("fp8", FP8),
        FeatureData::new("fp8dot2", FP8DOT2),
        FeatureData::new("fp8dot4", FP8DOT4),
        FeatureData::new("fp8fma", FP8FMA),
        FeatureData::new("frintts", FRINTTS),
        FeatureData::new("hbc", HBC),
        FeatureData::new("i8mm", I8MM),
        FeatureData::new("jsconv", JSCONV),
        FeatureData::new("lor", LOR),
        FeatureData::new("lse", LSE),
        FeatureData::new("lse128", LSE128),
        FeatureData::new("lse2", LSE2),
        FeatureData::new("lut", LUT),
        FeatureData::new("mops", MOPS),
        FeatureData::new("mte", MTE),
        FeatureData::new("neon", NEON),
        FeatureData::new("outline-atomics", OUTLINE_ATOMICS),
        FeatureData::new("paca", PACA),
        FeatureData::new("pacg", PACG),
        FeatureData::new("pan", PAN),
        FeatureData::new("pauth-lr", PAUTH_LR),
        FeatureData::new("pmuv3", PMUV3),
        FeatureData::new("rand", RAND),
        FeatureData::new("ras", RAS),
        FeatureData::new("rcpc", RCPC),
        FeatureData::new("rcpc2", RCPC2),
        FeatureData::new("rcpc3", RCPC3),
        FeatureData::new("rdm", RDM),
        FeatureData::new("sb", SB),
        FeatureData::new("sha2", SHA2),
        FeatureData::new("sha3", SHA3),
        FeatureData::new("sm4", SM4),
        FeatureData::new("sme", SME),
        FeatureData::new("sme-b16b16", SME_B16B16),
        FeatureData::new("sme-f16f16", SME_F16F16),
        FeatureData::new("sme-f64f64", SME_F64F64),
        FeatureData::new("sme-f8f16", SME_F8F16),
        FeatureData::new("sme-f8f32", SME_F8F32),
        FeatureData::new("sme-fa64", SME_FA64),
        FeatureData::new("sme-i16i64", SME_I16I64),
        FeatureData::new("sme-lutv2", SME_LUTV2),
        FeatureData::new("sme2", SME2),
        FeatureData::new("sme2p1", SME2P1),
        FeatureData::new("spe", SPE),
        FeatureData::new("ssbs", SSBS),
        FeatureData::new("ssve-fp8dot2", SSVE_FP8DOT2),
        FeatureData::new("ssve-fp8dot4", SSVE_FP8DOT4),
        FeatureData::new("ssve-fp8fma", SSVE_FP8FMA),
        FeatureData::new("sve", SVE),
        FeatureData::new("sve-b16b16", SVE_B16B16),
        FeatureData::new("sve2", SVE2),
        FeatureData::new("sve2-aes", SVE2_AES),
        FeatureData::new("sve2-bitperm", SVE2_BITPERM),
        FeatureData::new("sve2-sha3", SVE2_SHA3),
        FeatureData::new("sve2-sm4", SVE2_SM4),
        FeatureData::new("sve2p1", SVE2P1),
        FeatureData::new("v8.1a", V8_1A),
        FeatureData::new("v8.2a", V8_2A),
        FeatureData::new("v8.3a", V8_3A),
        FeatureData::new("v8.4a", V8_4A),
        FeatureData::new("v8.5a", V8_5A),
        FeatureData::new("v8.6a", V8_6A),
        FeatureData::new("v8.7a", V8_7A),
        FeatureData::new("v8.8a", V8_8A),
        FeatureData::new("v8.9a", V8_9A),
        FeatureData::new("v9.1a", V9_1A),
        FeatureData::new("v9.2a", V9_2A),
        FeatureData::new("v9.3a", V9_3A),
        FeatureData::new("v9.4a", V9_4A),
        FeatureData::new("v9.5a", V9_5A),
        FeatureData::new("v9a", V9A),
        FeatureData::new("vh", VH),
        FeatureData::new("wfxt", WFXT),
    ];

    #[cfg(target_arch = "aarch64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "aes")]
        let features = features.with(AES);
        #[cfg(target_feature = "bf16")]
        let features = features.with(BF16);
        #[cfg(target_feature = "bti")]
        let features = features.with(BTI);
        #[cfg(target_feature = "crc")]
        let features = features.with(CRC);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "cssc")]
        let features = features.with(CSSC);
        #[cfg(target_feature = "dit")]
        let features = features.with(DIT);
        #[cfg(target_feature = "dotprod")]
        let features = features.with(DOTPROD);
        #[cfg(target_feature = "dpb")]
        let features = features.with(DPB);
        #[cfg(target_feature = "dpb2")]
        let features = features.with(DPB2);
        #[cfg(target_feature = "ecv")]
        let features = features.with(ECV);
        #[cfg(target_feature = "f32mm")]
        let features = features.with(F32MM);
        #[cfg(target_feature = "f64mm")]
        let features = features.with(F64MM);
        #[cfg(target_feature = "faminmax")]
        let features = features.with(FAMINMAX);
        #[cfg(target_feature = "fcma")]
        let features = features.with(FCMA);
        #[cfg(target_feature = "fhm")]
        let features = features.with(FHM);
        #[cfg(target_feature = "flagm")]
        let features = features.with(FLAGM);
        #[cfg(target_feature = "flagm2")]
        let features = features.with(FLAGM2);
        #[cfg(target_feature = "fp16")]
        let features = features.with(FP16);
        #[cfg(target_feature = "fp8")]
        let features = features.with(FP8);
        #[cfg(target_feature = "fp8dot2")]
        let features = features.with(FP8DOT2);
        #[cfg(target_feature = "fp8dot4")]
        let features = features.with(FP8DOT4);
        #[cfg(target_feature = "fp8fma")]
        let features = features.with(FP8FMA);
        #[cfg(target_feature = "frintts")]
        let features = features.with(FRINTTS);
        #[cfg(target_feature = "hbc")]
        let features = features.with(HBC);
        #[cfg(target_feature = "i8mm")]
        let features = features.with(I8MM);
        #[cfg(target_feature = "jsconv")]
        let features = features.with(JSCONV);
        #[cfg(target_feature = "lor")]
        let features = features.with(LOR);
        #[cfg(target_feature = "lse")]
        let features = features.with(LSE);
        #[cfg(target_feature = "lse128")]
        let features = features.with(LSE128);
        #[cfg(target_feature = "lse2")]
        let features = features.with(LSE2);
        #[cfg(target_feature = "lut")]
        let features = features.with(LUT);
        #[cfg(target_feature = "mops")]
        let features = features.with(MOPS);
        #[cfg(target_feature = "mte")]
        let features = features.with(MTE);
        #[cfg(target_feature = "neon")]
        let features = features.with(NEON);
        #[cfg(target_feature = "outline-atomics")]
        let features = features.with(OUTLINE_ATOMICS);
        #[cfg(target_feature = "paca")]
        let features = features.with(PACA);
        #[cfg(target_feature = "pacg")]
        let features = features.with(PACG);
        #[cfg(target_feature = "pan")]
        let features = features.with(PAN);
        #[cfg(target_feature = "pauth-lr")]
        let features = features.with(PAUTH_LR);
        #[cfg(target_feature = "pmuv3")]
        let features = features.with(PMUV3);
        #[cfg(target_feature = "rand")]
        let features = features.with(RAND);
        #[cfg(target_feature = "ras")]
        let features = features.with(RAS);
        #[cfg(target_feature = "rcpc")]
        let features = features.with(RCPC);
        #[cfg(target_feature = "rcpc2")]
        let features = features.with(RCPC2);
        #[cfg(target_feature = "rcpc3")]
        let features = features.with(RCPC3);
        #[cfg(target_feature = "rdm")]
        let features = features.with(RDM);
        #[cfg(target_feature = "sb")]
        let features = features.with(SB);
        #[cfg(target_feature = "sha2")]
        let features = features.with(SHA2);
        #[cfg(target_feature = "sha3")]
        let features = features.with(SHA3);
        #[cfg(target_feature = "sm4")]
        let features = features.with(SM4);
        #[cfg(target_feature = "sme")]
        let features = features.with(SME);
        #[cfg(target_feature = "sme-b16b16")]
        let features = features.with(SME_B16B16);
        #[cfg(target_feature = "sme-f16f16")]
        let features = features.with(SME_F16F16);
        #[cfg(target_feature = "sme-f64f64")]
        let features = features.with(SME_F64F64);
        #[cfg(target_feature = "sme-f8f16")]
        let features = features.with(SME_F8F16);
        #[cfg(target_feature = "sme-f8f32")]
        let features = features.with(SME_F8F32);
        #[cfg(target_feature = "sme-fa64")]
        let features = features.with(SME_FA64);
        #[cfg(target_feature = "sme-i16i64")]
        let features = features.with(SME_I16I64);
        #[cfg(target_feature = "sme-lutv2")]
        let features = features.with(SME_LUTV2);
        #[cfg(target_feature = "sme2")]
        let features = features.with(SME2);
        #[cfg(target_feature = "sme2p1")]
        let features = features.with(SME2P1);
        #[cfg(target_feature = "spe")]
        let features = features.with(SPE);
        #[cfg(target_feature = "ssbs")]
        let features = features.with(SSBS);
        #[cfg(target_feature = "ssve-fp8dot2")]
        let features = features.with(SSVE_FP8DOT2);
        #[cfg(target_feature = "ssve-fp8dot4")]
        let features = features.with(SSVE_FP8DOT4);
        #[cfg(target_feature = "ssve-fp8fma")]
        let features = features.with(SSVE_FP8FMA);
        #[cfg(target_feature = "sve")]
        let features = features.with(SVE);
        #[cfg(target_feature = "sve-b16b16")]
        let features = features.with(SVE_B16B16);
        #[cfg(target_feature = "sve2")]
        let features = features.with(SVE2);
        #[cfg(target_feature = "sve2-aes")]
        let features = features.with(SVE2_AES);
        #[cfg(target_feature = "sve2-bitperm")]
        let features = features.with(SVE2_BITPERM);
        #[cfg(target_feature = "sve2-sha3")]
        let features = features.with(SVE2_SHA3);
        #[cfg(target_feature = "sve2-sm4")]
        let features = features.with(SVE2_SM4);
        #[cfg(target_feature = "sve2p1")]
        let features = features.with(SVE2P1);
        #[cfg(target_feature = "v8.1a")]
        let features = features.with(V8_1A);
        #[cfg(target_feature = "v8.2a")]
        let features = features.with(V8_2A);
        #[cfg(target_feature = "v8.3a")]
        let features = features.with(V8_3A);
        #[cfg(target_feature = "v8.4a")]
        let features = features.with(V8_4A);
        #[cfg(target_feature = "v8.5a")]
        let features = features.with(V8_5A);
        #[cfg(target_feature = "v8.6a")]
        let features = features.with(V8_6A);
        #[cfg(target_feature = "v8.7a")]
        let features = features.with(V8_7A);
        #[cfg(target_feature = "v8.8a")]
        let features = features.with(V8_8A);
        #[cfg(target_feature = "v8.9a")]
        let features = features.with(V8_9A);
        #[cfg(target_feature = "v9.1a")]
        let features = features.with(V9_1A);
        #[cfg(target_feature = "v9.2a")]
        let features = features.with(V9_2A);
        #[cfg(target_feature = "v9.3a")]
        let features = features.with(V9_3A);
        #[cfg(target_feature = "v9.4a")]
        let features = features.with(V9_4A);
        #[cfg(target_feature = "v9.5a")]
        let features = features.with(V9_5A);
        #[cfg(target_feature = "v9a")]
        let features = features.with(V9A);
        #[cfg(target_feature = "vh")]
        let features = features.with(VH);
        #[cfg(target_feature = "wfxt")]
        let features = features.with(WFXT);
        features
    }

}
#[cfg(any(doc, target_arch = "arm64ec"))]
#[rustfmt::skip]
pub mod arm64ec {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        AES,
        BF16,
        BTI,
        CRC,
        CRT_STATIC,
        CSSC,
        DIT,
        DOTPROD,
        DPB,
        DPB2,
        ECV,
        F32MM,
        F64MM,
        FAMINMAX,
        FCMA,
        FHM,
        FLAGM,
        FLAGM2,
        FP16,
        FP8,
        FP8DOT2,
        FP8DOT4,
        FP8FMA,
        FRINTTS,
        HBC,
        I8MM,
        JSCONV,
        LOR,
        LSE,
        LSE128,
        LSE2,
        LUT,
        MOPS,
        MTE,
        NEON,
        OUTLINE_ATOMICS,
        PACA,
        PACG,
        PAN,
        PAUTH_LR,
        PMUV3,
        RAND,
        RAS,
        RCPC,
        RCPC2,
        RCPC3,
        RDM,
        SB,
        SHA2,
        SHA3,
        SM4,
        SME,
        SME_B16B16,
        SME_F16F16,
        SME_F64F64,
        SME_F8F16,
        SME_F8F32,
        SME_FA64,
        SME_I16I64,
        SME_LUTV2,
        SME2,
        SME2P1,
        SPE,
        SSBS,
        SSVE_FP8DOT2,
        SSVE_FP8DOT4,
        SSVE_FP8FMA,
        SVE,
        SVE_B16B16,
        SVE2,
        SVE2_AES,
        SVE2_BITPERM,
        SVE2_SHA3,
        SVE2_SM4,
        SVE2P1,
        V8_1A,
        V8_2A,
        V8_3A,
        V8_4A,
        V8_5A,
        V8_6A,
        V8_7A,
        V8_8A,
        V8_9A,
        V9_1A,
        V9_2A,
        V9_3A,
        V9_4A,
        V9_5A,
        V9A,
        VH,
        WFXT,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "arm64ec")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "arm64ec")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enable AES support."]
    pub const AES: TargetFeatures = feature_set!(AES, NEON);

    #[doc = "Enable BFloat16 Extension."]
    pub const BF16: TargetFeatures = feature_set!(BF16);

    #[doc = "Enable Branch Target Identification."]
    pub const BTI: TargetFeatures = feature_set!(BTI);

    #[doc = "Enable Armv8.0-A CRC-32 checksum instructions."]
    pub const CRC: TargetFeatures = feature_set!(CRC);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Enable Common Short Sequence Compression (CSSC) instructions."]
    pub const CSSC: TargetFeatures = feature_set!(CSSC);

    #[doc = "Enable Armv8.4-A Data Independent Timing instructions."]
    pub const DIT: TargetFeatures = feature_set!(DIT);

    #[doc = "Enable dot product support."]
    pub const DOTPROD: TargetFeatures = feature_set!(DOTPROD, NEON);

    #[doc = "Enable Armv8.2-A data Cache Clean to Point of Persistence."]
    pub const DPB: TargetFeatures = feature_set!(DPB);

    #[doc = "Enable Armv8.5-A Cache Clean to Point of Deep Persistence."]
    pub const DPB2: TargetFeatures = feature_set!(DPB, DPB2);

    #[doc = "Enable enhanced counter virtualization extension."]
    pub const ECV: TargetFeatures = feature_set!(ECV);

    #[doc = "Enable Matrix Multiply FP32 Extension."]
    pub const F32MM: TargetFeatures = feature_set!(F32MM, FP16, NEON, SVE);

    #[doc = "Enable Matrix Multiply FP64 Extension."]
    pub const F64MM: TargetFeatures = feature_set!(F64MM, FP16, NEON, SVE);

    #[doc = "Enable FAMIN and FAMAX instructions."]
    pub const FAMINMAX: TargetFeatures = feature_set!(FAMINMAX);

    #[doc = "Enable Armv8.3-A Floating-point complex number support."]
    pub const FCMA: TargetFeatures = feature_set!(FCMA, NEON);

    #[doc = "Enable FP16 FML instructions."]
    pub const FHM: TargetFeatures = feature_set!(FHM, FP16, NEON);

    #[doc = "Enable Armv8.4-A Flag Manipulation instructions."]
    pub const FLAGM: TargetFeatures = feature_set!(FLAGM);

    #[doc = "Enable alternative NZCV format for floating point comparisons."]
    pub const FLAGM2: TargetFeatures = feature_set!(FLAGM2);

    #[doc = "Enable half-precision floating-point data processing."]
    pub const FP16: TargetFeatures = feature_set!(FP16, NEON);

    #[doc = "Enable FP8 instructions."]
    pub const FP8: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT);

    #[doc = "Enable FP8 2-way dot instructions."]
    pub const FP8DOT2: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, FP8DOT2, FP8DOT4, FP8FMA, LUT);

    #[doc = "Enable FP8 4-way dot instructions."]
    pub const FP8DOT4: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, FP8DOT4, FP8FMA, LUT);

    #[doc = "Enable Armv9.5-A FP8 multiply-add instructions."]
    pub const FP8FMA: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, FP8FMA, LUT);

    #[doc = "Enable FRInt\\[32\\|64\\]\\[Z\\|X\\] instructions that round a floating-point number to an integer (in FP format) forcing it to fit into a 32- or 64-bit int."]
    pub const FRINTTS: TargetFeatures = feature_set!(FRINTTS);

    #[doc = "Enable Armv8.8-A Hinted Conditional Branches Extension."]
    pub const HBC: TargetFeatures = feature_set!(HBC);

    #[doc = "Enable Matrix Multiply Int8 Extension."]
    pub const I8MM: TargetFeatures = feature_set!(I8MM);

    #[doc = "Enable Armv8.3-A JavaScript FP conversion instructions."]
    pub const JSCONV: TargetFeatures = feature_set!(JSCONV, NEON);

    #[doc = "Enable Armv8.1-A Limited Ordering Regions extension."]
    pub const LOR: TargetFeatures = feature_set!(LOR);

    #[doc = "Enable Armv8.1-A Large System Extension (LSE) atomic instructions."]
    pub const LSE: TargetFeatures = feature_set!(LSE);

    #[doc = "Enable Armv9.4-A 128-bit Atomic instructions."]
    pub const LSE128: TargetFeatures = feature_set!(LSE, LSE128);

    #[doc = "Enable Armv8.4-A Large System Extension 2 (LSE2) atomicity rules."]
    pub const LSE2: TargetFeatures = feature_set!(LSE2);

    #[doc = "Enable Lookup Table instructions."]
    pub const LUT: TargetFeatures = feature_set!(LUT);

    #[doc = "Enable Armv8.8-A memcpy and memset acceleration instructions."]
    pub const MOPS: TargetFeatures = feature_set!(MOPS);

    #[doc = "Enable Memory Tagging Extension."]
    pub const MTE: TargetFeatures = feature_set!(MTE);

    #[doc = "Enable Advanced SIMD instructions."]
    pub const NEON: TargetFeatures = feature_set!(NEON);

    #[doc = "Enable out of line atomics to support LSE instructions."]
    pub const OUTLINE_ATOMICS: TargetFeatures = feature_set!(OUTLINE_ATOMICS);

    #[doc = "Enable Armv8.3-A Pointer Authentication extension."]
    pub const PACA: TargetFeatures = feature_set!(PACA, PACG);

    #[doc = "Enable Armv8.3-A Pointer Authentication extension."]
    pub const PACG: TargetFeatures = feature_set!(PACA, PACG);

    #[doc = "Enable Armv8.1-A Privileged Access-Never extension."]
    pub const PAN: TargetFeatures = feature_set!(PAN);

    #[doc = "Enable Armv9.5-A PAC enhancements."]
    pub const PAUTH_LR: TargetFeatures = feature_set!(PAUTH_LR);

    #[doc = "Enable Armv8.0-A PMUv3 Performance Monitors extension."]
    pub const PMUV3: TargetFeatures = feature_set!(PMUV3);

    #[doc = "Enable Random Number generation instructions."]
    pub const RAND: TargetFeatures = feature_set!(RAND);

    #[doc = "Enable Armv8.0-A Reliability, Availability and Serviceability Extensions."]
    pub const RAS: TargetFeatures = feature_set!(RAS);

    #[doc = "Enable support for RCPC extension."]
    pub const RCPC: TargetFeatures = feature_set!(RCPC);

    #[doc = "Enable Armv8.4-A RCPC instructions with Immediate Offsets."]
    pub const RCPC2: TargetFeatures = feature_set!(RCPC, RCPC2);

    #[doc = "Enable Armv8.9-A RCPC instructions for A64 and Advanced SIMD and floating-point instruction set."]
    pub const RCPC3: TargetFeatures = feature_set!(RCPC, RCPC2, RCPC3);

    #[doc = "Enable Armv8.1-A Rounding Double Multiply Add/Subtract instructions."]
    pub const RDM: TargetFeatures = feature_set!(NEON, RDM);

    #[doc = "Enable Armv8.5-A Speculation Barrier."]
    pub const SB: TargetFeatures = feature_set!(SB);

    #[doc = "Enable SHA1 and SHA256 support."]
    pub const SHA2: TargetFeatures = feature_set!(NEON, SHA2);

    #[doc = "Enable SHA512 and SHA3 support."]
    pub const SHA3: TargetFeatures = feature_set!(NEON, SHA2, SHA3);

    #[doc = "Enable SM3 and SM4 support."]
    pub const SM4: TargetFeatures = feature_set!(NEON, SM4);

    #[doc = "Enable Scalable Matrix Extension (SME)."]
    pub const SME: TargetFeatures = feature_set!(BF16, SME);

    #[doc = "Enable SME2.1 ZA-targeting non-widening BFloat16 instructions."]
    pub const SME_B16B16: TargetFeatures = feature_set!(BF16, SME, SME_B16B16, SME2, SVE_B16B16);

    #[doc = "Enable SME non-widening Float16 instructions."]
    pub const SME_F16F16: TargetFeatures = feature_set!(BF16, SME, SME_F16F16, SME2);

    #[doc = "Enable Scalable Matrix Extension (SME) F64F64 instructions."]
    pub const SME_F64F64: TargetFeatures = feature_set!(BF16, SME, SME_F64F64);

    #[doc = "Enable Scalable Matrix Extension (SME) F8F16 instructions."]
    pub const SME_F8F16: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME_F8F16, SME_F8F32, SME2);

    #[doc = "Enable Scalable Matrix Extension (SME) F8F32 instructions."]
    pub const SME_F8F32: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME_F8F32, SME2);

    #[doc = "Enable the full A64 instruction set in streaming SVE mode."]
    pub const SME_FA64: TargetFeatures = feature_set!(BF16, FP16, NEON, SME, SME_FA64, SVE, SVE2);

    #[doc = "Enable Scalable Matrix Extension (SME) I16I64 instructions."]
    pub const SME_I16I64: TargetFeatures = feature_set!(BF16, SME, SME_I16I64);

    #[doc = "Enable Scalable Matrix Extension (SME) LUTv2 instructions."]
    pub const SME_LUTV2: TargetFeatures = feature_set!(SME_LUTV2);

    #[doc = "Enable Scalable Matrix Extension 2 (SME2) instructions."]
    pub const SME2: TargetFeatures = feature_set!(BF16, SME, SME2);

    #[doc = "Enable Scalable Matrix Extension 2.1 instructions."]
    pub const SME2P1: TargetFeatures = feature_set!(BF16, SME, SME2, SME2P1);

    #[doc = "Enable Statistical Profiling extension."]
    pub const SPE: TargetFeatures = feature_set!(SPE);

    #[doc = "Enable Speculative Store Bypass Safe bit."]
    pub const SSBS: TargetFeatures = feature_set!(SSBS);

    #[doc = "Enable SVE2 FP8 2-way dot product instructions."]
    pub const SSVE_FP8DOT2: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME2, SSVE_FP8DOT2, SSVE_FP8DOT4, SSVE_FP8FMA);

    #[doc = "Enable SVE2 FP8 4-way dot product instructions."]
    pub const SSVE_FP8DOT4: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME2, SSVE_FP8DOT4, SSVE_FP8FMA);

    #[doc = "Enable SVE2 FP8 multiply-add instructions."]
    pub const SSVE_FP8FMA: TargetFeatures = feature_set!(BF16, FAMINMAX, FP8, LUT, SME, SME2, SSVE_FP8FMA);

    #[doc = "Enable Scalable Vector Extension (SVE) instructions."]
    pub const SVE: TargetFeatures = feature_set!(FP16, NEON, SVE);

    #[doc = "Enable SVE2 non-widening and SME2 Z-targeting non-widening BFloat16 instructions."]
    pub const SVE_B16B16: TargetFeatures = feature_set!(BF16, SVE_B16B16);

    #[doc = "Enable Scalable Vector Extension 2 (SVE2) instructions."]
    pub const SVE2: TargetFeatures = feature_set!(FP16, NEON, SVE, SVE2);

    #[doc = "Shorthand for +sve2+sve-aes."]
    pub const SVE2_AES: TargetFeatures = feature_set!(AES, FP16, NEON, SVE, SVE2, SVE2_AES);

    #[doc = "Shorthand for +sve2+sve-bitperm."]
    pub const SVE2_BITPERM: TargetFeatures = feature_set!(FP16, NEON, SVE, SVE2, SVE2_BITPERM);

    #[doc = "Shorthand for +sve2+sve-sha3."]
    pub const SVE2_SHA3: TargetFeatures = feature_set!(FP16, NEON, SHA2, SHA3, SVE, SVE2, SVE2_SHA3);

    #[doc = "Shorthand for +sve2+sve-sm4."]
    pub const SVE2_SM4: TargetFeatures = feature_set!(FP16, NEON, SM4, SVE, SVE2, SVE2_SM4);

    #[doc = "Enable Scalable Vector Extension 2.1 instructions."]
    pub const SVE2P1: TargetFeatures = feature_set!(FP16, NEON, SVE, SVE2, SVE2P1);

    #[doc = "Support ARM v8.1a architecture."]
    pub const V8_1A: TargetFeatures = feature_set!(CRC, LOR, LSE, NEON, PAN, RDM, V8_1A, VH);

    #[doc = "Support ARM v8.2a architecture."]
    pub const V8_2A: TargetFeatures = feature_set!(CRC, DPB, LOR, LSE, NEON, PAN, RAS, RDM, V8_1A, V8_2A, VH);

    #[doc = "Support ARM v8.3a architecture."]
    pub const V8_3A: TargetFeatures = feature_set!(CRC, DPB, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, V8_1A, V8_2A, V8_3A, VH);

    #[doc = "Support ARM v8.4a architecture."]
    pub const V8_4A: TargetFeatures = feature_set!(CRC, DIT, DOTPROD, DPB, FLAGM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, V8_1A, V8_2A, V8_3A, V8_4A, VH);

    #[doc = "Support ARM v8.5a architecture."]
    pub const V8_5A: TargetFeatures = feature_set!(BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, VH);

    #[doc = "Support ARM v8.6a architecture."]
    pub const V8_6A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, I8MM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, VH);

    #[doc = "Support ARM v8.7a architecture."]
    pub const V8_7A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, I8MM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, VH, WFXT);

    #[doc = "Support ARM v8.8a architecture."]
    pub const V8_8A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, VH, WFXT);

    #[doc = "Support ARM v8.9a architecture."]
    pub const V8_9A: TargetFeatures = feature_set!(BF16, BTI, CRC, CSSC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, V8_9A, VH, WFXT);

    #[doc = "Support ARM v9.1a architecture."]
    pub const V9_1A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, I8MM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V9_1A, V9A, VH);

    #[doc = "Support ARM v9.2a architecture."]
    pub const V9_2A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, I8MM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V9_1A, V9_2A, V9A, VH, WFXT);

    #[doc = "Support ARM v9.3a architecture."]
    pub const V9_3A: TargetFeatures = feature_set!(BF16, BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, V9_1A, V9_2A, V9_3A, V9A, VH, WFXT);

    #[doc = "Support ARM v9.4a architecture."]
    pub const V9_4A: TargetFeatures = feature_set!(BF16, BTI, CRC, CSSC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, V8_9A, V9_1A, V9_2A, V9_3A, V9_4A, V9A, VH, WFXT);

    #[doc = "Support ARM v9.5a architecture."]
    pub const V9_5A: TargetFeatures = feature_set!(BF16, BTI, CRC, CSSC, DIT, DOTPROD, DPB, DPB2, FLAGM, HBC, I8MM, JSCONV, LOR, LSE, MOPS, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V8_6A, V8_7A, V8_8A, V8_9A, V9_1A, V9_2A, V9_3A, V9_4A, V9_5A, V9A, VH, WFXT);

    #[doc = "Support ARM v9a architecture."]
    pub const V9A: TargetFeatures = feature_set!(BTI, CRC, DIT, DOTPROD, DPB, DPB2, FLAGM, JSCONV, LOR, LSE, NEON, PACA, PACG, PAN, RAS, RCPC, RDM, SB, SSBS, V8_1A, V8_2A, V8_3A, V8_4A, V8_5A, V9A, VH);

    #[doc = "Enable Armv8.1-A Virtual Host extension."]
    pub const VH: TargetFeatures = feature_set!(VH);

    #[doc = "Enable Armv8.7-A WFET and WFIT instruction."]
    pub const WFXT: TargetFeatures = feature_set!(WFXT);


    #[cfg(target_arch = "arm64ec")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("aes", AES),
        FeatureData::new("bf16", BF16),
        FeatureData::new("bti", BTI),
        FeatureData::new("crc", CRC),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("cssc", CSSC),
        FeatureData::new("dit", DIT),
        FeatureData::new("dotprod", DOTPROD),
        FeatureData::new("dpb", DPB),
        FeatureData::new("dpb2", DPB2),
        FeatureData::new("ecv", ECV),
        FeatureData::new("f32mm", F32MM),
        FeatureData::new("f64mm", F64MM),
        FeatureData::new("faminmax", FAMINMAX),
        FeatureData::new("fcma", FCMA),
        FeatureData::new("fhm", FHM),
        FeatureData::new("flagm", FLAGM),
        FeatureData::new("flagm2", FLAGM2),
        FeatureData::new("fp16", FP16),
        FeatureData::new("fp8", FP8),
        FeatureData::new("fp8dot2", FP8DOT2),
        FeatureData::new("fp8dot4", FP8DOT4),
        FeatureData::new("fp8fma", FP8FMA),
        FeatureData::new("frintts", FRINTTS),
        FeatureData::new("hbc", HBC),
        FeatureData::new("i8mm", I8MM),
        FeatureData::new("jsconv", JSCONV),
        FeatureData::new("lor", LOR),
        FeatureData::new("lse", LSE),
        FeatureData::new("lse128", LSE128),
        FeatureData::new("lse2", LSE2),
        FeatureData::new("lut", LUT),
        FeatureData::new("mops", MOPS),
        FeatureData::new("mte", MTE),
        FeatureData::new("neon", NEON),
        FeatureData::new("outline-atomics", OUTLINE_ATOMICS),
        FeatureData::new("paca", PACA),
        FeatureData::new("pacg", PACG),
        FeatureData::new("pan", PAN),
        FeatureData::new("pauth-lr", PAUTH_LR),
        FeatureData::new("pmuv3", PMUV3),
        FeatureData::new("rand", RAND),
        FeatureData::new("ras", RAS),
        FeatureData::new("rcpc", RCPC),
        FeatureData::new("rcpc2", RCPC2),
        FeatureData::new("rcpc3", RCPC3),
        FeatureData::new("rdm", RDM),
        FeatureData::new("sb", SB),
        FeatureData::new("sha2", SHA2),
        FeatureData::new("sha3", SHA3),
        FeatureData::new("sm4", SM4),
        FeatureData::new("sme", SME),
        FeatureData::new("sme-b16b16", SME_B16B16),
        FeatureData::new("sme-f16f16", SME_F16F16),
        FeatureData::new("sme-f64f64", SME_F64F64),
        FeatureData::new("sme-f8f16", SME_F8F16),
        FeatureData::new("sme-f8f32", SME_F8F32),
        FeatureData::new("sme-fa64", SME_FA64),
        FeatureData::new("sme-i16i64", SME_I16I64),
        FeatureData::new("sme-lutv2", SME_LUTV2),
        FeatureData::new("sme2", SME2),
        FeatureData::new("sme2p1", SME2P1),
        FeatureData::new("spe", SPE),
        FeatureData::new("ssbs", SSBS),
        FeatureData::new("ssve-fp8dot2", SSVE_FP8DOT2),
        FeatureData::new("ssve-fp8dot4", SSVE_FP8DOT4),
        FeatureData::new("ssve-fp8fma", SSVE_FP8FMA),
        FeatureData::new("sve", SVE),
        FeatureData::new("sve-b16b16", SVE_B16B16),
        FeatureData::new("sve2", SVE2),
        FeatureData::new("sve2-aes", SVE2_AES),
        FeatureData::new("sve2-bitperm", SVE2_BITPERM),
        FeatureData::new("sve2-sha3", SVE2_SHA3),
        FeatureData::new("sve2-sm4", SVE2_SM4),
        FeatureData::new("sve2p1", SVE2P1),
        FeatureData::new("v8.1a", V8_1A),
        FeatureData::new("v8.2a", V8_2A),
        FeatureData::new("v8.3a", V8_3A),
        FeatureData::new("v8.4a", V8_4A),
        FeatureData::new("v8.5a", V8_5A),
        FeatureData::new("v8.6a", V8_6A),
        FeatureData::new("v8.7a", V8_7A),
        FeatureData::new("v8.8a", V8_8A),
        FeatureData::new("v8.9a", V8_9A),
        FeatureData::new("v9.1a", V9_1A),
        FeatureData::new("v9.2a", V9_2A),
        FeatureData::new("v9.3a", V9_3A),
        FeatureData::new("v9.4a", V9_4A),
        FeatureData::new("v9.5a", V9_5A),
        FeatureData::new("v9a", V9A),
        FeatureData::new("vh", VH),
        FeatureData::new("wfxt", WFXT),
    ];

    #[cfg(target_arch = "arm64ec")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "aes")]
        let features = features.with(AES);
        #[cfg(target_feature = "bf16")]
        let features = features.with(BF16);
        #[cfg(target_feature = "bti")]
        let features = features.with(BTI);
        #[cfg(target_feature = "crc")]
        let features = features.with(CRC);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "cssc")]
        let features = features.with(CSSC);
        #[cfg(target_feature = "dit")]
        let features = features.with(DIT);
        #[cfg(target_feature = "dotprod")]
        let features = features.with(DOTPROD);
        #[cfg(target_feature = "dpb")]
        let features = features.with(DPB);
        #[cfg(target_feature = "dpb2")]
        let features = features.with(DPB2);
        #[cfg(target_feature = "ecv")]
        let features = features.with(ECV);
        #[cfg(target_feature = "f32mm")]
        let features = features.with(F32MM);
        #[cfg(target_feature = "f64mm")]
        let features = features.with(F64MM);
        #[cfg(target_feature = "faminmax")]
        let features = features.with(FAMINMAX);
        #[cfg(target_feature = "fcma")]
        let features = features.with(FCMA);
        #[cfg(target_feature = "fhm")]
        let features = features.with(FHM);
        #[cfg(target_feature = "flagm")]
        let features = features.with(FLAGM);
        #[cfg(target_feature = "flagm2")]
        let features = features.with(FLAGM2);
        #[cfg(target_feature = "fp16")]
        let features = features.with(FP16);
        #[cfg(target_feature = "fp8")]
        let features = features.with(FP8);
        #[cfg(target_feature = "fp8dot2")]
        let features = features.with(FP8DOT2);
        #[cfg(target_feature = "fp8dot4")]
        let features = features.with(FP8DOT4);
        #[cfg(target_feature = "fp8fma")]
        let features = features.with(FP8FMA);
        #[cfg(target_feature = "frintts")]
        let features = features.with(FRINTTS);
        #[cfg(target_feature = "hbc")]
        let features = features.with(HBC);
        #[cfg(target_feature = "i8mm")]
        let features = features.with(I8MM);
        #[cfg(target_feature = "jsconv")]
        let features = features.with(JSCONV);
        #[cfg(target_feature = "lor")]
        let features = features.with(LOR);
        #[cfg(target_feature = "lse")]
        let features = features.with(LSE);
        #[cfg(target_feature = "lse128")]
        let features = features.with(LSE128);
        #[cfg(target_feature = "lse2")]
        let features = features.with(LSE2);
        #[cfg(target_feature = "lut")]
        let features = features.with(LUT);
        #[cfg(target_feature = "mops")]
        let features = features.with(MOPS);
        #[cfg(target_feature = "mte")]
        let features = features.with(MTE);
        #[cfg(target_feature = "neon")]
        let features = features.with(NEON);
        #[cfg(target_feature = "outline-atomics")]
        let features = features.with(OUTLINE_ATOMICS);
        #[cfg(target_feature = "paca")]
        let features = features.with(PACA);
        #[cfg(target_feature = "pacg")]
        let features = features.with(PACG);
        #[cfg(target_feature = "pan")]
        let features = features.with(PAN);
        #[cfg(target_feature = "pauth-lr")]
        let features = features.with(PAUTH_LR);
        #[cfg(target_feature = "pmuv3")]
        let features = features.with(PMUV3);
        #[cfg(target_feature = "rand")]
        let features = features.with(RAND);
        #[cfg(target_feature = "ras")]
        let features = features.with(RAS);
        #[cfg(target_feature = "rcpc")]
        let features = features.with(RCPC);
        #[cfg(target_feature = "rcpc2")]
        let features = features.with(RCPC2);
        #[cfg(target_feature = "rcpc3")]
        let features = features.with(RCPC3);
        #[cfg(target_feature = "rdm")]
        let features = features.with(RDM);
        #[cfg(target_feature = "sb")]
        let features = features.with(SB);
        #[cfg(target_feature = "sha2")]
        let features = features.with(SHA2);
        #[cfg(target_feature = "sha3")]
        let features = features.with(SHA3);
        #[cfg(target_feature = "sm4")]
        let features = features.with(SM4);
        #[cfg(target_feature = "sme")]
        let features = features.with(SME);
        #[cfg(target_feature = "sme-b16b16")]
        let features = features.with(SME_B16B16);
        #[cfg(target_feature = "sme-f16f16")]
        let features = features.with(SME_F16F16);
        #[cfg(target_feature = "sme-f64f64")]
        let features = features.with(SME_F64F64);
        #[cfg(target_feature = "sme-f8f16")]
        let features = features.with(SME_F8F16);
        #[cfg(target_feature = "sme-f8f32")]
        let features = features.with(SME_F8F32);
        #[cfg(target_feature = "sme-fa64")]
        let features = features.with(SME_FA64);
        #[cfg(target_feature = "sme-i16i64")]
        let features = features.with(SME_I16I64);
        #[cfg(target_feature = "sme-lutv2")]
        let features = features.with(SME_LUTV2);
        #[cfg(target_feature = "sme2")]
        let features = features.with(SME2);
        #[cfg(target_feature = "sme2p1")]
        let features = features.with(SME2P1);
        #[cfg(target_feature = "spe")]
        let features = features.with(SPE);
        #[cfg(target_feature = "ssbs")]
        let features = features.with(SSBS);
        #[cfg(target_feature = "ssve-fp8dot2")]
        let features = features.with(SSVE_FP8DOT2);
        #[cfg(target_feature = "ssve-fp8dot4")]
        let features = features.with(SSVE_FP8DOT4);
        #[cfg(target_feature = "ssve-fp8fma")]
        let features = features.with(SSVE_FP8FMA);
        #[cfg(target_feature = "sve")]
        let features = features.with(SVE);
        #[cfg(target_feature = "sve-b16b16")]
        let features = features.with(SVE_B16B16);
        #[cfg(target_feature = "sve2")]
        let features = features.with(SVE2);
        #[cfg(target_feature = "sve2-aes")]
        let features = features.with(SVE2_AES);
        #[cfg(target_feature = "sve2-bitperm")]
        let features = features.with(SVE2_BITPERM);
        #[cfg(target_feature = "sve2-sha3")]
        let features = features.with(SVE2_SHA3);
        #[cfg(target_feature = "sve2-sm4")]
        let features = features.with(SVE2_SM4);
        #[cfg(target_feature = "sve2p1")]
        let features = features.with(SVE2P1);
        #[cfg(target_feature = "v8.1a")]
        let features = features.with(V8_1A);
        #[cfg(target_feature = "v8.2a")]
        let features = features.with(V8_2A);
        #[cfg(target_feature = "v8.3a")]
        let features = features.with(V8_3A);
        #[cfg(target_feature = "v8.4a")]
        let features = features.with(V8_4A);
        #[cfg(target_feature = "v8.5a")]
        let features = features.with(V8_5A);
        #[cfg(target_feature = "v8.6a")]
        let features = features.with(V8_6A);
        #[cfg(target_feature = "v8.7a")]
        let features = features.with(V8_7A);
        #[cfg(target_feature = "v8.8a")]
        let features = features.with(V8_8A);
        #[cfg(target_feature = "v8.9a")]
        let features = features.with(V8_9A);
        #[cfg(target_feature = "v9.1a")]
        let features = features.with(V9_1A);
        #[cfg(target_feature = "v9.2a")]
        let features = features.with(V9_2A);
        #[cfg(target_feature = "v9.3a")]
        let features = features.with(V9_3A);
        #[cfg(target_feature = "v9.4a")]
        let features = features.with(V9_4A);
        #[cfg(target_feature = "v9.5a")]
        let features = features.with(V9_5A);
        #[cfg(target_feature = "v9a")]
        let features = features.with(V9A);
        #[cfg(target_feature = "vh")]
        let features = features.with(VH);
        #[cfg(target_feature = "wfxt")]
        let features = features.with(WFXT);
        features
    }

}
#[cfg(any(doc, target_arch = "bpf"))]
#[rustfmt::skip]
pub mod bpf {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        ALLOWS_MISALIGNED_MEM_ACCESS,
        ALU32,
        CRT_STATIC,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "bpf")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "bpf")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Allows misaligned memory access."]
    pub const ALLOWS_MISALIGNED_MEM_ACCESS: TargetFeatures = feature_set!(ALLOWS_MISALIGNED_MEM_ACCESS);

    #[doc = "Enable ALU32 instructions."]
    pub const ALU32: TargetFeatures = feature_set!(ALU32);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);


    #[cfg(target_arch = "bpf")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("allows-misaligned-mem-access", ALLOWS_MISALIGNED_MEM_ACCESS),
        FeatureData::new("alu32", ALU32),
        FeatureData::new("crt-static", CRT_STATIC),
    ];

    #[cfg(target_arch = "bpf")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "allows-misaligned-mem-access")]
        let features = features.with(ALLOWS_MISALIGNED_MEM_ACCESS);
        #[cfg(target_feature = "alu32")]
        let features = features.with(ALU32);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        features
    }

}
#[cfg(any(doc, target_arch = "hexagon"))]
#[rustfmt::skip]
pub mod hexagon {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        AUDIO,
        CRT_STATIC,
        HVX,
        HVX_IEEE_FP,
        HVX_LENGTH128B,
        HVX_LENGTH64B,
        HVX_QFLOAT,
        HVXV60,
        HVXV62,
        HVXV65,
        HVXV66,
        HVXV67,
        HVXV68,
        HVXV69,
        HVXV71,
        HVXV73,
        HVXV75,
        HVXV79,
        V60,
        V62,
        V65,
        V66,
        V67,
        V68,
        V69,
        V71,
        V73,
        V75,
        V79,
        ZREG,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "hexagon")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "hexagon")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Hexagon Audio extension instructions."]
    pub const AUDIO: TargetFeatures = feature_set!(AUDIO);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Hexagon HVX instructions."]
    pub const HVX: TargetFeatures = feature_set!(HVX);

    #[doc = "Hexagon HVX IEEE floating point instructions."]
    pub const HVX_IEEE_FP: TargetFeatures = feature_set!(HVX, HVX_IEEE_FP);

    #[doc = "Hexagon HVX 128B instructions."]
    pub const HVX_LENGTH128B: TargetFeatures = feature_set!(HVX, HVX_LENGTH128B);

    #[doc = "Hexagon HVX 64B instructions."]
    pub const HVX_LENGTH64B: TargetFeatures = feature_set!(HVX, HVX_LENGTH64B);

    #[doc = "Hexagon HVX QFloating point instructions."]
    pub const HVX_QFLOAT: TargetFeatures = feature_set!(HVX, HVX_QFLOAT);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV60: TargetFeatures = feature_set!(HVX, HVXV60);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV62: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV65: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV66: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65, HVXV66, ZREG);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV67: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65, HVXV66, HVXV67, ZREG);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV68: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65, HVXV66, HVXV67, HVXV68, ZREG);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV69: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65, HVXV66, HVXV67, HVXV68, HVXV69, ZREG);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV71: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65, HVXV66, HVXV67, HVXV68, HVXV69, HVXV71, ZREG);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV73: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65, HVXV66, HVXV67, HVXV68, HVXV69, HVXV71, HVXV73, ZREG);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV75: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65, HVXV66, HVXV67, HVXV68, HVXV69, HVXV71, HVXV73, HVXV75, ZREG);

    #[doc = "Hexagon HVX instructions."]
    pub const HVXV79: TargetFeatures = feature_set!(HVX, HVXV60, HVXV62, HVXV65, HVXV66, HVXV67, HVXV68, HVXV69, HVXV71, HVXV73, HVXV75, HVXV79, ZREG);

    #[doc = "Enable Hexagon V60 architecture."]
    pub const V60: TargetFeatures = feature_set!(V60);

    #[doc = "Enable Hexagon V62 architecture."]
    pub const V62: TargetFeatures = feature_set!(V60, V62);

    #[doc = "Enable Hexagon V65 architecture."]
    pub const V65: TargetFeatures = feature_set!(V60, V62, V65);

    #[doc = "Enable Hexagon V66 architecture."]
    pub const V66: TargetFeatures = feature_set!(V60, V62, V65, V66);

    #[doc = "Enable Hexagon V67 architecture."]
    pub const V67: TargetFeatures = feature_set!(V60, V62, V65, V66, V67);

    #[doc = "Enable Hexagon V68 architecture."]
    pub const V68: TargetFeatures = feature_set!(V60, V62, V65, V66, V67, V68);

    #[doc = "Enable Hexagon V69 architecture."]
    pub const V69: TargetFeatures = feature_set!(V60, V62, V65, V66, V67, V68, V69);

    #[doc = "Enable Hexagon V71 architecture."]
    pub const V71: TargetFeatures = feature_set!(V60, V62, V65, V66, V67, V68, V69, V71);

    #[doc = "Enable Hexagon V73 architecture."]
    pub const V73: TargetFeatures = feature_set!(V60, V62, V65, V66, V67, V68, V69, V71, V73);

    #[doc = "Enable Hexagon V75 architecture."]
    pub const V75: TargetFeatures = feature_set!(V60, V62, V65, V66, V67, V68, V69, V71, V73, V75);

    #[doc = "Enable Hexagon V79 architecture."]
    pub const V79: TargetFeatures = feature_set!(V60, V62, V65, V66, V67, V68, V69, V71, V73, V75, V79);

    #[doc = "Hexagon ZReg extension instructions."]
    pub const ZREG: TargetFeatures = feature_set!(ZREG);


    #[cfg(target_arch = "hexagon")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("audio", AUDIO),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("hvx", HVX),
        FeatureData::new("hvx-ieee-fp", HVX_IEEE_FP),
        FeatureData::new("hvx-length128b", HVX_LENGTH128B),
        FeatureData::new("hvx-length64b", HVX_LENGTH64B),
        FeatureData::new("hvx-qfloat", HVX_QFLOAT),
        FeatureData::new("hvxv60", HVXV60),
        FeatureData::new("hvxv62", HVXV62),
        FeatureData::new("hvxv65", HVXV65),
        FeatureData::new("hvxv66", HVXV66),
        FeatureData::new("hvxv67", HVXV67),
        FeatureData::new("hvxv68", HVXV68),
        FeatureData::new("hvxv69", HVXV69),
        FeatureData::new("hvxv71", HVXV71),
        FeatureData::new("hvxv73", HVXV73),
        FeatureData::new("hvxv75", HVXV75),
        FeatureData::new("hvxv79", HVXV79),
        FeatureData::new("v60", V60),
        FeatureData::new("v62", V62),
        FeatureData::new("v65", V65),
        FeatureData::new("v66", V66),
        FeatureData::new("v67", V67),
        FeatureData::new("v68", V68),
        FeatureData::new("v69", V69),
        FeatureData::new("v71", V71),
        FeatureData::new("v73", V73),
        FeatureData::new("v75", V75),
        FeatureData::new("v79", V79),
        FeatureData::new("zreg", ZREG),
    ];

    #[cfg(target_arch = "hexagon")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "audio")]
        let features = features.with(AUDIO);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "hvx")]
        let features = features.with(HVX);
        #[cfg(target_feature = "hvx-ieee-fp")]
        let features = features.with(HVX_IEEE_FP);
        #[cfg(target_feature = "hvx-length128b")]
        let features = features.with(HVX_LENGTH128B);
        #[cfg(target_feature = "hvx-length64b")]
        let features = features.with(HVX_LENGTH64B);
        #[cfg(target_feature = "hvx-qfloat")]
        let features = features.with(HVX_QFLOAT);
        #[cfg(target_feature = "hvxv60")]
        let features = features.with(HVXV60);
        #[cfg(target_feature = "hvxv62")]
        let features = features.with(HVXV62);
        #[cfg(target_feature = "hvxv65")]
        let features = features.with(HVXV65);
        #[cfg(target_feature = "hvxv66")]
        let features = features.with(HVXV66);
        #[cfg(target_feature = "hvxv67")]
        let features = features.with(HVXV67);
        #[cfg(target_feature = "hvxv68")]
        let features = features.with(HVXV68);
        #[cfg(target_feature = "hvxv69")]
        let features = features.with(HVXV69);
        #[cfg(target_feature = "hvxv71")]
        let features = features.with(HVXV71);
        #[cfg(target_feature = "hvxv73")]
        let features = features.with(HVXV73);
        #[cfg(target_feature = "hvxv75")]
        let features = features.with(HVXV75);
        #[cfg(target_feature = "hvxv79")]
        let features = features.with(HVXV79);
        #[cfg(target_feature = "v60")]
        let features = features.with(V60);
        #[cfg(target_feature = "v62")]
        let features = features.with(V62);
        #[cfg(target_feature = "v65")]
        let features = features.with(V65);
        #[cfg(target_feature = "v66")]
        let features = features.with(V66);
        #[cfg(target_feature = "v67")]
        let features = features.with(V67);
        #[cfg(target_feature = "v68")]
        let features = features.with(V68);
        #[cfg(target_feature = "v69")]
        let features = features.with(V69);
        #[cfg(target_feature = "v71")]
        let features = features.with(V71);
        #[cfg(target_feature = "v73")]
        let features = features.with(V73);
        #[cfg(target_feature = "v75")]
        let features = features.with(V75);
        #[cfg(target_feature = "v79")]
        let features = features.with(V79);
        #[cfg(target_feature = "zreg")]
        let features = features.with(ZREG);
        features
    }

}
#[cfg(any(doc, target_arch = "mips"))]
#[rustfmt::skip]
pub mod mips {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        CRT_STATIC,
        FP64,
        MSA,
        VIRT,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "mips")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "mips")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Support 64-bit FP registers."]
    pub const FP64: TargetFeatures = feature_set!(FP64);

    #[doc = "Mips MSA ASE."]
    pub const MSA: TargetFeatures = feature_set!(MSA);

    #[doc = "Mips Virtualization ASE."]
    pub const VIRT: TargetFeatures = feature_set!(VIRT);


    #[cfg(target_arch = "mips")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("fp64", FP64),
        FeatureData::new("msa", MSA),
        FeatureData::new("virt", VIRT),
    ];

    #[cfg(target_arch = "mips")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "fp64")]
        let features = features.with(FP64);
        #[cfg(target_feature = "msa")]
        let features = features.with(MSA);
        #[cfg(target_feature = "virt")]
        let features = features.with(VIRT);
        features
    }

}
#[cfg(any(doc, target_arch = "mips64"))]
#[rustfmt::skip]
pub mod mips64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        CRT_STATIC,
        FP64,
        MSA,
        VIRT,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "mips64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "mips64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Support 64-bit FP registers."]
    pub const FP64: TargetFeatures = feature_set!(FP64);

    #[doc = "Mips MSA ASE."]
    pub const MSA: TargetFeatures = feature_set!(MSA);

    #[doc = "Mips Virtualization ASE."]
    pub const VIRT: TargetFeatures = feature_set!(VIRT);


    #[cfg(target_arch = "mips64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("fp64", FP64),
        FeatureData::new("msa", MSA),
        FeatureData::new("virt", VIRT),
    ];

    #[cfg(target_arch = "mips64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "fp64")]
        let features = features.with(FP64);
        #[cfg(target_feature = "msa")]
        let features = features.with(MSA);
        #[cfg(target_feature = "virt")]
        let features = features.with(VIRT);
        features
    }

}
#[cfg(any(doc, target_arch = "loongarch32"))]
#[rustfmt::skip]
pub mod loongarch32 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        F_32S,
        CRT_STATIC,
        D,
        DIV32,
        F,
        FRECIPE,
        LAM_BH,
        LAMCAS,
        LASX,
        LBT,
        LD_SEQ_SA,
        LSX,
        LVZ,
        RELAX,
        SCQ,
        UAL,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "loongarch32")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "loongarch32")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "LA32 Standard Basic Instruction Extension."]
    pub const F_32S: TargetFeatures = feature_set!(F_32S);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "'D' (Double-Precision Floating-Point)."]
    pub const D: TargetFeatures = feature_set!(D, F);

    #[doc = "Assume div.w\\[u\\] and mod.w\\[u\\] can handle inputs that are not sign-extended."]
    pub const DIV32: TargetFeatures = feature_set!(DIV32);

    #[doc = "'F' (Single-Precision Floating-Point)."]
    pub const F: TargetFeatures = feature_set!(F);

    #[doc = "Support frecipe.{s/d} and frsqrte.{s/d} instructions."]
    pub const FRECIPE: TargetFeatures = feature_set!(FRECIPE);

    #[doc = "Support amswap\\[_db\\].{b/h} and amadd\\[_db\\].{b/h} instructions."]
    pub const LAM_BH: TargetFeatures = feature_set!(LAM_BH);

    #[doc = "Support amcas\\[_db\\].{b/h/w/d}."]
    pub const LAMCAS: TargetFeatures = feature_set!(LAMCAS);

    #[doc = "'LASX' (Loongson Advanced SIMD Extension)."]
    pub const LASX: TargetFeatures = feature_set!(D, F, LASX, LSX);

    #[doc = "'LBT' (Loongson Binary Translation Extension)."]
    pub const LBT: TargetFeatures = feature_set!(LBT);

    #[doc = "Don't use a same-address load-load barrier (dbar 0x700)."]
    pub const LD_SEQ_SA: TargetFeatures = feature_set!(LD_SEQ_SA);

    #[doc = "'LSX' (Loongson SIMD Extension)."]
    pub const LSX: TargetFeatures = feature_set!(D, F, LSX);

    #[doc = "'LVZ' (Loongson Virtualization Extension)."]
    pub const LVZ: TargetFeatures = feature_set!(LVZ);

    #[doc = "Enable Linker relaxation."]
    pub const RELAX: TargetFeatures = feature_set!(RELAX);

    #[doc = "Support sc.q instruction."]
    pub const SCQ: TargetFeatures = feature_set!(SCQ);

    #[doc = "Allow memory accesses to be unaligned."]
    pub const UAL: TargetFeatures = feature_set!(UAL);


    #[cfg(target_arch = "loongarch32")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("32s", F_32S),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("d", D),
        FeatureData::new("div32", DIV32),
        FeatureData::new("f", F),
        FeatureData::new("frecipe", FRECIPE),
        FeatureData::new("lam-bh", LAM_BH),
        FeatureData::new("lamcas", LAMCAS),
        FeatureData::new("lasx", LASX),
        FeatureData::new("lbt", LBT),
        FeatureData::new("ld-seq-sa", LD_SEQ_SA),
        FeatureData::new("lsx", LSX),
        FeatureData::new("lvz", LVZ),
        FeatureData::new("relax", RELAX),
        FeatureData::new("scq", SCQ),
        FeatureData::new("ual", UAL),
    ];

    #[cfg(target_arch = "loongarch32")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "32s")]
        let features = features.with(F_32S);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "d")]
        let features = features.with(D);
        #[cfg(target_feature = "div32")]
        let features = features.with(DIV32);
        #[cfg(target_feature = "f")]
        let features = features.with(F);
        #[cfg(target_feature = "frecipe")]
        let features = features.with(FRECIPE);
        #[cfg(target_feature = "lam-bh")]
        let features = features.with(LAM_BH);
        #[cfg(target_feature = "lamcas")]
        let features = features.with(LAMCAS);
        #[cfg(target_feature = "lasx")]
        let features = features.with(LASX);
        #[cfg(target_feature = "lbt")]
        let features = features.with(LBT);
        #[cfg(target_feature = "ld-seq-sa")]
        let features = features.with(LD_SEQ_SA);
        #[cfg(target_feature = "lsx")]
        let features = features.with(LSX);
        #[cfg(target_feature = "lvz")]
        let features = features.with(LVZ);
        #[cfg(target_feature = "relax")]
        let features = features.with(RELAX);
        #[cfg(target_feature = "scq")]
        let features = features.with(SCQ);
        #[cfg(target_feature = "ual")]
        let features = features.with(UAL);
        features
    }

}
#[cfg(any(doc, target_arch = "loongarch64"))]
#[rustfmt::skip]
pub mod loongarch64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        F_32S,
        CRT_STATIC,
        D,
        DIV32,
        F,
        FRECIPE,
        LAM_BH,
        LAMCAS,
        LASX,
        LBT,
        LD_SEQ_SA,
        LSX,
        LVZ,
        RELAX,
        SCQ,
        UAL,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "loongarch64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "loongarch64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "LA32 Standard Basic Instruction Extension."]
    pub const F_32S: TargetFeatures = feature_set!(F_32S);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "'D' (Double-Precision Floating-Point)."]
    pub const D: TargetFeatures = feature_set!(D, F);

    #[doc = "Assume div.w\\[u\\] and mod.w\\[u\\] can handle inputs that are not sign-extended."]
    pub const DIV32: TargetFeatures = feature_set!(DIV32);

    #[doc = "'F' (Single-Precision Floating-Point)."]
    pub const F: TargetFeatures = feature_set!(F);

    #[doc = "Support frecipe.{s/d} and frsqrte.{s/d} instructions."]
    pub const FRECIPE: TargetFeatures = feature_set!(FRECIPE);

    #[doc = "Support amswap\\[_db\\].{b/h} and amadd\\[_db\\].{b/h} instructions."]
    pub const LAM_BH: TargetFeatures = feature_set!(LAM_BH);

    #[doc = "Support amcas\\[_db\\].{b/h/w/d}."]
    pub const LAMCAS: TargetFeatures = feature_set!(LAMCAS);

    #[doc = "'LASX' (Loongson Advanced SIMD Extension)."]
    pub const LASX: TargetFeatures = feature_set!(D, F, LASX, LSX);

    #[doc = "'LBT' (Loongson Binary Translation Extension)."]
    pub const LBT: TargetFeatures = feature_set!(LBT);

    #[doc = "Don't use a same-address load-load barrier (dbar 0x700)."]
    pub const LD_SEQ_SA: TargetFeatures = feature_set!(LD_SEQ_SA);

    #[doc = "'LSX' (Loongson SIMD Extension)."]
    pub const LSX: TargetFeatures = feature_set!(D, F, LSX);

    #[doc = "'LVZ' (Loongson Virtualization Extension)."]
    pub const LVZ: TargetFeatures = feature_set!(LVZ);

    #[doc = "Enable Linker relaxation."]
    pub const RELAX: TargetFeatures = feature_set!(RELAX);

    #[doc = "Support sc.q instruction."]
    pub const SCQ: TargetFeatures = feature_set!(SCQ);

    #[doc = "Allow memory accesses to be unaligned."]
    pub const UAL: TargetFeatures = feature_set!(UAL);


    #[cfg(target_arch = "loongarch64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("32s", F_32S),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("d", D),
        FeatureData::new("div32", DIV32),
        FeatureData::new("f", F),
        FeatureData::new("frecipe", FRECIPE),
        FeatureData::new("lam-bh", LAM_BH),
        FeatureData::new("lamcas", LAMCAS),
        FeatureData::new("lasx", LASX),
        FeatureData::new("lbt", LBT),
        FeatureData::new("ld-seq-sa", LD_SEQ_SA),
        FeatureData::new("lsx", LSX),
        FeatureData::new("lvz", LVZ),
        FeatureData::new("relax", RELAX),
        FeatureData::new("scq", SCQ),
        FeatureData::new("ual", UAL),
    ];

    #[cfg(target_arch = "loongarch64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "32s")]
        let features = features.with(F_32S);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "d")]
        let features = features.with(D);
        #[cfg(target_feature = "div32")]
        let features = features.with(DIV32);
        #[cfg(target_feature = "f")]
        let features = features.with(F);
        #[cfg(target_feature = "frecipe")]
        let features = features.with(FRECIPE);
        #[cfg(target_feature = "lam-bh")]
        let features = features.with(LAM_BH);
        #[cfg(target_feature = "lamcas")]
        let features = features.with(LAMCAS);
        #[cfg(target_feature = "lasx")]
        let features = features.with(LASX);
        #[cfg(target_feature = "lbt")]
        let features = features.with(LBT);
        #[cfg(target_feature = "ld-seq-sa")]
        let features = features.with(LD_SEQ_SA);
        #[cfg(target_feature = "lsx")]
        let features = features.with(LSX);
        #[cfg(target_feature = "lvz")]
        let features = features.with(LVZ);
        #[cfg(target_feature = "relax")]
        let features = features.with(RELAX);
        #[cfg(target_feature = "scq")]
        let features = features.with(SCQ);
        #[cfg(target_feature = "ual")]
        let features = features.with(UAL);
        features
    }

}
#[cfg(any(doc, target_arch = "nvptx64"))]
#[rustfmt::skip]
pub mod nvptx64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        CRT_STATIC,
        PTX70,
        PTX71,
        PTX72,
        PTX73,
        PTX74,
        PTX75,
        PTX76,
        PTX77,
        PTX78,
        PTX80,
        PTX81,
        PTX82,
        PTX83,
        PTX84,
        PTX85,
        PTX86,
        PTX87,
        SM_100,
        SM_100A,
        SM_101,
        SM_101A,
        SM_120,
        SM_120A,
        SM_70,
        SM_72,
        SM_75,
        SM_80,
        SM_86,
        SM_87,
        SM_89,
        SM_90,
        SM_90A,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "nvptx64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "nvptx64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Use PTX version 70."]
    pub const PTX70: TargetFeatures = feature_set!(PTX70);

    #[doc = "Use PTX version 71."]
    pub const PTX71: TargetFeatures = feature_set!(PTX70, PTX71);

    #[doc = "Use PTX version 72."]
    pub const PTX72: TargetFeatures = feature_set!(PTX70, PTX71, PTX72);

    #[doc = "Use PTX version 73."]
    pub const PTX73: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73);

    #[doc = "Use PTX version 74."]
    pub const PTX74: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74);

    #[doc = "Use PTX version 75."]
    pub const PTX75: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75);

    #[doc = "Use PTX version 76."]
    pub const PTX76: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76);

    #[doc = "Use PTX version 77."]
    pub const PTX77: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77);

    #[doc = "Use PTX version 78."]
    pub const PTX78: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78);

    #[doc = "Use PTX version 80."]
    pub const PTX80: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78, PTX80);

    #[doc = "Use PTX version 81."]
    pub const PTX81: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78, PTX80, PTX81);

    #[doc = "Use PTX version 82."]
    pub const PTX82: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78, PTX80, PTX81, PTX82);

    #[doc = "Use PTX version 83."]
    pub const PTX83: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78, PTX80, PTX81, PTX82, PTX83);

    #[doc = "Use PTX version 84."]
    pub const PTX84: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78, PTX80, PTX81, PTX82, PTX83, PTX84);

    #[doc = "Use PTX version 85."]
    pub const PTX85: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78, PTX80, PTX81, PTX82, PTX83, PTX84, PTX85);

    #[doc = "Use PTX version 86."]
    pub const PTX86: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78, PTX80, PTX81, PTX82, PTX83, PTX84, PTX85, PTX86);

    #[doc = "Use PTX version 87."]
    pub const PTX87: TargetFeatures = feature_set!(PTX70, PTX71, PTX72, PTX73, PTX74, PTX75, PTX76, PTX77, PTX78, PTX80, PTX81, PTX82, PTX83, PTX84, PTX85, PTX86, PTX87);

    #[doc = "Target SM 100."]
    pub const SM_100: TargetFeatures = feature_set!(SM_100, SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89, SM_90);

    #[doc = "Target SM 100a."]
    pub const SM_100A: TargetFeatures = feature_set!(SM_100, SM_100A, SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89, SM_90);

    #[doc = "Target SM 101."]
    pub const SM_101: TargetFeatures = feature_set!(SM_100, SM_101, SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89, SM_90);

    #[doc = "Target SM 101a."]
    pub const SM_101A: TargetFeatures = feature_set!(SM_100, SM_101, SM_101A, SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89, SM_90);

    #[doc = "Target SM 120."]
    pub const SM_120: TargetFeatures = feature_set!(SM_100, SM_101, SM_120, SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89, SM_90);

    #[doc = "Target SM 120a."]
    pub const SM_120A: TargetFeatures = feature_set!(SM_100, SM_101, SM_120, SM_120A, SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89, SM_90);

    #[doc = "Target SM 70."]
    pub const SM_70: TargetFeatures = feature_set!(SM_70);

    #[doc = "Target SM 72."]
    pub const SM_72: TargetFeatures = feature_set!(SM_70, SM_72);

    #[doc = "Target SM 75."]
    pub const SM_75: TargetFeatures = feature_set!(SM_70, SM_72, SM_75);

    #[doc = "Target SM 80."]
    pub const SM_80: TargetFeatures = feature_set!(SM_70, SM_72, SM_75, SM_80);

    #[doc = "Target SM 86."]
    pub const SM_86: TargetFeatures = feature_set!(SM_70, SM_72, SM_75, SM_80, SM_86);

    #[doc = "Target SM 87."]
    pub const SM_87: TargetFeatures = feature_set!(SM_70, SM_72, SM_75, SM_80, SM_86, SM_87);

    #[doc = "Target SM 89."]
    pub const SM_89: TargetFeatures = feature_set!(SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89);

    #[doc = "Target SM 90."]
    pub const SM_90: TargetFeatures = feature_set!(SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89, SM_90);

    #[doc = "Target SM 90a."]
    pub const SM_90A: TargetFeatures = feature_set!(SM_70, SM_72, SM_75, SM_80, SM_86, SM_87, SM_89, SM_90, SM_90A);


    #[cfg(target_arch = "nvptx64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("ptx70", PTX70),
        FeatureData::new("ptx71", PTX71),
        FeatureData::new("ptx72", PTX72),
        FeatureData::new("ptx73", PTX73),
        FeatureData::new("ptx74", PTX74),
        FeatureData::new("ptx75", PTX75),
        FeatureData::new("ptx76", PTX76),
        FeatureData::new("ptx77", PTX77),
        FeatureData::new("ptx78", PTX78),
        FeatureData::new("ptx80", PTX80),
        FeatureData::new("ptx81", PTX81),
        FeatureData::new("ptx82", PTX82),
        FeatureData::new("ptx83", PTX83),
        FeatureData::new("ptx84", PTX84),
        FeatureData::new("ptx85", PTX85),
        FeatureData::new("ptx86", PTX86),
        FeatureData::new("ptx87", PTX87),
        FeatureData::new("sm_100", SM_100),
        FeatureData::new("sm_100a", SM_100A),
        FeatureData::new("sm_101", SM_101),
        FeatureData::new("sm_101a", SM_101A),
        FeatureData::new("sm_120", SM_120),
        FeatureData::new("sm_120a", SM_120A),
        FeatureData::new("sm_70", SM_70),
        FeatureData::new("sm_72", SM_72),
        FeatureData::new("sm_75", SM_75),
        FeatureData::new("sm_80", SM_80),
        FeatureData::new("sm_86", SM_86),
        FeatureData::new("sm_87", SM_87),
        FeatureData::new("sm_89", SM_89),
        FeatureData::new("sm_90", SM_90),
        FeatureData::new("sm_90a", SM_90A),
    ];

    #[cfg(target_arch = "nvptx64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "ptx70")]
        let features = features.with(PTX70);
        #[cfg(target_feature = "ptx71")]
        let features = features.with(PTX71);
        #[cfg(target_feature = "ptx72")]
        let features = features.with(PTX72);
        #[cfg(target_feature = "ptx73")]
        let features = features.with(PTX73);
        #[cfg(target_feature = "ptx74")]
        let features = features.with(PTX74);
        #[cfg(target_feature = "ptx75")]
        let features = features.with(PTX75);
        #[cfg(target_feature = "ptx76")]
        let features = features.with(PTX76);
        #[cfg(target_feature = "ptx77")]
        let features = features.with(PTX77);
        #[cfg(target_feature = "ptx78")]
        let features = features.with(PTX78);
        #[cfg(target_feature = "ptx80")]
        let features = features.with(PTX80);
        #[cfg(target_feature = "ptx81")]
        let features = features.with(PTX81);
        #[cfg(target_feature = "ptx82")]
        let features = features.with(PTX82);
        #[cfg(target_feature = "ptx83")]
        let features = features.with(PTX83);
        #[cfg(target_feature = "ptx84")]
        let features = features.with(PTX84);
        #[cfg(target_feature = "ptx85")]
        let features = features.with(PTX85);
        #[cfg(target_feature = "ptx86")]
        let features = features.with(PTX86);
        #[cfg(target_feature = "ptx87")]
        let features = features.with(PTX87);
        #[cfg(target_feature = "sm_100")]
        let features = features.with(SM_100);
        #[cfg(target_feature = "sm_100a")]
        let features = features.with(SM_100A);
        #[cfg(target_feature = "sm_101")]
        let features = features.with(SM_101);
        #[cfg(target_feature = "sm_101a")]
        let features = features.with(SM_101A);
        #[cfg(target_feature = "sm_120")]
        let features = features.with(SM_120);
        #[cfg(target_feature = "sm_120a")]
        let features = features.with(SM_120A);
        #[cfg(target_feature = "sm_70")]
        let features = features.with(SM_70);
        #[cfg(target_feature = "sm_72")]
        let features = features.with(SM_72);
        #[cfg(target_feature = "sm_75")]
        let features = features.with(SM_75);
        #[cfg(target_feature = "sm_80")]
        let features = features.with(SM_80);
        #[cfg(target_feature = "sm_86")]
        let features = features.with(SM_86);
        #[cfg(target_feature = "sm_87")]
        let features = features.with(SM_87);
        #[cfg(target_feature = "sm_89")]
        let features = features.with(SM_89);
        #[cfg(target_feature = "sm_90")]
        let features = features.with(SM_90);
        #[cfg(target_feature = "sm_90a")]
        let features = features.with(SM_90A);
        features
    }

}
#[cfg(any(doc, target_arch = "powerpc"))]
#[rustfmt::skip]
pub mod powerpc {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        ALTIVEC,
        CRT_STATIC,
        MSYNC,
        PARTWORD_ATOMICS,
        POWER10_VECTOR,
        POWER8_ALTIVEC,
        POWER8_CRYPTO,
        POWER8_VECTOR,
        POWER9_ALTIVEC,
        POWER9_VECTOR,
        QUADWORD_ATOMICS,
        VSX,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "powerpc")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "powerpc")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enable Altivec instructions."]
    pub const ALTIVEC: TargetFeatures = feature_set!(ALTIVEC);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Has only the msync instruction instead of sync."]
    pub const MSYNC: TargetFeatures = feature_set!(MSYNC);

    #[doc = "Enable l\\[bh\\]arx and st\\[bh\\]cx.."]
    pub const PARTWORD_ATOMICS: TargetFeatures = feature_set!(PARTWORD_ATOMICS);

    #[doc = "Enable POWER10 vector instructions."]
    pub const POWER10_VECTOR: TargetFeatures = feature_set!(ALTIVEC, POWER10_VECTOR, POWER8_ALTIVEC, POWER8_VECTOR, POWER9_ALTIVEC, POWER9_VECTOR, VSX);

    #[doc = "Enable POWER8 Altivec instructions."]
    pub const POWER8_ALTIVEC: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC);

    #[doc = "Enable POWER8 Crypto instructions."]
    pub const POWER8_CRYPTO: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC, POWER8_CRYPTO);

    #[doc = "Enable POWER8 vector instructions."]
    pub const POWER8_VECTOR: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC, POWER8_VECTOR, VSX);

    #[doc = "Enable POWER9 Altivec instructions."]
    pub const POWER9_ALTIVEC: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC, POWER9_ALTIVEC);

    #[doc = "Enable POWER9 vector instructions."]
    pub const POWER9_VECTOR: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC, POWER8_VECTOR, POWER9_ALTIVEC, POWER9_VECTOR, VSX);

    #[doc = "Enable lqarx and stqcx.."]
    pub const QUADWORD_ATOMICS: TargetFeatures = feature_set!(QUADWORD_ATOMICS);

    #[doc = "Enable VSX instructions."]
    pub const VSX: TargetFeatures = feature_set!(ALTIVEC, VSX);


    #[cfg(target_arch = "powerpc")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("altivec", ALTIVEC),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("msync", MSYNC),
        FeatureData::new("partword-atomics", PARTWORD_ATOMICS),
        FeatureData::new("power10-vector", POWER10_VECTOR),
        FeatureData::new("power8-altivec", POWER8_ALTIVEC),
        FeatureData::new("power8-crypto", POWER8_CRYPTO),
        FeatureData::new("power8-vector", POWER8_VECTOR),
        FeatureData::new("power9-altivec", POWER9_ALTIVEC),
        FeatureData::new("power9-vector", POWER9_VECTOR),
        FeatureData::new("quadword-atomics", QUADWORD_ATOMICS),
        FeatureData::new("vsx", VSX),
    ];

    #[cfg(target_arch = "powerpc")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "altivec")]
        let features = features.with(ALTIVEC);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "msync")]
        let features = features.with(MSYNC);
        #[cfg(target_feature = "partword-atomics")]
        let features = features.with(PARTWORD_ATOMICS);
        #[cfg(target_feature = "power10-vector")]
        let features = features.with(POWER10_VECTOR);
        #[cfg(target_feature = "power8-altivec")]
        let features = features.with(POWER8_ALTIVEC);
        #[cfg(target_feature = "power8-crypto")]
        let features = features.with(POWER8_CRYPTO);
        #[cfg(target_feature = "power8-vector")]
        let features = features.with(POWER8_VECTOR);
        #[cfg(target_feature = "power9-altivec")]
        let features = features.with(POWER9_ALTIVEC);
        #[cfg(target_feature = "power9-vector")]
        let features = features.with(POWER9_VECTOR);
        #[cfg(target_feature = "quadword-atomics")]
        let features = features.with(QUADWORD_ATOMICS);
        #[cfg(target_feature = "vsx")]
        let features = features.with(VSX);
        features
    }

}
#[cfg(any(doc, target_arch = "powerpc64"))]
#[rustfmt::skip]
pub mod powerpc64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        ALTIVEC,
        CRT_STATIC,
        MSYNC,
        PARTWORD_ATOMICS,
        POWER10_VECTOR,
        POWER8_ALTIVEC,
        POWER8_CRYPTO,
        POWER8_VECTOR,
        POWER9_ALTIVEC,
        POWER9_VECTOR,
        QUADWORD_ATOMICS,
        VSX,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "powerpc64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "powerpc64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enable Altivec instructions."]
    pub const ALTIVEC: TargetFeatures = feature_set!(ALTIVEC);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Has only the msync instruction instead of sync."]
    pub const MSYNC: TargetFeatures = feature_set!(MSYNC);

    #[doc = "Enable l\\[bh\\]arx and st\\[bh\\]cx.."]
    pub const PARTWORD_ATOMICS: TargetFeatures = feature_set!(PARTWORD_ATOMICS);

    #[doc = "Enable POWER10 vector instructions."]
    pub const POWER10_VECTOR: TargetFeatures = feature_set!(ALTIVEC, POWER10_VECTOR, POWER8_ALTIVEC, POWER8_VECTOR, POWER9_ALTIVEC, POWER9_VECTOR, VSX);

    #[doc = "Enable POWER8 Altivec instructions."]
    pub const POWER8_ALTIVEC: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC);

    #[doc = "Enable POWER8 Crypto instructions."]
    pub const POWER8_CRYPTO: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC, POWER8_CRYPTO);

    #[doc = "Enable POWER8 vector instructions."]
    pub const POWER8_VECTOR: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC, POWER8_VECTOR, VSX);

    #[doc = "Enable POWER9 Altivec instructions."]
    pub const POWER9_ALTIVEC: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC, POWER9_ALTIVEC);

    #[doc = "Enable POWER9 vector instructions."]
    pub const POWER9_VECTOR: TargetFeatures = feature_set!(ALTIVEC, POWER8_ALTIVEC, POWER8_VECTOR, POWER9_ALTIVEC, POWER9_VECTOR, VSX);

    #[doc = "Enable lqarx and stqcx.."]
    pub const QUADWORD_ATOMICS: TargetFeatures = feature_set!(QUADWORD_ATOMICS);

    #[doc = "Enable VSX instructions."]
    pub const VSX: TargetFeatures = feature_set!(ALTIVEC, VSX);


    #[cfg(target_arch = "powerpc64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("altivec", ALTIVEC),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("msync", MSYNC),
        FeatureData::new("partword-atomics", PARTWORD_ATOMICS),
        FeatureData::new("power10-vector", POWER10_VECTOR),
        FeatureData::new("power8-altivec", POWER8_ALTIVEC),
        FeatureData::new("power8-crypto", POWER8_CRYPTO),
        FeatureData::new("power8-vector", POWER8_VECTOR),
        FeatureData::new("power9-altivec", POWER9_ALTIVEC),
        FeatureData::new("power9-vector", POWER9_VECTOR),
        FeatureData::new("quadword-atomics", QUADWORD_ATOMICS),
        FeatureData::new("vsx", VSX),
    ];

    #[cfg(target_arch = "powerpc64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "altivec")]
        let features = features.with(ALTIVEC);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "msync")]
        let features = features.with(MSYNC);
        #[cfg(target_feature = "partword-atomics")]
        let features = features.with(PARTWORD_ATOMICS);
        #[cfg(target_feature = "power10-vector")]
        let features = features.with(POWER10_VECTOR);
        #[cfg(target_feature = "power8-altivec")]
        let features = features.with(POWER8_ALTIVEC);
        #[cfg(target_feature = "power8-crypto")]
        let features = features.with(POWER8_CRYPTO);
        #[cfg(target_feature = "power8-vector")]
        let features = features.with(POWER8_VECTOR);
        #[cfg(target_feature = "power9-altivec")]
        let features = features.with(POWER9_ALTIVEC);
        #[cfg(target_feature = "power9-vector")]
        let features = features.with(POWER9_VECTOR);
        #[cfg(target_feature = "quadword-atomics")]
        let features = features.with(QUADWORD_ATOMICS);
        #[cfg(target_feature = "vsx")]
        let features = features.with(VSX);
        features
    }

}
#[cfg(any(doc, target_arch = "riscv32"))]
#[rustfmt::skip]
pub mod riscv32 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        A,
        B,
        C,
        CRT_STATIC,
        D,
        E,
        F,
        M,
        RELAX,
        RVA23U64,
        SUPM,
        UNALIGNED_SCALAR_MEM,
        UNALIGNED_VECTOR_MEM,
        V,
        ZA128RS,
        ZA64RS,
        ZAAMO,
        ZABHA,
        ZACAS,
        ZALRSC,
        ZAMA16B,
        ZAWRS,
        ZBA,
        ZBB,
        ZBC,
        ZBKB,
        ZBKC,
        ZBKX,
        ZBS,
        ZCA,
        ZCB,
        ZCMOP,
        ZDINX,
        ZFA,
        ZFBFMIN,
        ZFH,
        ZFHMIN,
        ZFINX,
        ZHINX,
        ZHINXMIN,
        ZIC64B,
        ZICBOM,
        ZICBOP,
        ZICBOZ,
        ZICCAMOA,
        ZICCIF,
        ZICCLSM,
        ZICCRSE,
        ZICNTR,
        ZICOND,
        ZICSR,
        ZIFENCEI,
        ZIHINTNTL,
        ZIHINTPAUSE,
        ZIHPM,
        ZIMOP,
        ZK,
        ZKN,
        ZKND,
        ZKNE,
        ZKNH,
        ZKR,
        ZKS,
        ZKSED,
        ZKSH,
        ZKT,
        ZTSO,
        ZVBB,
        ZVBC,
        ZVE32F,
        ZVE32X,
        ZVE64D,
        ZVE64F,
        ZVE64X,
        ZVFBFMIN,
        ZVFBFWMA,
        ZVFH,
        ZVFHMIN,
        ZVKB,
        ZVKG,
        ZVKN,
        ZVKNC,
        ZVKNED,
        ZVKNG,
        ZVKNHA,
        ZVKNHB,
        ZVKS,
        ZVKSC,
        ZVKSED,
        ZVKSG,
        ZVKSH,
        ZVKT,
        ZVL1024B,
        ZVL128B,
        ZVL16384B,
        ZVL2048B,
        ZVL256B,
        ZVL32768B,
        ZVL32B,
        ZVL4096B,
        ZVL512B,
        ZVL64B,
        ZVL65536B,
        ZVL8192B,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "riscv32")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "riscv32")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "'A' (Atomic Instructions)."]
    pub const A: TargetFeatures = feature_set!(A, ZAAMO, ZALRSC);

    #[doc = "'B' (the collection of the Zba, Zbb, Zbs extensions)."]
    pub const B: TargetFeatures = feature_set!(B, ZBA, ZBB, ZBS);

    #[doc = "'C' (Compressed Instructions)."]
    pub const C: TargetFeatures = feature_set!(C, ZCA);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "'D' (Double-Precision Floating-Point)."]
    pub const D: TargetFeatures = feature_set!(D, F, ZICSR);

    #[doc = "'E' (Embedded Instruction Set with 16 GPRs)."]
    pub const E: TargetFeatures = feature_set!(E);

    #[doc = "'F' (Single-Precision Floating-Point)."]
    pub const F: TargetFeatures = feature_set!(F, ZICSR);

    #[doc = "'M' (Integer Multiplication and Division)."]
    pub const M: TargetFeatures = feature_set!(M);

    #[doc = "Enable Linker relaxation."]
    pub const RELAX: TargetFeatures = feature_set!(RELAX);

    #[doc = "RISC-V rva23u64 profile."]
    pub const RVA23U64: TargetFeatures = feature_set!(A, B, C, D, F, M, RVA23U64, SUPM, V, ZA128RS, ZA64RS, ZAAMO, ZALRSC, ZAWRS, ZBA, ZBB, ZBS, ZCA, ZCB, ZCMOP, ZFA, ZFHMIN, ZIC64B, ZICBOM, ZICBOP, ZICBOZ, ZICCAMOA, ZICCIF, ZICCLSM, ZICCRSE, ZICNTR, ZICOND, ZICSR, ZIHINTNTL, ZIHINTPAUSE, ZIHPM, ZIMOP, ZKT, ZVBB, ZVE32F, ZVE32X, ZVE64D, ZVE64F, ZVE64X, ZVFHMIN, ZVKB, ZVKT, ZVL128B, ZVL32B, ZVL64B);

    #[doc = "'Supm' (Indicates User-mode Pointer Masking)."]
    pub const SUPM: TargetFeatures = feature_set!(SUPM);

    #[doc = "Has reasonably performant unaligned scalar loads and stores."]
    pub const UNALIGNED_SCALAR_MEM: TargetFeatures = feature_set!(UNALIGNED_SCALAR_MEM);

    #[doc = "Has reasonably performant unaligned vector loads and stores."]
    pub const UNALIGNED_VECTOR_MEM: TargetFeatures = feature_set!(UNALIGNED_VECTOR_MEM);

    #[doc = "'V' (Vector Extension for Application Processors)."]
    pub const V: TargetFeatures = feature_set!(D, F, V, ZICSR, ZVE32F, ZVE32X, ZVE64D, ZVE64F, ZVE64X, ZVL128B, ZVL32B, ZVL64B);

    #[doc = "'Za128rs' (Reservation Set Size of at Most 128 Bytes)."]
    pub const ZA128RS: TargetFeatures = feature_set!(ZA128RS);

    #[doc = "'Za64rs' (Reservation Set Size of at Most 64 Bytes)."]
    pub const ZA64RS: TargetFeatures = feature_set!(ZA128RS, ZA64RS);

    #[doc = "'Zaamo' (Atomic Memory Operations)."]
    pub const ZAAMO: TargetFeatures = feature_set!(ZAAMO);

    #[doc = "'Zabha' (Byte and Halfword Atomic Memory Operations)."]
    pub const ZABHA: TargetFeatures = feature_set!(ZAAMO, ZABHA);

    #[doc = "'Zacas' (Atomic Compare-And-Swap Instructions)."]
    pub const ZACAS: TargetFeatures = feature_set!(ZAAMO, ZACAS);

    #[doc = "'Zalrsc' (Load-Reserved/Store-Conditional)."]
    pub const ZALRSC: TargetFeatures = feature_set!(ZALRSC);

    #[doc = "'Zama16b' (Atomic 16-byte misaligned loads, stores and AMOs)."]
    pub const ZAMA16B: TargetFeatures = feature_set!(ZAMA16B);

    #[doc = "'Zawrs' (Wait on Reservation Set)."]
    pub const ZAWRS: TargetFeatures = feature_set!(ZAWRS);

    #[doc = "'Zba' (Address Generation Instructions)."]
    pub const ZBA: TargetFeatures = feature_set!(ZBA);

    #[doc = "'Zbb' (Basic Bit-Manipulation)."]
    pub const ZBB: TargetFeatures = feature_set!(ZBB);

    #[doc = "'Zbc' (Carry-Less Multiplication)."]
    pub const ZBC: TargetFeatures = feature_set!(ZBC, ZBKC);

    #[doc = "'Zbkb' (Bitmanip instructions for Cryptography)."]
    pub const ZBKB: TargetFeatures = feature_set!(ZBKB);

    #[doc = "'Zbkc' (Carry-less multiply instructions for Cryptography)."]
    pub const ZBKC: TargetFeatures = feature_set!(ZBKC);

    #[doc = "'Zbkx' (Crossbar permutation instructions)."]
    pub const ZBKX: TargetFeatures = feature_set!(ZBKX);

    #[doc = "'Zbs' (Single-Bit Instructions)."]
    pub const ZBS: TargetFeatures = feature_set!(ZBS);

    #[doc = "'Zca' (part of the C extension, excluding compressed floating point loads/stores)."]
    pub const ZCA: TargetFeatures = feature_set!(ZCA);

    #[doc = "'Zcb' (Compressed basic bit manipulation instructions)."]
    pub const ZCB: TargetFeatures = feature_set!(ZCA, ZCB);

    #[doc = "'Zcmop' (Compressed May-Be-Operations)."]
    pub const ZCMOP: TargetFeatures = feature_set!(ZCA, ZCMOP);

    #[doc = "'Zdinx' (Double in Integer)."]
    pub const ZDINX: TargetFeatures = feature_set!(ZDINX, ZFINX, ZICSR);

    #[doc = "'Zfa' (Additional Floating-Point)."]
    pub const ZFA: TargetFeatures = feature_set!(F, ZFA, ZICSR);

    #[doc = "'Zfbfmin' (Scalar BF16 Converts)."]
    pub const ZFBFMIN: TargetFeatures = feature_set!(F, ZFBFMIN, ZICSR);

    #[doc = "'Zfh' (Half-Precision Floating-Point)."]
    pub const ZFH: TargetFeatures = feature_set!(F, ZFH, ZFHMIN, ZICSR);

    #[doc = "'Zfhmin' (Half-Precision Floating-Point Minimal)."]
    pub const ZFHMIN: TargetFeatures = feature_set!(F, ZFHMIN, ZICSR);

    #[doc = "'Zfinx' (Float in Integer)."]
    pub const ZFINX: TargetFeatures = feature_set!(ZFINX, ZICSR);

    #[doc = "'Zhinx' (Half Float in Integer)."]
    pub const ZHINX: TargetFeatures = feature_set!(ZFINX, ZHINX, ZHINXMIN, ZICSR);

    #[doc = "'Zhinxmin' (Half Float in Integer Minimal)."]
    pub const ZHINXMIN: TargetFeatures = feature_set!(ZFINX, ZHINXMIN, ZICSR);

    #[doc = "'Zic64b' (Cache Block Size Is 64 Bytes)."]
    pub const ZIC64B: TargetFeatures = feature_set!(ZIC64B);

    #[doc = "'Zicbom' (Cache-Block Management Instructions)."]
    pub const ZICBOM: TargetFeatures = feature_set!(ZICBOM);

    #[doc = "'Zicbop' (Cache-Block Prefetch Instructions)."]
    pub const ZICBOP: TargetFeatures = feature_set!(ZICBOP);

    #[doc = "'Zicboz' (Cache-Block Zero Instructions)."]
    pub const ZICBOZ: TargetFeatures = feature_set!(ZICBOZ);

    #[doc = "'Ziccamoa' (Main Memory Supports All Atomics in A)."]
    pub const ZICCAMOA: TargetFeatures = feature_set!(ZICCAMOA);

    #[doc = "'Ziccif' (Main Memory Supports Instruction Fetch with Atomicity Requirement)."]
    pub const ZICCIF: TargetFeatures = feature_set!(ZICCIF);

    #[doc = "'Zicclsm' (Main Memory Supports Misaligned Loads/Stores)."]
    pub const ZICCLSM: TargetFeatures = feature_set!(ZICCLSM);

    #[doc = "'Ziccrse' (Main Memory Supports Forward Progress on LR/SC Sequences)."]
    pub const ZICCRSE: TargetFeatures = feature_set!(ZICCRSE);

    #[doc = "'Zicntr' (Base Counters and Timers)."]
    pub const ZICNTR: TargetFeatures = feature_set!(ZICNTR, ZICSR);

    #[doc = "'Zicond' (Integer Conditional Operations)."]
    pub const ZICOND: TargetFeatures = feature_set!(ZICOND);

    #[doc = "'Zicsr' (CSRs)."]
    pub const ZICSR: TargetFeatures = feature_set!(ZICSR);

    #[doc = "'Zifencei' (fence.i)."]
    pub const ZIFENCEI: TargetFeatures = feature_set!(ZIFENCEI);

    #[doc = "'Zihintntl' (Non-Temporal Locality Hints)."]
    pub const ZIHINTNTL: TargetFeatures = feature_set!(ZIHINTNTL);

    #[doc = "'Zihintpause' (Pause Hint)."]
    pub const ZIHINTPAUSE: TargetFeatures = feature_set!(ZIHINTPAUSE);

    #[doc = "'Zihpm' (Hardware Performance Counters)."]
    pub const ZIHPM: TargetFeatures = feature_set!(ZICSR, ZIHPM);

    #[doc = "'Zimop' (May-Be-Operations)."]
    pub const ZIMOP: TargetFeatures = feature_set!(ZIMOP);

    #[doc = "'Zk' (Standard scalar cryptography extension)."]
    pub const ZK: TargetFeatures = feature_set!(ZBKB, ZBKC, ZBKX, ZK, ZKN, ZKND, ZKNE, ZKNH, ZKR, ZKT);

    #[doc = "'Zkn' (NIST Algorithm Suite)."]
    pub const ZKN: TargetFeatures = feature_set!(ZBKB, ZBKC, ZBKX, ZKN, ZKND, ZKNE, ZKNH);

    #[doc = "'Zknd' (NIST Suite: AES Decryption)."]
    pub const ZKND: TargetFeatures = feature_set!(ZKND);

    #[doc = "'Zkne' (NIST Suite: AES Encryption)."]
    pub const ZKNE: TargetFeatures = feature_set!(ZKNE);

    #[doc = "'Zknh' (NIST Suite: Hash Function Instructions)."]
    pub const ZKNH: TargetFeatures = feature_set!(ZKNH);

    #[doc = "'Zkr' (Entropy Source Extension)."]
    pub const ZKR: TargetFeatures = feature_set!(ZKR);

    #[doc = "'Zks' (ShangMi Algorithm Suite)."]
    pub const ZKS: TargetFeatures = feature_set!(ZBKB, ZBKC, ZBKX, ZKS, ZKSED, ZKSH);

    #[doc = "'Zksed' (ShangMi Suite: SM4 Block Cipher Instructions)."]
    pub const ZKSED: TargetFeatures = feature_set!(ZKSED);

    #[doc = "'Zksh' (ShangMi Suite: SM3 Hash Function Instructions)."]
    pub const ZKSH: TargetFeatures = feature_set!(ZKSH);

    #[doc = "'Zkt' (Data Independent Execution Latency)."]
    pub const ZKT: TargetFeatures = feature_set!(ZKT);

    #[doc = "'Ztso' (Memory Model"]
    pub const ZTSO: TargetFeatures = feature_set!(ZTSO);

    #[doc = "'Zvbb' (Vector basic bit-manipulation instructions)."]
    pub const ZVBB: TargetFeatures = feature_set!(ZICSR, ZVBB, ZVE32X, ZVKB, ZVL32B);

    #[doc = "'Zvbc' (Vector Carryless Multiplication)."]
    pub const ZVBC: TargetFeatures = feature_set!(ZICSR, ZVBC, ZVE32X, ZVE64X, ZVL32B, ZVL64B);

    #[doc = "'Zve32f' (Vector Extensions for Embedded Processors with maximal 32 EEW and F extension)."]
    pub const ZVE32F: TargetFeatures = feature_set!(F, ZICSR, ZVE32F, ZVE32X, ZVL32B);

    #[doc = "'Zve32x' (Vector Extensions for Embedded Processors with maximal 32 EEW)."]
    pub const ZVE32X: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVL32B);

    #[doc = "'Zve64d' (Vector Extensions for Embedded Processors with maximal 64 EEW, F and D extension)."]
    pub const ZVE64D: TargetFeatures = feature_set!(D, F, ZICSR, ZVE32F, ZVE32X, ZVE64D, ZVE64F, ZVE64X, ZVL32B, ZVL64B);

    #[doc = "'Zve64f' (Vector Extensions for Embedded Processors with maximal 64 EEW and F extension)."]
    pub const ZVE64F: TargetFeatures = feature_set!(F, ZICSR, ZVE32F, ZVE32X, ZVE64F, ZVE64X, ZVL32B, ZVL64B);

    #[doc = "'Zve64x' (Vector Extensions for Embedded Processors with maximal 64 EEW)."]
    pub const ZVE64X: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVE64X, ZVL32B, ZVL64B);

    #[doc = "'Zvfbfmin' (Vector BF16 Converts)."]
    pub const ZVFBFMIN: TargetFeatures = feature_set!(F, ZICSR, ZVE32F, ZVE32X, ZVFBFMIN, ZVL32B);

    #[doc = "'Zvfbfwma' (Vector BF16 widening mul-add)."]
    pub const ZVFBFWMA: TargetFeatures = feature_set!(F, ZFBFMIN, ZICSR, ZVE32F, ZVE32X, ZVFBFMIN, ZVFBFWMA, ZVL32B);

    #[doc = "'Zvfh' (Vector Half-Precision Floating-Point)."]
    pub const ZVFH: TargetFeatures = feature_set!(F, ZFHMIN, ZICSR, ZVE32F, ZVE32X, ZVFH, ZVFHMIN, ZVL32B);

    #[doc = "'Zvfhmin' (Vector Half-Precision Floating-Point Minimal)."]
    pub const ZVFHMIN: TargetFeatures = feature_set!(F, ZICSR, ZVE32F, ZVE32X, ZVFHMIN, ZVL32B);

    #[doc = "'Zvkb' (Vector Bit-manipulation used in Cryptography)."]
    pub const ZVKB: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKB, ZVL32B);

    #[doc = "'Zvkg' (Vector GCM instructions for Cryptography)."]
    pub const ZVKG: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKG, ZVL32B);

    #[doc = "'Zvkn' (shorthand for 'Zvkned', 'Zvknhb', 'Zvkb', and 'Zvkt')."]
    pub const ZVKN: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVE64X, ZVKB, ZVKN, ZVKNED, ZVKNHA, ZVKNHB, ZVKT, ZVL32B, ZVL64B);

    #[doc = "'Zvknc' (shorthand for 'Zvknc' and 'Zvbc')."]
    pub const ZVKNC: TargetFeatures = feature_set!(ZICSR, ZVBC, ZVE32X, ZVE64X, ZVKB, ZVKN, ZVKNC, ZVKNED, ZVKNHA, ZVKNHB, ZVKT, ZVL32B, ZVL64B);

    #[doc = "'Zvkned' (Vector AES Encryption & Decryption (Single Round))."]
    pub const ZVKNED: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKNED, ZVL32B);

    #[doc = "'Zvkng' (shorthand for 'Zvkn' and 'Zvkg')."]
    pub const ZVKNG: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVE64X, ZVKB, ZVKG, ZVKN, ZVKNED, ZVKNG, ZVKNHA, ZVKNHB, ZVKT, ZVL32B, ZVL64B);

    #[doc = "'Zvknha' (Vector SHA-2 (SHA-256 only))."]
    pub const ZVKNHA: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKNHA, ZVL32B);

    #[doc = "'Zvknhb' (Vector SHA-2 (SHA-256 and SHA-512))."]
    pub const ZVKNHB: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVE64X, ZVKNHA, ZVKNHB, ZVL32B, ZVL64B);

    #[doc = "'Zvks' (shorthand for 'Zvksed', 'Zvksh', 'Zvkb', and 'Zvkt')."]
    pub const ZVKS: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKB, ZVKS, ZVKSED, ZVKSH, ZVKT, ZVL32B);

    #[doc = "'Zvksc' (shorthand for 'Zvks' and 'Zvbc')."]
    pub const ZVKSC: TargetFeatures = feature_set!(ZICSR, ZVBC, ZVE32X, ZVE64X, ZVKB, ZVKS, ZVKSC, ZVKSED, ZVKSH, ZVKT, ZVL32B, ZVL64B);

    #[doc = "'Zvksed' (SM4 Block Cipher Instructions)."]
    pub const ZVKSED: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKSED, ZVL32B);

    #[doc = "'Zvksg' (shorthand for 'Zvks' and 'Zvkg')."]
    pub const ZVKSG: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKB, ZVKG, ZVKS, ZVKSED, ZVKSG, ZVKSH, ZVKT, ZVL32B);

    #[doc = "'Zvksh' (SM3 Hash Function Instructions)."]
    pub const ZVKSH: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKSH, ZVL32B);

    #[doc = "'Zvkt' (Vector Data-Independent Execution Latency)."]
    pub const ZVKT: TargetFeatures = feature_set!(ZVKT);

    #[doc = "'Zvl1024b' (Minimum Vector Length 1024)."]
    pub const ZVL1024B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL256B, ZVL32B, ZVL512B, ZVL64B);

    #[doc = "'Zvl128b' (Minimum Vector Length 128)."]
    pub const ZVL128B: TargetFeatures = feature_set!(ZVL128B, ZVL32B, ZVL64B);

    #[doc = "'Zvl16384b' (Minimum Vector Length 16384)."]
    pub const ZVL16384B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL16384B, ZVL2048B, ZVL256B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B, ZVL8192B);

    #[doc = "'Zvl2048b' (Minimum Vector Length 2048)."]
    pub const ZVL2048B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL2048B, ZVL256B, ZVL32B, ZVL512B, ZVL64B);

    #[doc = "'Zvl256b' (Minimum Vector Length 256)."]
    pub const ZVL256B: TargetFeatures = feature_set!(ZVL128B, ZVL256B, ZVL32B, ZVL64B);

    #[doc = "'Zvl32768b' (Minimum Vector Length 32768)."]
    pub const ZVL32768B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL16384B, ZVL2048B, ZVL256B, ZVL32768B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B, ZVL8192B);

    #[doc = "'Zvl32b' (Minimum Vector Length 32)."]
    pub const ZVL32B: TargetFeatures = feature_set!(ZVL32B);

    #[doc = "'Zvl4096b' (Minimum Vector Length 4096)."]
    pub const ZVL4096B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL2048B, ZVL256B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B);

    #[doc = "'Zvl512b' (Minimum Vector Length 512)."]
    pub const ZVL512B: TargetFeatures = feature_set!(ZVL128B, ZVL256B, ZVL32B, ZVL512B, ZVL64B);

    #[doc = "'Zvl64b' (Minimum Vector Length 64)."]
    pub const ZVL64B: TargetFeatures = feature_set!(ZVL32B, ZVL64B);

    #[doc = "'Zvl65536b' (Minimum Vector Length 65536)."]
    pub const ZVL65536B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL16384B, ZVL2048B, ZVL256B, ZVL32768B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B, ZVL65536B, ZVL8192B);

    #[doc = "'Zvl8192b' (Minimum Vector Length 8192)."]
    pub const ZVL8192B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL2048B, ZVL256B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B, ZVL8192B);


    #[cfg(target_arch = "riscv32")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("a", A),
        FeatureData::new("b", B),
        FeatureData::new("c", C),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("d", D),
        FeatureData::new("e", E),
        FeatureData::new("f", F),
        FeatureData::new("m", M),
        FeatureData::new("relax", RELAX),
        FeatureData::new("rva23u64", RVA23U64),
        FeatureData::new("supm", SUPM),
        FeatureData::new("unaligned-scalar-mem", UNALIGNED_SCALAR_MEM),
        FeatureData::new("unaligned-vector-mem", UNALIGNED_VECTOR_MEM),
        FeatureData::new("v", V),
        FeatureData::new("za128rs", ZA128RS),
        FeatureData::new("za64rs", ZA64RS),
        FeatureData::new("zaamo", ZAAMO),
        FeatureData::new("zabha", ZABHA),
        FeatureData::new("zacas", ZACAS),
        FeatureData::new("zalrsc", ZALRSC),
        FeatureData::new("zama16b", ZAMA16B),
        FeatureData::new("zawrs", ZAWRS),
        FeatureData::new("zba", ZBA),
        FeatureData::new("zbb", ZBB),
        FeatureData::new("zbc", ZBC),
        FeatureData::new("zbkb", ZBKB),
        FeatureData::new("zbkc", ZBKC),
        FeatureData::new("zbkx", ZBKX),
        FeatureData::new("zbs", ZBS),
        FeatureData::new("zca", ZCA),
        FeatureData::new("zcb", ZCB),
        FeatureData::new("zcmop", ZCMOP),
        FeatureData::new("zdinx", ZDINX),
        FeatureData::new("zfa", ZFA),
        FeatureData::new("zfbfmin", ZFBFMIN),
        FeatureData::new("zfh", ZFH),
        FeatureData::new("zfhmin", ZFHMIN),
        FeatureData::new("zfinx", ZFINX),
        FeatureData::new("zhinx", ZHINX),
        FeatureData::new("zhinxmin", ZHINXMIN),
        FeatureData::new("zic64b", ZIC64B),
        FeatureData::new("zicbom", ZICBOM),
        FeatureData::new("zicbop", ZICBOP),
        FeatureData::new("zicboz", ZICBOZ),
        FeatureData::new("ziccamoa", ZICCAMOA),
        FeatureData::new("ziccif", ZICCIF),
        FeatureData::new("zicclsm", ZICCLSM),
        FeatureData::new("ziccrse", ZICCRSE),
        FeatureData::new("zicntr", ZICNTR),
        FeatureData::new("zicond", ZICOND),
        FeatureData::new("zicsr", ZICSR),
        FeatureData::new("zifencei", ZIFENCEI),
        FeatureData::new("zihintntl", ZIHINTNTL),
        FeatureData::new("zihintpause", ZIHINTPAUSE),
        FeatureData::new("zihpm", ZIHPM),
        FeatureData::new("zimop", ZIMOP),
        FeatureData::new("zk", ZK),
        FeatureData::new("zkn", ZKN),
        FeatureData::new("zknd", ZKND),
        FeatureData::new("zkne", ZKNE),
        FeatureData::new("zknh", ZKNH),
        FeatureData::new("zkr", ZKR),
        FeatureData::new("zks", ZKS),
        FeatureData::new("zksed", ZKSED),
        FeatureData::new("zksh", ZKSH),
        FeatureData::new("zkt", ZKT),
        FeatureData::new("ztso", ZTSO),
        FeatureData::new("zvbb", ZVBB),
        FeatureData::new("zvbc", ZVBC),
        FeatureData::new("zve32f", ZVE32F),
        FeatureData::new("zve32x", ZVE32X),
        FeatureData::new("zve64d", ZVE64D),
        FeatureData::new("zve64f", ZVE64F),
        FeatureData::new("zve64x", ZVE64X),
        FeatureData::new("zvfbfmin", ZVFBFMIN),
        FeatureData::new("zvfbfwma", ZVFBFWMA),
        FeatureData::new("zvfh", ZVFH),
        FeatureData::new("zvfhmin", ZVFHMIN),
        FeatureData::new("zvkb", ZVKB),
        FeatureData::new("zvkg", ZVKG),
        FeatureData::new("zvkn", ZVKN),
        FeatureData::new("zvknc", ZVKNC),
        FeatureData::new("zvkned", ZVKNED),
        FeatureData::new("zvkng", ZVKNG),
        FeatureData::new("zvknha", ZVKNHA),
        FeatureData::new("zvknhb", ZVKNHB),
        FeatureData::new("zvks", ZVKS),
        FeatureData::new("zvksc", ZVKSC),
        FeatureData::new("zvksed", ZVKSED),
        FeatureData::new("zvksg", ZVKSG),
        FeatureData::new("zvksh", ZVKSH),
        FeatureData::new("zvkt", ZVKT),
        FeatureData::new("zvl1024b", ZVL1024B),
        FeatureData::new("zvl128b", ZVL128B),
        FeatureData::new("zvl16384b", ZVL16384B),
        FeatureData::new("zvl2048b", ZVL2048B),
        FeatureData::new("zvl256b", ZVL256B),
        FeatureData::new("zvl32768b", ZVL32768B),
        FeatureData::new("zvl32b", ZVL32B),
        FeatureData::new("zvl4096b", ZVL4096B),
        FeatureData::new("zvl512b", ZVL512B),
        FeatureData::new("zvl64b", ZVL64B),
        FeatureData::new("zvl65536b", ZVL65536B),
        FeatureData::new("zvl8192b", ZVL8192B),
    ];

    #[cfg(target_arch = "riscv32")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "a")]
        let features = features.with(A);
        #[cfg(target_feature = "b")]
        let features = features.with(B);
        #[cfg(target_feature = "c")]
        let features = features.with(C);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "d")]
        let features = features.with(D);
        #[cfg(target_feature = "e")]
        let features = features.with(E);
        #[cfg(target_feature = "f")]
        let features = features.with(F);
        #[cfg(target_feature = "m")]
        let features = features.with(M);
        #[cfg(target_feature = "relax")]
        let features = features.with(RELAX);
        #[cfg(target_feature = "rva23u64")]
        let features = features.with(RVA23U64);
        #[cfg(target_feature = "supm")]
        let features = features.with(SUPM);
        #[cfg(target_feature = "unaligned-scalar-mem")]
        let features = features.with(UNALIGNED_SCALAR_MEM);
        #[cfg(target_feature = "unaligned-vector-mem")]
        let features = features.with(UNALIGNED_VECTOR_MEM);
        #[cfg(target_feature = "v")]
        let features = features.with(V);
        #[cfg(target_feature = "za128rs")]
        let features = features.with(ZA128RS);
        #[cfg(target_feature = "za64rs")]
        let features = features.with(ZA64RS);
        #[cfg(target_feature = "zaamo")]
        let features = features.with(ZAAMO);
        #[cfg(target_feature = "zabha")]
        let features = features.with(ZABHA);
        #[cfg(target_feature = "zacas")]
        let features = features.with(ZACAS);
        #[cfg(target_feature = "zalrsc")]
        let features = features.with(ZALRSC);
        #[cfg(target_feature = "zama16b")]
        let features = features.with(ZAMA16B);
        #[cfg(target_feature = "zawrs")]
        let features = features.with(ZAWRS);
        #[cfg(target_feature = "zba")]
        let features = features.with(ZBA);
        #[cfg(target_feature = "zbb")]
        let features = features.with(ZBB);
        #[cfg(target_feature = "zbc")]
        let features = features.with(ZBC);
        #[cfg(target_feature = "zbkb")]
        let features = features.with(ZBKB);
        #[cfg(target_feature = "zbkc")]
        let features = features.with(ZBKC);
        #[cfg(target_feature = "zbkx")]
        let features = features.with(ZBKX);
        #[cfg(target_feature = "zbs")]
        let features = features.with(ZBS);
        #[cfg(target_feature = "zca")]
        let features = features.with(ZCA);
        #[cfg(target_feature = "zcb")]
        let features = features.with(ZCB);
        #[cfg(target_feature = "zcmop")]
        let features = features.with(ZCMOP);
        #[cfg(target_feature = "zdinx")]
        let features = features.with(ZDINX);
        #[cfg(target_feature = "zfa")]
        let features = features.with(ZFA);
        #[cfg(target_feature = "zfbfmin")]
        let features = features.with(ZFBFMIN);
        #[cfg(target_feature = "zfh")]
        let features = features.with(ZFH);
        #[cfg(target_feature = "zfhmin")]
        let features = features.with(ZFHMIN);
        #[cfg(target_feature = "zfinx")]
        let features = features.with(ZFINX);
        #[cfg(target_feature = "zhinx")]
        let features = features.with(ZHINX);
        #[cfg(target_feature = "zhinxmin")]
        let features = features.with(ZHINXMIN);
        #[cfg(target_feature = "zic64b")]
        let features = features.with(ZIC64B);
        #[cfg(target_feature = "zicbom")]
        let features = features.with(ZICBOM);
        #[cfg(target_feature = "zicbop")]
        let features = features.with(ZICBOP);
        #[cfg(target_feature = "zicboz")]
        let features = features.with(ZICBOZ);
        #[cfg(target_feature = "ziccamoa")]
        let features = features.with(ZICCAMOA);
        #[cfg(target_feature = "ziccif")]
        let features = features.with(ZICCIF);
        #[cfg(target_feature = "zicclsm")]
        let features = features.with(ZICCLSM);
        #[cfg(target_feature = "ziccrse")]
        let features = features.with(ZICCRSE);
        #[cfg(target_feature = "zicntr")]
        let features = features.with(ZICNTR);
        #[cfg(target_feature = "zicond")]
        let features = features.with(ZICOND);
        #[cfg(target_feature = "zicsr")]
        let features = features.with(ZICSR);
        #[cfg(target_feature = "zifencei")]
        let features = features.with(ZIFENCEI);
        #[cfg(target_feature = "zihintntl")]
        let features = features.with(ZIHINTNTL);
        #[cfg(target_feature = "zihintpause")]
        let features = features.with(ZIHINTPAUSE);
        #[cfg(target_feature = "zihpm")]
        let features = features.with(ZIHPM);
        #[cfg(target_feature = "zimop")]
        let features = features.with(ZIMOP);
        #[cfg(target_feature = "zk")]
        let features = features.with(ZK);
        #[cfg(target_feature = "zkn")]
        let features = features.with(ZKN);
        #[cfg(target_feature = "zknd")]
        let features = features.with(ZKND);
        #[cfg(target_feature = "zkne")]
        let features = features.with(ZKNE);
        #[cfg(target_feature = "zknh")]
        let features = features.with(ZKNH);
        #[cfg(target_feature = "zkr")]
        let features = features.with(ZKR);
        #[cfg(target_feature = "zks")]
        let features = features.with(ZKS);
        #[cfg(target_feature = "zksed")]
        let features = features.with(ZKSED);
        #[cfg(target_feature = "zksh")]
        let features = features.with(ZKSH);
        #[cfg(target_feature = "zkt")]
        let features = features.with(ZKT);
        #[cfg(target_feature = "ztso")]
        let features = features.with(ZTSO);
        #[cfg(target_feature = "zvbb")]
        let features = features.with(ZVBB);
        #[cfg(target_feature = "zvbc")]
        let features = features.with(ZVBC);
        #[cfg(target_feature = "zve32f")]
        let features = features.with(ZVE32F);
        #[cfg(target_feature = "zve32x")]
        let features = features.with(ZVE32X);
        #[cfg(target_feature = "zve64d")]
        let features = features.with(ZVE64D);
        #[cfg(target_feature = "zve64f")]
        let features = features.with(ZVE64F);
        #[cfg(target_feature = "zve64x")]
        let features = features.with(ZVE64X);
        #[cfg(target_feature = "zvfbfmin")]
        let features = features.with(ZVFBFMIN);
        #[cfg(target_feature = "zvfbfwma")]
        let features = features.with(ZVFBFWMA);
        #[cfg(target_feature = "zvfh")]
        let features = features.with(ZVFH);
        #[cfg(target_feature = "zvfhmin")]
        let features = features.with(ZVFHMIN);
        #[cfg(target_feature = "zvkb")]
        let features = features.with(ZVKB);
        #[cfg(target_feature = "zvkg")]
        let features = features.with(ZVKG);
        #[cfg(target_feature = "zvkn")]
        let features = features.with(ZVKN);
        #[cfg(target_feature = "zvknc")]
        let features = features.with(ZVKNC);
        #[cfg(target_feature = "zvkned")]
        let features = features.with(ZVKNED);
        #[cfg(target_feature = "zvkng")]
        let features = features.with(ZVKNG);
        #[cfg(target_feature = "zvknha")]
        let features = features.with(ZVKNHA);
        #[cfg(target_feature = "zvknhb")]
        let features = features.with(ZVKNHB);
        #[cfg(target_feature = "zvks")]
        let features = features.with(ZVKS);
        #[cfg(target_feature = "zvksc")]
        let features = features.with(ZVKSC);
        #[cfg(target_feature = "zvksed")]
        let features = features.with(ZVKSED);
        #[cfg(target_feature = "zvksg")]
        let features = features.with(ZVKSG);
        #[cfg(target_feature = "zvksh")]
        let features = features.with(ZVKSH);
        #[cfg(target_feature = "zvkt")]
        let features = features.with(ZVKT);
        #[cfg(target_feature = "zvl1024b")]
        let features = features.with(ZVL1024B);
        #[cfg(target_feature = "zvl128b")]
        let features = features.with(ZVL128B);
        #[cfg(target_feature = "zvl16384b")]
        let features = features.with(ZVL16384B);
        #[cfg(target_feature = "zvl2048b")]
        let features = features.with(ZVL2048B);
        #[cfg(target_feature = "zvl256b")]
        let features = features.with(ZVL256B);
        #[cfg(target_feature = "zvl32768b")]
        let features = features.with(ZVL32768B);
        #[cfg(target_feature = "zvl32b")]
        let features = features.with(ZVL32B);
        #[cfg(target_feature = "zvl4096b")]
        let features = features.with(ZVL4096B);
        #[cfg(target_feature = "zvl512b")]
        let features = features.with(ZVL512B);
        #[cfg(target_feature = "zvl64b")]
        let features = features.with(ZVL64B);
        #[cfg(target_feature = "zvl65536b")]
        let features = features.with(ZVL65536B);
        #[cfg(target_feature = "zvl8192b")]
        let features = features.with(ZVL8192B);
        features
    }

}
#[cfg(any(doc, target_arch = "riscv64"))]
#[rustfmt::skip]
pub mod riscv64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        A,
        B,
        C,
        CRT_STATIC,
        D,
        E,
        F,
        M,
        RELAX,
        RVA23U64,
        SUPM,
        UNALIGNED_SCALAR_MEM,
        UNALIGNED_VECTOR_MEM,
        V,
        ZA128RS,
        ZA64RS,
        ZAAMO,
        ZABHA,
        ZACAS,
        ZALRSC,
        ZAMA16B,
        ZAWRS,
        ZBA,
        ZBB,
        ZBC,
        ZBKB,
        ZBKC,
        ZBKX,
        ZBS,
        ZCA,
        ZCB,
        ZCMOP,
        ZDINX,
        ZFA,
        ZFBFMIN,
        ZFH,
        ZFHMIN,
        ZFINX,
        ZHINX,
        ZHINXMIN,
        ZIC64B,
        ZICBOM,
        ZICBOP,
        ZICBOZ,
        ZICCAMOA,
        ZICCIF,
        ZICCLSM,
        ZICCRSE,
        ZICNTR,
        ZICOND,
        ZICSR,
        ZIFENCEI,
        ZIHINTNTL,
        ZIHINTPAUSE,
        ZIHPM,
        ZIMOP,
        ZK,
        ZKN,
        ZKND,
        ZKNE,
        ZKNH,
        ZKR,
        ZKS,
        ZKSED,
        ZKSH,
        ZKT,
        ZTSO,
        ZVBB,
        ZVBC,
        ZVE32F,
        ZVE32X,
        ZVE64D,
        ZVE64F,
        ZVE64X,
        ZVFBFMIN,
        ZVFBFWMA,
        ZVFH,
        ZVFHMIN,
        ZVKB,
        ZVKG,
        ZVKN,
        ZVKNC,
        ZVKNED,
        ZVKNG,
        ZVKNHA,
        ZVKNHB,
        ZVKS,
        ZVKSC,
        ZVKSED,
        ZVKSG,
        ZVKSH,
        ZVKT,
        ZVL1024B,
        ZVL128B,
        ZVL16384B,
        ZVL2048B,
        ZVL256B,
        ZVL32768B,
        ZVL32B,
        ZVL4096B,
        ZVL512B,
        ZVL64B,
        ZVL65536B,
        ZVL8192B,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "riscv64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "riscv64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "'A' (Atomic Instructions)."]
    pub const A: TargetFeatures = feature_set!(A, ZAAMO, ZALRSC);

    #[doc = "'B' (the collection of the Zba, Zbb, Zbs extensions)."]
    pub const B: TargetFeatures = feature_set!(B, ZBA, ZBB, ZBS);

    #[doc = "'C' (Compressed Instructions)."]
    pub const C: TargetFeatures = feature_set!(C, ZCA);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "'D' (Double-Precision Floating-Point)."]
    pub const D: TargetFeatures = feature_set!(D, F, ZICSR);

    #[doc = "'E' (Embedded Instruction Set with 16 GPRs)."]
    pub const E: TargetFeatures = feature_set!(E);

    #[doc = "'F' (Single-Precision Floating-Point)."]
    pub const F: TargetFeatures = feature_set!(F, ZICSR);

    #[doc = "'M' (Integer Multiplication and Division)."]
    pub const M: TargetFeatures = feature_set!(M);

    #[doc = "Enable Linker relaxation."]
    pub const RELAX: TargetFeatures = feature_set!(RELAX);

    #[doc = "RISC-V rva23u64 profile."]
    pub const RVA23U64: TargetFeatures = feature_set!(A, B, C, D, F, M, RVA23U64, SUPM, V, ZA128RS, ZA64RS, ZAAMO, ZALRSC, ZAWRS, ZBA, ZBB, ZBS, ZCA, ZCB, ZCMOP, ZFA, ZFHMIN, ZIC64B, ZICBOM, ZICBOP, ZICBOZ, ZICCAMOA, ZICCIF, ZICCLSM, ZICCRSE, ZICNTR, ZICOND, ZICSR, ZIHINTNTL, ZIHINTPAUSE, ZIHPM, ZIMOP, ZKT, ZVBB, ZVE32F, ZVE32X, ZVE64D, ZVE64F, ZVE64X, ZVFHMIN, ZVKB, ZVKT, ZVL128B, ZVL32B, ZVL64B);

    #[doc = "'Supm' (Indicates User-mode Pointer Masking)."]
    pub const SUPM: TargetFeatures = feature_set!(SUPM);

    #[doc = "Has reasonably performant unaligned scalar loads and stores."]
    pub const UNALIGNED_SCALAR_MEM: TargetFeatures = feature_set!(UNALIGNED_SCALAR_MEM);

    #[doc = "Has reasonably performant unaligned vector loads and stores."]
    pub const UNALIGNED_VECTOR_MEM: TargetFeatures = feature_set!(UNALIGNED_VECTOR_MEM);

    #[doc = "'V' (Vector Extension for Application Processors)."]
    pub const V: TargetFeatures = feature_set!(D, F, V, ZICSR, ZVE32F, ZVE32X, ZVE64D, ZVE64F, ZVE64X, ZVL128B, ZVL32B, ZVL64B);

    #[doc = "'Za128rs' (Reservation Set Size of at Most 128 Bytes)."]
    pub const ZA128RS: TargetFeatures = feature_set!(ZA128RS);

    #[doc = "'Za64rs' (Reservation Set Size of at Most 64 Bytes)."]
    pub const ZA64RS: TargetFeatures = feature_set!(ZA128RS, ZA64RS);

    #[doc = "'Zaamo' (Atomic Memory Operations)."]
    pub const ZAAMO: TargetFeatures = feature_set!(ZAAMO);

    #[doc = "'Zabha' (Byte and Halfword Atomic Memory Operations)."]
    pub const ZABHA: TargetFeatures = feature_set!(ZAAMO, ZABHA);

    #[doc = "'Zacas' (Atomic Compare-And-Swap Instructions)."]
    pub const ZACAS: TargetFeatures = feature_set!(ZAAMO, ZACAS);

    #[doc = "'Zalrsc' (Load-Reserved/Store-Conditional)."]
    pub const ZALRSC: TargetFeatures = feature_set!(ZALRSC);

    #[doc = "'Zama16b' (Atomic 16-byte misaligned loads, stores and AMOs)."]
    pub const ZAMA16B: TargetFeatures = feature_set!(ZAMA16B);

    #[doc = "'Zawrs' (Wait on Reservation Set)."]
    pub const ZAWRS: TargetFeatures = feature_set!(ZAWRS);

    #[doc = "'Zba' (Address Generation Instructions)."]
    pub const ZBA: TargetFeatures = feature_set!(ZBA);

    #[doc = "'Zbb' (Basic Bit-Manipulation)."]
    pub const ZBB: TargetFeatures = feature_set!(ZBB);

    #[doc = "'Zbc' (Carry-Less Multiplication)."]
    pub const ZBC: TargetFeatures = feature_set!(ZBC, ZBKC);

    #[doc = "'Zbkb' (Bitmanip instructions for Cryptography)."]
    pub const ZBKB: TargetFeatures = feature_set!(ZBKB);

    #[doc = "'Zbkc' (Carry-less multiply instructions for Cryptography)."]
    pub const ZBKC: TargetFeatures = feature_set!(ZBKC);

    #[doc = "'Zbkx' (Crossbar permutation instructions)."]
    pub const ZBKX: TargetFeatures = feature_set!(ZBKX);

    #[doc = "'Zbs' (Single-Bit Instructions)."]
    pub const ZBS: TargetFeatures = feature_set!(ZBS);

    #[doc = "'Zca' (part of the C extension, excluding compressed floating point loads/stores)."]
    pub const ZCA: TargetFeatures = feature_set!(ZCA);

    #[doc = "'Zcb' (Compressed basic bit manipulation instructions)."]
    pub const ZCB: TargetFeatures = feature_set!(ZCA, ZCB);

    #[doc = "'Zcmop' (Compressed May-Be-Operations)."]
    pub const ZCMOP: TargetFeatures = feature_set!(ZCA, ZCMOP);

    #[doc = "'Zdinx' (Double in Integer)."]
    pub const ZDINX: TargetFeatures = feature_set!(ZDINX, ZFINX, ZICSR);

    #[doc = "'Zfa' (Additional Floating-Point)."]
    pub const ZFA: TargetFeatures = feature_set!(F, ZFA, ZICSR);

    #[doc = "'Zfbfmin' (Scalar BF16 Converts)."]
    pub const ZFBFMIN: TargetFeatures = feature_set!(F, ZFBFMIN, ZICSR);

    #[doc = "'Zfh' (Half-Precision Floating-Point)."]
    pub const ZFH: TargetFeatures = feature_set!(F, ZFH, ZFHMIN, ZICSR);

    #[doc = "'Zfhmin' (Half-Precision Floating-Point Minimal)."]
    pub const ZFHMIN: TargetFeatures = feature_set!(F, ZFHMIN, ZICSR);

    #[doc = "'Zfinx' (Float in Integer)."]
    pub const ZFINX: TargetFeatures = feature_set!(ZFINX, ZICSR);

    #[doc = "'Zhinx' (Half Float in Integer)."]
    pub const ZHINX: TargetFeatures = feature_set!(ZFINX, ZHINX, ZHINXMIN, ZICSR);

    #[doc = "'Zhinxmin' (Half Float in Integer Minimal)."]
    pub const ZHINXMIN: TargetFeatures = feature_set!(ZFINX, ZHINXMIN, ZICSR);

    #[doc = "'Zic64b' (Cache Block Size Is 64 Bytes)."]
    pub const ZIC64B: TargetFeatures = feature_set!(ZIC64B);

    #[doc = "'Zicbom' (Cache-Block Management Instructions)."]
    pub const ZICBOM: TargetFeatures = feature_set!(ZICBOM);

    #[doc = "'Zicbop' (Cache-Block Prefetch Instructions)."]
    pub const ZICBOP: TargetFeatures = feature_set!(ZICBOP);

    #[doc = "'Zicboz' (Cache-Block Zero Instructions)."]
    pub const ZICBOZ: TargetFeatures = feature_set!(ZICBOZ);

    #[doc = "'Ziccamoa' (Main Memory Supports All Atomics in A)."]
    pub const ZICCAMOA: TargetFeatures = feature_set!(ZICCAMOA);

    #[doc = "'Ziccif' (Main Memory Supports Instruction Fetch with Atomicity Requirement)."]
    pub const ZICCIF: TargetFeatures = feature_set!(ZICCIF);

    #[doc = "'Zicclsm' (Main Memory Supports Misaligned Loads/Stores)."]
    pub const ZICCLSM: TargetFeatures = feature_set!(ZICCLSM);

    #[doc = "'Ziccrse' (Main Memory Supports Forward Progress on LR/SC Sequences)."]
    pub const ZICCRSE: TargetFeatures = feature_set!(ZICCRSE);

    #[doc = "'Zicntr' (Base Counters and Timers)."]
    pub const ZICNTR: TargetFeatures = feature_set!(ZICNTR, ZICSR);

    #[doc = "'Zicond' (Integer Conditional Operations)."]
    pub const ZICOND: TargetFeatures = feature_set!(ZICOND);

    #[doc = "'Zicsr' (CSRs)."]
    pub const ZICSR: TargetFeatures = feature_set!(ZICSR);

    #[doc = "'Zifencei' (fence.i)."]
    pub const ZIFENCEI: TargetFeatures = feature_set!(ZIFENCEI);

    #[doc = "'Zihintntl' (Non-Temporal Locality Hints)."]
    pub const ZIHINTNTL: TargetFeatures = feature_set!(ZIHINTNTL);

    #[doc = "'Zihintpause' (Pause Hint)."]
    pub const ZIHINTPAUSE: TargetFeatures = feature_set!(ZIHINTPAUSE);

    #[doc = "'Zihpm' (Hardware Performance Counters)."]
    pub const ZIHPM: TargetFeatures = feature_set!(ZICSR, ZIHPM);

    #[doc = "'Zimop' (May-Be-Operations)."]
    pub const ZIMOP: TargetFeatures = feature_set!(ZIMOP);

    #[doc = "'Zk' (Standard scalar cryptography extension)."]
    pub const ZK: TargetFeatures = feature_set!(ZBKB, ZBKC, ZBKX, ZK, ZKN, ZKND, ZKNE, ZKNH, ZKR, ZKT);

    #[doc = "'Zkn' (NIST Algorithm Suite)."]
    pub const ZKN: TargetFeatures = feature_set!(ZBKB, ZBKC, ZBKX, ZKN, ZKND, ZKNE, ZKNH);

    #[doc = "'Zknd' (NIST Suite: AES Decryption)."]
    pub const ZKND: TargetFeatures = feature_set!(ZKND);

    #[doc = "'Zkne' (NIST Suite: AES Encryption)."]
    pub const ZKNE: TargetFeatures = feature_set!(ZKNE);

    #[doc = "'Zknh' (NIST Suite: Hash Function Instructions)."]
    pub const ZKNH: TargetFeatures = feature_set!(ZKNH);

    #[doc = "'Zkr' (Entropy Source Extension)."]
    pub const ZKR: TargetFeatures = feature_set!(ZKR);

    #[doc = "'Zks' (ShangMi Algorithm Suite)."]
    pub const ZKS: TargetFeatures = feature_set!(ZBKB, ZBKC, ZBKX, ZKS, ZKSED, ZKSH);

    #[doc = "'Zksed' (ShangMi Suite: SM4 Block Cipher Instructions)."]
    pub const ZKSED: TargetFeatures = feature_set!(ZKSED);

    #[doc = "'Zksh' (ShangMi Suite: SM3 Hash Function Instructions)."]
    pub const ZKSH: TargetFeatures = feature_set!(ZKSH);

    #[doc = "'Zkt' (Data Independent Execution Latency)."]
    pub const ZKT: TargetFeatures = feature_set!(ZKT);

    #[doc = "'Ztso' (Memory Model"]
    pub const ZTSO: TargetFeatures = feature_set!(ZTSO);

    #[doc = "'Zvbb' (Vector basic bit-manipulation instructions)."]
    pub const ZVBB: TargetFeatures = feature_set!(ZICSR, ZVBB, ZVE32X, ZVKB, ZVL32B);

    #[doc = "'Zvbc' (Vector Carryless Multiplication)."]
    pub const ZVBC: TargetFeatures = feature_set!(ZICSR, ZVBC, ZVE32X, ZVE64X, ZVL32B, ZVL64B);

    #[doc = "'Zve32f' (Vector Extensions for Embedded Processors with maximal 32 EEW and F extension)."]
    pub const ZVE32F: TargetFeatures = feature_set!(F, ZICSR, ZVE32F, ZVE32X, ZVL32B);

    #[doc = "'Zve32x' (Vector Extensions for Embedded Processors with maximal 32 EEW)."]
    pub const ZVE32X: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVL32B);

    #[doc = "'Zve64d' (Vector Extensions for Embedded Processors with maximal 64 EEW, F and D extension)."]
    pub const ZVE64D: TargetFeatures = feature_set!(D, F, ZICSR, ZVE32F, ZVE32X, ZVE64D, ZVE64F, ZVE64X, ZVL32B, ZVL64B);

    #[doc = "'Zve64f' (Vector Extensions for Embedded Processors with maximal 64 EEW and F extension)."]
    pub const ZVE64F: TargetFeatures = feature_set!(F, ZICSR, ZVE32F, ZVE32X, ZVE64F, ZVE64X, ZVL32B, ZVL64B);

    #[doc = "'Zve64x' (Vector Extensions for Embedded Processors with maximal 64 EEW)."]
    pub const ZVE64X: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVE64X, ZVL32B, ZVL64B);

    #[doc = "'Zvfbfmin' (Vector BF16 Converts)."]
    pub const ZVFBFMIN: TargetFeatures = feature_set!(F, ZICSR, ZVE32F, ZVE32X, ZVFBFMIN, ZVL32B);

    #[doc = "'Zvfbfwma' (Vector BF16 widening mul-add)."]
    pub const ZVFBFWMA: TargetFeatures = feature_set!(F, ZFBFMIN, ZICSR, ZVE32F, ZVE32X, ZVFBFMIN, ZVFBFWMA, ZVL32B);

    #[doc = "'Zvfh' (Vector Half-Precision Floating-Point)."]
    pub const ZVFH: TargetFeatures = feature_set!(F, ZFHMIN, ZICSR, ZVE32F, ZVE32X, ZVFH, ZVFHMIN, ZVL32B);

    #[doc = "'Zvfhmin' (Vector Half-Precision Floating-Point Minimal)."]
    pub const ZVFHMIN: TargetFeatures = feature_set!(F, ZICSR, ZVE32F, ZVE32X, ZVFHMIN, ZVL32B);

    #[doc = "'Zvkb' (Vector Bit-manipulation used in Cryptography)."]
    pub const ZVKB: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKB, ZVL32B);

    #[doc = "'Zvkg' (Vector GCM instructions for Cryptography)."]
    pub const ZVKG: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKG, ZVL32B);

    #[doc = "'Zvkn' (shorthand for 'Zvkned', 'Zvknhb', 'Zvkb', and 'Zvkt')."]
    pub const ZVKN: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVE64X, ZVKB, ZVKN, ZVKNED, ZVKNHA, ZVKNHB, ZVKT, ZVL32B, ZVL64B);

    #[doc = "'Zvknc' (shorthand for 'Zvknc' and 'Zvbc')."]
    pub const ZVKNC: TargetFeatures = feature_set!(ZICSR, ZVBC, ZVE32X, ZVE64X, ZVKB, ZVKN, ZVKNC, ZVKNED, ZVKNHA, ZVKNHB, ZVKT, ZVL32B, ZVL64B);

    #[doc = "'Zvkned' (Vector AES Encryption & Decryption (Single Round))."]
    pub const ZVKNED: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKNED, ZVL32B);

    #[doc = "'Zvkng' (shorthand for 'Zvkn' and 'Zvkg')."]
    pub const ZVKNG: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVE64X, ZVKB, ZVKG, ZVKN, ZVKNED, ZVKNG, ZVKNHA, ZVKNHB, ZVKT, ZVL32B, ZVL64B);

    #[doc = "'Zvknha' (Vector SHA-2 (SHA-256 only))."]
    pub const ZVKNHA: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKNHA, ZVL32B);

    #[doc = "'Zvknhb' (Vector SHA-2 (SHA-256 and SHA-512))."]
    pub const ZVKNHB: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVE64X, ZVKNHA, ZVKNHB, ZVL32B, ZVL64B);

    #[doc = "'Zvks' (shorthand for 'Zvksed', 'Zvksh', 'Zvkb', and 'Zvkt')."]
    pub const ZVKS: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKB, ZVKS, ZVKSED, ZVKSH, ZVKT, ZVL32B);

    #[doc = "'Zvksc' (shorthand for 'Zvks' and 'Zvbc')."]
    pub const ZVKSC: TargetFeatures = feature_set!(ZICSR, ZVBC, ZVE32X, ZVE64X, ZVKB, ZVKS, ZVKSC, ZVKSED, ZVKSH, ZVKT, ZVL32B, ZVL64B);

    #[doc = "'Zvksed' (SM4 Block Cipher Instructions)."]
    pub const ZVKSED: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKSED, ZVL32B);

    #[doc = "'Zvksg' (shorthand for 'Zvks' and 'Zvkg')."]
    pub const ZVKSG: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKB, ZVKG, ZVKS, ZVKSED, ZVKSG, ZVKSH, ZVKT, ZVL32B);

    #[doc = "'Zvksh' (SM3 Hash Function Instructions)."]
    pub const ZVKSH: TargetFeatures = feature_set!(ZICSR, ZVE32X, ZVKSH, ZVL32B);

    #[doc = "'Zvkt' (Vector Data-Independent Execution Latency)."]
    pub const ZVKT: TargetFeatures = feature_set!(ZVKT);

    #[doc = "'Zvl1024b' (Minimum Vector Length 1024)."]
    pub const ZVL1024B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL256B, ZVL32B, ZVL512B, ZVL64B);

    #[doc = "'Zvl128b' (Minimum Vector Length 128)."]
    pub const ZVL128B: TargetFeatures = feature_set!(ZVL128B, ZVL32B, ZVL64B);

    #[doc = "'Zvl16384b' (Minimum Vector Length 16384)."]
    pub const ZVL16384B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL16384B, ZVL2048B, ZVL256B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B, ZVL8192B);

    #[doc = "'Zvl2048b' (Minimum Vector Length 2048)."]
    pub const ZVL2048B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL2048B, ZVL256B, ZVL32B, ZVL512B, ZVL64B);

    #[doc = "'Zvl256b' (Minimum Vector Length 256)."]
    pub const ZVL256B: TargetFeatures = feature_set!(ZVL128B, ZVL256B, ZVL32B, ZVL64B);

    #[doc = "'Zvl32768b' (Minimum Vector Length 32768)."]
    pub const ZVL32768B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL16384B, ZVL2048B, ZVL256B, ZVL32768B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B, ZVL8192B);

    #[doc = "'Zvl32b' (Minimum Vector Length 32)."]
    pub const ZVL32B: TargetFeatures = feature_set!(ZVL32B);

    #[doc = "'Zvl4096b' (Minimum Vector Length 4096)."]
    pub const ZVL4096B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL2048B, ZVL256B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B);

    #[doc = "'Zvl512b' (Minimum Vector Length 512)."]
    pub const ZVL512B: TargetFeatures = feature_set!(ZVL128B, ZVL256B, ZVL32B, ZVL512B, ZVL64B);

    #[doc = "'Zvl64b' (Minimum Vector Length 64)."]
    pub const ZVL64B: TargetFeatures = feature_set!(ZVL32B, ZVL64B);

    #[doc = "'Zvl65536b' (Minimum Vector Length 65536)."]
    pub const ZVL65536B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL16384B, ZVL2048B, ZVL256B, ZVL32768B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B, ZVL65536B, ZVL8192B);

    #[doc = "'Zvl8192b' (Minimum Vector Length 8192)."]
    pub const ZVL8192B: TargetFeatures = feature_set!(ZVL1024B, ZVL128B, ZVL2048B, ZVL256B, ZVL32B, ZVL4096B, ZVL512B, ZVL64B, ZVL8192B);


    #[cfg(target_arch = "riscv64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("a", A),
        FeatureData::new("b", B),
        FeatureData::new("c", C),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("d", D),
        FeatureData::new("e", E),
        FeatureData::new("f", F),
        FeatureData::new("m", M),
        FeatureData::new("relax", RELAX),
        FeatureData::new("rva23u64", RVA23U64),
        FeatureData::new("supm", SUPM),
        FeatureData::new("unaligned-scalar-mem", UNALIGNED_SCALAR_MEM),
        FeatureData::new("unaligned-vector-mem", UNALIGNED_VECTOR_MEM),
        FeatureData::new("v", V),
        FeatureData::new("za128rs", ZA128RS),
        FeatureData::new("za64rs", ZA64RS),
        FeatureData::new("zaamo", ZAAMO),
        FeatureData::new("zabha", ZABHA),
        FeatureData::new("zacas", ZACAS),
        FeatureData::new("zalrsc", ZALRSC),
        FeatureData::new("zama16b", ZAMA16B),
        FeatureData::new("zawrs", ZAWRS),
        FeatureData::new("zba", ZBA),
        FeatureData::new("zbb", ZBB),
        FeatureData::new("zbc", ZBC),
        FeatureData::new("zbkb", ZBKB),
        FeatureData::new("zbkc", ZBKC),
        FeatureData::new("zbkx", ZBKX),
        FeatureData::new("zbs", ZBS),
        FeatureData::new("zca", ZCA),
        FeatureData::new("zcb", ZCB),
        FeatureData::new("zcmop", ZCMOP),
        FeatureData::new("zdinx", ZDINX),
        FeatureData::new("zfa", ZFA),
        FeatureData::new("zfbfmin", ZFBFMIN),
        FeatureData::new("zfh", ZFH),
        FeatureData::new("zfhmin", ZFHMIN),
        FeatureData::new("zfinx", ZFINX),
        FeatureData::new("zhinx", ZHINX),
        FeatureData::new("zhinxmin", ZHINXMIN),
        FeatureData::new("zic64b", ZIC64B),
        FeatureData::new("zicbom", ZICBOM),
        FeatureData::new("zicbop", ZICBOP),
        FeatureData::new("zicboz", ZICBOZ),
        FeatureData::new("ziccamoa", ZICCAMOA),
        FeatureData::new("ziccif", ZICCIF),
        FeatureData::new("zicclsm", ZICCLSM),
        FeatureData::new("ziccrse", ZICCRSE),
        FeatureData::new("zicntr", ZICNTR),
        FeatureData::new("zicond", ZICOND),
        FeatureData::new("zicsr", ZICSR),
        FeatureData::new("zifencei", ZIFENCEI),
        FeatureData::new("zihintntl", ZIHINTNTL),
        FeatureData::new("zihintpause", ZIHINTPAUSE),
        FeatureData::new("zihpm", ZIHPM),
        FeatureData::new("zimop", ZIMOP),
        FeatureData::new("zk", ZK),
        FeatureData::new("zkn", ZKN),
        FeatureData::new("zknd", ZKND),
        FeatureData::new("zkne", ZKNE),
        FeatureData::new("zknh", ZKNH),
        FeatureData::new("zkr", ZKR),
        FeatureData::new("zks", ZKS),
        FeatureData::new("zksed", ZKSED),
        FeatureData::new("zksh", ZKSH),
        FeatureData::new("zkt", ZKT),
        FeatureData::new("ztso", ZTSO),
        FeatureData::new("zvbb", ZVBB),
        FeatureData::new("zvbc", ZVBC),
        FeatureData::new("zve32f", ZVE32F),
        FeatureData::new("zve32x", ZVE32X),
        FeatureData::new("zve64d", ZVE64D),
        FeatureData::new("zve64f", ZVE64F),
        FeatureData::new("zve64x", ZVE64X),
        FeatureData::new("zvfbfmin", ZVFBFMIN),
        FeatureData::new("zvfbfwma", ZVFBFWMA),
        FeatureData::new("zvfh", ZVFH),
        FeatureData::new("zvfhmin", ZVFHMIN),
        FeatureData::new("zvkb", ZVKB),
        FeatureData::new("zvkg", ZVKG),
        FeatureData::new("zvkn", ZVKN),
        FeatureData::new("zvknc", ZVKNC),
        FeatureData::new("zvkned", ZVKNED),
        FeatureData::new("zvkng", ZVKNG),
        FeatureData::new("zvknha", ZVKNHA),
        FeatureData::new("zvknhb", ZVKNHB),
        FeatureData::new("zvks", ZVKS),
        FeatureData::new("zvksc", ZVKSC),
        FeatureData::new("zvksed", ZVKSED),
        FeatureData::new("zvksg", ZVKSG),
        FeatureData::new("zvksh", ZVKSH),
        FeatureData::new("zvkt", ZVKT),
        FeatureData::new("zvl1024b", ZVL1024B),
        FeatureData::new("zvl128b", ZVL128B),
        FeatureData::new("zvl16384b", ZVL16384B),
        FeatureData::new("zvl2048b", ZVL2048B),
        FeatureData::new("zvl256b", ZVL256B),
        FeatureData::new("zvl32768b", ZVL32768B),
        FeatureData::new("zvl32b", ZVL32B),
        FeatureData::new("zvl4096b", ZVL4096B),
        FeatureData::new("zvl512b", ZVL512B),
        FeatureData::new("zvl64b", ZVL64B),
        FeatureData::new("zvl65536b", ZVL65536B),
        FeatureData::new("zvl8192b", ZVL8192B),
    ];

    #[cfg(target_arch = "riscv64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "a")]
        let features = features.with(A);
        #[cfg(target_feature = "b")]
        let features = features.with(B);
        #[cfg(target_feature = "c")]
        let features = features.with(C);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "d")]
        let features = features.with(D);
        #[cfg(target_feature = "e")]
        let features = features.with(E);
        #[cfg(target_feature = "f")]
        let features = features.with(F);
        #[cfg(target_feature = "m")]
        let features = features.with(M);
        #[cfg(target_feature = "relax")]
        let features = features.with(RELAX);
        #[cfg(target_feature = "rva23u64")]
        let features = features.with(RVA23U64);
        #[cfg(target_feature = "supm")]
        let features = features.with(SUPM);
        #[cfg(target_feature = "unaligned-scalar-mem")]
        let features = features.with(UNALIGNED_SCALAR_MEM);
        #[cfg(target_feature = "unaligned-vector-mem")]
        let features = features.with(UNALIGNED_VECTOR_MEM);
        #[cfg(target_feature = "v")]
        let features = features.with(V);
        #[cfg(target_feature = "za128rs")]
        let features = features.with(ZA128RS);
        #[cfg(target_feature = "za64rs")]
        let features = features.with(ZA64RS);
        #[cfg(target_feature = "zaamo")]
        let features = features.with(ZAAMO);
        #[cfg(target_feature = "zabha")]
        let features = features.with(ZABHA);
        #[cfg(target_feature = "zacas")]
        let features = features.with(ZACAS);
        #[cfg(target_feature = "zalrsc")]
        let features = features.with(ZALRSC);
        #[cfg(target_feature = "zama16b")]
        let features = features.with(ZAMA16B);
        #[cfg(target_feature = "zawrs")]
        let features = features.with(ZAWRS);
        #[cfg(target_feature = "zba")]
        let features = features.with(ZBA);
        #[cfg(target_feature = "zbb")]
        let features = features.with(ZBB);
        #[cfg(target_feature = "zbc")]
        let features = features.with(ZBC);
        #[cfg(target_feature = "zbkb")]
        let features = features.with(ZBKB);
        #[cfg(target_feature = "zbkc")]
        let features = features.with(ZBKC);
        #[cfg(target_feature = "zbkx")]
        let features = features.with(ZBKX);
        #[cfg(target_feature = "zbs")]
        let features = features.with(ZBS);
        #[cfg(target_feature = "zca")]
        let features = features.with(ZCA);
        #[cfg(target_feature = "zcb")]
        let features = features.with(ZCB);
        #[cfg(target_feature = "zcmop")]
        let features = features.with(ZCMOP);
        #[cfg(target_feature = "zdinx")]
        let features = features.with(ZDINX);
        #[cfg(target_feature = "zfa")]
        let features = features.with(ZFA);
        #[cfg(target_feature = "zfbfmin")]
        let features = features.with(ZFBFMIN);
        #[cfg(target_feature = "zfh")]
        let features = features.with(ZFH);
        #[cfg(target_feature = "zfhmin")]
        let features = features.with(ZFHMIN);
        #[cfg(target_feature = "zfinx")]
        let features = features.with(ZFINX);
        #[cfg(target_feature = "zhinx")]
        let features = features.with(ZHINX);
        #[cfg(target_feature = "zhinxmin")]
        let features = features.with(ZHINXMIN);
        #[cfg(target_feature = "zic64b")]
        let features = features.with(ZIC64B);
        #[cfg(target_feature = "zicbom")]
        let features = features.with(ZICBOM);
        #[cfg(target_feature = "zicbop")]
        let features = features.with(ZICBOP);
        #[cfg(target_feature = "zicboz")]
        let features = features.with(ZICBOZ);
        #[cfg(target_feature = "ziccamoa")]
        let features = features.with(ZICCAMOA);
        #[cfg(target_feature = "ziccif")]
        let features = features.with(ZICCIF);
        #[cfg(target_feature = "zicclsm")]
        let features = features.with(ZICCLSM);
        #[cfg(target_feature = "ziccrse")]
        let features = features.with(ZICCRSE);
        #[cfg(target_feature = "zicntr")]
        let features = features.with(ZICNTR);
        #[cfg(target_feature = "zicond")]
        let features = features.with(ZICOND);
        #[cfg(target_feature = "zicsr")]
        let features = features.with(ZICSR);
        #[cfg(target_feature = "zifencei")]
        let features = features.with(ZIFENCEI);
        #[cfg(target_feature = "zihintntl")]
        let features = features.with(ZIHINTNTL);
        #[cfg(target_feature = "zihintpause")]
        let features = features.with(ZIHINTPAUSE);
        #[cfg(target_feature = "zihpm")]
        let features = features.with(ZIHPM);
        #[cfg(target_feature = "zimop")]
        let features = features.with(ZIMOP);
        #[cfg(target_feature = "zk")]
        let features = features.with(ZK);
        #[cfg(target_feature = "zkn")]
        let features = features.with(ZKN);
        #[cfg(target_feature = "zknd")]
        let features = features.with(ZKND);
        #[cfg(target_feature = "zkne")]
        let features = features.with(ZKNE);
        #[cfg(target_feature = "zknh")]
        let features = features.with(ZKNH);
        #[cfg(target_feature = "zkr")]
        let features = features.with(ZKR);
        #[cfg(target_feature = "zks")]
        let features = features.with(ZKS);
        #[cfg(target_feature = "zksed")]
        let features = features.with(ZKSED);
        #[cfg(target_feature = "zksh")]
        let features = features.with(ZKSH);
        #[cfg(target_feature = "zkt")]
        let features = features.with(ZKT);
        #[cfg(target_feature = "ztso")]
        let features = features.with(ZTSO);
        #[cfg(target_feature = "zvbb")]
        let features = features.with(ZVBB);
        #[cfg(target_feature = "zvbc")]
        let features = features.with(ZVBC);
        #[cfg(target_feature = "zve32f")]
        let features = features.with(ZVE32F);
        #[cfg(target_feature = "zve32x")]
        let features = features.with(ZVE32X);
        #[cfg(target_feature = "zve64d")]
        let features = features.with(ZVE64D);
        #[cfg(target_feature = "zve64f")]
        let features = features.with(ZVE64F);
        #[cfg(target_feature = "zve64x")]
        let features = features.with(ZVE64X);
        #[cfg(target_feature = "zvfbfmin")]
        let features = features.with(ZVFBFMIN);
        #[cfg(target_feature = "zvfbfwma")]
        let features = features.with(ZVFBFWMA);
        #[cfg(target_feature = "zvfh")]
        let features = features.with(ZVFH);
        #[cfg(target_feature = "zvfhmin")]
        let features = features.with(ZVFHMIN);
        #[cfg(target_feature = "zvkb")]
        let features = features.with(ZVKB);
        #[cfg(target_feature = "zvkg")]
        let features = features.with(ZVKG);
        #[cfg(target_feature = "zvkn")]
        let features = features.with(ZVKN);
        #[cfg(target_feature = "zvknc")]
        let features = features.with(ZVKNC);
        #[cfg(target_feature = "zvkned")]
        let features = features.with(ZVKNED);
        #[cfg(target_feature = "zvkng")]
        let features = features.with(ZVKNG);
        #[cfg(target_feature = "zvknha")]
        let features = features.with(ZVKNHA);
        #[cfg(target_feature = "zvknhb")]
        let features = features.with(ZVKNHB);
        #[cfg(target_feature = "zvks")]
        let features = features.with(ZVKS);
        #[cfg(target_feature = "zvksc")]
        let features = features.with(ZVKSC);
        #[cfg(target_feature = "zvksed")]
        let features = features.with(ZVKSED);
        #[cfg(target_feature = "zvksg")]
        let features = features.with(ZVKSG);
        #[cfg(target_feature = "zvksh")]
        let features = features.with(ZVKSH);
        #[cfg(target_feature = "zvkt")]
        let features = features.with(ZVKT);
        #[cfg(target_feature = "zvl1024b")]
        let features = features.with(ZVL1024B);
        #[cfg(target_feature = "zvl128b")]
        let features = features.with(ZVL128B);
        #[cfg(target_feature = "zvl16384b")]
        let features = features.with(ZVL16384B);
        #[cfg(target_feature = "zvl2048b")]
        let features = features.with(ZVL2048B);
        #[cfg(target_feature = "zvl256b")]
        let features = features.with(ZVL256B);
        #[cfg(target_feature = "zvl32768b")]
        let features = features.with(ZVL32768B);
        #[cfg(target_feature = "zvl32b")]
        let features = features.with(ZVL32B);
        #[cfg(target_feature = "zvl4096b")]
        let features = features.with(ZVL4096B);
        #[cfg(target_feature = "zvl512b")]
        let features = features.with(ZVL512B);
        #[cfg(target_feature = "zvl64b")]
        let features = features.with(ZVL64B);
        #[cfg(target_feature = "zvl65536b")]
        let features = features.with(ZVL65536B);
        #[cfg(target_feature = "zvl8192b")]
        let features = features.with(ZVL8192B);
        features
    }

}
#[cfg(any(doc, target_arch = "s390x"))]
#[rustfmt::skip]
pub mod s390x {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        BACKCHAIN,
        CONCURRENT_FUNCTIONS,
        CRT_STATIC,
        DEFLATE_CONVERSION,
        ENHANCED_SORT,
        GUARDED_STORAGE,
        HIGH_WORD,
        MESSAGE_SECURITY_ASSIST_EXTENSION12,
        MESSAGE_SECURITY_ASSIST_EXTENSION3,
        MESSAGE_SECURITY_ASSIST_EXTENSION4,
        MESSAGE_SECURITY_ASSIST_EXTENSION5,
        MESSAGE_SECURITY_ASSIST_EXTENSION8,
        MESSAGE_SECURITY_ASSIST_EXTENSION9,
        MISCELLANEOUS_EXTENSIONS_2,
        MISCELLANEOUS_EXTENSIONS_3,
        MISCELLANEOUS_EXTENSIONS_4,
        NNP_ASSIST,
        TRANSACTIONAL_EXECUTION,
        VECTOR,
        VECTOR_ENHANCEMENTS_1,
        VECTOR_ENHANCEMENTS_2,
        VECTOR_ENHANCEMENTS_3,
        VECTOR_PACKED_DECIMAL,
        VECTOR_PACKED_DECIMAL_ENHANCEMENT,
        VECTOR_PACKED_DECIMAL_ENHANCEMENT_2,
        VECTOR_PACKED_DECIMAL_ENHANCEMENT_3,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "s390x")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "s390x")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Store the address of the caller's frame into the callee's stack frame."]
    pub const BACKCHAIN: TargetFeatures = feature_set!(BACKCHAIN);

    #[doc = "Assume that the concurrent-functions facility is installed."]
    pub const CONCURRENT_FUNCTIONS: TargetFeatures = feature_set!(CONCURRENT_FUNCTIONS);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Assume that the deflate-conversion facility is installed."]
    pub const DEFLATE_CONVERSION: TargetFeatures = feature_set!(DEFLATE_CONVERSION);

    #[doc = "Assume that the enhanced-sort facility is installed."]
    pub const ENHANCED_SORT: TargetFeatures = feature_set!(ENHANCED_SORT);

    #[doc = "Assume that the guarded-storage facility is installed."]
    pub const GUARDED_STORAGE: TargetFeatures = feature_set!(GUARDED_STORAGE);

    #[doc = "Assume that the high-word facility is installed."]
    pub const HIGH_WORD: TargetFeatures = feature_set!(HIGH_WORD);

    #[doc = "Assume that the message-security-assist extension facility 12 is installed."]
    pub const MESSAGE_SECURITY_ASSIST_EXTENSION12: TargetFeatures = feature_set!(MESSAGE_SECURITY_ASSIST_EXTENSION12);

    #[doc = "Assume that the message-security-assist extension facility 3 is installed."]
    pub const MESSAGE_SECURITY_ASSIST_EXTENSION3: TargetFeatures = feature_set!(MESSAGE_SECURITY_ASSIST_EXTENSION3);

    #[doc = "Assume that the message-security-assist extension facility 4 is installed."]
    pub const MESSAGE_SECURITY_ASSIST_EXTENSION4: TargetFeatures = feature_set!(MESSAGE_SECURITY_ASSIST_EXTENSION4);

    #[doc = "Assume that the message-security-assist extension facility 5 is installed."]
    pub const MESSAGE_SECURITY_ASSIST_EXTENSION5: TargetFeatures = feature_set!(MESSAGE_SECURITY_ASSIST_EXTENSION5);

    #[doc = "Assume that the message-security-assist extension facility 8 is installed."]
    pub const MESSAGE_SECURITY_ASSIST_EXTENSION8: TargetFeatures = feature_set!(MESSAGE_SECURITY_ASSIST_EXTENSION3, MESSAGE_SECURITY_ASSIST_EXTENSION8);

    #[doc = "Assume that the message-security-assist extension facility 9 is installed."]
    pub const MESSAGE_SECURITY_ASSIST_EXTENSION9: TargetFeatures = feature_set!(MESSAGE_SECURITY_ASSIST_EXTENSION3, MESSAGE_SECURITY_ASSIST_EXTENSION4, MESSAGE_SECURITY_ASSIST_EXTENSION9);

    #[doc = "Assume that the miscellaneous-extensions facility 2 is installed."]
    pub const MISCELLANEOUS_EXTENSIONS_2: TargetFeatures = feature_set!(MISCELLANEOUS_EXTENSIONS_2);

    #[doc = "Assume that the miscellaneous-extensions facility 3 is installed."]
    pub const MISCELLANEOUS_EXTENSIONS_3: TargetFeatures = feature_set!(MISCELLANEOUS_EXTENSIONS_3);

    #[doc = "Assume that the miscellaneous-extensions facility 4 is installed."]
    pub const MISCELLANEOUS_EXTENSIONS_4: TargetFeatures = feature_set!(MISCELLANEOUS_EXTENSIONS_4);

    #[doc = "Assume that the NNP-assist facility is installed."]
    pub const NNP_ASSIST: TargetFeatures = feature_set!(NNP_ASSIST, VECTOR);

    #[doc = "Assume that the transactional-execution facility is installed."]
    pub const TRANSACTIONAL_EXECUTION: TargetFeatures = feature_set!(TRANSACTIONAL_EXECUTION);

    #[doc = "Assume that the vectory facility is installed."]
    pub const VECTOR: TargetFeatures = feature_set!(VECTOR);

    #[doc = "Assume that the vector enhancements facility 1 is installed."]
    pub const VECTOR_ENHANCEMENTS_1: TargetFeatures = feature_set!(VECTOR, VECTOR_ENHANCEMENTS_1);

    #[doc = "Assume that the vector enhancements facility 2 is installed."]
    pub const VECTOR_ENHANCEMENTS_2: TargetFeatures = feature_set!(VECTOR, VECTOR_ENHANCEMENTS_1, VECTOR_ENHANCEMENTS_2);

    #[doc = "Assume that the vector enhancements facility 3 is installed."]
    pub const VECTOR_ENHANCEMENTS_3: TargetFeatures = feature_set!(VECTOR, VECTOR_ENHANCEMENTS_1, VECTOR_ENHANCEMENTS_2, VECTOR_ENHANCEMENTS_3);

    #[doc = "Assume that the vector packed decimal facility is installed."]
    pub const VECTOR_PACKED_DECIMAL: TargetFeatures = feature_set!(VECTOR, VECTOR_PACKED_DECIMAL);

    #[doc = "Assume that the vector packed decimal enhancement facility is installed."]
    pub const VECTOR_PACKED_DECIMAL_ENHANCEMENT: TargetFeatures = feature_set!(VECTOR, VECTOR_PACKED_DECIMAL, VECTOR_PACKED_DECIMAL_ENHANCEMENT);

    #[doc = "Assume that the vector packed decimal enhancement facility 2 is installed."]
    pub const VECTOR_PACKED_DECIMAL_ENHANCEMENT_2: TargetFeatures = feature_set!(VECTOR, VECTOR_PACKED_DECIMAL, VECTOR_PACKED_DECIMAL_ENHANCEMENT, VECTOR_PACKED_DECIMAL_ENHANCEMENT_2);

    #[doc = "Assume that the vector packed decimal enhancement facility 3 is installed."]
    pub const VECTOR_PACKED_DECIMAL_ENHANCEMENT_3: TargetFeatures = feature_set!(VECTOR, VECTOR_PACKED_DECIMAL, VECTOR_PACKED_DECIMAL_ENHANCEMENT, VECTOR_PACKED_DECIMAL_ENHANCEMENT_2, VECTOR_PACKED_DECIMAL_ENHANCEMENT_3);


    #[cfg(target_arch = "s390x")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("backchain", BACKCHAIN),
        FeatureData::new("concurrent-functions", CONCURRENT_FUNCTIONS),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("deflate-conversion", DEFLATE_CONVERSION),
        FeatureData::new("enhanced-sort", ENHANCED_SORT),
        FeatureData::new("guarded-storage", GUARDED_STORAGE),
        FeatureData::new("high-word", HIGH_WORD),
        FeatureData::new("message-security-assist-extension12", MESSAGE_SECURITY_ASSIST_EXTENSION12),
        FeatureData::new("message-security-assist-extension3", MESSAGE_SECURITY_ASSIST_EXTENSION3),
        FeatureData::new("message-security-assist-extension4", MESSAGE_SECURITY_ASSIST_EXTENSION4),
        FeatureData::new("message-security-assist-extension5", MESSAGE_SECURITY_ASSIST_EXTENSION5),
        FeatureData::new("message-security-assist-extension8", MESSAGE_SECURITY_ASSIST_EXTENSION8),
        FeatureData::new("message-security-assist-extension9", MESSAGE_SECURITY_ASSIST_EXTENSION9),
        FeatureData::new("miscellaneous-extensions-2", MISCELLANEOUS_EXTENSIONS_2),
        FeatureData::new("miscellaneous-extensions-3", MISCELLANEOUS_EXTENSIONS_3),
        FeatureData::new("miscellaneous-extensions-4", MISCELLANEOUS_EXTENSIONS_4),
        FeatureData::new("nnp-assist", NNP_ASSIST),
        FeatureData::new("transactional-execution", TRANSACTIONAL_EXECUTION),
        FeatureData::new("vector", VECTOR),
        FeatureData::new("vector-enhancements-1", VECTOR_ENHANCEMENTS_1),
        FeatureData::new("vector-enhancements-2", VECTOR_ENHANCEMENTS_2),
        FeatureData::new("vector-enhancements-3", VECTOR_ENHANCEMENTS_3),
        FeatureData::new("vector-packed-decimal", VECTOR_PACKED_DECIMAL),
        FeatureData::new("vector-packed-decimal-enhancement", VECTOR_PACKED_DECIMAL_ENHANCEMENT),
        FeatureData::new("vector-packed-decimal-enhancement-2", VECTOR_PACKED_DECIMAL_ENHANCEMENT_2),
        FeatureData::new("vector-packed-decimal-enhancement-3", VECTOR_PACKED_DECIMAL_ENHANCEMENT_3),
    ];

    #[cfg(target_arch = "s390x")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "backchain")]
        let features = features.with(BACKCHAIN);
        #[cfg(target_feature = "concurrent-functions")]
        let features = features.with(CONCURRENT_FUNCTIONS);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "deflate-conversion")]
        let features = features.with(DEFLATE_CONVERSION);
        #[cfg(target_feature = "enhanced-sort")]
        let features = features.with(ENHANCED_SORT);
        #[cfg(target_feature = "guarded-storage")]
        let features = features.with(GUARDED_STORAGE);
        #[cfg(target_feature = "high-word")]
        let features = features.with(HIGH_WORD);
        #[cfg(target_feature = "message-security-assist-extension12")]
        let features = features.with(MESSAGE_SECURITY_ASSIST_EXTENSION12);
        #[cfg(target_feature = "message-security-assist-extension3")]
        let features = features.with(MESSAGE_SECURITY_ASSIST_EXTENSION3);
        #[cfg(target_feature = "message-security-assist-extension4")]
        let features = features.with(MESSAGE_SECURITY_ASSIST_EXTENSION4);
        #[cfg(target_feature = "message-security-assist-extension5")]
        let features = features.with(MESSAGE_SECURITY_ASSIST_EXTENSION5);
        #[cfg(target_feature = "message-security-assist-extension8")]
        let features = features.with(MESSAGE_SECURITY_ASSIST_EXTENSION8);
        #[cfg(target_feature = "message-security-assist-extension9")]
        let features = features.with(MESSAGE_SECURITY_ASSIST_EXTENSION9);
        #[cfg(target_feature = "miscellaneous-extensions-2")]
        let features = features.with(MISCELLANEOUS_EXTENSIONS_2);
        #[cfg(target_feature = "miscellaneous-extensions-3")]
        let features = features.with(MISCELLANEOUS_EXTENSIONS_3);
        #[cfg(target_feature = "miscellaneous-extensions-4")]
        let features = features.with(MISCELLANEOUS_EXTENSIONS_4);
        #[cfg(target_feature = "nnp-assist")]
        let features = features.with(NNP_ASSIST);
        #[cfg(target_feature = "transactional-execution")]
        let features = features.with(TRANSACTIONAL_EXECUTION);
        #[cfg(target_feature = "vector")]
        let features = features.with(VECTOR);
        #[cfg(target_feature = "vector-enhancements-1")]
        let features = features.with(VECTOR_ENHANCEMENTS_1);
        #[cfg(target_feature = "vector-enhancements-2")]
        let features = features.with(VECTOR_ENHANCEMENTS_2);
        #[cfg(target_feature = "vector-enhancements-3")]
        let features = features.with(VECTOR_ENHANCEMENTS_3);
        #[cfg(target_feature = "vector-packed-decimal")]
        let features = features.with(VECTOR_PACKED_DECIMAL);
        #[cfg(target_feature = "vector-packed-decimal-enhancement")]
        let features = features.with(VECTOR_PACKED_DECIMAL_ENHANCEMENT);
        #[cfg(target_feature = "vector-packed-decimal-enhancement-2")]
        let features = features.with(VECTOR_PACKED_DECIMAL_ENHANCEMENT_2);
        #[cfg(target_feature = "vector-packed-decimal-enhancement-3")]
        let features = features.with(VECTOR_PACKED_DECIMAL_ENHANCEMENT_3);
        features
    }

}
#[cfg(any(doc, target_arch = "sparc"))]
#[rustfmt::skip]
pub mod sparc {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        CRT_STATIC,
        LEONCASA,
        V8PLUS,
        V9,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "sparc")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "sparc")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Enable CASA instruction for LEON3 and LEON4 processors."]
    pub const LEONCASA: TargetFeatures = feature_set!(LEONCASA);

    #[doc = "Enable V8+ mode, allowing use of 64-bit V9 instructions in 32-bit code."]
    pub const V8PLUS: TargetFeatures = feature_set!(V8PLUS);

    #[doc = "Enable SPARC-V9 instructions."]
    pub const V9: TargetFeatures = feature_set!(V9);


    #[cfg(target_arch = "sparc")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("leoncasa", LEONCASA),
        FeatureData::new("v8plus", V8PLUS),
        FeatureData::new("v9", V9),
    ];

    #[cfg(target_arch = "sparc")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "leoncasa")]
        let features = features.with(LEONCASA);
        #[cfg(target_feature = "v8plus")]
        let features = features.with(V8PLUS);
        #[cfg(target_feature = "v9")]
        let features = features.with(V9);
        features
    }

}
#[cfg(any(doc, target_arch = "sparc64"))]
#[rustfmt::skip]
pub mod sparc64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        CRT_STATIC,
        LEONCASA,
        V8PLUS,
        V9,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "sparc64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "sparc64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Enable CASA instruction for LEON3 and LEON4 processors."]
    pub const LEONCASA: TargetFeatures = feature_set!(LEONCASA);

    #[doc = "Enable V8+ mode, allowing use of 64-bit V9 instructions in 32-bit code."]
    pub const V8PLUS: TargetFeatures = feature_set!(V8PLUS);

    #[doc = "Enable SPARC-V9 instructions."]
    pub const V9: TargetFeatures = feature_set!(V9);


    #[cfg(target_arch = "sparc64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("leoncasa", LEONCASA),
        FeatureData::new("v8plus", V8PLUS),
        FeatureData::new("v9", V9),
    ];

    #[cfg(target_arch = "sparc64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "leoncasa")]
        let features = features.with(LEONCASA);
        #[cfg(target_feature = "v8plus")]
        let features = features.with(V8PLUS);
        #[cfg(target_feature = "v9")]
        let features = features.with(V9);
        features
    }

}
#[cfg(any(doc, target_arch = "wasm32"))]
#[rustfmt::skip]
pub mod wasm32 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        ATOMICS,
        BULK_MEMORY,
        CRT_STATIC,
        EXCEPTION_HANDLING,
        EXTENDED_CONST,
        GC,
        MULTIVALUE,
        MUTABLE_GLOBALS,
        NONTRAPPING_FPTOINT,
        REFERENCE_TYPES,
        RELAXED_SIMD,
        SIGN_EXT,
        SIMD128,
        TAIL_CALL,
        WIDE_ARITHMETIC,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "wasm32")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "wasm32")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enable Atomics."]
    pub const ATOMICS: TargetFeatures = feature_set!(ATOMICS);

    #[doc = "Enable bulk memory operations."]
    pub const BULK_MEMORY: TargetFeatures = feature_set!(BULK_MEMORY);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Enable Wasm exception handling."]
    pub const EXCEPTION_HANDLING: TargetFeatures = feature_set!(EXCEPTION_HANDLING);

    #[doc = "Enable extended const expressions."]
    pub const EXTENDED_CONST: TargetFeatures = feature_set!(EXTENDED_CONST);

    #[doc = "Enable wasm gc."]
    pub const GC: TargetFeatures = feature_set!(GC, REFERENCE_TYPES);

    #[doc = "Enable multivalue blocks, instructions, and functions."]
    pub const MULTIVALUE: TargetFeatures = feature_set!(MULTIVALUE);

    #[doc = "Enable mutable globals."]
    pub const MUTABLE_GLOBALS: TargetFeatures = feature_set!(MUTABLE_GLOBALS);

    #[doc = "Enable non-trapping float-to-int conversion operators."]
    pub const NONTRAPPING_FPTOINT: TargetFeatures = feature_set!(NONTRAPPING_FPTOINT);

    #[doc = "Enable reference types."]
    pub const REFERENCE_TYPES: TargetFeatures = feature_set!(REFERENCE_TYPES);

    #[doc = "Enable relaxed-simd instructions."]
    pub const RELAXED_SIMD: TargetFeatures = feature_set!(RELAXED_SIMD, SIMD128);

    #[doc = "Enable sign extension operators."]
    pub const SIGN_EXT: TargetFeatures = feature_set!(SIGN_EXT);

    #[doc = "Enable 128-bit SIMD."]
    pub const SIMD128: TargetFeatures = feature_set!(SIMD128);

    #[doc = "Enable tail call instructions."]
    pub const TAIL_CALL: TargetFeatures = feature_set!(TAIL_CALL);

    #[doc = "Enable wide-arithmetic instructions."]
    pub const WIDE_ARITHMETIC: TargetFeatures = feature_set!(WIDE_ARITHMETIC);


    #[cfg(target_arch = "wasm32")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("atomics", ATOMICS),
        FeatureData::new("bulk-memory", BULK_MEMORY),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("exception-handling", EXCEPTION_HANDLING),
        FeatureData::new("extended-const", EXTENDED_CONST),
        FeatureData::new("gc", GC),
        FeatureData::new("multivalue", MULTIVALUE),
        FeatureData::new("mutable-globals", MUTABLE_GLOBALS),
        FeatureData::new("nontrapping-fptoint", NONTRAPPING_FPTOINT),
        FeatureData::new("reference-types", REFERENCE_TYPES),
        FeatureData::new("relaxed-simd", RELAXED_SIMD),
        FeatureData::new("sign-ext", SIGN_EXT),
        FeatureData::new("simd128", SIMD128),
        FeatureData::new("tail-call", TAIL_CALL),
        FeatureData::new("wide-arithmetic", WIDE_ARITHMETIC),
    ];

    #[cfg(target_arch = "wasm32")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "atomics")]
        let features = features.with(ATOMICS);
        #[cfg(target_feature = "bulk-memory")]
        let features = features.with(BULK_MEMORY);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "exception-handling")]
        let features = features.with(EXCEPTION_HANDLING);
        #[cfg(target_feature = "extended-const")]
        let features = features.with(EXTENDED_CONST);
        #[cfg(target_feature = "gc")]
        let features = features.with(GC);
        #[cfg(target_feature = "multivalue")]
        let features = features.with(MULTIVALUE);
        #[cfg(target_feature = "mutable-globals")]
        let features = features.with(MUTABLE_GLOBALS);
        #[cfg(target_feature = "nontrapping-fptoint")]
        let features = features.with(NONTRAPPING_FPTOINT);
        #[cfg(target_feature = "reference-types")]
        let features = features.with(REFERENCE_TYPES);
        #[cfg(target_feature = "relaxed-simd")]
        let features = features.with(RELAXED_SIMD);
        #[cfg(target_feature = "sign-ext")]
        let features = features.with(SIGN_EXT);
        #[cfg(target_feature = "simd128")]
        let features = features.with(SIMD128);
        #[cfg(target_feature = "tail-call")]
        let features = features.with(TAIL_CALL);
        #[cfg(target_feature = "wide-arithmetic")]
        let features = features.with(WIDE_ARITHMETIC);
        features
    }

}
#[cfg(any(doc, target_arch = "wasm64"))]
#[rustfmt::skip]
pub mod wasm64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        ATOMICS,
        BULK_MEMORY,
        CRT_STATIC,
        EXCEPTION_HANDLING,
        EXTENDED_CONST,
        GC,
        MULTIVALUE,
        MUTABLE_GLOBALS,
        NONTRAPPING_FPTOINT,
        REFERENCE_TYPES,
        RELAXED_SIMD,
        SIGN_EXT,
        SIMD128,
        TAIL_CALL,
        WIDE_ARITHMETIC,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "wasm64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "wasm64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Enable Atomics."]
    pub const ATOMICS: TargetFeatures = feature_set!(ATOMICS);

    #[doc = "Enable bulk memory operations."]
    pub const BULK_MEMORY: TargetFeatures = feature_set!(BULK_MEMORY);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "Enable Wasm exception handling."]
    pub const EXCEPTION_HANDLING: TargetFeatures = feature_set!(EXCEPTION_HANDLING);

    #[doc = "Enable extended const expressions."]
    pub const EXTENDED_CONST: TargetFeatures = feature_set!(EXTENDED_CONST);

    #[doc = "Enable wasm gc."]
    pub const GC: TargetFeatures = feature_set!(GC, REFERENCE_TYPES);

    #[doc = "Enable multivalue blocks, instructions, and functions."]
    pub const MULTIVALUE: TargetFeatures = feature_set!(MULTIVALUE);

    #[doc = "Enable mutable globals."]
    pub const MUTABLE_GLOBALS: TargetFeatures = feature_set!(MUTABLE_GLOBALS);

    #[doc = "Enable non-trapping float-to-int conversion operators."]
    pub const NONTRAPPING_FPTOINT: TargetFeatures = feature_set!(NONTRAPPING_FPTOINT);

    #[doc = "Enable reference types."]
    pub const REFERENCE_TYPES: TargetFeatures = feature_set!(REFERENCE_TYPES);

    #[doc = "Enable relaxed-simd instructions."]
    pub const RELAXED_SIMD: TargetFeatures = feature_set!(RELAXED_SIMD, SIMD128);

    #[doc = "Enable sign extension operators."]
    pub const SIGN_EXT: TargetFeatures = feature_set!(SIGN_EXT);

    #[doc = "Enable 128-bit SIMD."]
    pub const SIMD128: TargetFeatures = feature_set!(SIMD128);

    #[doc = "Enable tail call instructions."]
    pub const TAIL_CALL: TargetFeatures = feature_set!(TAIL_CALL);

    #[doc = "Enable wide-arithmetic instructions."]
    pub const WIDE_ARITHMETIC: TargetFeatures = feature_set!(WIDE_ARITHMETIC);


    #[cfg(target_arch = "wasm64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("atomics", ATOMICS),
        FeatureData::new("bulk-memory", BULK_MEMORY),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("exception-handling", EXCEPTION_HANDLING),
        FeatureData::new("extended-const", EXTENDED_CONST),
        FeatureData::new("gc", GC),
        FeatureData::new("multivalue", MULTIVALUE),
        FeatureData::new("mutable-globals", MUTABLE_GLOBALS),
        FeatureData::new("nontrapping-fptoint", NONTRAPPING_FPTOINT),
        FeatureData::new("reference-types", REFERENCE_TYPES),
        FeatureData::new("relaxed-simd", RELAXED_SIMD),
        FeatureData::new("sign-ext", SIGN_EXT),
        FeatureData::new("simd128", SIMD128),
        FeatureData::new("tail-call", TAIL_CALL),
        FeatureData::new("wide-arithmetic", WIDE_ARITHMETIC),
    ];

    #[cfg(target_arch = "wasm64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "atomics")]
        let features = features.with(ATOMICS);
        #[cfg(target_feature = "bulk-memory")]
        let features = features.with(BULK_MEMORY);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "exception-handling")]
        let features = features.with(EXCEPTION_HANDLING);
        #[cfg(target_feature = "extended-const")]
        let features = features.with(EXTENDED_CONST);
        #[cfg(target_feature = "gc")]
        let features = features.with(GC);
        #[cfg(target_feature = "multivalue")]
        let features = features.with(MULTIVALUE);
        #[cfg(target_feature = "mutable-globals")]
        let features = features.with(MUTABLE_GLOBALS);
        #[cfg(target_feature = "nontrapping-fptoint")]
        let features = features.with(NONTRAPPING_FPTOINT);
        #[cfg(target_feature = "reference-types")]
        let features = features.with(REFERENCE_TYPES);
        #[cfg(target_feature = "relaxed-simd")]
        let features = features.with(RELAXED_SIMD);
        #[cfg(target_feature = "sign-ext")]
        let features = features.with(SIGN_EXT);
        #[cfg(target_feature = "simd128")]
        let features = features.with(SIMD128);
        #[cfg(target_feature = "tail-call")]
        let features = features.with(TAIL_CALL);
        #[cfg(target_feature = "wide-arithmetic")]
        let features = features.with(WIDE_ARITHMETIC);
        features
    }

}
#[cfg(any(doc, target_arch = "x86"))]
#[rustfmt::skip]
pub mod x86 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        ADX,
        AES,
        AMX_AVX512,
        AMX_BF16,
        AMX_COMPLEX,
        AMX_FP16,
        AMX_FP8,
        AMX_INT8,
        AMX_MOVRS,
        AMX_TILE,
        APXF,
        AVX,
        AVX10_1,
        AVX10_2,
        AVX2,
        AVX512BF16,
        AVX512BITALG,
        AVX512BW,
        AVX512CD,
        AVX512DQ,
        AVX512F,
        AVX512FP16,
        AVX512IFMA,
        AVX512VBMI,
        AVX512VBMI2,
        AVX512VL,
        AVX512VNNI,
        AVX512VP2INTERSECT,
        AVX512VPOPCNTDQ,
        AVXIFMA,
        AVXNECONVERT,
        AVXVNNI,
        AVXVNNIINT16,
        AVXVNNIINT8,
        BMI1,
        BMI2,
        CLFLUSHOPT,
        CMPXCHG16B,
        CRT_STATIC,
        ERMSB,
        F16C,
        FMA,
        FMA4,
        FXSR,
        GFNI,
        KL,
        LAHFSAHF,
        LZCNT,
        MOVBE,
        MOVRS,
        PCLMULQDQ,
        POPCNT,
        PRFCHW,
        RDRAND,
        RDSEED,
        RTM,
        SHA,
        SHA512,
        SM3,
        SM4,
        SSE,
        SSE2,
        SSE3,
        SSE4_1,
        SSE4_2,
        SSE4A,
        SSSE3,
        TBM,
        VAES,
        VPCLMULQDQ,
        WIDEKL,
        X87,
        XOP,
        XSAVE,
        XSAVEC,
        XSAVEOPT,
        XSAVES,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "x86")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "x86")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Support ADX instructions."]
    pub const ADX: TargetFeatures = feature_set!(ADX);

    #[doc = "Enable AES instructions."]
    pub const AES: TargetFeatures = feature_set!(AES, SSE, SSE2);

    #[doc = "Support AMX-AVX512 instructions."]
    pub const AMX_AVX512: TargetFeatures = feature_set!(AMX_AVX512, AMX_TILE);

    #[doc = "Support AMX-BF16 instructions."]
    pub const AMX_BF16: TargetFeatures = feature_set!(AMX_BF16, AMX_TILE);

    #[doc = "Support AMX-COMPLEX instructions."]
    pub const AMX_COMPLEX: TargetFeatures = feature_set!(AMX_COMPLEX, AMX_TILE);

    #[doc = "Support AMX amx-fp16 instructions."]
    pub const AMX_FP16: TargetFeatures = feature_set!(AMX_FP16, AMX_TILE);

    #[doc = "Support AMX-FP8 instructions."]
    pub const AMX_FP8: TargetFeatures = feature_set!(AMX_FP8, AMX_TILE);

    #[doc = "Support AMX-INT8 instructions."]
    pub const AMX_INT8: TargetFeatures = feature_set!(AMX_INT8, AMX_TILE);

    #[doc = "Support AMX-MOVRS instructions."]
    pub const AMX_MOVRS: TargetFeatures = feature_set!(AMX_MOVRS, AMX_TILE);

    #[doc = "Support AMX-TILE instructions."]
    pub const AMX_TILE: TargetFeatures = feature_set!(AMX_TILE);

    #[doc = "Support extended general purpose register."]
    pub const APXF: TargetFeatures = feature_set!(APXF);

    #[doc = "Enable AVX instructions."]
    pub const AVX: TargetFeatures = feature_set!(AVX, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support AVX10.1 instruction."]
    pub const AVX10_1: TargetFeatures = feature_set!(AVX, AVX10_1, AVX2, AVX512BF16, AVX512BITALG, AVX512BW, AVX512CD, AVX512DQ, AVX512F, AVX512FP16, AVX512IFMA, AVX512VBMI, AVX512VBMI2, AVX512VL, AVX512VNNI, AVX512VPOPCNTDQ, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support AVX10.2 instruction."]
    pub const AVX10_2: TargetFeatures = feature_set!(AVX, AVX10_1, AVX10_2, AVX2, AVX512BF16, AVX512BITALG, AVX512BW, AVX512CD, AVX512DQ, AVX512F, AVX512FP16, AVX512IFMA, AVX512VBMI, AVX512VBMI2, AVX512VL, AVX512VNNI, AVX512VPOPCNTDQ, AVXVNNI, AVXVNNIINT16, AVXVNNIINT8, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX2 instructions."]
    pub const AVX2: TargetFeatures = feature_set!(AVX, AVX2, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support bfloat16 floating point."]
    pub const AVX512BF16: TargetFeatures = feature_set!(AVX, AVX2, AVX512BF16, AVX512BW, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Bit Algorithms."]
    pub const AVX512BITALG: TargetFeatures = feature_set!(AVX, AVX2, AVX512BITALG, AVX512BW, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Byte and Word Instructions."]
    pub const AVX512BW: TargetFeatures = feature_set!(AVX, AVX2, AVX512BW, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Conflict Detection Instructions."]
    pub const AVX512CD: TargetFeatures = feature_set!(AVX, AVX2, AVX512CD, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Doubleword and Quadword Instructions."]
    pub const AVX512DQ: TargetFeatures = feature_set!(AVX, AVX2, AVX512DQ, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 instructions."]
    pub const AVX512F: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support 16-bit floating point."]
    pub const AVX512FP16: TargetFeatures = feature_set!(AVX, AVX2, AVX512BW, AVX512F, AVX512FP16, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Integer Fused Multiple-Add."]
    pub const AVX512IFMA: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512IFMA, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Vector Byte Manipulation Instructions."]
    pub const AVX512VBMI: TargetFeatures = feature_set!(AVX, AVX2, AVX512BW, AVX512F, AVX512VBMI, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 further Vector Byte Manipulation Instructions."]
    pub const AVX512VBMI2: TargetFeatures = feature_set!(AVX, AVX2, AVX512BW, AVX512F, AVX512VBMI2, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Vector Length eXtensions."]
    pub const AVX512VL: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512VL, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Vector Neural Network Instructions."]
    pub const AVX512VNNI: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512VNNI, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 vp2intersect."]
    pub const AVX512VP2INTERSECT: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512VP2INTERSECT, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Population Count Instructions."]
    pub const AVX512VPOPCNTDQ: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512VPOPCNTDQ, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-IFMA."]
    pub const AVXIFMA: TargetFeatures = feature_set!(AVX, AVX2, AVXIFMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support AVX-NE-CONVERT instructions."]
    pub const AVXNECONVERT: TargetFeatures = feature_set!(AVX, AVX2, AVXNECONVERT, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support AVX_VNNI encoding."]
    pub const AVXVNNI: TargetFeatures = feature_set!(AVX, AVX2, AVXVNNI, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-VNNI-INT16."]
    pub const AVXVNNIINT16: TargetFeatures = feature_set!(AVX, AVX2, AVXVNNIINT16, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-VNNI-INT8."]
    pub const AVXVNNIINT8: TargetFeatures = feature_set!(AVX, AVX2, AVXVNNIINT8, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support BMI instructions."]
    pub const BMI1: TargetFeatures = feature_set!(BMI1);

    #[doc = "Support BMI2 instructions."]
    pub const BMI2: TargetFeatures = feature_set!(BMI2);

    #[doc = "Flush A Cache Line Optimized."]
    pub const CLFLUSHOPT: TargetFeatures = feature_set!(CLFLUSHOPT);

    #[doc = "64-bit with cmpxchg16b (this is true for most x86-64 chips, but not the first AMD chips)."]
    pub const CMPXCHG16B: TargetFeatures = feature_set!(CMPXCHG16B);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "REP MOVS/STOS are fast."]
    pub const ERMSB: TargetFeatures = feature_set!(ERMSB);

    #[doc = "Support 16-bit floating point conversion instructions."]
    pub const F16C: TargetFeatures = feature_set!(AVX, F16C, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable three-operand fused multiple-add."]
    pub const FMA: TargetFeatures = feature_set!(AVX, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable four-operand fused multiple-add."]
    pub const FMA4: TargetFeatures = feature_set!(AVX, FMA4, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSE4A, SSSE3);

    #[doc = "Support fxsave/fxrestore instructions."]
    pub const FXSR: TargetFeatures = feature_set!(FXSR);

    #[doc = "Enable Galois Field Arithmetic Instructions."]
    pub const GFNI: TargetFeatures = feature_set!(GFNI, SSE, SSE2);

    #[doc = "Support Key Locker kl Instructions."]
    pub const KL: TargetFeatures = feature_set!(KL, SSE, SSE2);

    #[doc = "Support LAHF and SAHF instructions in 64-bit mode."]
    pub const LAHFSAHF: TargetFeatures = feature_set!(LAHFSAHF);

    #[doc = "Support LZCNT instruction."]
    pub const LZCNT: TargetFeatures = feature_set!(LZCNT);

    #[doc = "Support MOVBE instruction."]
    pub const MOVBE: TargetFeatures = feature_set!(MOVBE);

    #[doc = "Enable MOVRS."]
    pub const MOVRS: TargetFeatures = feature_set!(MOVRS);

    #[doc = "Enable packed carry-less multiplication instructions."]
    pub const PCLMULQDQ: TargetFeatures = feature_set!(PCLMULQDQ, SSE, SSE2);

    #[doc = "Support POPCNT instruction."]
    pub const POPCNT: TargetFeatures = feature_set!(POPCNT);

    #[doc = "Support PRFCHW instructions."]
    pub const PRFCHW: TargetFeatures = feature_set!(PRFCHW);

    #[doc = "Support RDRAND instruction."]
    pub const RDRAND: TargetFeatures = feature_set!(RDRAND);

    #[doc = "Support RDSEED instruction."]
    pub const RDSEED: TargetFeatures = feature_set!(RDSEED);

    #[doc = "Support RTM instructions."]
    pub const RTM: TargetFeatures = feature_set!(RTM);

    #[doc = "Enable SHA instructions."]
    pub const SHA: TargetFeatures = feature_set!(SHA, SSE, SSE2);

    #[doc = "Support SHA512 instructions."]
    pub const SHA512: TargetFeatures = feature_set!(AVX, AVX2, SHA512, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support SM3 instructions."]
    pub const SM3: TargetFeatures = feature_set!(AVX, SM3, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support SM4 instructions."]
    pub const SM4: TargetFeatures = feature_set!(AVX, AVX2, SM4, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable SSE instructions."]
    pub const SSE: TargetFeatures = feature_set!(SSE);

    #[doc = "Enable SSE2 instructions."]
    pub const SSE2: TargetFeatures = feature_set!(SSE, SSE2);

    #[doc = "Enable SSE3 instructions."]
    pub const SSE3: TargetFeatures = feature_set!(SSE, SSE2, SSE3);

    #[doc = "Enable SSE 4.1 instructions."]
    pub const SSE4_1: TargetFeatures = feature_set!(SSE, SSE2, SSE3, SSE4_1, SSSE3);

    #[doc = "Enable SSE 4.2 instructions."]
    pub const SSE4_2: TargetFeatures = feature_set!(SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support SSE 4a instructions."]
    pub const SSE4A: TargetFeatures = feature_set!(SSE, SSE2, SSE3, SSE4A);

    #[doc = "Enable SSSE3 instructions."]
    pub const SSSE3: TargetFeatures = feature_set!(SSE, SSE2, SSE3, SSSE3);

    #[doc = "Enable TBM instructions."]
    pub const TBM: TargetFeatures = feature_set!(TBM);

    #[doc = "Promote selected AES instructions to AVX512/AVX registers."]
    pub const VAES: TargetFeatures = feature_set!(AES, AVX, AVX2, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3, VAES);

    #[doc = "Enable vpclmulqdq instructions."]
    pub const VPCLMULQDQ: TargetFeatures = feature_set!(AVX, PCLMULQDQ, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3, VPCLMULQDQ);

    #[doc = "Support Key Locker wide Instructions."]
    pub const WIDEKL: TargetFeatures = feature_set!(KL, SSE, SSE2, WIDEKL);

    #[doc = "Enable X87 float instructions."]
    pub const X87: TargetFeatures = feature_set!(X87);

    #[doc = "Enable XOP instructions."]
    pub const XOP: TargetFeatures = feature_set!(AVX, FMA4, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSE4A, SSSE3, XOP);

    #[doc = "Support xsave instructions."]
    pub const XSAVE: TargetFeatures = feature_set!(XSAVE);

    #[doc = "Support xsavec instructions."]
    pub const XSAVEC: TargetFeatures = feature_set!(XSAVE, XSAVEC);

    #[doc = "Support xsaveopt instructions."]
    pub const XSAVEOPT: TargetFeatures = feature_set!(XSAVE, XSAVEOPT);

    #[doc = "Support xsaves instructions."]
    pub const XSAVES: TargetFeatures = feature_set!(XSAVE, XSAVES);


    #[cfg(target_arch = "x86")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("adx", ADX),
        FeatureData::new("aes", AES),
        FeatureData::new("amx-avx512", AMX_AVX512),
        FeatureData::new("amx-bf16", AMX_BF16),
        FeatureData::new("amx-complex", AMX_COMPLEX),
        FeatureData::new("amx-fp16", AMX_FP16),
        FeatureData::new("amx-fp8", AMX_FP8),
        FeatureData::new("amx-int8", AMX_INT8),
        FeatureData::new("amx-movrs", AMX_MOVRS),
        FeatureData::new("amx-tile", AMX_TILE),
        FeatureData::new("apxf", APXF),
        FeatureData::new("avx", AVX),
        FeatureData::new("avx10.1", AVX10_1),
        FeatureData::new("avx10.2", AVX10_2),
        FeatureData::new("avx2", AVX2),
        FeatureData::new("avx512bf16", AVX512BF16),
        FeatureData::new("avx512bitalg", AVX512BITALG),
        FeatureData::new("avx512bw", AVX512BW),
        FeatureData::new("avx512cd", AVX512CD),
        FeatureData::new("avx512dq", AVX512DQ),
        FeatureData::new("avx512f", AVX512F),
        FeatureData::new("avx512fp16", AVX512FP16),
        FeatureData::new("avx512ifma", AVX512IFMA),
        FeatureData::new("avx512vbmi", AVX512VBMI),
        FeatureData::new("avx512vbmi2", AVX512VBMI2),
        FeatureData::new("avx512vl", AVX512VL),
        FeatureData::new("avx512vnni", AVX512VNNI),
        FeatureData::new("avx512vp2intersect", AVX512VP2INTERSECT),
        FeatureData::new("avx512vpopcntdq", AVX512VPOPCNTDQ),
        FeatureData::new("avxifma", AVXIFMA),
        FeatureData::new("avxneconvert", AVXNECONVERT),
        FeatureData::new("avxvnni", AVXVNNI),
        FeatureData::new("avxvnniint16", AVXVNNIINT16),
        FeatureData::new("avxvnniint8", AVXVNNIINT8),
        FeatureData::new("bmi1", BMI1),
        FeatureData::new("bmi2", BMI2),
        FeatureData::new("clflushopt", CLFLUSHOPT),
        FeatureData::new("cmpxchg16b", CMPXCHG16B),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("ermsb", ERMSB),
        FeatureData::new("f16c", F16C),
        FeatureData::new("fma", FMA),
        FeatureData::new("fma4", FMA4),
        FeatureData::new("fxsr", FXSR),
        FeatureData::new("gfni", GFNI),
        FeatureData::new("kl", KL),
        FeatureData::new("lahfsahf", LAHFSAHF),
        FeatureData::new("lzcnt", LZCNT),
        FeatureData::new("movbe", MOVBE),
        FeatureData::new("movrs", MOVRS),
        FeatureData::new("pclmulqdq", PCLMULQDQ),
        FeatureData::new("popcnt", POPCNT),
        FeatureData::new("prfchw", PRFCHW),
        FeatureData::new("rdrand", RDRAND),
        FeatureData::new("rdseed", RDSEED),
        FeatureData::new("rtm", RTM),
        FeatureData::new("sha", SHA),
        FeatureData::new("sha512", SHA512),
        FeatureData::new("sm3", SM3),
        FeatureData::new("sm4", SM4),
        FeatureData::new("sse", SSE),
        FeatureData::new("sse2", SSE2),
        FeatureData::new("sse3", SSE3),
        FeatureData::new("sse4.1", SSE4_1),
        FeatureData::new("sse4.2", SSE4_2),
        FeatureData::new("sse4a", SSE4A),
        FeatureData::new("ssse3", SSSE3),
        FeatureData::new("tbm", TBM),
        FeatureData::new("vaes", VAES),
        FeatureData::new("vpclmulqdq", VPCLMULQDQ),
        FeatureData::new("widekl", WIDEKL),
        FeatureData::new("x87", X87),
        FeatureData::new("xop", XOP),
        FeatureData::new("xsave", XSAVE),
        FeatureData::new("xsavec", XSAVEC),
        FeatureData::new("xsaveopt", XSAVEOPT),
        FeatureData::new("xsaves", XSAVES),
    ];

    #[cfg(target_arch = "x86")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "adx")]
        let features = features.with(ADX);
        #[cfg(target_feature = "aes")]
        let features = features.with(AES);
        #[cfg(target_feature = "amx-avx512")]
        let features = features.with(AMX_AVX512);
        #[cfg(target_feature = "amx-bf16")]
        let features = features.with(AMX_BF16);
        #[cfg(target_feature = "amx-complex")]
        let features = features.with(AMX_COMPLEX);
        #[cfg(target_feature = "amx-fp16")]
        let features = features.with(AMX_FP16);
        #[cfg(target_feature = "amx-fp8")]
        let features = features.with(AMX_FP8);
        #[cfg(target_feature = "amx-int8")]
        let features = features.with(AMX_INT8);
        #[cfg(target_feature = "amx-movrs")]
        let features = features.with(AMX_MOVRS);
        #[cfg(target_feature = "amx-tile")]
        let features = features.with(AMX_TILE);
        #[cfg(target_feature = "apxf")]
        let features = features.with(APXF);
        #[cfg(target_feature = "avx")]
        let features = features.with(AVX);
        #[cfg(target_feature = "avx10.1")]
        let features = features.with(AVX10_1);
        #[cfg(target_feature = "avx10.2")]
        let features = features.with(AVX10_2);
        #[cfg(target_feature = "avx2")]
        let features = features.with(AVX2);
        #[cfg(target_feature = "avx512bf16")]
        let features = features.with(AVX512BF16);
        #[cfg(target_feature = "avx512bitalg")]
        let features = features.with(AVX512BITALG);
        #[cfg(target_feature = "avx512bw")]
        let features = features.with(AVX512BW);
        #[cfg(target_feature = "avx512cd")]
        let features = features.with(AVX512CD);
        #[cfg(target_feature = "avx512dq")]
        let features = features.with(AVX512DQ);
        #[cfg(target_feature = "avx512f")]
        let features = features.with(AVX512F);
        #[cfg(target_feature = "avx512fp16")]
        let features = features.with(AVX512FP16);
        #[cfg(target_feature = "avx512ifma")]
        let features = features.with(AVX512IFMA);
        #[cfg(target_feature = "avx512vbmi")]
        let features = features.with(AVX512VBMI);
        #[cfg(target_feature = "avx512vbmi2")]
        let features = features.with(AVX512VBMI2);
        #[cfg(target_feature = "avx512vl")]
        let features = features.with(AVX512VL);
        #[cfg(target_feature = "avx512vnni")]
        let features = features.with(AVX512VNNI);
        #[cfg(target_feature = "avx512vp2intersect")]
        let features = features.with(AVX512VP2INTERSECT);
        #[cfg(target_feature = "avx512vpopcntdq")]
        let features = features.with(AVX512VPOPCNTDQ);
        #[cfg(target_feature = "avxifma")]
        let features = features.with(AVXIFMA);
        #[cfg(target_feature = "avxneconvert")]
        let features = features.with(AVXNECONVERT);
        #[cfg(target_feature = "avxvnni")]
        let features = features.with(AVXVNNI);
        #[cfg(target_feature = "avxvnniint16")]
        let features = features.with(AVXVNNIINT16);
        #[cfg(target_feature = "avxvnniint8")]
        let features = features.with(AVXVNNIINT8);
        #[cfg(target_feature = "bmi1")]
        let features = features.with(BMI1);
        #[cfg(target_feature = "bmi2")]
        let features = features.with(BMI2);
        #[cfg(target_feature = "clflushopt")]
        let features = features.with(CLFLUSHOPT);
        #[cfg(target_feature = "cmpxchg16b")]
        let features = features.with(CMPXCHG16B);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "ermsb")]
        let features = features.with(ERMSB);
        #[cfg(target_feature = "f16c")]
        let features = features.with(F16C);
        #[cfg(target_feature = "fma")]
        let features = features.with(FMA);
        #[cfg(target_feature = "fma4")]
        let features = features.with(FMA4);
        #[cfg(target_feature = "fxsr")]
        let features = features.with(FXSR);
        #[cfg(target_feature = "gfni")]
        let features = features.with(GFNI);
        #[cfg(target_feature = "kl")]
        let features = features.with(KL);
        #[cfg(target_feature = "lahfsahf")]
        let features = features.with(LAHFSAHF);
        #[cfg(target_feature = "lzcnt")]
        let features = features.with(LZCNT);
        #[cfg(target_feature = "movbe")]
        let features = features.with(MOVBE);
        #[cfg(target_feature = "movrs")]
        let features = features.with(MOVRS);
        #[cfg(target_feature = "pclmulqdq")]
        let features = features.with(PCLMULQDQ);
        #[cfg(target_feature = "popcnt")]
        let features = features.with(POPCNT);
        #[cfg(target_feature = "prfchw")]
        let features = features.with(PRFCHW);
        #[cfg(target_feature = "rdrand")]
        let features = features.with(RDRAND);
        #[cfg(target_feature = "rdseed")]
        let features = features.with(RDSEED);
        #[cfg(target_feature = "rtm")]
        let features = features.with(RTM);
        #[cfg(target_feature = "sha")]
        let features = features.with(SHA);
        #[cfg(target_feature = "sha512")]
        let features = features.with(SHA512);
        #[cfg(target_feature = "sm3")]
        let features = features.with(SM3);
        #[cfg(target_feature = "sm4")]
        let features = features.with(SM4);
        #[cfg(target_feature = "sse")]
        let features = features.with(SSE);
        #[cfg(target_feature = "sse2")]
        let features = features.with(SSE2);
        #[cfg(target_feature = "sse3")]
        let features = features.with(SSE3);
        #[cfg(target_feature = "sse4.1")]
        let features = features.with(SSE4_1);
        #[cfg(target_feature = "sse4.2")]
        let features = features.with(SSE4_2);
        #[cfg(target_feature = "sse4a")]
        let features = features.with(SSE4A);
        #[cfg(target_feature = "ssse3")]
        let features = features.with(SSSE3);
        #[cfg(target_feature = "tbm")]
        let features = features.with(TBM);
        #[cfg(target_feature = "vaes")]
        let features = features.with(VAES);
        #[cfg(target_feature = "vpclmulqdq")]
        let features = features.with(VPCLMULQDQ);
        #[cfg(target_feature = "widekl")]
        let features = features.with(WIDEKL);
        #[cfg(target_feature = "x87")]
        let features = features.with(X87);
        #[cfg(target_feature = "xop")]
        let features = features.with(XOP);
        #[cfg(target_feature = "xsave")]
        let features = features.with(XSAVE);
        #[cfg(target_feature = "xsavec")]
        let features = features.with(XSAVEC);
        #[cfg(target_feature = "xsaveopt")]
        let features = features.with(XSAVEOPT);
        #[cfg(target_feature = "xsaves")]
        let features = features.with(XSAVES);
        features
    }

}
#[cfg(any(doc, target_arch = "x86_64"))]
#[rustfmt::skip]
pub mod x86_64 {
    use super::*;

    #[repr(u16)]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    enum FeatureId {
        ADX,
        AES,
        AMX_AVX512,
        AMX_BF16,
        AMX_COMPLEX,
        AMX_FP16,
        AMX_FP8,
        AMX_INT8,
        AMX_MOVRS,
        AMX_TILE,
        APXF,
        AVX,
        AVX10_1,
        AVX10_2,
        AVX2,
        AVX512BF16,
        AVX512BITALG,
        AVX512BW,
        AVX512CD,
        AVX512DQ,
        AVX512F,
        AVX512FP16,
        AVX512IFMA,
        AVX512VBMI,
        AVX512VBMI2,
        AVX512VL,
        AVX512VNNI,
        AVX512VP2INTERSECT,
        AVX512VPOPCNTDQ,
        AVXIFMA,
        AVXNECONVERT,
        AVXVNNI,
        AVXVNNIINT16,
        AVXVNNIINT8,
        BMI1,
        BMI2,
        CLFLUSHOPT,
        CMPXCHG16B,
        CRT_STATIC,
        ERMSB,
        F16C,
        FMA,
        FMA4,
        FXSR,
        GFNI,
        KL,
        LAHFSAHF,
        LZCNT,
        MOVBE,
        MOVRS,
        PCLMULQDQ,
        POPCNT,
        PRFCHW,
        RDRAND,
        RDSEED,
        RTM,
        SHA,
        SHA512,
        SM3,
        SM4,
        SSE,
        SSE2,
        SSE3,
        SSE4_1,
        SSE4_2,
        SSE4A,
        SSSE3,
        TBM,
        VAES,
        VPCLMULQDQ,
        WIDEKL,
        X87,
        XOP,
        XSAVE,
        XSAVEC,
        XSAVEOPT,
        XSAVES,
    }

    macro_rules! feature_set {
        ($($feature:ident),* $(,)?) => { {
            #[cfg(target_arch = "x86_64")]
            {
                let features = TargetFeatures::empty();
                $(let features = features.with_bit(FeatureId::$feature as usize);)*
                features
            }
            #[cfg(all(doc, not(target_arch = "x86_64")))]
            {
                TargetFeatures::empty()
            }
        } };
    }
    #[doc = "Support ADX instructions."]
    pub const ADX: TargetFeatures = feature_set!(ADX);

    #[doc = "Enable AES instructions."]
    pub const AES: TargetFeatures = feature_set!(AES, SSE, SSE2);

    #[doc = "Support AMX-AVX512 instructions."]
    pub const AMX_AVX512: TargetFeatures = feature_set!(AMX_AVX512, AMX_TILE);

    #[doc = "Support AMX-BF16 instructions."]
    pub const AMX_BF16: TargetFeatures = feature_set!(AMX_BF16, AMX_TILE);

    #[doc = "Support AMX-COMPLEX instructions."]
    pub const AMX_COMPLEX: TargetFeatures = feature_set!(AMX_COMPLEX, AMX_TILE);

    #[doc = "Support AMX amx-fp16 instructions."]
    pub const AMX_FP16: TargetFeatures = feature_set!(AMX_FP16, AMX_TILE);

    #[doc = "Support AMX-FP8 instructions."]
    pub const AMX_FP8: TargetFeatures = feature_set!(AMX_FP8, AMX_TILE);

    #[doc = "Support AMX-INT8 instructions."]
    pub const AMX_INT8: TargetFeatures = feature_set!(AMX_INT8, AMX_TILE);

    #[doc = "Support AMX-MOVRS instructions."]
    pub const AMX_MOVRS: TargetFeatures = feature_set!(AMX_MOVRS, AMX_TILE);

    #[doc = "Support AMX-TILE instructions."]
    pub const AMX_TILE: TargetFeatures = feature_set!(AMX_TILE);

    #[doc = "Support extended general purpose register."]
    pub const APXF: TargetFeatures = feature_set!(APXF);

    #[doc = "Enable AVX instructions."]
    pub const AVX: TargetFeatures = feature_set!(AVX, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support AVX10.1 instruction."]
    pub const AVX10_1: TargetFeatures = feature_set!(AVX, AVX10_1, AVX2, AVX512BF16, AVX512BITALG, AVX512BW, AVX512CD, AVX512DQ, AVX512F, AVX512FP16, AVX512IFMA, AVX512VBMI, AVX512VBMI2, AVX512VL, AVX512VNNI, AVX512VPOPCNTDQ, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support AVX10.2 instruction."]
    pub const AVX10_2: TargetFeatures = feature_set!(AVX, AVX10_1, AVX10_2, AVX2, AVX512BF16, AVX512BITALG, AVX512BW, AVX512CD, AVX512DQ, AVX512F, AVX512FP16, AVX512IFMA, AVX512VBMI, AVX512VBMI2, AVX512VL, AVX512VNNI, AVX512VPOPCNTDQ, AVXVNNI, AVXVNNIINT16, AVXVNNIINT8, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX2 instructions."]
    pub const AVX2: TargetFeatures = feature_set!(AVX, AVX2, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support bfloat16 floating point."]
    pub const AVX512BF16: TargetFeatures = feature_set!(AVX, AVX2, AVX512BF16, AVX512BW, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Bit Algorithms."]
    pub const AVX512BITALG: TargetFeatures = feature_set!(AVX, AVX2, AVX512BITALG, AVX512BW, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Byte and Word Instructions."]
    pub const AVX512BW: TargetFeatures = feature_set!(AVX, AVX2, AVX512BW, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Conflict Detection Instructions."]
    pub const AVX512CD: TargetFeatures = feature_set!(AVX, AVX2, AVX512CD, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Doubleword and Quadword Instructions."]
    pub const AVX512DQ: TargetFeatures = feature_set!(AVX, AVX2, AVX512DQ, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 instructions."]
    pub const AVX512F: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support 16-bit floating point."]
    pub const AVX512FP16: TargetFeatures = feature_set!(AVX, AVX2, AVX512BW, AVX512F, AVX512FP16, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Integer Fused Multiple-Add."]
    pub const AVX512IFMA: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512IFMA, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Vector Byte Manipulation Instructions."]
    pub const AVX512VBMI: TargetFeatures = feature_set!(AVX, AVX2, AVX512BW, AVX512F, AVX512VBMI, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 further Vector Byte Manipulation Instructions."]
    pub const AVX512VBMI2: TargetFeatures = feature_set!(AVX, AVX2, AVX512BW, AVX512F, AVX512VBMI2, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Vector Length eXtensions."]
    pub const AVX512VL: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512VL, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Vector Neural Network Instructions."]
    pub const AVX512VNNI: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512VNNI, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 vp2intersect."]
    pub const AVX512VP2INTERSECT: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512VP2INTERSECT, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-512 Population Count Instructions."]
    pub const AVX512VPOPCNTDQ: TargetFeatures = feature_set!(AVX, AVX2, AVX512F, AVX512VPOPCNTDQ, F16C, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-IFMA."]
    pub const AVXIFMA: TargetFeatures = feature_set!(AVX, AVX2, AVXIFMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support AVX-NE-CONVERT instructions."]
    pub const AVXNECONVERT: TargetFeatures = feature_set!(AVX, AVX2, AVXNECONVERT, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support AVX_VNNI encoding."]
    pub const AVXVNNI: TargetFeatures = feature_set!(AVX, AVX2, AVXVNNI, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-VNNI-INT16."]
    pub const AVXVNNIINT16: TargetFeatures = feature_set!(AVX, AVX2, AVXVNNIINT16, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable AVX-VNNI-INT8."]
    pub const AVXVNNIINT8: TargetFeatures = feature_set!(AVX, AVX2, AVXVNNIINT8, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support BMI instructions."]
    pub const BMI1: TargetFeatures = feature_set!(BMI1);

    #[doc = "Support BMI2 instructions."]
    pub const BMI2: TargetFeatures = feature_set!(BMI2);

    #[doc = "Flush A Cache Line Optimized."]
    pub const CLFLUSHOPT: TargetFeatures = feature_set!(CLFLUSHOPT);

    #[doc = "64-bit with cmpxchg16b (this is true for most x86-64 chips, but not the first AMD chips)."]
    pub const CMPXCHG16B: TargetFeatures = feature_set!(CMPXCHG16B);

    #[doc = "Enables C Run-time Libraries to be statically linked."]
    pub const CRT_STATIC: TargetFeatures = feature_set!(CRT_STATIC);

    #[doc = "REP MOVS/STOS are fast."]
    pub const ERMSB: TargetFeatures = feature_set!(ERMSB);

    #[doc = "Support 16-bit floating point conversion instructions."]
    pub const F16C: TargetFeatures = feature_set!(AVX, F16C, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable three-operand fused multiple-add."]
    pub const FMA: TargetFeatures = feature_set!(AVX, FMA, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable four-operand fused multiple-add."]
    pub const FMA4: TargetFeatures = feature_set!(AVX, FMA4, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSE4A, SSSE3);

    #[doc = "Support fxsave/fxrestore instructions."]
    pub const FXSR: TargetFeatures = feature_set!(FXSR);

    #[doc = "Enable Galois Field Arithmetic Instructions."]
    pub const GFNI: TargetFeatures = feature_set!(GFNI, SSE, SSE2);

    #[doc = "Support Key Locker kl Instructions."]
    pub const KL: TargetFeatures = feature_set!(KL, SSE, SSE2);

    #[doc = "Support LAHF and SAHF instructions in 64-bit mode."]
    pub const LAHFSAHF: TargetFeatures = feature_set!(LAHFSAHF);

    #[doc = "Support LZCNT instruction."]
    pub const LZCNT: TargetFeatures = feature_set!(LZCNT);

    #[doc = "Support MOVBE instruction."]
    pub const MOVBE: TargetFeatures = feature_set!(MOVBE);

    #[doc = "Enable MOVRS."]
    pub const MOVRS: TargetFeatures = feature_set!(MOVRS);

    #[doc = "Enable packed carry-less multiplication instructions."]
    pub const PCLMULQDQ: TargetFeatures = feature_set!(PCLMULQDQ, SSE, SSE2);

    #[doc = "Support POPCNT instruction."]
    pub const POPCNT: TargetFeatures = feature_set!(POPCNT);

    #[doc = "Support PRFCHW instructions."]
    pub const PRFCHW: TargetFeatures = feature_set!(PRFCHW);

    #[doc = "Support RDRAND instruction."]
    pub const RDRAND: TargetFeatures = feature_set!(RDRAND);

    #[doc = "Support RDSEED instruction."]
    pub const RDSEED: TargetFeatures = feature_set!(RDSEED);

    #[doc = "Support RTM instructions."]
    pub const RTM: TargetFeatures = feature_set!(RTM);

    #[doc = "Enable SHA instructions."]
    pub const SHA: TargetFeatures = feature_set!(SHA, SSE, SSE2);

    #[doc = "Support SHA512 instructions."]
    pub const SHA512: TargetFeatures = feature_set!(AVX, AVX2, SHA512, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support SM3 instructions."]
    pub const SM3: TargetFeatures = feature_set!(AVX, SM3, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support SM4 instructions."]
    pub const SM4: TargetFeatures = feature_set!(AVX, AVX2, SM4, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Enable SSE instructions."]
    pub const SSE: TargetFeatures = feature_set!(SSE);

    #[doc = "Enable SSE2 instructions."]
    pub const SSE2: TargetFeatures = feature_set!(SSE, SSE2);

    #[doc = "Enable SSE3 instructions."]
    pub const SSE3: TargetFeatures = feature_set!(SSE, SSE2, SSE3);

    #[doc = "Enable SSE 4.1 instructions."]
    pub const SSE4_1: TargetFeatures = feature_set!(SSE, SSE2, SSE3, SSE4_1, SSSE3);

    #[doc = "Enable SSE 4.2 instructions."]
    pub const SSE4_2: TargetFeatures = feature_set!(SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3);

    #[doc = "Support SSE 4a instructions."]
    pub const SSE4A: TargetFeatures = feature_set!(SSE, SSE2, SSE3, SSE4A);

    #[doc = "Enable SSSE3 instructions."]
    pub const SSSE3: TargetFeatures = feature_set!(SSE, SSE2, SSE3, SSSE3);

    #[doc = "Enable TBM instructions."]
    pub const TBM: TargetFeatures = feature_set!(TBM);

    #[doc = "Promote selected AES instructions to AVX512/AVX registers."]
    pub const VAES: TargetFeatures = feature_set!(AES, AVX, AVX2, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3, VAES);

    #[doc = "Enable vpclmulqdq instructions."]
    pub const VPCLMULQDQ: TargetFeatures = feature_set!(AVX, PCLMULQDQ, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSSE3, VPCLMULQDQ);

    #[doc = "Support Key Locker wide Instructions."]
    pub const WIDEKL: TargetFeatures = feature_set!(KL, SSE, SSE2, WIDEKL);

    #[doc = "Enable X87 float instructions."]
    pub const X87: TargetFeatures = feature_set!(X87);

    #[doc = "Enable XOP instructions."]
    pub const XOP: TargetFeatures = feature_set!(AVX, FMA4, SSE, SSE2, SSE3, SSE4_1, SSE4_2, SSE4A, SSSE3, XOP);

    #[doc = "Support xsave instructions."]
    pub const XSAVE: TargetFeatures = feature_set!(XSAVE);

    #[doc = "Support xsavec instructions."]
    pub const XSAVEC: TargetFeatures = feature_set!(XSAVE, XSAVEC);

    #[doc = "Support xsaveopt instructions."]
    pub const XSAVEOPT: TargetFeatures = feature_set!(XSAVE, XSAVEOPT);

    #[doc = "Support xsaves instructions."]
    pub const XSAVES: TargetFeatures = feature_set!(XSAVE, XSAVES);


    #[cfg(target_arch = "x86_64")]
    pub(crate) const FEATURES: &[FeatureData] = &[
        FeatureData::new("adx", ADX),
        FeatureData::new("aes", AES),
        FeatureData::new("amx-avx512", AMX_AVX512),
        FeatureData::new("amx-bf16", AMX_BF16),
        FeatureData::new("amx-complex", AMX_COMPLEX),
        FeatureData::new("amx-fp16", AMX_FP16),
        FeatureData::new("amx-fp8", AMX_FP8),
        FeatureData::new("amx-int8", AMX_INT8),
        FeatureData::new("amx-movrs", AMX_MOVRS),
        FeatureData::new("amx-tile", AMX_TILE),
        FeatureData::new("apxf", APXF),
        FeatureData::new("avx", AVX),
        FeatureData::new("avx10.1", AVX10_1),
        FeatureData::new("avx10.2", AVX10_2),
        FeatureData::new("avx2", AVX2),
        FeatureData::new("avx512bf16", AVX512BF16),
        FeatureData::new("avx512bitalg", AVX512BITALG),
        FeatureData::new("avx512bw", AVX512BW),
        FeatureData::new("avx512cd", AVX512CD),
        FeatureData::new("avx512dq", AVX512DQ),
        FeatureData::new("avx512f", AVX512F),
        FeatureData::new("avx512fp16", AVX512FP16),
        FeatureData::new("avx512ifma", AVX512IFMA),
        FeatureData::new("avx512vbmi", AVX512VBMI),
        FeatureData::new("avx512vbmi2", AVX512VBMI2),
        FeatureData::new("avx512vl", AVX512VL),
        FeatureData::new("avx512vnni", AVX512VNNI),
        FeatureData::new("avx512vp2intersect", AVX512VP2INTERSECT),
        FeatureData::new("avx512vpopcntdq", AVX512VPOPCNTDQ),
        FeatureData::new("avxifma", AVXIFMA),
        FeatureData::new("avxneconvert", AVXNECONVERT),
        FeatureData::new("avxvnni", AVXVNNI),
        FeatureData::new("avxvnniint16", AVXVNNIINT16),
        FeatureData::new("avxvnniint8", AVXVNNIINT8),
        FeatureData::new("bmi1", BMI1),
        FeatureData::new("bmi2", BMI2),
        FeatureData::new("clflushopt", CLFLUSHOPT),
        FeatureData::new("cmpxchg16b", CMPXCHG16B),
        FeatureData::new("crt-static", CRT_STATIC),
        FeatureData::new("ermsb", ERMSB),
        FeatureData::new("f16c", F16C),
        FeatureData::new("fma", FMA),
        FeatureData::new("fma4", FMA4),
        FeatureData::new("fxsr", FXSR),
        FeatureData::new("gfni", GFNI),
        FeatureData::new("kl", KL),
        FeatureData::new("lahfsahf", LAHFSAHF),
        FeatureData::new("lzcnt", LZCNT),
        FeatureData::new("movbe", MOVBE),
        FeatureData::new("movrs", MOVRS),
        FeatureData::new("pclmulqdq", PCLMULQDQ),
        FeatureData::new("popcnt", POPCNT),
        FeatureData::new("prfchw", PRFCHW),
        FeatureData::new("rdrand", RDRAND),
        FeatureData::new("rdseed", RDSEED),
        FeatureData::new("rtm", RTM),
        FeatureData::new("sha", SHA),
        FeatureData::new("sha512", SHA512),
        FeatureData::new("sm3", SM3),
        FeatureData::new("sm4", SM4),
        FeatureData::new("sse", SSE),
        FeatureData::new("sse2", SSE2),
        FeatureData::new("sse3", SSE3),
        FeatureData::new("sse4.1", SSE4_1),
        FeatureData::new("sse4.2", SSE4_2),
        FeatureData::new("sse4a", SSE4A),
        FeatureData::new("ssse3", SSSE3),
        FeatureData::new("tbm", TBM),
        FeatureData::new("vaes", VAES),
        FeatureData::new("vpclmulqdq", VPCLMULQDQ),
        FeatureData::new("widekl", WIDEKL),
        FeatureData::new("x87", X87),
        FeatureData::new("xop", XOP),
        FeatureData::new("xsave", XSAVE),
        FeatureData::new("xsavec", XSAVEC),
        FeatureData::new("xsaveopt", XSAVEOPT),
        FeatureData::new("xsaves", XSAVES),
    ];

    #[cfg(target_arch = "x86_64")]
    #[allow(unknown_lints, unexpected_cfgs, clippy::let_and_return)]
    pub(crate) const fn enabled_for_target() -> TargetFeatures {
        let features = TargetFeatures::empty();
        #[cfg(target_feature = "adx")]
        let features = features.with(ADX);
        #[cfg(target_feature = "aes")]
        let features = features.with(AES);
        #[cfg(target_feature = "amx-avx512")]
        let features = features.with(AMX_AVX512);
        #[cfg(target_feature = "amx-bf16")]
        let features = features.with(AMX_BF16);
        #[cfg(target_feature = "amx-complex")]
        let features = features.with(AMX_COMPLEX);
        #[cfg(target_feature = "amx-fp16")]
        let features = features.with(AMX_FP16);
        #[cfg(target_feature = "amx-fp8")]
        let features = features.with(AMX_FP8);
        #[cfg(target_feature = "amx-int8")]
        let features = features.with(AMX_INT8);
        #[cfg(target_feature = "amx-movrs")]
        let features = features.with(AMX_MOVRS);
        #[cfg(target_feature = "amx-tile")]
        let features = features.with(AMX_TILE);
        #[cfg(target_feature = "apxf")]
        let features = features.with(APXF);
        #[cfg(target_feature = "avx")]
        let features = features.with(AVX);
        #[cfg(target_feature = "avx10.1")]
        let features = features.with(AVX10_1);
        #[cfg(target_feature = "avx10.2")]
        let features = features.with(AVX10_2);
        #[cfg(target_feature = "avx2")]
        let features = features.with(AVX2);
        #[cfg(target_feature = "avx512bf16")]
        let features = features.with(AVX512BF16);
        #[cfg(target_feature = "avx512bitalg")]
        let features = features.with(AVX512BITALG);
        #[cfg(target_feature = "avx512bw")]
        let features = features.with(AVX512BW);
        #[cfg(target_feature = "avx512cd")]
        let features = features.with(AVX512CD);
        #[cfg(target_feature = "avx512dq")]
        let features = features.with(AVX512DQ);
        #[cfg(target_feature = "avx512f")]
        let features = features.with(AVX512F);
        #[cfg(target_feature = "avx512fp16")]
        let features = features.with(AVX512FP16);
        #[cfg(target_feature = "avx512ifma")]
        let features = features.with(AVX512IFMA);
        #[cfg(target_feature = "avx512vbmi")]
        let features = features.with(AVX512VBMI);
        #[cfg(target_feature = "avx512vbmi2")]
        let features = features.with(AVX512VBMI2);
        #[cfg(target_feature = "avx512vl")]
        let features = features.with(AVX512VL);
        #[cfg(target_feature = "avx512vnni")]
        let features = features.with(AVX512VNNI);
        #[cfg(target_feature = "avx512vp2intersect")]
        let features = features.with(AVX512VP2INTERSECT);
        #[cfg(target_feature = "avx512vpopcntdq")]
        let features = features.with(AVX512VPOPCNTDQ);
        #[cfg(target_feature = "avxifma")]
        let features = features.with(AVXIFMA);
        #[cfg(target_feature = "avxneconvert")]
        let features = features.with(AVXNECONVERT);
        #[cfg(target_feature = "avxvnni")]
        let features = features.with(AVXVNNI);
        #[cfg(target_feature = "avxvnniint16")]
        let features = features.with(AVXVNNIINT16);
        #[cfg(target_feature = "avxvnniint8")]
        let features = features.with(AVXVNNIINT8);
        #[cfg(target_feature = "bmi1")]
        let features = features.with(BMI1);
        #[cfg(target_feature = "bmi2")]
        let features = features.with(BMI2);
        #[cfg(target_feature = "clflushopt")]
        let features = features.with(CLFLUSHOPT);
        #[cfg(target_feature = "cmpxchg16b")]
        let features = features.with(CMPXCHG16B);
        #[cfg(target_feature = "crt-static")]
        let features = features.with(CRT_STATIC);
        #[cfg(target_feature = "ermsb")]
        let features = features.with(ERMSB);
        #[cfg(target_feature = "f16c")]
        let features = features.with(F16C);
        #[cfg(target_feature = "fma")]
        let features = features.with(FMA);
        #[cfg(target_feature = "fma4")]
        let features = features.with(FMA4);
        #[cfg(target_feature = "fxsr")]
        let features = features.with(FXSR);
        #[cfg(target_feature = "gfni")]
        let features = features.with(GFNI);
        #[cfg(target_feature = "kl")]
        let features = features.with(KL);
        #[cfg(target_feature = "lahfsahf")]
        let features = features.with(LAHFSAHF);
        #[cfg(target_feature = "lzcnt")]
        let features = features.with(LZCNT);
        #[cfg(target_feature = "movbe")]
        let features = features.with(MOVBE);
        #[cfg(target_feature = "movrs")]
        let features = features.with(MOVRS);
        #[cfg(target_feature = "pclmulqdq")]
        let features = features.with(PCLMULQDQ);
        #[cfg(target_feature = "popcnt")]
        let features = features.with(POPCNT);
        #[cfg(target_feature = "prfchw")]
        let features = features.with(PRFCHW);
        #[cfg(target_feature = "rdrand")]
        let features = features.with(RDRAND);
        #[cfg(target_feature = "rdseed")]
        let features = features.with(RDSEED);
        #[cfg(target_feature = "rtm")]
        let features = features.with(RTM);
        #[cfg(target_feature = "sha")]
        let features = features.with(SHA);
        #[cfg(target_feature = "sha512")]
        let features = features.with(SHA512);
        #[cfg(target_feature = "sm3")]
        let features = features.with(SM3);
        #[cfg(target_feature = "sm4")]
        let features = features.with(SM4);
        #[cfg(target_feature = "sse")]
        let features = features.with(SSE);
        #[cfg(target_feature = "sse2")]
        let features = features.with(SSE2);
        #[cfg(target_feature = "sse3")]
        let features = features.with(SSE3);
        #[cfg(target_feature = "sse4.1")]
        let features = features.with(SSE4_1);
        #[cfg(target_feature = "sse4.2")]
        let features = features.with(SSE4_2);
        #[cfg(target_feature = "sse4a")]
        let features = features.with(SSE4A);
        #[cfg(target_feature = "ssse3")]
        let features = features.with(SSSE3);
        #[cfg(target_feature = "tbm")]
        let features = features.with(TBM);
        #[cfg(target_feature = "vaes")]
        let features = features.with(VAES);
        #[cfg(target_feature = "vpclmulqdq")]
        let features = features.with(VPCLMULQDQ);
        #[cfg(target_feature = "widekl")]
        let features = features.with(WIDEKL);
        #[cfg(target_feature = "x87")]
        let features = features.with(X87);
        #[cfg(target_feature = "xop")]
        let features = features.with(XOP);
        #[cfg(target_feature = "xsave")]
        let features = features.with(XSAVE);
        #[cfg(target_feature = "xsavec")]
        let features = features.with(XSAVEC);
        #[cfg(target_feature = "xsaveopt")]
        let features = features.with(XSAVEOPT);
        #[cfg(target_feature = "xsaves")]
        let features = features.with(XSAVES);
        features
    }

}

#[cfg(target_arch = "arm")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("aclass") => { $crate::arm::ACLASS };
    ("acquire-release") => { $crate::arm::ACQUIRE_RELEASE };
    ("aes") => { $crate::arm::AES };
    ("crc") => { $crate::arm::CRC };
    ("crt-static") => { $crate::arm::CRT_STATIC };
    ("d32") => { $crate::arm::D32 };
    ("dotprod") => { $crate::arm::DOTPROD };
    ("dsp") => { $crate::arm::DSP };
    ("fp-armv8") => { $crate::arm::FP_ARMV8 };
    ("fp16") => { $crate::arm::FP16 };
    ("fp64") => { $crate::arm::FP64 };
    ("fpregs") => { $crate::arm::FPREGS };
    ("i8mm") => { $crate::arm::I8MM };
    ("mclass") => { $crate::arm::MCLASS };
    ("mve") => { $crate::arm::MVE };
    ("mve.fp") => { $crate::arm::MVE_FP };
    ("neon") => { $crate::arm::NEON };
    ("rclass") => { $crate::arm::RCLASS };
    ("sha2") => { $crate::arm::SHA2 };
    ("soft-float") => { $crate::arm::SOFT_FLOAT };
    ("thumb-mode") => { $crate::arm::THUMB_MODE };
    ("thumb2") => { $crate::arm::THUMB2 };
    ("trustzone") => { $crate::arm::TRUSTZONE };
    ("v5te") => { $crate::arm::V5TE };
    ("v6") => { $crate::arm::V6 };
    ("v6k") => { $crate::arm::V6K };
    ("v6m") => { $crate::arm::V6M };
    ("v6t2") => { $crate::arm::V6T2 };
    ("v7") => { $crate::arm::V7 };
    ("v8") => { $crate::arm::V8 };
    ("v8.1m.main") => { $crate::arm::V8_1M_MAIN };
    ("v8m") => { $crate::arm::V8M };
    ("v8m.main") => { $crate::arm::V8M_MAIN };
    ("vfp2") => { $crate::arm::VFP2 };
    ("vfp2sp") => { $crate::arm::VFP2SP };
    ("vfp3") => { $crate::arm::VFP3 };
    ("vfp4") => { $crate::arm::VFP4 };
    ("virtualization") => { $crate::arm::VIRTUALIZATION };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "aarch64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("aes") => { $crate::aarch64::AES };
    ("bf16") => { $crate::aarch64::BF16 };
    ("bti") => { $crate::aarch64::BTI };
    ("crc") => { $crate::aarch64::CRC };
    ("crt-static") => { $crate::aarch64::CRT_STATIC };
    ("cssc") => { $crate::aarch64::CSSC };
    ("dit") => { $crate::aarch64::DIT };
    ("dotprod") => { $crate::aarch64::DOTPROD };
    ("dpb") => { $crate::aarch64::DPB };
    ("dpb2") => { $crate::aarch64::DPB2 };
    ("ecv") => { $crate::aarch64::ECV };
    ("f32mm") => { $crate::aarch64::F32MM };
    ("f64mm") => { $crate::aarch64::F64MM };
    ("faminmax") => { $crate::aarch64::FAMINMAX };
    ("fcma") => { $crate::aarch64::FCMA };
    ("fhm") => { $crate::aarch64::FHM };
    ("flagm") => { $crate::aarch64::FLAGM };
    ("flagm2") => { $crate::aarch64::FLAGM2 };
    ("fp16") => { $crate::aarch64::FP16 };
    ("fp8") => { $crate::aarch64::FP8 };
    ("fp8dot2") => { $crate::aarch64::FP8DOT2 };
    ("fp8dot4") => { $crate::aarch64::FP8DOT4 };
    ("fp8fma") => { $crate::aarch64::FP8FMA };
    ("frintts") => { $crate::aarch64::FRINTTS };
    ("hbc") => { $crate::aarch64::HBC };
    ("i8mm") => { $crate::aarch64::I8MM };
    ("jsconv") => { $crate::aarch64::JSCONV };
    ("lor") => { $crate::aarch64::LOR };
    ("lse") => { $crate::aarch64::LSE };
    ("lse128") => { $crate::aarch64::LSE128 };
    ("lse2") => { $crate::aarch64::LSE2 };
    ("lut") => { $crate::aarch64::LUT };
    ("mops") => { $crate::aarch64::MOPS };
    ("mte") => { $crate::aarch64::MTE };
    ("neon") => { $crate::aarch64::NEON };
    ("outline-atomics") => { $crate::aarch64::OUTLINE_ATOMICS };
    ("paca") => { $crate::aarch64::PACA };
    ("pacg") => { $crate::aarch64::PACG };
    ("pan") => { $crate::aarch64::PAN };
    ("pauth-lr") => { $crate::aarch64::PAUTH_LR };
    ("pmuv3") => { $crate::aarch64::PMUV3 };
    ("rand") => { $crate::aarch64::RAND };
    ("ras") => { $crate::aarch64::RAS };
    ("rcpc") => { $crate::aarch64::RCPC };
    ("rcpc2") => { $crate::aarch64::RCPC2 };
    ("rcpc3") => { $crate::aarch64::RCPC3 };
    ("rdm") => { $crate::aarch64::RDM };
    ("sb") => { $crate::aarch64::SB };
    ("sha2") => { $crate::aarch64::SHA2 };
    ("sha3") => { $crate::aarch64::SHA3 };
    ("sm4") => { $crate::aarch64::SM4 };
    ("sme") => { $crate::aarch64::SME };
    ("sme-b16b16") => { $crate::aarch64::SME_B16B16 };
    ("sme-f16f16") => { $crate::aarch64::SME_F16F16 };
    ("sme-f64f64") => { $crate::aarch64::SME_F64F64 };
    ("sme-f8f16") => { $crate::aarch64::SME_F8F16 };
    ("sme-f8f32") => { $crate::aarch64::SME_F8F32 };
    ("sme-fa64") => { $crate::aarch64::SME_FA64 };
    ("sme-i16i64") => { $crate::aarch64::SME_I16I64 };
    ("sme-lutv2") => { $crate::aarch64::SME_LUTV2 };
    ("sme2") => { $crate::aarch64::SME2 };
    ("sme2p1") => { $crate::aarch64::SME2P1 };
    ("spe") => { $crate::aarch64::SPE };
    ("ssbs") => { $crate::aarch64::SSBS };
    ("ssve-fp8dot2") => { $crate::aarch64::SSVE_FP8DOT2 };
    ("ssve-fp8dot4") => { $crate::aarch64::SSVE_FP8DOT4 };
    ("ssve-fp8fma") => { $crate::aarch64::SSVE_FP8FMA };
    ("sve") => { $crate::aarch64::SVE };
    ("sve-b16b16") => { $crate::aarch64::SVE_B16B16 };
    ("sve2") => { $crate::aarch64::SVE2 };
    ("sve2-aes") => { $crate::aarch64::SVE2_AES };
    ("sve2-bitperm") => { $crate::aarch64::SVE2_BITPERM };
    ("sve2-sha3") => { $crate::aarch64::SVE2_SHA3 };
    ("sve2-sm4") => { $crate::aarch64::SVE2_SM4 };
    ("sve2p1") => { $crate::aarch64::SVE2P1 };
    ("v8.1a") => { $crate::aarch64::V8_1A };
    ("v8.2a") => { $crate::aarch64::V8_2A };
    ("v8.3a") => { $crate::aarch64::V8_3A };
    ("v8.4a") => { $crate::aarch64::V8_4A };
    ("v8.5a") => { $crate::aarch64::V8_5A };
    ("v8.6a") => { $crate::aarch64::V8_6A };
    ("v8.7a") => { $crate::aarch64::V8_7A };
    ("v8.8a") => { $crate::aarch64::V8_8A };
    ("v8.9a") => { $crate::aarch64::V8_9A };
    ("v9.1a") => { $crate::aarch64::V9_1A };
    ("v9.2a") => { $crate::aarch64::V9_2A };
    ("v9.3a") => { $crate::aarch64::V9_3A };
    ("v9.4a") => { $crate::aarch64::V9_4A };
    ("v9.5a") => { $crate::aarch64::V9_5A };
    ("v9a") => { $crate::aarch64::V9A };
    ("vh") => { $crate::aarch64::VH };
    ("wfxt") => { $crate::aarch64::WFXT };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "arm64ec")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("aes") => { $crate::arm64ec::AES };
    ("bf16") => { $crate::arm64ec::BF16 };
    ("bti") => { $crate::arm64ec::BTI };
    ("crc") => { $crate::arm64ec::CRC };
    ("crt-static") => { $crate::arm64ec::CRT_STATIC };
    ("cssc") => { $crate::arm64ec::CSSC };
    ("dit") => { $crate::arm64ec::DIT };
    ("dotprod") => { $crate::arm64ec::DOTPROD };
    ("dpb") => { $crate::arm64ec::DPB };
    ("dpb2") => { $crate::arm64ec::DPB2 };
    ("ecv") => { $crate::arm64ec::ECV };
    ("f32mm") => { $crate::arm64ec::F32MM };
    ("f64mm") => { $crate::arm64ec::F64MM };
    ("faminmax") => { $crate::arm64ec::FAMINMAX };
    ("fcma") => { $crate::arm64ec::FCMA };
    ("fhm") => { $crate::arm64ec::FHM };
    ("flagm") => { $crate::arm64ec::FLAGM };
    ("flagm2") => { $crate::arm64ec::FLAGM2 };
    ("fp16") => { $crate::arm64ec::FP16 };
    ("fp8") => { $crate::arm64ec::FP8 };
    ("fp8dot2") => { $crate::arm64ec::FP8DOT2 };
    ("fp8dot4") => { $crate::arm64ec::FP8DOT4 };
    ("fp8fma") => { $crate::arm64ec::FP8FMA };
    ("frintts") => { $crate::arm64ec::FRINTTS };
    ("hbc") => { $crate::arm64ec::HBC };
    ("i8mm") => { $crate::arm64ec::I8MM };
    ("jsconv") => { $crate::arm64ec::JSCONV };
    ("lor") => { $crate::arm64ec::LOR };
    ("lse") => { $crate::arm64ec::LSE };
    ("lse128") => { $crate::arm64ec::LSE128 };
    ("lse2") => { $crate::arm64ec::LSE2 };
    ("lut") => { $crate::arm64ec::LUT };
    ("mops") => { $crate::arm64ec::MOPS };
    ("mte") => { $crate::arm64ec::MTE };
    ("neon") => { $crate::arm64ec::NEON };
    ("outline-atomics") => { $crate::arm64ec::OUTLINE_ATOMICS };
    ("paca") => { $crate::arm64ec::PACA };
    ("pacg") => { $crate::arm64ec::PACG };
    ("pan") => { $crate::arm64ec::PAN };
    ("pauth-lr") => { $crate::arm64ec::PAUTH_LR };
    ("pmuv3") => { $crate::arm64ec::PMUV3 };
    ("rand") => { $crate::arm64ec::RAND };
    ("ras") => { $crate::arm64ec::RAS };
    ("rcpc") => { $crate::arm64ec::RCPC };
    ("rcpc2") => { $crate::arm64ec::RCPC2 };
    ("rcpc3") => { $crate::arm64ec::RCPC3 };
    ("rdm") => { $crate::arm64ec::RDM };
    ("sb") => { $crate::arm64ec::SB };
    ("sha2") => { $crate::arm64ec::SHA2 };
    ("sha3") => { $crate::arm64ec::SHA3 };
    ("sm4") => { $crate::arm64ec::SM4 };
    ("sme") => { $crate::arm64ec::SME };
    ("sme-b16b16") => { $crate::arm64ec::SME_B16B16 };
    ("sme-f16f16") => { $crate::arm64ec::SME_F16F16 };
    ("sme-f64f64") => { $crate::arm64ec::SME_F64F64 };
    ("sme-f8f16") => { $crate::arm64ec::SME_F8F16 };
    ("sme-f8f32") => { $crate::arm64ec::SME_F8F32 };
    ("sme-fa64") => { $crate::arm64ec::SME_FA64 };
    ("sme-i16i64") => { $crate::arm64ec::SME_I16I64 };
    ("sme-lutv2") => { $crate::arm64ec::SME_LUTV2 };
    ("sme2") => { $crate::arm64ec::SME2 };
    ("sme2p1") => { $crate::arm64ec::SME2P1 };
    ("spe") => { $crate::arm64ec::SPE };
    ("ssbs") => { $crate::arm64ec::SSBS };
    ("ssve-fp8dot2") => { $crate::arm64ec::SSVE_FP8DOT2 };
    ("ssve-fp8dot4") => { $crate::arm64ec::SSVE_FP8DOT4 };
    ("ssve-fp8fma") => { $crate::arm64ec::SSVE_FP8FMA };
    ("sve") => { $crate::arm64ec::SVE };
    ("sve-b16b16") => { $crate::arm64ec::SVE_B16B16 };
    ("sve2") => { $crate::arm64ec::SVE2 };
    ("sve2-aes") => { $crate::arm64ec::SVE2_AES };
    ("sve2-bitperm") => { $crate::arm64ec::SVE2_BITPERM };
    ("sve2-sha3") => { $crate::arm64ec::SVE2_SHA3 };
    ("sve2-sm4") => { $crate::arm64ec::SVE2_SM4 };
    ("sve2p1") => { $crate::arm64ec::SVE2P1 };
    ("v8.1a") => { $crate::arm64ec::V8_1A };
    ("v8.2a") => { $crate::arm64ec::V8_2A };
    ("v8.3a") => { $crate::arm64ec::V8_3A };
    ("v8.4a") => { $crate::arm64ec::V8_4A };
    ("v8.5a") => { $crate::arm64ec::V8_5A };
    ("v8.6a") => { $crate::arm64ec::V8_6A };
    ("v8.7a") => { $crate::arm64ec::V8_7A };
    ("v8.8a") => { $crate::arm64ec::V8_8A };
    ("v8.9a") => { $crate::arm64ec::V8_9A };
    ("v9.1a") => { $crate::arm64ec::V9_1A };
    ("v9.2a") => { $crate::arm64ec::V9_2A };
    ("v9.3a") => { $crate::arm64ec::V9_3A };
    ("v9.4a") => { $crate::arm64ec::V9_4A };
    ("v9.5a") => { $crate::arm64ec::V9_5A };
    ("v9a") => { $crate::arm64ec::V9A };
    ("vh") => { $crate::arm64ec::VH };
    ("wfxt") => { $crate::arm64ec::WFXT };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "bpf")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("allows-misaligned-mem-access") => { $crate::bpf::ALLOWS_MISALIGNED_MEM_ACCESS };
    ("alu32") => { $crate::bpf::ALU32 };
    ("crt-static") => { $crate::bpf::CRT_STATIC };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "hexagon")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("audio") => { $crate::hexagon::AUDIO };
    ("crt-static") => { $crate::hexagon::CRT_STATIC };
    ("hvx") => { $crate::hexagon::HVX };
    ("hvx-ieee-fp") => { $crate::hexagon::HVX_IEEE_FP };
    ("hvx-length128b") => { $crate::hexagon::HVX_LENGTH128B };
    ("hvx-length64b") => { $crate::hexagon::HVX_LENGTH64B };
    ("hvx-qfloat") => { $crate::hexagon::HVX_QFLOAT };
    ("hvxv60") => { $crate::hexagon::HVXV60 };
    ("hvxv62") => { $crate::hexagon::HVXV62 };
    ("hvxv65") => { $crate::hexagon::HVXV65 };
    ("hvxv66") => { $crate::hexagon::HVXV66 };
    ("hvxv67") => { $crate::hexagon::HVXV67 };
    ("hvxv68") => { $crate::hexagon::HVXV68 };
    ("hvxv69") => { $crate::hexagon::HVXV69 };
    ("hvxv71") => { $crate::hexagon::HVXV71 };
    ("hvxv73") => { $crate::hexagon::HVXV73 };
    ("hvxv75") => { $crate::hexagon::HVXV75 };
    ("hvxv79") => { $crate::hexagon::HVXV79 };
    ("v60") => { $crate::hexagon::V60 };
    ("v62") => { $crate::hexagon::V62 };
    ("v65") => { $crate::hexagon::V65 };
    ("v66") => { $crate::hexagon::V66 };
    ("v67") => { $crate::hexagon::V67 };
    ("v68") => { $crate::hexagon::V68 };
    ("v69") => { $crate::hexagon::V69 };
    ("v71") => { $crate::hexagon::V71 };
    ("v73") => { $crate::hexagon::V73 };
    ("v75") => { $crate::hexagon::V75 };
    ("v79") => { $crate::hexagon::V79 };
    ("zreg") => { $crate::hexagon::ZREG };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "mips")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("crt-static") => { $crate::mips::CRT_STATIC };
    ("fp64") => { $crate::mips::FP64 };
    ("msa") => { $crate::mips::MSA };
    ("virt") => { $crate::mips::VIRT };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "mips64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("crt-static") => { $crate::mips64::CRT_STATIC };
    ("fp64") => { $crate::mips64::FP64 };
    ("msa") => { $crate::mips64::MSA };
    ("virt") => { $crate::mips64::VIRT };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "loongarch32")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("32s") => { $crate::loongarch32::F_32S };
    ("crt-static") => { $crate::loongarch32::CRT_STATIC };
    ("d") => { $crate::loongarch32::D };
    ("div32") => { $crate::loongarch32::DIV32 };
    ("f") => { $crate::loongarch32::F };
    ("frecipe") => { $crate::loongarch32::FRECIPE };
    ("lam-bh") => { $crate::loongarch32::LAM_BH };
    ("lamcas") => { $crate::loongarch32::LAMCAS };
    ("lasx") => { $crate::loongarch32::LASX };
    ("lbt") => { $crate::loongarch32::LBT };
    ("ld-seq-sa") => { $crate::loongarch32::LD_SEQ_SA };
    ("lsx") => { $crate::loongarch32::LSX };
    ("lvz") => { $crate::loongarch32::LVZ };
    ("relax") => { $crate::loongarch32::RELAX };
    ("scq") => { $crate::loongarch32::SCQ };
    ("ual") => { $crate::loongarch32::UAL };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "loongarch64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("32s") => { $crate::loongarch64::F_32S };
    ("crt-static") => { $crate::loongarch64::CRT_STATIC };
    ("d") => { $crate::loongarch64::D };
    ("div32") => { $crate::loongarch64::DIV32 };
    ("f") => { $crate::loongarch64::F };
    ("frecipe") => { $crate::loongarch64::FRECIPE };
    ("lam-bh") => { $crate::loongarch64::LAM_BH };
    ("lamcas") => { $crate::loongarch64::LAMCAS };
    ("lasx") => { $crate::loongarch64::LASX };
    ("lbt") => { $crate::loongarch64::LBT };
    ("ld-seq-sa") => { $crate::loongarch64::LD_SEQ_SA };
    ("lsx") => { $crate::loongarch64::LSX };
    ("lvz") => { $crate::loongarch64::LVZ };
    ("relax") => { $crate::loongarch64::RELAX };
    ("scq") => { $crate::loongarch64::SCQ };
    ("ual") => { $crate::loongarch64::UAL };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "nvptx64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("crt-static") => { $crate::nvptx64::CRT_STATIC };
    ("ptx70") => { $crate::nvptx64::PTX70 };
    ("ptx71") => { $crate::nvptx64::PTX71 };
    ("ptx72") => { $crate::nvptx64::PTX72 };
    ("ptx73") => { $crate::nvptx64::PTX73 };
    ("ptx74") => { $crate::nvptx64::PTX74 };
    ("ptx75") => { $crate::nvptx64::PTX75 };
    ("ptx76") => { $crate::nvptx64::PTX76 };
    ("ptx77") => { $crate::nvptx64::PTX77 };
    ("ptx78") => { $crate::nvptx64::PTX78 };
    ("ptx80") => { $crate::nvptx64::PTX80 };
    ("ptx81") => { $crate::nvptx64::PTX81 };
    ("ptx82") => { $crate::nvptx64::PTX82 };
    ("ptx83") => { $crate::nvptx64::PTX83 };
    ("ptx84") => { $crate::nvptx64::PTX84 };
    ("ptx85") => { $crate::nvptx64::PTX85 };
    ("ptx86") => { $crate::nvptx64::PTX86 };
    ("ptx87") => { $crate::nvptx64::PTX87 };
    ("sm_100") => { $crate::nvptx64::SM_100 };
    ("sm_100a") => { $crate::nvptx64::SM_100A };
    ("sm_101") => { $crate::nvptx64::SM_101 };
    ("sm_101a") => { $crate::nvptx64::SM_101A };
    ("sm_120") => { $crate::nvptx64::SM_120 };
    ("sm_120a") => { $crate::nvptx64::SM_120A };
    ("sm_70") => { $crate::nvptx64::SM_70 };
    ("sm_72") => { $crate::nvptx64::SM_72 };
    ("sm_75") => { $crate::nvptx64::SM_75 };
    ("sm_80") => { $crate::nvptx64::SM_80 };
    ("sm_86") => { $crate::nvptx64::SM_86 };
    ("sm_87") => { $crate::nvptx64::SM_87 };
    ("sm_89") => { $crate::nvptx64::SM_89 };
    ("sm_90") => { $crate::nvptx64::SM_90 };
    ("sm_90a") => { $crate::nvptx64::SM_90A };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "powerpc")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("altivec") => { $crate::powerpc::ALTIVEC };
    ("crt-static") => { $crate::powerpc::CRT_STATIC };
    ("msync") => { $crate::powerpc::MSYNC };
    ("partword-atomics") => { $crate::powerpc::PARTWORD_ATOMICS };
    ("power10-vector") => { $crate::powerpc::POWER10_VECTOR };
    ("power8-altivec") => { $crate::powerpc::POWER8_ALTIVEC };
    ("power8-crypto") => { $crate::powerpc::POWER8_CRYPTO };
    ("power8-vector") => { $crate::powerpc::POWER8_VECTOR };
    ("power9-altivec") => { $crate::powerpc::POWER9_ALTIVEC };
    ("power9-vector") => { $crate::powerpc::POWER9_VECTOR };
    ("quadword-atomics") => { $crate::powerpc::QUADWORD_ATOMICS };
    ("vsx") => { $crate::powerpc::VSX };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "powerpc64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("altivec") => { $crate::powerpc64::ALTIVEC };
    ("crt-static") => { $crate::powerpc64::CRT_STATIC };
    ("msync") => { $crate::powerpc64::MSYNC };
    ("partword-atomics") => { $crate::powerpc64::PARTWORD_ATOMICS };
    ("power10-vector") => { $crate::powerpc64::POWER10_VECTOR };
    ("power8-altivec") => { $crate::powerpc64::POWER8_ALTIVEC };
    ("power8-crypto") => { $crate::powerpc64::POWER8_CRYPTO };
    ("power8-vector") => { $crate::powerpc64::POWER8_VECTOR };
    ("power9-altivec") => { $crate::powerpc64::POWER9_ALTIVEC };
    ("power9-vector") => { $crate::powerpc64::POWER9_VECTOR };
    ("quadword-atomics") => { $crate::powerpc64::QUADWORD_ATOMICS };
    ("vsx") => { $crate::powerpc64::VSX };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "riscv32")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("a") => { $crate::riscv32::A };
    ("b") => { $crate::riscv32::B };
    ("c") => { $crate::riscv32::C };
    ("crt-static") => { $crate::riscv32::CRT_STATIC };
    ("d") => { $crate::riscv32::D };
    ("e") => { $crate::riscv32::E };
    ("f") => { $crate::riscv32::F };
    ("m") => { $crate::riscv32::M };
    ("relax") => { $crate::riscv32::RELAX };
    ("rva23u64") => { $crate::riscv32::RVA23U64 };
    ("supm") => { $crate::riscv32::SUPM };
    ("unaligned-scalar-mem") => { $crate::riscv32::UNALIGNED_SCALAR_MEM };
    ("unaligned-vector-mem") => { $crate::riscv32::UNALIGNED_VECTOR_MEM };
    ("v") => { $crate::riscv32::V };
    ("za128rs") => { $crate::riscv32::ZA128RS };
    ("za64rs") => { $crate::riscv32::ZA64RS };
    ("zaamo") => { $crate::riscv32::ZAAMO };
    ("zabha") => { $crate::riscv32::ZABHA };
    ("zacas") => { $crate::riscv32::ZACAS };
    ("zalrsc") => { $crate::riscv32::ZALRSC };
    ("zama16b") => { $crate::riscv32::ZAMA16B };
    ("zawrs") => { $crate::riscv32::ZAWRS };
    ("zba") => { $crate::riscv32::ZBA };
    ("zbb") => { $crate::riscv32::ZBB };
    ("zbc") => { $crate::riscv32::ZBC };
    ("zbkb") => { $crate::riscv32::ZBKB };
    ("zbkc") => { $crate::riscv32::ZBKC };
    ("zbkx") => { $crate::riscv32::ZBKX };
    ("zbs") => { $crate::riscv32::ZBS };
    ("zca") => { $crate::riscv32::ZCA };
    ("zcb") => { $crate::riscv32::ZCB };
    ("zcmop") => { $crate::riscv32::ZCMOP };
    ("zdinx") => { $crate::riscv32::ZDINX };
    ("zfa") => { $crate::riscv32::ZFA };
    ("zfbfmin") => { $crate::riscv32::ZFBFMIN };
    ("zfh") => { $crate::riscv32::ZFH };
    ("zfhmin") => { $crate::riscv32::ZFHMIN };
    ("zfinx") => { $crate::riscv32::ZFINX };
    ("zhinx") => { $crate::riscv32::ZHINX };
    ("zhinxmin") => { $crate::riscv32::ZHINXMIN };
    ("zic64b") => { $crate::riscv32::ZIC64B };
    ("zicbom") => { $crate::riscv32::ZICBOM };
    ("zicbop") => { $crate::riscv32::ZICBOP };
    ("zicboz") => { $crate::riscv32::ZICBOZ };
    ("ziccamoa") => { $crate::riscv32::ZICCAMOA };
    ("ziccif") => { $crate::riscv32::ZICCIF };
    ("zicclsm") => { $crate::riscv32::ZICCLSM };
    ("ziccrse") => { $crate::riscv32::ZICCRSE };
    ("zicntr") => { $crate::riscv32::ZICNTR };
    ("zicond") => { $crate::riscv32::ZICOND };
    ("zicsr") => { $crate::riscv32::ZICSR };
    ("zifencei") => { $crate::riscv32::ZIFENCEI };
    ("zihintntl") => { $crate::riscv32::ZIHINTNTL };
    ("zihintpause") => { $crate::riscv32::ZIHINTPAUSE };
    ("zihpm") => { $crate::riscv32::ZIHPM };
    ("zimop") => { $crate::riscv32::ZIMOP };
    ("zk") => { $crate::riscv32::ZK };
    ("zkn") => { $crate::riscv32::ZKN };
    ("zknd") => { $crate::riscv32::ZKND };
    ("zkne") => { $crate::riscv32::ZKNE };
    ("zknh") => { $crate::riscv32::ZKNH };
    ("zkr") => { $crate::riscv32::ZKR };
    ("zks") => { $crate::riscv32::ZKS };
    ("zksed") => { $crate::riscv32::ZKSED };
    ("zksh") => { $crate::riscv32::ZKSH };
    ("zkt") => { $crate::riscv32::ZKT };
    ("ztso") => { $crate::riscv32::ZTSO };
    ("zvbb") => { $crate::riscv32::ZVBB };
    ("zvbc") => { $crate::riscv32::ZVBC };
    ("zve32f") => { $crate::riscv32::ZVE32F };
    ("zve32x") => { $crate::riscv32::ZVE32X };
    ("zve64d") => { $crate::riscv32::ZVE64D };
    ("zve64f") => { $crate::riscv32::ZVE64F };
    ("zve64x") => { $crate::riscv32::ZVE64X };
    ("zvfbfmin") => { $crate::riscv32::ZVFBFMIN };
    ("zvfbfwma") => { $crate::riscv32::ZVFBFWMA };
    ("zvfh") => { $crate::riscv32::ZVFH };
    ("zvfhmin") => { $crate::riscv32::ZVFHMIN };
    ("zvkb") => { $crate::riscv32::ZVKB };
    ("zvkg") => { $crate::riscv32::ZVKG };
    ("zvkn") => { $crate::riscv32::ZVKN };
    ("zvknc") => { $crate::riscv32::ZVKNC };
    ("zvkned") => { $crate::riscv32::ZVKNED };
    ("zvkng") => { $crate::riscv32::ZVKNG };
    ("zvknha") => { $crate::riscv32::ZVKNHA };
    ("zvknhb") => { $crate::riscv32::ZVKNHB };
    ("zvks") => { $crate::riscv32::ZVKS };
    ("zvksc") => { $crate::riscv32::ZVKSC };
    ("zvksed") => { $crate::riscv32::ZVKSED };
    ("zvksg") => { $crate::riscv32::ZVKSG };
    ("zvksh") => { $crate::riscv32::ZVKSH };
    ("zvkt") => { $crate::riscv32::ZVKT };
    ("zvl1024b") => { $crate::riscv32::ZVL1024B };
    ("zvl128b") => { $crate::riscv32::ZVL128B };
    ("zvl16384b") => { $crate::riscv32::ZVL16384B };
    ("zvl2048b") => { $crate::riscv32::ZVL2048B };
    ("zvl256b") => { $crate::riscv32::ZVL256B };
    ("zvl32768b") => { $crate::riscv32::ZVL32768B };
    ("zvl32b") => { $crate::riscv32::ZVL32B };
    ("zvl4096b") => { $crate::riscv32::ZVL4096B };
    ("zvl512b") => { $crate::riscv32::ZVL512B };
    ("zvl64b") => { $crate::riscv32::ZVL64B };
    ("zvl65536b") => { $crate::riscv32::ZVL65536B };
    ("zvl8192b") => { $crate::riscv32::ZVL8192B };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "riscv64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("a") => { $crate::riscv64::A };
    ("b") => { $crate::riscv64::B };
    ("c") => { $crate::riscv64::C };
    ("crt-static") => { $crate::riscv64::CRT_STATIC };
    ("d") => { $crate::riscv64::D };
    ("e") => { $crate::riscv64::E };
    ("f") => { $crate::riscv64::F };
    ("m") => { $crate::riscv64::M };
    ("relax") => { $crate::riscv64::RELAX };
    ("rva23u64") => { $crate::riscv64::RVA23U64 };
    ("supm") => { $crate::riscv64::SUPM };
    ("unaligned-scalar-mem") => { $crate::riscv64::UNALIGNED_SCALAR_MEM };
    ("unaligned-vector-mem") => { $crate::riscv64::UNALIGNED_VECTOR_MEM };
    ("v") => { $crate::riscv64::V };
    ("za128rs") => { $crate::riscv64::ZA128RS };
    ("za64rs") => { $crate::riscv64::ZA64RS };
    ("zaamo") => { $crate::riscv64::ZAAMO };
    ("zabha") => { $crate::riscv64::ZABHA };
    ("zacas") => { $crate::riscv64::ZACAS };
    ("zalrsc") => { $crate::riscv64::ZALRSC };
    ("zama16b") => { $crate::riscv64::ZAMA16B };
    ("zawrs") => { $crate::riscv64::ZAWRS };
    ("zba") => { $crate::riscv64::ZBA };
    ("zbb") => { $crate::riscv64::ZBB };
    ("zbc") => { $crate::riscv64::ZBC };
    ("zbkb") => { $crate::riscv64::ZBKB };
    ("zbkc") => { $crate::riscv64::ZBKC };
    ("zbkx") => { $crate::riscv64::ZBKX };
    ("zbs") => { $crate::riscv64::ZBS };
    ("zca") => { $crate::riscv64::ZCA };
    ("zcb") => { $crate::riscv64::ZCB };
    ("zcmop") => { $crate::riscv64::ZCMOP };
    ("zdinx") => { $crate::riscv64::ZDINX };
    ("zfa") => { $crate::riscv64::ZFA };
    ("zfbfmin") => { $crate::riscv64::ZFBFMIN };
    ("zfh") => { $crate::riscv64::ZFH };
    ("zfhmin") => { $crate::riscv64::ZFHMIN };
    ("zfinx") => { $crate::riscv64::ZFINX };
    ("zhinx") => { $crate::riscv64::ZHINX };
    ("zhinxmin") => { $crate::riscv64::ZHINXMIN };
    ("zic64b") => { $crate::riscv64::ZIC64B };
    ("zicbom") => { $crate::riscv64::ZICBOM };
    ("zicbop") => { $crate::riscv64::ZICBOP };
    ("zicboz") => { $crate::riscv64::ZICBOZ };
    ("ziccamoa") => { $crate::riscv64::ZICCAMOA };
    ("ziccif") => { $crate::riscv64::ZICCIF };
    ("zicclsm") => { $crate::riscv64::ZICCLSM };
    ("ziccrse") => { $crate::riscv64::ZICCRSE };
    ("zicntr") => { $crate::riscv64::ZICNTR };
    ("zicond") => { $crate::riscv64::ZICOND };
    ("zicsr") => { $crate::riscv64::ZICSR };
    ("zifencei") => { $crate::riscv64::ZIFENCEI };
    ("zihintntl") => { $crate::riscv64::ZIHINTNTL };
    ("zihintpause") => { $crate::riscv64::ZIHINTPAUSE };
    ("zihpm") => { $crate::riscv64::ZIHPM };
    ("zimop") => { $crate::riscv64::ZIMOP };
    ("zk") => { $crate::riscv64::ZK };
    ("zkn") => { $crate::riscv64::ZKN };
    ("zknd") => { $crate::riscv64::ZKND };
    ("zkne") => { $crate::riscv64::ZKNE };
    ("zknh") => { $crate::riscv64::ZKNH };
    ("zkr") => { $crate::riscv64::ZKR };
    ("zks") => { $crate::riscv64::ZKS };
    ("zksed") => { $crate::riscv64::ZKSED };
    ("zksh") => { $crate::riscv64::ZKSH };
    ("zkt") => { $crate::riscv64::ZKT };
    ("ztso") => { $crate::riscv64::ZTSO };
    ("zvbb") => { $crate::riscv64::ZVBB };
    ("zvbc") => { $crate::riscv64::ZVBC };
    ("zve32f") => { $crate::riscv64::ZVE32F };
    ("zve32x") => { $crate::riscv64::ZVE32X };
    ("zve64d") => { $crate::riscv64::ZVE64D };
    ("zve64f") => { $crate::riscv64::ZVE64F };
    ("zve64x") => { $crate::riscv64::ZVE64X };
    ("zvfbfmin") => { $crate::riscv64::ZVFBFMIN };
    ("zvfbfwma") => { $crate::riscv64::ZVFBFWMA };
    ("zvfh") => { $crate::riscv64::ZVFH };
    ("zvfhmin") => { $crate::riscv64::ZVFHMIN };
    ("zvkb") => { $crate::riscv64::ZVKB };
    ("zvkg") => { $crate::riscv64::ZVKG };
    ("zvkn") => { $crate::riscv64::ZVKN };
    ("zvknc") => { $crate::riscv64::ZVKNC };
    ("zvkned") => { $crate::riscv64::ZVKNED };
    ("zvkng") => { $crate::riscv64::ZVKNG };
    ("zvknha") => { $crate::riscv64::ZVKNHA };
    ("zvknhb") => { $crate::riscv64::ZVKNHB };
    ("zvks") => { $crate::riscv64::ZVKS };
    ("zvksc") => { $crate::riscv64::ZVKSC };
    ("zvksed") => { $crate::riscv64::ZVKSED };
    ("zvksg") => { $crate::riscv64::ZVKSG };
    ("zvksh") => { $crate::riscv64::ZVKSH };
    ("zvkt") => { $crate::riscv64::ZVKT };
    ("zvl1024b") => { $crate::riscv64::ZVL1024B };
    ("zvl128b") => { $crate::riscv64::ZVL128B };
    ("zvl16384b") => { $crate::riscv64::ZVL16384B };
    ("zvl2048b") => { $crate::riscv64::ZVL2048B };
    ("zvl256b") => { $crate::riscv64::ZVL256B };
    ("zvl32768b") => { $crate::riscv64::ZVL32768B };
    ("zvl32b") => { $crate::riscv64::ZVL32B };
    ("zvl4096b") => { $crate::riscv64::ZVL4096B };
    ("zvl512b") => { $crate::riscv64::ZVL512B };
    ("zvl64b") => { $crate::riscv64::ZVL64B };
    ("zvl65536b") => { $crate::riscv64::ZVL65536B };
    ("zvl8192b") => { $crate::riscv64::ZVL8192B };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "s390x")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("backchain") => { $crate::s390x::BACKCHAIN };
    ("concurrent-functions") => { $crate::s390x::CONCURRENT_FUNCTIONS };
    ("crt-static") => { $crate::s390x::CRT_STATIC };
    ("deflate-conversion") => { $crate::s390x::DEFLATE_CONVERSION };
    ("enhanced-sort") => { $crate::s390x::ENHANCED_SORT };
    ("guarded-storage") => { $crate::s390x::GUARDED_STORAGE };
    ("high-word") => { $crate::s390x::HIGH_WORD };
    ("message-security-assist-extension12") => { $crate::s390x::MESSAGE_SECURITY_ASSIST_EXTENSION12 };
    ("message-security-assist-extension3") => { $crate::s390x::MESSAGE_SECURITY_ASSIST_EXTENSION3 };
    ("message-security-assist-extension4") => { $crate::s390x::MESSAGE_SECURITY_ASSIST_EXTENSION4 };
    ("message-security-assist-extension5") => { $crate::s390x::MESSAGE_SECURITY_ASSIST_EXTENSION5 };
    ("message-security-assist-extension8") => { $crate::s390x::MESSAGE_SECURITY_ASSIST_EXTENSION8 };
    ("message-security-assist-extension9") => { $crate::s390x::MESSAGE_SECURITY_ASSIST_EXTENSION9 };
    ("miscellaneous-extensions-2") => { $crate::s390x::MISCELLANEOUS_EXTENSIONS_2 };
    ("miscellaneous-extensions-3") => { $crate::s390x::MISCELLANEOUS_EXTENSIONS_3 };
    ("miscellaneous-extensions-4") => { $crate::s390x::MISCELLANEOUS_EXTENSIONS_4 };
    ("nnp-assist") => { $crate::s390x::NNP_ASSIST };
    ("transactional-execution") => { $crate::s390x::TRANSACTIONAL_EXECUTION };
    ("vector") => { $crate::s390x::VECTOR };
    ("vector-enhancements-1") => { $crate::s390x::VECTOR_ENHANCEMENTS_1 };
    ("vector-enhancements-2") => { $crate::s390x::VECTOR_ENHANCEMENTS_2 };
    ("vector-enhancements-3") => { $crate::s390x::VECTOR_ENHANCEMENTS_3 };
    ("vector-packed-decimal") => { $crate::s390x::VECTOR_PACKED_DECIMAL };
    ("vector-packed-decimal-enhancement") => { $crate::s390x::VECTOR_PACKED_DECIMAL_ENHANCEMENT };
    ("vector-packed-decimal-enhancement-2") => { $crate::s390x::VECTOR_PACKED_DECIMAL_ENHANCEMENT_2 };
    ("vector-packed-decimal-enhancement-3") => { $crate::s390x::VECTOR_PACKED_DECIMAL_ENHANCEMENT_3 };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "sparc")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("crt-static") => { $crate::sparc::CRT_STATIC };
    ("leoncasa") => { $crate::sparc::LEONCASA };
    ("v8plus") => { $crate::sparc::V8PLUS };
    ("v9") => { $crate::sparc::V9 };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "sparc64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("crt-static") => { $crate::sparc64::CRT_STATIC };
    ("leoncasa") => { $crate::sparc64::LEONCASA };
    ("v8plus") => { $crate::sparc64::V8PLUS };
    ("v9") => { $crate::sparc64::V9 };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("atomics") => { $crate::wasm32::ATOMICS };
    ("bulk-memory") => { $crate::wasm32::BULK_MEMORY };
    ("crt-static") => { $crate::wasm32::CRT_STATIC };
    ("exception-handling") => { $crate::wasm32::EXCEPTION_HANDLING };
    ("extended-const") => { $crate::wasm32::EXTENDED_CONST };
    ("gc") => { $crate::wasm32::GC };
    ("multivalue") => { $crate::wasm32::MULTIVALUE };
    ("mutable-globals") => { $crate::wasm32::MUTABLE_GLOBALS };
    ("nontrapping-fptoint") => { $crate::wasm32::NONTRAPPING_FPTOINT };
    ("reference-types") => { $crate::wasm32::REFERENCE_TYPES };
    ("relaxed-simd") => { $crate::wasm32::RELAXED_SIMD };
    ("sign-ext") => { $crate::wasm32::SIGN_EXT };
    ("simd128") => { $crate::wasm32::SIMD128 };
    ("tail-call") => { $crate::wasm32::TAIL_CALL };
    ("wide-arithmetic") => { $crate::wasm32::WIDE_ARITHMETIC };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "wasm64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("atomics") => { $crate::wasm64::ATOMICS };
    ("bulk-memory") => { $crate::wasm64::BULK_MEMORY };
    ("crt-static") => { $crate::wasm64::CRT_STATIC };
    ("exception-handling") => { $crate::wasm64::EXCEPTION_HANDLING };
    ("extended-const") => { $crate::wasm64::EXTENDED_CONST };
    ("gc") => { $crate::wasm64::GC };
    ("multivalue") => { $crate::wasm64::MULTIVALUE };
    ("mutable-globals") => { $crate::wasm64::MUTABLE_GLOBALS };
    ("nontrapping-fptoint") => { $crate::wasm64::NONTRAPPING_FPTOINT };
    ("reference-types") => { $crate::wasm64::REFERENCE_TYPES };
    ("relaxed-simd") => { $crate::wasm64::RELAXED_SIMD };
    ("sign-ext") => { $crate::wasm64::SIGN_EXT };
    ("simd128") => { $crate::wasm64::SIMD128 };
    ("tail-call") => { $crate::wasm64::TAIL_CALL };
    ("wide-arithmetic") => { $crate::wasm64::WIDE_ARITHMETIC };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "x86")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("adx") => { $crate::x86::ADX };
    ("aes") => { $crate::x86::AES };
    ("amx-avx512") => { $crate::x86::AMX_AVX512 };
    ("amx-bf16") => { $crate::x86::AMX_BF16 };
    ("amx-complex") => { $crate::x86::AMX_COMPLEX };
    ("amx-fp16") => { $crate::x86::AMX_FP16 };
    ("amx-fp8") => { $crate::x86::AMX_FP8 };
    ("amx-int8") => { $crate::x86::AMX_INT8 };
    ("amx-movrs") => { $crate::x86::AMX_MOVRS };
    ("amx-tile") => { $crate::x86::AMX_TILE };
    ("apxf") => { $crate::x86::APXF };
    ("avx") => { $crate::x86::AVX };
    ("avx10.1") => { $crate::x86::AVX10_1 };
    ("avx10.2") => { $crate::x86::AVX10_2 };
    ("avx2") => { $crate::x86::AVX2 };
    ("avx512bf16") => { $crate::x86::AVX512BF16 };
    ("avx512bitalg") => { $crate::x86::AVX512BITALG };
    ("avx512bw") => { $crate::x86::AVX512BW };
    ("avx512cd") => { $crate::x86::AVX512CD };
    ("avx512dq") => { $crate::x86::AVX512DQ };
    ("avx512f") => { $crate::x86::AVX512F };
    ("avx512fp16") => { $crate::x86::AVX512FP16 };
    ("avx512ifma") => { $crate::x86::AVX512IFMA };
    ("avx512vbmi") => { $crate::x86::AVX512VBMI };
    ("avx512vbmi2") => { $crate::x86::AVX512VBMI2 };
    ("avx512vl") => { $crate::x86::AVX512VL };
    ("avx512vnni") => { $crate::x86::AVX512VNNI };
    ("avx512vp2intersect") => { $crate::x86::AVX512VP2INTERSECT };
    ("avx512vpopcntdq") => { $crate::x86::AVX512VPOPCNTDQ };
    ("avxifma") => { $crate::x86::AVXIFMA };
    ("avxneconvert") => { $crate::x86::AVXNECONVERT };
    ("avxvnni") => { $crate::x86::AVXVNNI };
    ("avxvnniint16") => { $crate::x86::AVXVNNIINT16 };
    ("avxvnniint8") => { $crate::x86::AVXVNNIINT8 };
    ("bmi1") => { $crate::x86::BMI1 };
    ("bmi2") => { $crate::x86::BMI2 };
    ("clflushopt") => { $crate::x86::CLFLUSHOPT };
    ("cmpxchg16b") => { $crate::x86::CMPXCHG16B };
    ("crt-static") => { $crate::x86::CRT_STATIC };
    ("ermsb") => { $crate::x86::ERMSB };
    ("f16c") => { $crate::x86::F16C };
    ("fma") => { $crate::x86::FMA };
    ("fma4") => { $crate::x86::FMA4 };
    ("fxsr") => { $crate::x86::FXSR };
    ("gfni") => { $crate::x86::GFNI };
    ("kl") => { $crate::x86::KL };
    ("lahfsahf") => { $crate::x86::LAHFSAHF };
    ("lzcnt") => { $crate::x86::LZCNT };
    ("movbe") => { $crate::x86::MOVBE };
    ("movrs") => { $crate::x86::MOVRS };
    ("pclmulqdq") => { $crate::x86::PCLMULQDQ };
    ("popcnt") => { $crate::x86::POPCNT };
    ("prfchw") => { $crate::x86::PRFCHW };
    ("rdrand") => { $crate::x86::RDRAND };
    ("rdseed") => { $crate::x86::RDSEED };
    ("rtm") => { $crate::x86::RTM };
    ("sha") => { $crate::x86::SHA };
    ("sha512") => { $crate::x86::SHA512 };
    ("sm3") => { $crate::x86::SM3 };
    ("sm4") => { $crate::x86::SM4 };
    ("sse") => { $crate::x86::SSE };
    ("sse2") => { $crate::x86::SSE2 };
    ("sse3") => { $crate::x86::SSE3 };
    ("sse4.1") => { $crate::x86::SSE4_1 };
    ("sse4.2") => { $crate::x86::SSE4_2 };
    ("sse4a") => { $crate::x86::SSE4A };
    ("ssse3") => { $crate::x86::SSSE3 };
    ("tbm") => { $crate::x86::TBM };
    ("vaes") => { $crate::x86::VAES };
    ("vpclmulqdq") => { $crate::x86::VPCLMULQDQ };
    ("widekl") => { $crate::x86::WIDEKL };
    ("x87") => { $crate::x86::X87 };
    ("xop") => { $crate::x86::XOP };
    ("xsave") => { $crate::x86::XSAVE };
    ("xsavec") => { $crate::x86::XSAVEC };
    ("xsaveopt") => { $crate::x86::XSAVEOPT };
    ("xsaves") => { $crate::x86::XSAVES };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}
#[cfg(target_arch = "x86_64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __target_feature {
    ("adx") => { $crate::x86_64::ADX };
    ("aes") => { $crate::x86_64::AES };
    ("amx-avx512") => { $crate::x86_64::AMX_AVX512 };
    ("amx-bf16") => { $crate::x86_64::AMX_BF16 };
    ("amx-complex") => { $crate::x86_64::AMX_COMPLEX };
    ("amx-fp16") => { $crate::x86_64::AMX_FP16 };
    ("amx-fp8") => { $crate::x86_64::AMX_FP8 };
    ("amx-int8") => { $crate::x86_64::AMX_INT8 };
    ("amx-movrs") => { $crate::x86_64::AMX_MOVRS };
    ("amx-tile") => { $crate::x86_64::AMX_TILE };
    ("apxf") => { $crate::x86_64::APXF };
    ("avx") => { $crate::x86_64::AVX };
    ("avx10.1") => { $crate::x86_64::AVX10_1 };
    ("avx10.2") => { $crate::x86_64::AVX10_2 };
    ("avx2") => { $crate::x86_64::AVX2 };
    ("avx512bf16") => { $crate::x86_64::AVX512BF16 };
    ("avx512bitalg") => { $crate::x86_64::AVX512BITALG };
    ("avx512bw") => { $crate::x86_64::AVX512BW };
    ("avx512cd") => { $crate::x86_64::AVX512CD };
    ("avx512dq") => { $crate::x86_64::AVX512DQ };
    ("avx512f") => { $crate::x86_64::AVX512F };
    ("avx512fp16") => { $crate::x86_64::AVX512FP16 };
    ("avx512ifma") => { $crate::x86_64::AVX512IFMA };
    ("avx512vbmi") => { $crate::x86_64::AVX512VBMI };
    ("avx512vbmi2") => { $crate::x86_64::AVX512VBMI2 };
    ("avx512vl") => { $crate::x86_64::AVX512VL };
    ("avx512vnni") => { $crate::x86_64::AVX512VNNI };
    ("avx512vp2intersect") => { $crate::x86_64::AVX512VP2INTERSECT };
    ("avx512vpopcntdq") => { $crate::x86_64::AVX512VPOPCNTDQ };
    ("avxifma") => { $crate::x86_64::AVXIFMA };
    ("avxneconvert") => { $crate::x86_64::AVXNECONVERT };
    ("avxvnni") => { $crate::x86_64::AVXVNNI };
    ("avxvnniint16") => { $crate::x86_64::AVXVNNIINT16 };
    ("avxvnniint8") => { $crate::x86_64::AVXVNNIINT8 };
    ("bmi1") => { $crate::x86_64::BMI1 };
    ("bmi2") => { $crate::x86_64::BMI2 };
    ("clflushopt") => { $crate::x86_64::CLFLUSHOPT };
    ("cmpxchg16b") => { $crate::x86_64::CMPXCHG16B };
    ("crt-static") => { $crate::x86_64::CRT_STATIC };
    ("ermsb") => { $crate::x86_64::ERMSB };
    ("f16c") => { $crate::x86_64::F16C };
    ("fma") => { $crate::x86_64::FMA };
    ("fma4") => { $crate::x86_64::FMA4 };
    ("fxsr") => { $crate::x86_64::FXSR };
    ("gfni") => { $crate::x86_64::GFNI };
    ("kl") => { $crate::x86_64::KL };
    ("lahfsahf") => { $crate::x86_64::LAHFSAHF };
    ("lzcnt") => { $crate::x86_64::LZCNT };
    ("movbe") => { $crate::x86_64::MOVBE };
    ("movrs") => { $crate::x86_64::MOVRS };
    ("pclmulqdq") => { $crate::x86_64::PCLMULQDQ };
    ("popcnt") => { $crate::x86_64::POPCNT };
    ("prfchw") => { $crate::x86_64::PRFCHW };
    ("rdrand") => { $crate::x86_64::RDRAND };
    ("rdseed") => { $crate::x86_64::RDSEED };
    ("rtm") => { $crate::x86_64::RTM };
    ("sha") => { $crate::x86_64::SHA };
    ("sha512") => { $crate::x86_64::SHA512 };
    ("sm3") => { $crate::x86_64::SM3 };
    ("sm4") => { $crate::x86_64::SM4 };
    ("sse") => { $crate::x86_64::SSE };
    ("sse2") => { $crate::x86_64::SSE2 };
    ("sse3") => { $crate::x86_64::SSE3 };
    ("sse4.1") => { $crate::x86_64::SSE4_1 };
    ("sse4.2") => { $crate::x86_64::SSE4_2 };
    ("sse4a") => { $crate::x86_64::SSE4A };
    ("ssse3") => { $crate::x86_64::SSSE3 };
    ("tbm") => { $crate::x86_64::TBM };
    ("vaes") => { $crate::x86_64::VAES };
    ("vpclmulqdq") => { $crate::x86_64::VPCLMULQDQ };
    ("widekl") => { $crate::x86_64::WIDEKL };
    ("x87") => { $crate::x86_64::X87 };
    ("xop") => { $crate::x86_64::XOP };
    ("xsave") => { $crate::x86_64::XSAVE };
    ("xsavec") => { $crate::x86_64::XSAVEC };
    ("xsaveopt") => { $crate::x86_64::XSAVEOPT };
    ("xsaves") => { $crate::x86_64::XSAVES };
    ($feature:tt) => {
        compile_error!(concat!("unknown target feature: ", stringify!($feature)))
    };
}

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "bpf",
    target_arch = "hexagon",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "loongarch32",
    target_arch = "loongarch64",
    target_arch = "nvptx64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "s390x",
    target_arch = "sparc",
    target_arch = "sparc64",
    target_arch = "wasm32",
    target_arch = "wasm64",
    target_arch = "x86",
    target_arch = "x86_64",
)))]
#[doc(hidden)]
#[macro_export]
macro_rules! __target_feature {
    ($feature:tt) => {
        compile_error!(concat!(
            "target features are unavailable for this architecture: ",
            stringify!($feature)
        ))
    };
}

/// Constructs a [`TargetFeatures`] value from target feature names.
///
/// Each name identifies a target feature.
/// Multiple names are combined using [`TargetFeatures::with`].
///
/// ```
/// # #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/// const REQUIRED: target_features::TargetFeatures =
///     target_features::target_features!("avx", "fma", "bmi2");
/// ```
#[macro_export]
macro_rules! target_features {
    ($($feature:tt),* $(,)?) => { {
        let features = $crate::TargetFeatures::empty();
        $(let features = features.with($crate::__target_feature!($feature));)*
        features
    } };
}

#[allow(unused_variables)]
pub(crate) const fn enabled_for_target() -> TargetFeatures {
    let features = TargetFeatures::empty();
    #[cfg(target_arch = "arm")]
    let features = arm::enabled_for_target();
    #[cfg(target_arch = "aarch64")]
    let features = aarch64::enabled_for_target();
    #[cfg(target_arch = "arm64ec")]
    let features = arm64ec::enabled_for_target();
    #[cfg(target_arch = "bpf")]
    let features = bpf::enabled_for_target();
    #[cfg(target_arch = "hexagon")]
    let features = hexagon::enabled_for_target();
    #[cfg(target_arch = "mips")]
    let features = mips::enabled_for_target();
    #[cfg(target_arch = "mips64")]
    let features = mips64::enabled_for_target();
    #[cfg(target_arch = "loongarch32")]
    let features = loongarch32::enabled_for_target();
    #[cfg(target_arch = "loongarch64")]
    let features = loongarch64::enabled_for_target();
    #[cfg(target_arch = "nvptx64")]
    let features = nvptx64::enabled_for_target();
    #[cfg(target_arch = "powerpc")]
    let features = powerpc::enabled_for_target();
    #[cfg(target_arch = "powerpc64")]
    let features = powerpc64::enabled_for_target();
    #[cfg(target_arch = "riscv32")]
    let features = riscv32::enabled_for_target();
    #[cfg(target_arch = "riscv64")]
    let features = riscv64::enabled_for_target();
    #[cfg(target_arch = "s390x")]
    let features = s390x::enabled_for_target();
    #[cfg(target_arch = "sparc")]
    let features = sparc::enabled_for_target();
    #[cfg(target_arch = "sparc64")]
    let features = sparc64::enabled_for_target();
    #[cfg(target_arch = "wasm32")]
    let features = wasm32::enabled_for_target();
    #[cfg(target_arch = "wasm64")]
    let features = wasm64::enabled_for_target();
    #[cfg(target_arch = "x86")]
    let features = x86::enabled_for_target();
    #[cfg(target_arch = "x86_64")]
    let features = x86_64::enabled_for_target();
    features
}

#[allow(unused_variables)]
pub(crate) fn features() -> &'static [FeatureData] {
    let features: &'static [FeatureData] = &[];
    #[cfg(target_arch = "arm")]
    let features = arm::FEATURES;
    #[cfg(target_arch = "aarch64")]
    let features = aarch64::FEATURES;
    #[cfg(target_arch = "arm64ec")]
    let features = arm64ec::FEATURES;
    #[cfg(target_arch = "bpf")]
    let features = bpf::FEATURES;
    #[cfg(target_arch = "hexagon")]
    let features = hexagon::FEATURES;
    #[cfg(target_arch = "mips")]
    let features = mips::FEATURES;
    #[cfg(target_arch = "mips64")]
    let features = mips64::FEATURES;
    #[cfg(target_arch = "loongarch32")]
    let features = loongarch32::FEATURES;
    #[cfg(target_arch = "loongarch64")]
    let features = loongarch64::FEATURES;
    #[cfg(target_arch = "nvptx64")]
    let features = nvptx64::FEATURES;
    #[cfg(target_arch = "powerpc")]
    let features = powerpc::FEATURES;
    #[cfg(target_arch = "powerpc64")]
    let features = powerpc64::FEATURES;
    #[cfg(target_arch = "riscv32")]
    let features = riscv32::FEATURES;
    #[cfg(target_arch = "riscv64")]
    let features = riscv64::FEATURES;
    #[cfg(target_arch = "s390x")]
    let features = s390x::FEATURES;
    #[cfg(target_arch = "sparc")]
    let features = sparc::FEATURES;
    #[cfg(target_arch = "sparc64")]
    let features = sparc64::FEATURES;
    #[cfg(target_arch = "wasm32")]
    let features = wasm32::FEATURES;
    #[cfg(target_arch = "wasm64")]
    let features = wasm64::FEATURES;
    #[cfg(target_arch = "x86")]
    let features = x86::FEATURES;
    #[cfg(target_arch = "x86_64")]
    let features = x86_64::FEATURES;
    features
}
