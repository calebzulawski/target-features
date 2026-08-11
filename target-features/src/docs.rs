/// arm documentation
pub mod arm {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `aclass` | Is application profile ('A' series). |  |
    /// | `acquire-release` | Has v8 acquire/release (lda/ldaex  etc) instructions. |  |
    /// | `aes` | Enable AES support. | `d32`, `neon`, `vfp2`, `vfp3` |
    /// | `crc` | Enable support for CRC instructions. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `d32` | Extend FP to 32 double registers. |  |
    /// | `dotprod` | Enable support for dot product instructions. | `d32`, `neon`, `vfp2`, `vfp3` |
    /// | `dsp` | Supports DSP instructions in ARM and/or Thumb2. |  |
    /// | `fp-armv8` | Enable ARMv8 FP. | `d32`, `vfp2`, `vfp3`, `vfp4` |
    /// | `fp16` | Enable full half-precision floating point. | `d32`, `neon`, `vfp2`, `vfp3` |
    /// | `fpregs` | Enable FP registers. |  |
    /// | `i8mm` | Enable Matrix Multiply Int8 Extension. | `d32`, `neon`, `vfp2`, `vfp3` |
    /// | `mclass` | Is microcontroller profile ('M' series). |  |
    /// | `mve` | Support M-Class Vector Extension with integer ops. | `dsp`, `fpregs`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8.1m.main`, `v8m`, `v8m.main` |
    /// | `mve.fp` | Support M-Class Vector Extension with integer and floating ops. | `dsp`, `fpregs`, `mve`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8.1m.main`, `v8m`, `v8m.main` |
    /// | `neon` | Enable NEON instructions. | `d32`, `vfp2`, `vfp3` |
    /// | `rclass` | Is realtime profile ('R' series). |  |
    /// | `sha2` | Enable SHA1 and SHA256 support. | `d32`, `neon`, `vfp2`, `vfp3` |
    /// | `soft-float` | Use software floating point features.. |  |
    /// | `thumb-mode` | Thumb mode. |  |
    /// | `thumb2` | Enable Thumb2 instructions. |  |
    /// | `trustzone` | Enable support for TrustZone security extensions. |  |
    /// | `v5te` | Support ARM v5TE, v5TEj, and v5TExp instructions. |  |
    /// | `v6` | Support ARM v6 instructions. | `v5te` |
    /// | `v6k` | Support ARM v6k instructions. | `v5te`, `v6` |
    /// | `v6m` | Support ARM v6M instructions. | `v5te`, `v6` |
    /// | `v6t2` | Support ARM v6t2 instructions. | `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v8m` |
    /// | `v7` | Support ARM v7 instructions. | `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v8m` |
    /// | `v8` | Support ARM v8 instructions. | `acquire-release`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m` |
    /// | `v8.1m.main` | Support ARM v8-1M Mainline instructions. | `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `v8m.main` |
    /// | `v8m` | Support ARM v8M Baseline instructions. | `v5te`, `v6`, `v6m` |
    /// | `v8m.main` | Support ARM v8M Mainline instructions. | `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m` |
    /// | `vfp2` | Enable VFP2 instructions. |  |
    /// | `vfp3` | Enable VFP3 instructions. | `d32`, `vfp2` |
    /// | `vfp4` | Enable VFP4 instructions. | `d32`, `vfp2`, `vfp3` |
    /// | `virtualization` | Supports Virtualization extension. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `arm1020e` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm1020t` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm1022e` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm10e` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm10tdmi` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm1136j-s` | `dsp`, `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm1136jf-s` | `dsp`, `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm1156t2-s` | `dsp`, `fpregs`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v8m`, `vfp2` |
    /// | `arm1156t2f-s` | `dsp`, `fpregs`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v8m`, `vfp2` |
    /// | `arm1176jz-s` | `fpregs`, `trustzone`, `v5te`, `v6`, `v6k`, `vfp2` |
    /// | `arm1176jzf-s` | `fpregs`, `trustzone`, `v5te`, `v6`, `v6k`, `vfp2` |
    /// | `arm710t` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm720t` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm7tdmi` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm7tdmi-s` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm8` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm810` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm9` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm920` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm920t` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm922t` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm926ej-s` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm940t` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm946e-s` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm966e-s` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm968e-s` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm9e` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `arm9tdmi` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `cortex-a12` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a15` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a17` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a32` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a35` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a5` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3`, `vfp4` |
    /// | `cortex-a510` | `aclass`, `acquire-release`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `i8mm`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a53` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a55` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a57` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a7` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a710` | `aclass`, `acquire-release`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `i8mm`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a72` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a73` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a75` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a76` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a76ae` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a77` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a78` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a78ae` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a78c` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-a8` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3` |
    /// | `cortex-a9` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3` |
    /// | `cortex-m0` | `fpregs`, `mclass`, `thumb-mode`, `v5te`, `v6`, `v6m`, `vfp2` |
    /// | `cortex-m0plus` | `fpregs`, `mclass`, `thumb-mode`, `v5te`, `v6`, `v6m`, `vfp2` |
    /// | `cortex-m1` | `fpregs`, `mclass`, `thumb-mode`, `v5te`, `v6`, `v6m`, `vfp2` |
    /// | `cortex-m23` | `acquire-release`, `fpregs`, `mclass`, `thumb-mode`, `v5te`, `v6`, `v6m`, `v8m`, `vfp2` |
    /// | `cortex-m3` | `fpregs`, `mclass`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `cortex-m33` | `acquire-release`, `dsp`, `fpregs`, `mclass`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `v8m.main`, `vfp2` |
    /// | `cortex-m35p` | `acquire-release`, `dsp`, `fpregs`, `mclass`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `v8m.main`, `vfp2` |
    /// | `cortex-m4` | `dsp`, `fpregs`, `mclass`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `cortex-m52` | `acquire-release`, `d32`, `dsp`, `fp16`, `fpregs`, `mclass`, `mve`, `mve.fp`, `neon`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8.1m.main`, `v8m`, `v8m.main`, `vfp2`, `vfp3` |
    /// | `cortex-m55` | `acquire-release`, `d32`, `dsp`, `fp16`, `fpregs`, `mclass`, `mve`, `mve.fp`, `neon`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8.1m.main`, `v8m`, `v8m.main`, `vfp2`, `vfp3` |
    /// | `cortex-m7` | `dsp`, `fpregs`, `mclass`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `cortex-m85` | `acquire-release`, `d32`, `dsp`, `fp16`, `fpregs`, `mclass`, `mve`, `mve.fp`, `neon`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8.1m.main`, `v8m`, `v8m.main`, `vfp2`, `vfp3` |
    /// | `cortex-r4` | `dsp`, `fpregs`, `rclass`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `cortex-r4f` | `dsp`, `fpregs`, `rclass`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `cortex-r5` | `dsp`, `fpregs`, `rclass`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `cortex-r52` | `acquire-release`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `rclass`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-r52plus` | `acquire-release`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `rclass`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-r7` | `dsp`, `fpregs`, `rclass`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `cortex-r8` | `dsp`, `fpregs`, `rclass`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `cortex-x1` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cortex-x1c` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `cyclone` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `ep9312` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `exynos-m3` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `exynos-m4` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `exynos-m5` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `generic` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `iwmmxt` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `krait` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3`, `vfp4` |
    /// | `kryo` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `mpcore` | `fpregs`, `v5te`, `v6`, `v6k`, `vfp2` |
    /// | `mpcorenovfp` | `fpregs`, `v5te`, `v6`, `v6k`, `vfp2` |
    /// | `neoverse-n1` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fpregs`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `neoverse-n2` | `aclass`, `acquire-release`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `i8mm`, `neon`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `neoverse-v1` | `aclass`, `acquire-release`, `aes`, `crc`, `d32`, `dotprod`, `dsp`, `fp-armv8`, `fp16`, `fpregs`, `i8mm`, `neon`, `sha2`, `thumb2`, `trustzone`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8`, `v8m`, `vfp2`, `vfp3`, `vfp4`, `virtualization` |
    /// | `sc000` | `fpregs`, `mclass`, `thumb-mode`, `v5te`, `v6`, `v6m`, `vfp2` |
    /// | `sc300` | `fpregs`, `mclass`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2` |
    /// | `star-mc1` | `acquire-release`, `dsp`, `fpregs`, `mclass`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `v8m.main`, `vfp2` |
    /// | `star-mc3` | `acquire-release`, `d32`, `dsp`, `fp16`, `fpregs`, `mclass`, `mve`, `mve.fp`, `neon`, `thumb-mode`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8.1m.main`, `v8m`, `v8m.main`, `vfp2`, `vfp3` |
    /// | `strongarm` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `strongarm110` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `strongarm1100` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `strongarm1110` | `fpregs`, `v5te`, `v6`, `vfp2` |
    /// | `swift` | `aclass`, `d32`, `dsp`, `fpregs`, `neon`, `thumb2`, `v5te`, `v6`, `v6k`, `v6m`, `v6t2`, `v7`, `v8m`, `vfp2`, `vfp3`, `vfp4` |
    /// | `xscale` | `fpregs`, `v5te`, `v6`, `vfp2` |
    pub mod cpus {}
}
/// aarch64 documentation
pub mod aarch64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `aes` | Enable AES support. | `neon` |
    /// | `bf16` | Enable BFloat16 Extension. |  |
    /// | `bti` | Enable Branch Target Identification. |  |
    /// | `crc` | Enable Armv8.0-A CRC-32 checksum instructions. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `cssc` | Enable Common Short Sequence Compression (CSSC) instructions. |  |
    /// | `dit` | Enable Armv8.4-A Data Independent Timing instructions. |  |
    /// | `dotprod` | Enable dot product support. | `neon` |
    /// | `dpb` | Enable Armv8.2-A data Cache Clean to Point of Persistence. |  |
    /// | `dpb2` | Enable Armv8.5-A Cache Clean to Point of Deep Persistence. | `dpb` |
    /// | `ecv` | Enable enhanced counter virtualization extension. |  |
    /// | `f32mm` | Enable Matrix Multiply FP32 Extension. | `fp16`, `neon`, `sve` |
    /// | `f64mm` | Enable Matrix Multiply FP64 Extension. | `fp16`, `neon`, `sve` |
    /// | `faminmax` | Enable FAMIN and FAMAX instructions. |  |
    /// | `fcma` | Enable Armv8.3-A Floating-point complex number support. | `neon` |
    /// | `fhm` | Enable FP16 FML instructions. | `fp16`, `neon` |
    /// | `flagm` | Enable Armv8.4-A Flag Manipulation instructions. |  |
    /// | `flagm2` | Enable alternative NZCV format for floating point comparisons. |  |
    /// | `fp16` | Enable half-precision floating-point data processing. | `neon` |
    /// | `fp8` | Enable FP8 instructions. | `bf16`, `faminmax`, `lut` |
    /// | `fp8dot2` | Enable FP8 2-way dot instructions. | `bf16`, `faminmax`, `fp8`, `fp8dot4`, `fp8fma`, `lut` |
    /// | `fp8dot4` | Enable FP8 4-way dot instructions. | `bf16`, `faminmax`, `fp8`, `fp8fma`, `lut` |
    /// | `fp8fma` | Enable Armv9.5-A FP8 multiply-add instructions. | `bf16`, `faminmax`, `fp8`, `lut` |
    /// | `frintts` | Enable FRInt\[32\|64\]\[Z\|X\] instructions that round a floating-point number to an integer (in FP format) forcing it to fit into a 32- or 64-bit int. |  |
    /// | `hbc` | Enable Armv8.8-A Hinted Conditional Branches Extension. |  |
    /// | `i8mm` | Enable Matrix Multiply Int8 Extension. |  |
    /// | `jsconv` | Enable Armv8.3-A JavaScript FP conversion instructions. | `neon` |
    /// | `lor` | Enable Armv8.1-A Limited Ordering Regions extension. |  |
    /// | `lse` | Enable Armv8.1-A Large System Extension (LSE) atomic instructions. |  |
    /// | `lse128` | Enable Armv9.4-A 128-bit Atomic instructions. | `lse` |
    /// | `lse2` | Enable Armv8.4-A Large System Extension 2 (LSE2) atomicity rules. |  |
    /// | `lut` | Enable Lookup Table instructions. |  |
    /// | `mops` | Enable Armv8.8-A memcpy and memset acceleration instructions. |  |
    /// | `mte` | Enable Memory Tagging Extension. |  |
    /// | `neon` | Enable Advanced SIMD instructions. |  |
    /// | `outline-atomics` | Enable out of line atomics to support LSE instructions. |  |
    /// | `paca` | Enable Armv8.3-A Pointer Authentication extension. |  |
    /// | `pacg` | Enable Armv8.3-A Pointer Authentication extension. |  |
    /// | `pan` | Enable Armv8.1-A Privileged Access-Never extension. |  |
    /// | `pauth-lr` | Enable Armv9.5-A PAC enhancements. |  |
    /// | `pmuv3` | Enable Armv8.0-A PMUv3 Performance Monitors extension. |  |
    /// | `rand` | Enable Random Number generation instructions. |  |
    /// | `ras` | Enable Armv8.0-A Reliability, Availability and Serviceability Extensions. |  |
    /// | `rcpc` | Enable support for RCPC extension. |  |
    /// | `rcpc2` | Enable Armv8.4-A RCPC instructions with Immediate Offsets. | `rcpc` |
    /// | `rcpc3` | Enable Armv8.9-A RCPC instructions for A64 and Advanced SIMD and floating-point instruction set. | `rcpc`, `rcpc2` |
    /// | `rdm` | Enable Armv8.1-A Rounding Double Multiply Add/Subtract instructions. | `neon` |
    /// | `sb` | Enable Armv8.5-A Speculation Barrier. |  |
    /// | `sha2` | Enable SHA1 and SHA256 support. | `neon` |
    /// | `sha3` | Enable SHA512 and SHA3 support. | `neon`, `sha2` |
    /// | `sm4` | Enable SM3 and SM4 support. | `neon` |
    /// | `sme` | Enable Scalable Matrix Extension (SME). | `bf16` |
    /// | `sme-b16b16` | Enable SME2.1 ZA-targeting non-widening BFloat16 instructions. | `bf16`, `sme`, `sme2`, `sve-b16b16` |
    /// | `sme-f16f16` | Enable SME non-widening Float16 instructions. | `bf16`, `sme`, `sme2` |
    /// | `sme-f64f64` | Enable Scalable Matrix Extension (SME) F64F64 instructions. | `bf16`, `sme` |
    /// | `sme-f8f16` | Enable Scalable Matrix Extension (SME) F8F16 instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme-f8f32`, `sme2` |
    /// | `sme-f8f32` | Enable Scalable Matrix Extension (SME) F8F32 instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme2` |
    /// | `sme-fa64` | Enable the full A64 instruction set in streaming SVE mode. | `bf16`, `fp16`, `neon`, `sme`, `sve`, `sve2` |
    /// | `sme-i16i64` | Enable Scalable Matrix Extension (SME) I16I64 instructions. | `bf16`, `sme` |
    /// | `sme-lutv2` | Enable Scalable Matrix Extension (SME) LUTv2 instructions. |  |
    /// | `sme2` | Enable Scalable Matrix Extension 2 (SME2) instructions. | `bf16`, `sme` |
    /// | `sme2p1` | Enable Scalable Matrix Extension 2.1 instructions. | `bf16`, `sme`, `sme2` |
    /// | `spe` | Enable Statistical Profiling extension. |  |
    /// | `ssbs` | Enable Speculative Store Bypass Safe bit. |  |
    /// | `ssve-fp8dot2` | Enable SVE2 FP8 2-way dot product instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme2`, `ssve-fp8dot4`, `ssve-fp8fma` |
    /// | `ssve-fp8dot4` | Enable SVE2 FP8 4-way dot product instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme2`, `ssve-fp8fma` |
    /// | `ssve-fp8fma` | Enable SVE2 FP8 multiply-add instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme2` |
    /// | `sve` | Enable Scalable Vector Extension (SVE) instructions. | `fp16`, `neon` |
    /// | `sve-b16b16` | Enable SVE2 non-widening and SME2 Z-targeting non-widening BFloat16 instructions. | `bf16` |
    /// | `sve2` | Enable Scalable Vector Extension 2 (SVE2) instructions. | `fp16`, `neon`, `sve` |
    /// | `sve2-aes` | Shorthand for +sve2+sve-aes. | `aes`, `fp16`, `neon`, `sve`, `sve2` |
    /// | `sve2-bitperm` | Shorthand for +sve2+sve-bitperm. | `fp16`, `neon`, `sve`, `sve2` |
    /// | `sve2-sha3` | Shorthand for +sve2+sve-sha3. | `fp16`, `neon`, `sha2`, `sha3`, `sve`, `sve2` |
    /// | `sve2-sm4` | Shorthand for +sve2+sve-sm4. | `fp16`, `neon`, `sm4`, `sve`, `sve2` |
    /// | `sve2p1` | Enable Scalable Vector Extension 2.1 instructions. | `fp16`, `neon`, `sve`, `sve2` |
    /// | `v8.1a` | Support ARM v8.1a architecture. | `crc`, `lor`, `lse`, `neon`, `pan`, `rdm`, `vh` |
    /// | `v8.2a` | Support ARM v8.2a architecture. | `crc`, `dpb`, `lor`, `lse`, `neon`, `pan`, `ras`, `rdm`, `v8.1a`, `vh` |
    /// | `v8.3a` | Support ARM v8.3a architecture. | `crc`, `dpb`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `v8.1a`, `v8.2a`, `vh` |
    /// | `v8.4a` | Support ARM v8.4a architecture. | `crc`, `dit`, `dotprod`, `dpb`, `flagm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `v8.1a`, `v8.2a`, `v8.3a`, `vh` |
    /// | `v8.5a` | Support ARM v8.5a architecture. | `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `v8.6a` | Support ARM v8.6a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `i8mm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `vh` |
    /// | `v8.7a` | Support ARM v8.7a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `i8mm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh`, `wfxt` |
    /// | `v8.8a` | Support ARM v8.8a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `v8.9a` | Support ARM v8.9a architecture. | `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `vh`, `wfxt` |
    /// | `v9.1a` | Support ARM v9.1a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `i8mm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v9a`, `vh` |
    /// | `v9.2a` | Support ARM v9.2a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `i8mm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9a`, `vh`, `wfxt` |
    /// | `v9.3a` | Support ARM v9.3a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `v9.4a` | Support ARM v9.4a architecture. | `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v8.9a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `v9.5a` | Support ARM v9.5a architecture. | `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v8.9a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9.4a`, `v9a`, `vh`, `wfxt` |
    /// | `v9a` | Support ARM v9a architecture. | `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `vh` |
    /// | `vh` | Enable Armv8.1-A Virtual Host extension. |  |
    /// | `wfxt` | Enable Armv8.7-A WFET and WFIT instruction. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `a64fx` | `aes`, `crc`, `dpb`, `fcma`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `sve`, `v8.1a`, `v8.2a`, `vh` |
    /// | `ampere1` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `ampere1a` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `ampere1b` | `aes`, `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `ampere1c` | `aes`, `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `faminmax`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `fp8`, `fp8fma`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `lut`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `ssbs`, `sve`, `sve-b16b16`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `apple-a10` | `aes`, `crc`, `lor`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `rdm`, `sha2`, `vh` |
    /// | `apple-a11` | `aes`, `crc`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `apple-a12` | `aes`, `crc`, `dpb`, `fcma`, `fp16`, `jsconv`, `lor`, `lse`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `v8.3a`, `vh` |
    /// | `apple-a13` | `aes`, `crc`, `dit`, `dotprod`, `dpb`, `fcma`, `fhm`, `flagm`, `fp16`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `sha3`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `apple-a14` | `aes`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `apple-a15` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `apple-a16` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `apple-a17` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `apple-a7` | `aes`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `apple-m4` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sme`, `sme-f64f64`, `sme-i16i64`, `sme2`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `apple-m5` | `aes`, `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sme`, `sme-b16b16`, `sme-f16f16`, `sme-f64f64`, `sme-i16i64`, `sme2`, `sme2p1`, `ssbs`, `sve-b16b16`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `c1-nano` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mops`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sme`, `sme2`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `c1-premium` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mops`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sme`, `sme2`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `c1-pro` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mops`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sme`, `sme2`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `c1-ultra` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mops`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sme`, `sme2`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `carmel` | `aes`, `crc`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `ras`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a320` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a34` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `cortex-a35` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `cortex-a510` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-a520` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a520ae` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a53` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `cortex-a55` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a57` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `cortex-a65` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a65ae` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a710` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-a715` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-a72` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `cortex-a720` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a720ae` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a725` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a73` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `cortex-a75` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a76` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a76ae` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a77` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a78` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a78ae` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a78c` | `aes`, `crc`, `dotprod`, `dpb`, `flagm`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-r82` | `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `fp16`, `jsconv`, `lse`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs` |
    /// | `cortex-r82ae` | `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `fp16`, `jsconv`, `lse`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs` |
    /// | `cortex-x1` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-x1c` | `aes`, `crc`, `dotprod`, `dpb`, `flagm`, `fp16`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-x2` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-x3` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-x4` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-x925` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `exynos-m3` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `exynos-m4` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `exynos-m5` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `falkor` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `rdm`, `sha2` |
    /// | `fujitsu-monaka` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `faminmax`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `fp8`, `fp8dot2`, `fp8dot4`, `fp8fma`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `lut`, `mops`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `gb10` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `generic` | `neon`, `outline-atomics` |
    /// | `grace` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `hip12` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `kryo` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `neoverse-512tvb` | `aes`, `bf16`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `fp16`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `neoverse-e1` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `neoverse-n1` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `neoverse-n2` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `neoverse-n3` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `neoverse-v1` | `aes`, `bf16`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `fp16`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `neoverse-v2` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `neoverse-v3` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `neoverse-v3ae` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `olympus` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `faminmax`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `fp8`, `fp8dot2`, `fp8dot4`, `fp8fma`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `lut`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `oryon-1` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `rigel` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `faminmax`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `fp8`, `fp8dot2`, `fp8dot4`, `fp8fma`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `lut`, `mte`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `saphira` | `aes`, `crc`, `dit`, `dotprod`, `dpb`, `fcma`, `flagm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `spe`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `thunderx` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `thunderx2t99` | `aes`, `crc`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `rdm`, `sha2`, `v8.1a`, `vh` |
    /// | `thunderx3t110` | `aes`, `crc`, `dpb`, `fcma`, `jsconv`, `lor`, `lse`, `neon`, `outline-atomics`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `v8.3a`, `vh` |
    /// | `thunderxt81` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `thunderxt83` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `thunderxt88` | `aes`, `crc`, `neon`, `outline-atomics`, `pmuv3`, `sha2` |
    /// | `tsv110` | `aes`, `crc`, `dotprod`, `dpb`, `fcma`, `fhm`, `fp16`, `jsconv`, `lor`, `lse`, `neon`, `outline-atomics`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `spe`, `v8.1a`, `v8.2a`, `vh` |
    pub mod cpus {}
}
/// arm64ec documentation
pub mod arm64ec {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `aes` | Enable AES support. | `neon` |
    /// | `bf16` | Enable BFloat16 Extension. |  |
    /// | `bti` | Enable Branch Target Identification. |  |
    /// | `crc` | Enable Armv8.0-A CRC-32 checksum instructions. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `cssc` | Enable Common Short Sequence Compression (CSSC) instructions. |  |
    /// | `dit` | Enable Armv8.4-A Data Independent Timing instructions. |  |
    /// | `dotprod` | Enable dot product support. | `neon` |
    /// | `dpb` | Enable Armv8.2-A data Cache Clean to Point of Persistence. |  |
    /// | `dpb2` | Enable Armv8.5-A Cache Clean to Point of Deep Persistence. | `dpb` |
    /// | `ecv` | Enable enhanced counter virtualization extension. |  |
    /// | `f32mm` | Enable Matrix Multiply FP32 Extension. | `fp16`, `neon`, `sve` |
    /// | `f64mm` | Enable Matrix Multiply FP64 Extension. | `fp16`, `neon`, `sve` |
    /// | `faminmax` | Enable FAMIN and FAMAX instructions. |  |
    /// | `fcma` | Enable Armv8.3-A Floating-point complex number support. | `neon` |
    /// | `fhm` | Enable FP16 FML instructions. | `fp16`, `neon` |
    /// | `flagm` | Enable Armv8.4-A Flag Manipulation instructions. |  |
    /// | `flagm2` | Enable alternative NZCV format for floating point comparisons. |  |
    /// | `fp16` | Enable half-precision floating-point data processing. | `neon` |
    /// | `fp8` | Enable FP8 instructions. | `bf16`, `faminmax`, `lut` |
    /// | `fp8dot2` | Enable FP8 2-way dot instructions. | `bf16`, `faminmax`, `fp8`, `fp8dot4`, `fp8fma`, `lut` |
    /// | `fp8dot4` | Enable FP8 4-way dot instructions. | `bf16`, `faminmax`, `fp8`, `fp8fma`, `lut` |
    /// | `fp8fma` | Enable Armv9.5-A FP8 multiply-add instructions. | `bf16`, `faminmax`, `fp8`, `lut` |
    /// | `frintts` | Enable FRInt\[32\|64\]\[Z\|X\] instructions that round a floating-point number to an integer (in FP format) forcing it to fit into a 32- or 64-bit int. |  |
    /// | `hbc` | Enable Armv8.8-A Hinted Conditional Branches Extension. |  |
    /// | `i8mm` | Enable Matrix Multiply Int8 Extension. |  |
    /// | `jsconv` | Enable Armv8.3-A JavaScript FP conversion instructions. | `neon` |
    /// | `lor` | Enable Armv8.1-A Limited Ordering Regions extension. |  |
    /// | `lse` | Enable Armv8.1-A Large System Extension (LSE) atomic instructions. |  |
    /// | `lse128` | Enable Armv9.4-A 128-bit Atomic instructions. | `lse` |
    /// | `lse2` | Enable Armv8.4-A Large System Extension 2 (LSE2) atomicity rules. |  |
    /// | `lut` | Enable Lookup Table instructions. |  |
    /// | `mops` | Enable Armv8.8-A memcpy and memset acceleration instructions. |  |
    /// | `mte` | Enable Memory Tagging Extension. |  |
    /// | `neon` | Enable Advanced SIMD instructions. |  |
    /// | `outline-atomics` | Enable out of line atomics to support LSE instructions. |  |
    /// | `paca` | Enable Armv8.3-A Pointer Authentication extension. |  |
    /// | `pacg` | Enable Armv8.3-A Pointer Authentication extension. |  |
    /// | `pan` | Enable Armv8.1-A Privileged Access-Never extension. |  |
    /// | `pauth-lr` | Enable Armv9.5-A PAC enhancements. |  |
    /// | `pmuv3` | Enable Armv8.0-A PMUv3 Performance Monitors extension. |  |
    /// | `rand` | Enable Random Number generation instructions. |  |
    /// | `ras` | Enable Armv8.0-A Reliability, Availability and Serviceability Extensions. |  |
    /// | `rcpc` | Enable support for RCPC extension. |  |
    /// | `rcpc2` | Enable Armv8.4-A RCPC instructions with Immediate Offsets. | `rcpc` |
    /// | `rcpc3` | Enable Armv8.9-A RCPC instructions for A64 and Advanced SIMD and floating-point instruction set. | `rcpc`, `rcpc2` |
    /// | `rdm` | Enable Armv8.1-A Rounding Double Multiply Add/Subtract instructions. | `neon` |
    /// | `sb` | Enable Armv8.5-A Speculation Barrier. |  |
    /// | `sha2` | Enable SHA1 and SHA256 support. | `neon` |
    /// | `sha3` | Enable SHA512 and SHA3 support. | `neon`, `sha2` |
    /// | `sm4` | Enable SM3 and SM4 support. | `neon` |
    /// | `sme` | Enable Scalable Matrix Extension (SME). | `bf16` |
    /// | `sme-b16b16` | Enable SME2.1 ZA-targeting non-widening BFloat16 instructions. | `bf16`, `sme`, `sme2`, `sve-b16b16` |
    /// | `sme-f16f16` | Enable SME non-widening Float16 instructions. | `bf16`, `sme`, `sme2` |
    /// | `sme-f64f64` | Enable Scalable Matrix Extension (SME) F64F64 instructions. | `bf16`, `sme` |
    /// | `sme-f8f16` | Enable Scalable Matrix Extension (SME) F8F16 instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme-f8f32`, `sme2` |
    /// | `sme-f8f32` | Enable Scalable Matrix Extension (SME) F8F32 instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme2` |
    /// | `sme-fa64` | Enable the full A64 instruction set in streaming SVE mode. | `bf16`, `fp16`, `neon`, `sme`, `sve`, `sve2` |
    /// | `sme-i16i64` | Enable Scalable Matrix Extension (SME) I16I64 instructions. | `bf16`, `sme` |
    /// | `sme-lutv2` | Enable Scalable Matrix Extension (SME) LUTv2 instructions. |  |
    /// | `sme2` | Enable Scalable Matrix Extension 2 (SME2) instructions. | `bf16`, `sme` |
    /// | `sme2p1` | Enable Scalable Matrix Extension 2.1 instructions. | `bf16`, `sme`, `sme2` |
    /// | `spe` | Enable Statistical Profiling extension. |  |
    /// | `ssbs` | Enable Speculative Store Bypass Safe bit. |  |
    /// | `ssve-fp8dot2` | Enable SVE2 FP8 2-way dot product instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme2`, `ssve-fp8dot4`, `ssve-fp8fma` |
    /// | `ssve-fp8dot4` | Enable SVE2 FP8 4-way dot product instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme2`, `ssve-fp8fma` |
    /// | `ssve-fp8fma` | Enable SVE2 FP8 multiply-add instructions. | `bf16`, `faminmax`, `fp8`, `lut`, `sme`, `sme2` |
    /// | `sve` | Enable Scalable Vector Extension (SVE) instructions. | `fp16`, `neon` |
    /// | `sve-b16b16` | Enable SVE2 non-widening and SME2 Z-targeting non-widening BFloat16 instructions. | `bf16` |
    /// | `sve2` | Enable Scalable Vector Extension 2 (SVE2) instructions. | `fp16`, `neon`, `sve` |
    /// | `sve2-aes` | Shorthand for +sve2+sve-aes. | `aes`, `fp16`, `neon`, `sve`, `sve2` |
    /// | `sve2-bitperm` | Shorthand for +sve2+sve-bitperm. | `fp16`, `neon`, `sve`, `sve2` |
    /// | `sve2-sha3` | Shorthand for +sve2+sve-sha3. | `fp16`, `neon`, `sha2`, `sha3`, `sve`, `sve2` |
    /// | `sve2-sm4` | Shorthand for +sve2+sve-sm4. | `fp16`, `neon`, `sm4`, `sve`, `sve2` |
    /// | `sve2p1` | Enable Scalable Vector Extension 2.1 instructions. | `fp16`, `neon`, `sve`, `sve2` |
    /// | `v8.1a` | Support ARM v8.1a architecture. | `crc`, `lor`, `lse`, `neon`, `pan`, `rdm`, `vh` |
    /// | `v8.2a` | Support ARM v8.2a architecture. | `crc`, `dpb`, `lor`, `lse`, `neon`, `pan`, `ras`, `rdm`, `v8.1a`, `vh` |
    /// | `v8.3a` | Support ARM v8.3a architecture. | `crc`, `dpb`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `v8.1a`, `v8.2a`, `vh` |
    /// | `v8.4a` | Support ARM v8.4a architecture. | `crc`, `dit`, `dotprod`, `dpb`, `flagm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `v8.1a`, `v8.2a`, `v8.3a`, `vh` |
    /// | `v8.5a` | Support ARM v8.5a architecture. | `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `v8.6a` | Support ARM v8.6a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `i8mm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `vh` |
    /// | `v8.7a` | Support ARM v8.7a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `i8mm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh`, `wfxt` |
    /// | `v8.8a` | Support ARM v8.8a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `v8.9a` | Support ARM v8.9a architecture. | `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `vh`, `wfxt` |
    /// | `v9.1a` | Support ARM v9.1a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `i8mm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v9a`, `vh` |
    /// | `v9.2a` | Support ARM v9.2a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `i8mm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9a`, `vh`, `wfxt` |
    /// | `v9.3a` | Support ARM v9.3a architecture. | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `v9.4a` | Support ARM v9.4a architecture. | `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v8.9a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `v9.5a` | Support ARM v9.5a architecture. | `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `mops`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v8.9a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9.4a`, `v9a`, `vh`, `wfxt` |
    /// | `v9a` | Support ARM v9a architecture. | `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `flagm`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `ras`, `rcpc`, `rdm`, `sb`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `vh` |
    /// | `vh` | Enable Armv8.1-A Virtual Host extension. |  |
    /// | `wfxt` | Enable Armv8.7-A WFET and WFIT instruction. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `a64fx` | `aes`, `crc`, `dpb`, `fcma`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `sve`, `v8.1a`, `v8.2a`, `vh` |
    /// | `ampere1` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `ampere1a` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `ampere1b` | `aes`, `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `ampere1c` | `aes`, `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `faminmax`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `fp8`, `fp8fma`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `lut`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `ssbs`, `sve`, `sve-b16b16`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `apple-a10` | `aes`, `crc`, `lor`, `neon`, `pan`, `pmuv3`, `rdm`, `sha2`, `vh` |
    /// | `apple-a11` | `aes`, `crc`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `apple-a12` | `aes`, `crc`, `dpb`, `fcma`, `fp16`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `v8.3a`, `vh` |
    /// | `apple-a13` | `aes`, `crc`, `dit`, `dotprod`, `dpb`, `fcma`, `fhm`, `flagm`, `fp16`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `sha3`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `apple-a14` | `aes`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `apple-a15` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `apple-a16` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `apple-a17` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `apple-a7` | `aes`, `neon`, `pmuv3`, `sha2` |
    /// | `apple-m4` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sme`, `sme-f64f64`, `sme-i16i64`, `sme2`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `apple-m5` | `aes`, `bf16`, `bti`, `crc`, `cssc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sme`, `sme-b16b16`, `sme-f16f16`, `sme-f64f64`, `sme-i16i64`, `sme2`, `sme2p1`, `ssbs`, `sve-b16b16`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `c1-nano` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mops`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sme`, `sme2`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `c1-premium` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mops`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sme`, `sme2`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `c1-pro` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mops`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sme`, `sme2`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `c1-ultra` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mops`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sme`, `sme2`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `carmel` | `aes`, `crc`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `ras`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a320` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a34` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `cortex-a35` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `cortex-a510` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-a520` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a520ae` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a53` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `cortex-a55` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a57` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `cortex-a65` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a65ae` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a710` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-a715` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-a72` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `cortex-a720` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a720ae` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a725` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-a73` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `cortex-a75` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a76` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a76ae` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a77` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a78` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a78ae` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-a78c` | `aes`, `crc`, `dotprod`, `dpb`, `flagm`, `fp16`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-r82` | `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `fp16`, `jsconv`, `lse`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs` |
    /// | `cortex-r82ae` | `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `fp16`, `jsconv`, `lse`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs` |
    /// | `cortex-x1` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-x1c` | `aes`, `crc`, `dotprod`, `dpb`, `flagm`, `fp16`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `cortex-x2` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-x3` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `cortex-x4` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `cortex-x925` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `exynos-m3` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `exynos-m4` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `exynos-m5` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `vh` |
    /// | `falkor` | `aes`, `crc`, `neon`, `pmuv3`, `rdm`, `sha2` |
    /// | `fujitsu-monaka` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `faminmax`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `fp8`, `fp8dot2`, `fp8dot4`, `fp8fma`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `lut`, `mops`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v8.8a`, `v9.1a`, `v9.2a`, `v9.3a`, `v9a`, `vh`, `wfxt` |
    /// | `gb10` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `generic` | `neon` |
    /// | `grace` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `hip12` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `hbc`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rcpc3`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `vh`, `wfxt` |
    /// | `kryo` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `neoverse-512tvb` | `aes`, `bf16`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `fp16`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `neoverse-e1` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `neoverse-n1` | `aes`, `crc`, `dotprod`, `dpb`, `fp16`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `vh` |
    /// | `neoverse-n2` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `neoverse-n3` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `neoverse-v1` | `aes`, `bf16`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `fp16`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `neoverse-v2` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v9a`, `vh` |
    /// | `neoverse-v3` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `neoverse-v3ae` | `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `olympus` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `faminmax`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `fp8`, `fp8dot2`, `fp8dot4`, `fp8fma`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `lut`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `oryon-1` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `vh` |
    /// | `rigel` | `aes`, `bf16`, `bti`, `crc`, `dit`, `dotprod`, `dpb`, `dpb2`, `ecv`, `faminmax`, `fcma`, `fhm`, `flagm`, `flagm2`, `fp16`, `fp8`, `fp8dot2`, `fp8dot4`, `fp8fma`, `frintts`, `i8mm`, `jsconv`, `lor`, `lse`, `lse2`, `lut`, `mte`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `rand`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sb`, `sha2`, `sha3`, `sm4`, `spe`, `ssbs`, `sve`, `sve2`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `v8.5a`, `v8.6a`, `v8.7a`, `v9.1a`, `v9.2a`, `v9a`, `vh`, `wfxt` |
    /// | `saphira` | `aes`, `crc`, `dit`, `dotprod`, `dpb`, `fcma`, `flagm`, `jsconv`, `lor`, `lse`, `lse2`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rcpc2`, `rdm`, `sha2`, `spe`, `v8.1a`, `v8.2a`, `v8.3a`, `v8.4a`, `vh` |
    /// | `thunderx` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `thunderx2t99` | `aes`, `crc`, `lor`, `lse`, `neon`, `pan`, `rdm`, `sha2`, `v8.1a`, `vh` |
    /// | `thunderx3t110` | `aes`, `crc`, `dpb`, `fcma`, `jsconv`, `lor`, `lse`, `neon`, `paca`, `pacg`, `pan`, `pmuv3`, `ras`, `rcpc`, `rdm`, `sha2`, `v8.1a`, `v8.2a`, `v8.3a`, `vh` |
    /// | `thunderxt81` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `thunderxt83` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `thunderxt88` | `aes`, `crc`, `neon`, `pmuv3`, `sha2` |
    /// | `tsv110` | `aes`, `crc`, `dotprod`, `dpb`, `fcma`, `fhm`, `fp16`, `jsconv`, `lor`, `lse`, `neon`, `pan`, `pmuv3`, `ras`, `rdm`, `sha2`, `spe`, `v8.1a`, `v8.2a`, `vh` |
    pub mod cpus {}
}
/// bpf documentation
pub mod bpf {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `allows-misaligned-mem-access` | Allows misaligned memory access. |  |
    /// | `alu32` | Enable ALU32 instructions. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `generic` |  |
    /// | `probe` |  |
    /// | `v1` |  |
    /// | `v2` |  |
    /// | `v3` | `alu32` |
    /// | `v4` | `alu32` |
    pub mod cpus {}
}
/// hexagon documentation
pub mod hexagon {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `audio` | Hexagon Audio extension instructions. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `hvx` | Hexagon HVX instructions. |  |
    /// | `hvx-ieee-fp` | Hexagon HVX IEEE floating point instructions. | `hvx` |
    /// | `hvx-length128b` | Hexagon HVX 128B instructions. | `hvx` |
    /// | `hvx-length64b` | Hexagon HVX 64B instructions. | `hvx` |
    /// | `hvx-qfloat` | Hexagon HVX QFloating point instructions. | `hvx` |
    /// | `hvxv60` | Hexagon HVX instructions. | `hvx` |
    /// | `hvxv62` | Hexagon HVX instructions. | `hvx`, `hvxv60` |
    /// | `hvxv65` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62` |
    /// | `hvxv66` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62`, `hvxv65`, `zreg` |
    /// | `hvxv67` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `zreg` |
    /// | `hvxv68` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `zreg` |
    /// | `hvxv69` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `zreg` |
    /// | `hvxv71` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `zreg` |
    /// | `hvxv73` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `zreg` |
    /// | `hvxv75` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `hvxv73`, `zreg` |
    /// | `hvxv79` | Hexagon HVX instructions. | `hvx`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `hvxv73`, `hvxv75`, `zreg` |
    /// | `v60` | Enable Hexagon V60 architecture. |  |
    /// | `v62` | Enable Hexagon V62 architecture. | `v60` |
    /// | `v65` | Enable Hexagon V65 architecture. | `v60`, `v62` |
    /// | `v66` | Enable Hexagon V66 architecture. | `v60`, `v62`, `v65` |
    /// | `v67` | Enable Hexagon V67 architecture. | `v60`, `v62`, `v65`, `v66` |
    /// | `v68` | Enable Hexagon V68 architecture. | `v60`, `v62`, `v65`, `v66`, `v67` |
    /// | `v69` | Enable Hexagon V69 architecture. | `v60`, `v62`, `v65`, `v66`, `v67`, `v68` |
    /// | `v71` | Enable Hexagon V71 architecture. | `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69` |
    /// | `v73` | Enable Hexagon V73 architecture. | `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71` |
    /// | `v75` | Enable Hexagon V75 architecture. | `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71`, `v73` |
    /// | `v79` | Enable Hexagon V79 architecture. | `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71`, `v73`, `v75` |
    /// | `zreg` | Hexagon ZReg extension instructions. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `generic` | `hvx`, `hvx-length128b`, `hvxv60`, `v60` |
    /// | `hexagonv5` | `hvx`, `hvx-length128b` |
    /// | `hexagonv55` | `hvx`, `hvx-length128b` |
    /// | `hexagonv60` | `hvx`, `hvx-length128b`, `hvxv60`, `v60` |
    /// | `hexagonv62` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `v60`, `v62` |
    /// | `hexagonv65` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `v60`, `v62`, `v65` |
    /// | `hexagonv66` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `v60`, `v62`, `v65`, `v66`, `zreg` |
    /// | `hexagonv67` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `v60`, `v62`, `v65`, `v66`, `v67`, `zreg` |
    /// | `hexagonv67t` | `audio`, `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `v60`, `v62`, `v65`, `v66`, `v67`, `zreg` |
    /// | `hexagonv68` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `zreg` |
    /// | `hexagonv69` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `zreg` |
    /// | `hexagonv71` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71`, `zreg` |
    /// | `hexagonv71t` | `audio`, `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71`, `zreg` |
    /// | `hexagonv73` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `hvxv73`, `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71`, `v73`, `zreg` |
    /// | `hexagonv75` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `hvxv73`, `hvxv75`, `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71`, `v73`, `v75`, `zreg` |
    /// | `hexagonv79` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `hvxv73`, `hvxv75`, `hvxv79`, `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71`, `v73`, `v75`, `v79`, `zreg` |
    /// | `hexagonv81` | `hvx`, `hvx-length128b`, `hvxv60`, `hvxv62`, `hvxv65`, `hvxv66`, `hvxv67`, `hvxv68`, `hvxv69`, `hvxv71`, `hvxv73`, `hvxv75`, `hvxv79`, `v60`, `v62`, `v65`, `v66`, `v67`, `v68`, `v69`, `v71`, `v73`, `v75`, `v79`, `zreg` |
    pub mod cpus {}
}
/// mips documentation
pub mod mips {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `fp64` | Support 64-bit FP registers. |  |
    /// | `msa` | Mips MSA ASE. |  |
    /// | `virt` | Mips Virtualization ASE. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `generic` |  |
    /// | `i6400` | `fp64`, `msa` |
    /// | `i6500` | `fp64`, `msa` |
    /// | `mips1` |  |
    /// | `mips2` |  |
    /// | `mips3` | `fp64` |
    /// | `mips32` |  |
    /// | `mips32r2` |  |
    /// | `mips32r3` |  |
    /// | `mips32r5` |  |
    /// | `mips32r6` | `fp64` |
    /// | `mips4` | `fp64` |
    /// | `mips64` | `fp64` |
    /// | `mips64r2` | `fp64` |
    /// | `mips64r3` | `fp64` |
    /// | `mips64r5` | `fp64` |
    /// | `mips64r6` | `fp64` |
    /// | `octeon` | `fp64` |
    /// | `octeon+` | `fp64` |
    /// | `p5600` |  |
    /// | `r5900` | `fp64` |
    pub mod cpus {}
}
/// mips64 documentation
pub mod mips64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `fp64` | Support 64-bit FP registers. |  |
    /// | `msa` | Mips MSA ASE. |  |
    /// | `virt` | Mips Virtualization ASE. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `generic` | `fp64` |
    /// | `i6400` | `fp64`, `msa` |
    /// | `i6500` | `fp64`, `msa` |
    /// | `mips1` | `fp64` |
    /// | `mips2` | `fp64` |
    /// | `mips3` | `fp64` |
    /// | `mips32` | `fp64` |
    /// | `mips32r2` | `fp64` |
    /// | `mips32r3` | `fp64` |
    /// | `mips32r5` | `fp64` |
    /// | `mips32r6` | `fp64` |
    /// | `mips4` | `fp64` |
    /// | `mips64` | `fp64` |
    /// | `mips64r2` | `fp64` |
    /// | `mips64r3` | `fp64` |
    /// | `mips64r5` | `fp64` |
    /// | `mips64r6` | `fp64` |
    /// | `octeon` | `fp64` |
    /// | `octeon+` | `fp64` |
    /// | `p5600` | `fp64` |
    /// | `r5900` | `fp64` |
    pub mod cpus {}
}
/// loongarch32 documentation
pub mod loongarch32 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `32s` | LA32 Standard Basic Instruction Extension. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `d` | 'D' (Double-Precision Floating-Point). | `f` |
    /// | `div32` | Assume div.w\[u\] and mod.w\[u\] can handle inputs that are not sign-extended. |  |
    /// | `f` | 'F' (Single-Precision Floating-Point). |  |
    /// | `frecipe` | Support frecipe.{s/d} and frsqrte.{s/d} instructions. |  |
    /// | `lam-bh` | Support amswap\[_db\].{b/h} and amadd\[_db\].{b/h} instructions. |  |
    /// | `lamcas` | Support amcas\[_db\].{b/h/w/d}. |  |
    /// | `lasx` | 'LASX' (Loongson Advanced SIMD Extension). | `d`, `f`, `lsx` |
    /// | `lbt` | 'LBT' (Loongson Binary Translation Extension). |  |
    /// | `ld-seq-sa` | Don't use a same-address load-load barrier (dbar 0x700). |  |
    /// | `lsx` | 'LSX' (Loongson SIMD Extension). | `d`, `f` |
    /// | `lvz` | 'LVZ' (Loongson Virtualization Extension). |  |
    /// | `relax` | Enable Linker relaxation. |  |
    /// | `scq` | Support sc.q instruction. |  |
    /// | `ual` | Allow memory accesses to be unaligned. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `generic` | `d`, `f` |
    /// | `generic-la32` | `d`, `f` |
    /// | `generic-la64` | `32s`, `d`, `f`, `lsx`, `ual` |
    /// | `la464` | `32s`, `d`, `f`, `lasx`, `lbt`, `lsx`, `lvz`, `ual` |
    /// | `la664` | `32s`, `d`, `div32`, `f`, `frecipe`, `lam-bh`, `lamcas`, `lasx`, `lbt`, `ld-seq-sa`, `lsx`, `lvz`, `scq`, `ual` |
    /// | `loongarch32` | `d`, `f` |
    /// | `loongarch64` | `32s`, `d`, `f`, `ual` |
    pub mod cpus {}
}
/// loongarch64 documentation
pub mod loongarch64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `32s` | LA32 Standard Basic Instruction Extension. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `d` | 'D' (Double-Precision Floating-Point). | `f` |
    /// | `div32` | Assume div.w\[u\] and mod.w\[u\] can handle inputs that are not sign-extended. |  |
    /// | `f` | 'F' (Single-Precision Floating-Point). |  |
    /// | `frecipe` | Support frecipe.{s/d} and frsqrte.{s/d} instructions. |  |
    /// | `lam-bh` | Support amswap\[_db\].{b/h} and amadd\[_db\].{b/h} instructions. |  |
    /// | `lamcas` | Support amcas\[_db\].{b/h/w/d}. |  |
    /// | `lasx` | 'LASX' (Loongson Advanced SIMD Extension). | `d`, `f`, `lsx` |
    /// | `lbt` | 'LBT' (Loongson Binary Translation Extension). |  |
    /// | `ld-seq-sa` | Don't use a same-address load-load barrier (dbar 0x700). |  |
    /// | `lsx` | 'LSX' (Loongson SIMD Extension). | `d`, `f` |
    /// | `lvz` | 'LVZ' (Loongson Virtualization Extension). |  |
    /// | `relax` | Enable Linker relaxation. |  |
    /// | `scq` | Support sc.q instruction. |  |
    /// | `ual` | Allow memory accesses to be unaligned. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `generic` | `32s`, `d`, `f`, `lsx`, `relax`, `ual` |
    /// | `generic-la32` | `d`, `f`, `lsx`, `relax` |
    /// | `generic-la64` | `32s`, `d`, `f`, `lsx`, `relax`, `ual` |
    /// | `la464` | `32s`, `d`, `f`, `lasx`, `lbt`, `lsx`, `lvz`, `relax`, `ual` |
    /// | `la664` | `32s`, `d`, `div32`, `f`, `frecipe`, `lam-bh`, `lamcas`, `lasx`, `lbt`, `ld-seq-sa`, `lsx`, `lvz`, `relax`, `scq`, `ual` |
    /// | `loongarch32` | `d`, `f`, `lsx`, `relax` |
    /// | `loongarch64` | `32s`, `d`, `f`, `lsx`, `relax`, `ual` |
    pub mod cpus {}
}
/// nvptx64 documentation
pub mod nvptx64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `ptx70` | Use PTX version 70. |  |
    /// | `ptx71` | Use PTX version 71. | `ptx70` |
    /// | `ptx72` | Use PTX version 72. | `ptx70`, `ptx71` |
    /// | `ptx73` | Use PTX version 73. | `ptx70`, `ptx71`, `ptx72` |
    /// | `ptx74` | Use PTX version 74. | `ptx70`, `ptx71`, `ptx72`, `ptx73` |
    /// | `ptx75` | Use PTX version 75. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74` |
    /// | `ptx76` | Use PTX version 76. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75` |
    /// | `ptx77` | Use PTX version 77. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76` |
    /// | `ptx78` | Use PTX version 78. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77` |
    /// | `ptx80` | Use PTX version 80. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77`, `ptx78` |
    /// | `ptx81` | Use PTX version 81. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77`, `ptx78`, `ptx80` |
    /// | `ptx82` | Use PTX version 82. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77`, `ptx78`, `ptx80`, `ptx81` |
    /// | `ptx83` | Use PTX version 83. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77`, `ptx78`, `ptx80`, `ptx81`, `ptx82` |
    /// | `ptx84` | Use PTX version 84. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77`, `ptx78`, `ptx80`, `ptx81`, `ptx82`, `ptx83` |
    /// | `ptx85` | Use PTX version 85. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77`, `ptx78`, `ptx80`, `ptx81`, `ptx82`, `ptx83`, `ptx84` |
    /// | `ptx86` | Use PTX version 86. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77`, `ptx78`, `ptx80`, `ptx81`, `ptx82`, `ptx83`, `ptx84`, `ptx85` |
    /// | `ptx87` | Use PTX version 87. | `ptx70`, `ptx71`, `ptx72`, `ptx73`, `ptx74`, `ptx75`, `ptx76`, `ptx77`, `ptx78`, `ptx80`, `ptx81`, `ptx82`, `ptx83`, `ptx84`, `ptx85`, `ptx86` |
    /// | `sm_100` | Target SM 100. | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_100a` | Target SM 100a. | `sm_100`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_101` | Target SM 101. | `sm_100`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_101a` | Target SM 101a. | `sm_100`, `sm_101`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_120` | Target SM 120. | `sm_100`, `sm_101`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_120a` | Target SM 120a. | `sm_100`, `sm_101`, `sm_120`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_70` | Target SM 70. |  |
    /// | `sm_72` | Target SM 72. | `sm_70` |
    /// | `sm_75` | Target SM 75. | `sm_70`, `sm_72` |
    /// | `sm_80` | Target SM 80. | `sm_70`, `sm_72`, `sm_75` |
    /// | `sm_86` | Target SM 86. | `sm_70`, `sm_72`, `sm_75`, `sm_80` |
    /// | `sm_87` | Target SM 87. | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86` |
    /// | `sm_89` | Target SM 89. | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87` |
    /// | `sm_90` | Target SM 90. | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89` |
    /// | `sm_90a` | Target SM 90a. | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `sm_100` | `sm_100`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_100a` | `sm_100`, `sm_100a`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_100f` |  |
    /// | `sm_101` | `sm_100`, `sm_101`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_101a` | `sm_100`, `sm_101`, `sm_101a`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_101f` |  |
    /// | `sm_103` |  |
    /// | `sm_103a` |  |
    /// | `sm_103f` |  |
    /// | `sm_110` |  |
    /// | `sm_110a` |  |
    /// | `sm_110f` |  |
    /// | `sm_120` | `sm_100`, `sm_101`, `sm_120`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_120a` | `sm_100`, `sm_101`, `sm_120`, `sm_120a`, `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_120f` |  |
    /// | `sm_121` |  |
    /// | `sm_121a` |  |
    /// | `sm_121f` |  |
    /// | `sm_70` | `ptx70`, `sm_70` |
    /// | `sm_72` | `ptx70`, `sm_70`, `sm_72` |
    /// | `sm_75` | `ptx70`, `sm_70`, `sm_72`, `sm_75` |
    /// | `sm_80` | `sm_70`, `sm_72`, `sm_75`, `sm_80` |
    /// | `sm_86` | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86` |
    /// | `sm_87` | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87` |
    /// | `sm_88` |  |
    /// | `sm_89` | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89` |
    /// | `sm_90` | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90` |
    /// | `sm_90a` | `sm_70`, `sm_72`, `sm_75`, `sm_80`, `sm_86`, `sm_87`, `sm_89`, `sm_90`, `sm_90a` |
    pub mod cpus {}
}
/// powerpc documentation
pub mod powerpc {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `altivec` | Enable Altivec instructions. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `msync` | Has only the msync instruction instead of sync. |  |
    /// | `partword-atomics` | Enable l\[bh\]arx and st\[bh\]cx.. |  |
    /// | `power10-vector` | Enable POWER10 vector instructions. | `altivec`, `power8-altivec`, `power8-vector`, `power9-altivec`, `power9-vector`, `vsx` |
    /// | `power8-altivec` | Enable POWER8 Altivec instructions. | `altivec` |
    /// | `power8-crypto` | Enable POWER8 Crypto instructions. | `altivec`, `power8-altivec` |
    /// | `power8-vector` | Enable POWER8 vector instructions. | `altivec`, `power8-altivec`, `vsx` |
    /// | `power9-altivec` | Enable POWER9 Altivec instructions. | `altivec`, `power8-altivec` |
    /// | `power9-vector` | Enable POWER9 vector instructions. | `altivec`, `power8-altivec`, `power8-vector`, `power9-altivec`, `vsx` |
    /// | `quadword-atomics` | Enable lqarx and stqcx.. |  |
    /// | `vsx` | Enable VSX instructions. | `altivec` |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `440` | `msync` |
    /// | `450` | `msync` |
    /// | `601` |  |
    /// | `602` |  |
    /// | `603` |  |
    /// | `603e` |  |
    /// | `603ev` |  |
    /// | `604` |  |
    /// | `604e` |  |
    /// | `620` |  |
    /// | `7400` | `altivec` |
    /// | `7450` | `altivec` |
    /// | `750` |  |
    /// | `970` | `altivec` |
    /// | `a2` |  |
    /// | `e500` | `msync` |
    /// | `e500mc` |  |
    /// | `e5500` |  |
    /// | `future` | `altivec`, `partword-atomics`, `power10-vector`, `power8-altivec`, `power8-crypto`, `power8-vector`, `power9-altivec`, `power9-vector`, `quadword-atomics`, `vsx` |
    /// | `g3` |  |
    /// | `g4` | `altivec` |
    /// | `g4+` | `altivec` |
    /// | `g5` | `altivec` |
    /// | `generic` |  |
    /// | `ppc` |  |
    /// | `ppc32` |  |
    /// | `ppc64` | `altivec` |
    /// | `ppc64le` | `altivec`, `partword-atomics`, `power8-altivec`, `power8-crypto`, `power8-vector`, `quadword-atomics`, `vsx` |
    /// | `pwr10` | `altivec`, `partword-atomics`, `power10-vector`, `power8-altivec`, `power8-crypto`, `power8-vector`, `power9-altivec`, `power9-vector`, `quadword-atomics`, `vsx` |
    /// | `pwr11` | `altivec`, `partword-atomics`, `power10-vector`, `power8-altivec`, `power8-crypto`, `power8-vector`, `power9-altivec`, `power9-vector`, `quadword-atomics`, `vsx` |
    /// | `pwr3` | `altivec` |
    /// | `pwr4` | `altivec` |
    /// | `pwr5` | `altivec` |
    /// | `pwr5x` | `altivec` |
    /// | `pwr6` | `altivec` |
    /// | `pwr6x` | `altivec` |
    /// | `pwr7` | `altivec`, `vsx` |
    /// | `pwr8` | `altivec`, `partword-atomics`, `power8-altivec`, `power8-crypto`, `power8-vector`, `quadword-atomics`, `vsx` |
    /// | `pwr9` | `altivec`, `partword-atomics`, `power8-altivec`, `power8-crypto`, `power8-vector`, `power9-altivec`, `power9-vector`, `quadword-atomics`, `vsx` |
    pub mod cpus {}
}
/// powerpc64 documentation
pub mod powerpc64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `altivec` | Enable Altivec instructions. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `msync` | Has only the msync instruction instead of sync. |  |
    /// | `partword-atomics` | Enable l\[bh\]arx and st\[bh\]cx.. |  |
    /// | `power10-vector` | Enable POWER10 vector instructions. | `altivec`, `power8-altivec`, `power8-vector`, `power9-altivec`, `power9-vector`, `vsx` |
    /// | `power8-altivec` | Enable POWER8 Altivec instructions. | `altivec` |
    /// | `power8-crypto` | Enable POWER8 Crypto instructions. | `altivec`, `power8-altivec` |
    /// | `power8-vector` | Enable POWER8 vector instructions. | `altivec`, `power8-altivec`, `vsx` |
    /// | `power9-altivec` | Enable POWER9 Altivec instructions. | `altivec`, `power8-altivec` |
    /// | `power9-vector` | Enable POWER9 vector instructions. | `altivec`, `power8-altivec`, `power8-vector`, `power9-altivec`, `vsx` |
    /// | `quadword-atomics` | Enable lqarx and stqcx.. |  |
    /// | `vsx` | Enable VSX instructions. | `altivec` |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `440` | `msync` |
    /// | `450` | `msync` |
    /// | `601` |  |
    /// | `602` |  |
    /// | `603` |  |
    /// | `603e` |  |
    /// | `603ev` |  |
    /// | `604` |  |
    /// | `604e` |  |
    /// | `620` |  |
    /// | `7400` | `altivec` |
    /// | `7450` | `altivec` |
    /// | `750` |  |
    /// | `970` | `altivec` |
    /// | `a2` |  |
    /// | `e500` | `msync` |
    /// | `e500mc` |  |
    /// | `e5500` |  |
    /// | `future` | `altivec`, `partword-atomics`, `power10-vector`, `power8-altivec`, `power8-crypto`, `power8-vector`, `power9-altivec`, `power9-vector`, `quadword-atomics`, `vsx` |
    /// | `g3` |  |
    /// | `g4` | `altivec` |
    /// | `g4+` | `altivec` |
    /// | `g5` | `altivec` |
    /// | `generic` |  |
    /// | `ppc` |  |
    /// | `ppc32` |  |
    /// | `ppc64` | `altivec` |
    /// | `ppc64le` | `altivec`, `partword-atomics`, `power8-altivec`, `power8-crypto`, `power8-vector`, `quadword-atomics`, `vsx` |
    /// | `pwr10` | `altivec`, `partword-atomics`, `power10-vector`, `power8-altivec`, `power8-crypto`, `power8-vector`, `power9-altivec`, `power9-vector`, `quadword-atomics`, `vsx` |
    /// | `pwr11` | `altivec`, `partword-atomics`, `power10-vector`, `power8-altivec`, `power8-crypto`, `power8-vector`, `power9-altivec`, `power9-vector`, `quadword-atomics`, `vsx` |
    /// | `pwr3` | `altivec` |
    /// | `pwr4` | `altivec` |
    /// | `pwr5` | `altivec` |
    /// | `pwr5x` | `altivec` |
    /// | `pwr6` | `altivec` |
    /// | `pwr6x` | `altivec` |
    /// | `pwr7` | `altivec`, `vsx` |
    /// | `pwr8` | `altivec`, `partword-atomics`, `power8-altivec`, `power8-crypto`, `power8-vector`, `quadword-atomics`, `vsx` |
    /// | `pwr9` | `altivec`, `partword-atomics`, `power8-altivec`, `power8-crypto`, `power8-vector`, `power9-altivec`, `power9-vector`, `quadword-atomics`, `vsx` |
    pub mod cpus {}
}
/// riscv32 documentation
pub mod riscv32 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `a` | 'A' (Atomic Instructions). | `zaamo`, `zalrsc` |
    /// | `b` | 'B' (the collection of the Zba, Zbb, Zbs extensions). | `zba`, `zbb`, `zbs` |
    /// | `c` | 'C' (Compressed Instructions). | `zca` |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `d` | 'D' (Double-Precision Floating-Point). | `f`, `zicsr` |
    /// | `e` | 'E' (Embedded Instruction Set with 16 GPRs). |  |
    /// | `f` | 'F' (Single-Precision Floating-Point). | `zicsr` |
    /// | `m` | 'M' (Integer Multiplication and Division). |  |
    /// | `relax` | Enable Linker relaxation. |  |
    /// | `rva23u64` | RISC-V rva23u64 profile. | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfhmin`, `zvkb`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `supm` | 'Supm' (Indicates User-mode Pointer Masking). |  |
    /// | `unaligned-scalar-mem` | Has reasonably performant unaligned scalar loads and stores. |  |
    /// | `unaligned-vector-mem` | Has reasonably performant unaligned vector loads and stores. |  |
    /// | `v` | 'V' (Vector Extension for Application Processors). | `d`, `f`, `zicsr`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `za128rs` | 'Za128rs' (Reservation Set Size of at Most 128 Bytes). |  |
    /// | `za64rs` | 'Za64rs' (Reservation Set Size of at Most 64 Bytes). | `za128rs` |
    /// | `zaamo` | 'Zaamo' (Atomic Memory Operations). |  |
    /// | `zabha` | 'Zabha' (Byte and Halfword Atomic Memory Operations). | `zaamo` |
    /// | `zacas` | 'Zacas' (Atomic Compare-And-Swap Instructions). | `zaamo` |
    /// | `zalrsc` | 'Zalrsc' (Load-Reserved/Store-Conditional). |  |
    /// | `zama16b` | 'Zama16b' (Atomic 16-byte misaligned loads, stores and AMOs). |  |
    /// | `zawrs` | 'Zawrs' (Wait on Reservation Set). |  |
    /// | `zba` | 'Zba' (Address Generation Instructions). |  |
    /// | `zbb` | 'Zbb' (Basic Bit-Manipulation). |  |
    /// | `zbc` | 'Zbc' (Carry-Less Multiplication). | `zbkc` |
    /// | `zbkb` | 'Zbkb' (Bitmanip instructions for Cryptography). |  |
    /// | `zbkc` | 'Zbkc' (Carry-less multiply instructions for Cryptography). |  |
    /// | `zbkx` | 'Zbkx' (Crossbar permutation instructions). |  |
    /// | `zbs` | 'Zbs' (Single-Bit Instructions). |  |
    /// | `zca` | 'Zca' (part of the C extension, excluding compressed floating point loads/stores). |  |
    /// | `zcb` | 'Zcb' (Compressed basic bit manipulation instructions). | `zca` |
    /// | `zcmop` | 'Zcmop' (Compressed May-Be-Operations). | `zca` |
    /// | `zdinx` | 'Zdinx' (Double in Integer). | `zfinx`, `zicsr` |
    /// | `zfa` | 'Zfa' (Additional Floating-Point). | `f`, `zicsr` |
    /// | `zfbfmin` | 'Zfbfmin' (Scalar BF16 Converts). | `f`, `zicsr` |
    /// | `zfh` | 'Zfh' (Half-Precision Floating-Point). | `f`, `zfhmin`, `zicsr` |
    /// | `zfhmin` | 'Zfhmin' (Half-Precision Floating-Point Minimal). | `f`, `zicsr` |
    /// | `zfinx` | 'Zfinx' (Float in Integer). | `zicsr` |
    /// | `zhinx` | 'Zhinx' (Half Float in Integer). | `zfinx`, `zhinxmin`, `zicsr` |
    /// | `zhinxmin` | 'Zhinxmin' (Half Float in Integer Minimal). | `zfinx`, `zicsr` |
    /// | `zic64b` | 'Zic64b' (Cache Block Size Is 64 Bytes). |  |
    /// | `zicbom` | 'Zicbom' (Cache-Block Management Instructions). |  |
    /// | `zicbop` | 'Zicbop' (Cache-Block Prefetch Instructions). |  |
    /// | `zicboz` | 'Zicboz' (Cache-Block Zero Instructions). |  |
    /// | `ziccamoa` | 'Ziccamoa' (Main Memory Supports All Atomics in A). |  |
    /// | `ziccif` | 'Ziccif' (Main Memory Supports Instruction Fetch with Atomicity Requirement). |  |
    /// | `zicclsm` | 'Zicclsm' (Main Memory Supports Misaligned Loads/Stores). |  |
    /// | `ziccrse` | 'Ziccrse' (Main Memory Supports Forward Progress on LR/SC Sequences). |  |
    /// | `zicntr` | 'Zicntr' (Base Counters and Timers). | `zicsr` |
    /// | `zicond` | 'Zicond' (Integer Conditional Operations). |  |
    /// | `zicsr` | 'Zicsr' (CSRs). |  |
    /// | `zifencei` | 'Zifencei' (fence.i). |  |
    /// | `zihintntl` | 'Zihintntl' (Non-Temporal Locality Hints). |  |
    /// | `zihintpause` | 'Zihintpause' (Pause Hint). |  |
    /// | `zihpm` | 'Zihpm' (Hardware Performance Counters). | `zicsr` |
    /// | `zimop` | 'Zimop' (May-Be-Operations). |  |
    /// | `zk` | 'Zk' (Standard scalar cryptography extension). | `zbkb`, `zbkc`, `zbkx`, `zkn`, `zknd`, `zkne`, `zknh`, `zkr`, `zkt` |
    /// | `zkn` | 'Zkn' (NIST Algorithm Suite). | `zbkb`, `zbkc`, `zbkx`, `zknd`, `zkne`, `zknh` |
    /// | `zknd` | 'Zknd' (NIST Suite: AES Decryption). |  |
    /// | `zkne` | 'Zkne' (NIST Suite: AES Encryption). |  |
    /// | `zknh` | 'Zknh' (NIST Suite: Hash Function Instructions). |  |
    /// | `zkr` | 'Zkr' (Entropy Source Extension). |  |
    /// | `zks` | 'Zks' (ShangMi Algorithm Suite). | `zbkb`, `zbkc`, `zbkx`, `zksed`, `zksh` |
    /// | `zksed` | 'Zksed' (ShangMi Suite: SM4 Block Cipher Instructions). |  |
    /// | `zksh` | 'Zksh' (ShangMi Suite: SM3 Hash Function Instructions). |  |
    /// | `zkt` | 'Zkt' (Data Independent Execution Latency). |  |
    /// | `ztso` | 'Ztso' (Memory Model |  |
    /// | `zvbb` | 'Zvbb' (Vector basic bit-manipulation instructions). | `zicsr`, `zve32x`, `zvkb`, `zvl32b` |
    /// | `zvbc` | 'Zvbc' (Vector Carryless Multiplication). | `zicsr`, `zve32x`, `zve64x`, `zvl32b`, `zvl64b` |
    /// | `zve32f` | 'Zve32f' (Vector Extensions for Embedded Processors with maximal 32 EEW and F extension). | `f`, `zicsr`, `zve32x`, `zvl32b` |
    /// | `zve32x` | 'Zve32x' (Vector Extensions for Embedded Processors with maximal 32 EEW). | `zicsr`, `zvl32b` |
    /// | `zve64d` | 'Zve64d' (Vector Extensions for Embedded Processors with maximal 64 EEW, F and D extension). | `d`, `f`, `zicsr`, `zve32f`, `zve32x`, `zve64f`, `zve64x`, `zvl32b`, `zvl64b` |
    /// | `zve64f` | 'Zve64f' (Vector Extensions for Embedded Processors with maximal 64 EEW and F extension). | `f`, `zicsr`, `zve32f`, `zve32x`, `zve64x`, `zvl32b`, `zvl64b` |
    /// | `zve64x` | 'Zve64x' (Vector Extensions for Embedded Processors with maximal 64 EEW). | `zicsr`, `zve32x`, `zvl32b`, `zvl64b` |
    /// | `zvfbfmin` | 'Zvfbfmin' (Vector BF16 Converts). | `f`, `zicsr`, `zve32f`, `zve32x`, `zvl32b` |
    /// | `zvfbfwma` | 'Zvfbfwma' (Vector BF16 widening mul-add). | `f`, `zfbfmin`, `zicsr`, `zve32f`, `zve32x`, `zvfbfmin`, `zvl32b` |
    /// | `zvfh` | 'Zvfh' (Vector Half-Precision Floating-Point). | `f`, `zfhmin`, `zicsr`, `zve32f`, `zve32x`, `zvfhmin`, `zvl32b` |
    /// | `zvfhmin` | 'Zvfhmin' (Vector Half-Precision Floating-Point Minimal). | `f`, `zicsr`, `zve32f`, `zve32x`, `zvl32b` |
    /// | `zvkb` | 'Zvkb' (Vector Bit-manipulation used in Cryptography). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvkg` | 'Zvkg' (Vector GCM instructions for Cryptography). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvkn` | 'Zvkn' (shorthand for 'Zvkned', 'Zvknhb', 'Zvkb', and 'Zvkt'). | `zicsr`, `zve32x`, `zve64x`, `zvkb`, `zvkned`, `zvknha`, `zvknhb`, `zvkt`, `zvl32b`, `zvl64b` |
    /// | `zvknc` | 'Zvknc' (shorthand for 'Zvknc' and 'Zvbc'). | `zicsr`, `zvbc`, `zve32x`, `zve64x`, `zvkb`, `zvkn`, `zvkned`, `zvknha`, `zvknhb`, `zvkt`, `zvl32b`, `zvl64b` |
    /// | `zvkned` | 'Zvkned' (Vector AES Encryption & Decryption (Single Round)). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvkng` | 'Zvkng' (shorthand for 'Zvkn' and 'Zvkg'). | `zicsr`, `zve32x`, `zve64x`, `zvkb`, `zvkg`, `zvkn`, `zvkned`, `zvknha`, `zvknhb`, `zvkt`, `zvl32b`, `zvl64b` |
    /// | `zvknha` | 'Zvknha' (Vector SHA-2 (SHA-256 only)). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvknhb` | 'Zvknhb' (Vector SHA-2 (SHA-256 and SHA-512)). | `zicsr`, `zve32x`, `zve64x`, `zvknha`, `zvl32b`, `zvl64b` |
    /// | `zvks` | 'Zvks' (shorthand for 'Zvksed', 'Zvksh', 'Zvkb', and 'Zvkt'). | `zicsr`, `zve32x`, `zvkb`, `zvksed`, `zvksh`, `zvkt`, `zvl32b` |
    /// | `zvksc` | 'Zvksc' (shorthand for 'Zvks' and 'Zvbc'). | `zicsr`, `zvbc`, `zve32x`, `zve64x`, `zvkb`, `zvks`, `zvksed`, `zvksh`, `zvkt`, `zvl32b`, `zvl64b` |
    /// | `zvksed` | 'Zvksed' (SM4 Block Cipher Instructions). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvksg` | 'Zvksg' (shorthand for 'Zvks' and 'Zvkg'). | `zicsr`, `zve32x`, `zvkb`, `zvkg`, `zvks`, `zvksed`, `zvksh`, `zvkt`, `zvl32b` |
    /// | `zvksh` | 'Zvksh' (SM3 Hash Function Instructions). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvkt` | 'Zvkt' (Vector Data-Independent Execution Latency). |  |
    /// | `zvl1024b` | 'Zvl1024b' (Minimum Vector Length 1024). | `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `zvl128b` | 'Zvl128b' (Minimum Vector Length 128). | `zvl32b`, `zvl64b` |
    /// | `zvl16384b` | 'Zvl16384b' (Minimum Vector Length 16384). | `zvl1024b`, `zvl128b`, `zvl2048b`, `zvl256b`, `zvl32b`, `zvl4096b`, `zvl512b`, `zvl64b`, `zvl8192b` |
    /// | `zvl2048b` | 'Zvl2048b' (Minimum Vector Length 2048). | `zvl1024b`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `zvl256b` | 'Zvl256b' (Minimum Vector Length 256). | `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `zvl32768b` | 'Zvl32768b' (Minimum Vector Length 32768). | `zvl1024b`, `zvl128b`, `zvl16384b`, `zvl2048b`, `zvl256b`, `zvl32b`, `zvl4096b`, `zvl512b`, `zvl64b`, `zvl8192b` |
    /// | `zvl32b` | 'Zvl32b' (Minimum Vector Length 32). |  |
    /// | `zvl4096b` | 'Zvl4096b' (Minimum Vector Length 4096). | `zvl1024b`, `zvl128b`, `zvl2048b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `zvl512b` | 'Zvl512b' (Minimum Vector Length 512). | `zvl128b`, `zvl256b`, `zvl32b`, `zvl64b` |
    /// | `zvl64b` | 'Zvl64b' (Minimum Vector Length 64). | `zvl32b` |
    /// | `zvl65536b` | 'Zvl65536b' (Minimum Vector Length 65536). | `zvl1024b`, `zvl128b`, `zvl16384b`, `zvl2048b`, `zvl256b`, `zvl32768b`, `zvl32b`, `zvl4096b`, `zvl512b`, `zvl64b`, `zvl8192b` |
    /// | `zvl8192b` | 'Zvl8192b' (Minimum Vector Length 8192). | `zvl1024b`, `zvl128b`, `zvl2048b`, `zvl256b`, `zvl32b`, `zvl4096b`, `zvl512b`, `zvl64b` |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `an-erbium` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-45-series` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-a25` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-a45` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-ax25` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-ax45` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-ax45mpv` | `a`, `c`, `d`, `f`, `m`, `v`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `andes-n45` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-nx45` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `et-soc1` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `generic` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `generic-ooo` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `generic-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `generic-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `mips-p8700` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zca`, `zicsr`, `zifencei` |
    /// | `rocket` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `rocket-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `rocket-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `rp2350-hazard3` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbkb`, `zbs`, `zca`, `zcb`, `zicsr`, `zifencei` |
    /// | `sifive-7-series` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e20` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e21` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e24` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e31` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e34` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e76` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-p450` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbs`, `zca`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zkt` |
    /// | `sifive-p470` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbs`, `zca`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvkb`, `zvkg`, `zvkn`, `zvknc`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-p550` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-p670` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbs`, `zca`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvkb`, `zvkg`, `zvkn`, `zvknc`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-p870-d` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zama16b`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkr`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkg`, `zvkn`, `zvknc`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-s21` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-s51` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-s54` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-s76` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei`, `zihintpause` |
    /// | `sifive-u54` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-u74` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-x160` | `a`, `b`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zicbom`, `zicbop`, `zicboz`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-x180` | `a`, `b`, `c`, `d`, `f`, `m`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `ziccrse`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-x280` | `a`, `c`, `d`, `f`, `m`, `v`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zca`, `zfh`, `zfhmin`, `zicsr`, `zifencei`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfh`, `zvfhmin`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `sifive-x390` | `a`, `b`, `c`, `d`, `f`, `m`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkr`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkt`, `zvl1024b`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `spacemit-a100` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `unaligned-scalar-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkg`, `zvkn`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl1024b`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `spacemit-x100` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `unaligned-scalar-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkg`, `zvkn`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl64b` |
    /// | `spacemit-x60` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintpause`, `zihpm`, `zkt`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfh`, `zvfhmin`, `zvkt`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl64b` |
    /// | `syntacore-scr1-base` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr1-max` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr3-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr3-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr4-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr4-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr5-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr5-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr7` | `a`, `c`, `d`, `f`, `m`, `v`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbc`, `zbkb`, `zbkc`, `zbkx`, `zbs`, `zca`, `zicsr`, `zifencei`, `zkn`, `zknd`, `zkne`, `zknh`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `tt-ascalon-x` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkr`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkg`, `zvkn`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvkt`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl64b` |
    /// | `veyron-v1` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zicbom`, `zicbop`, `zicboz`, `zicntr`, `zicsr`, `zifencei`, `zihintpause`, `zihpm` |
    /// | `xiangshan-kunminghu` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `v`, `za128rs`, `za64rs`, `zaamo`, `zacas`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkb`, `zbkc`, `zbkx`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkn`, `zknd`, `zkne`, `zknh`, `zks`, `zksed`, `zksh`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfh`, `zvfhmin`, `zvkb`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `xiangshan-nanhu` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbc`, `zbkb`, `zbkc`, `zbkx`, `zbs`, `zca`, `zicbom`, `zicboz`, `zicsr`, `zifencei`, `zkn`, `zknd`, `zkne`, `zknh`, `zksed`, `zksh` |
    /// | `xt-c910v2` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt` |
    /// | `xt-c920v2` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvl128b`, `zvl32b`, `zvl64b` |
    pub mod cpus {}
}
/// riscv64 documentation
pub mod riscv64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `a` | 'A' (Atomic Instructions). | `zaamo`, `zalrsc` |
    /// | `b` | 'B' (the collection of the Zba, Zbb, Zbs extensions). | `zba`, `zbb`, `zbs` |
    /// | `c` | 'C' (Compressed Instructions). | `zca` |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `d` | 'D' (Double-Precision Floating-Point). | `f`, `zicsr` |
    /// | `e` | 'E' (Embedded Instruction Set with 16 GPRs). |  |
    /// | `f` | 'F' (Single-Precision Floating-Point). | `zicsr` |
    /// | `m` | 'M' (Integer Multiplication and Division). |  |
    /// | `relax` | Enable Linker relaxation. |  |
    /// | `rva23u64` | RISC-V rva23u64 profile. | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfhmin`, `zvkb`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `supm` | 'Supm' (Indicates User-mode Pointer Masking). |  |
    /// | `unaligned-scalar-mem` | Has reasonably performant unaligned scalar loads and stores. |  |
    /// | `unaligned-vector-mem` | Has reasonably performant unaligned vector loads and stores. |  |
    /// | `v` | 'V' (Vector Extension for Application Processors). | `d`, `f`, `zicsr`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `za128rs` | 'Za128rs' (Reservation Set Size of at Most 128 Bytes). |  |
    /// | `za64rs` | 'Za64rs' (Reservation Set Size of at Most 64 Bytes). | `za128rs` |
    /// | `zaamo` | 'Zaamo' (Atomic Memory Operations). |  |
    /// | `zabha` | 'Zabha' (Byte and Halfword Atomic Memory Operations). | `zaamo` |
    /// | `zacas` | 'Zacas' (Atomic Compare-And-Swap Instructions). | `zaamo` |
    /// | `zalrsc` | 'Zalrsc' (Load-Reserved/Store-Conditional). |  |
    /// | `zama16b` | 'Zama16b' (Atomic 16-byte misaligned loads, stores and AMOs). |  |
    /// | `zawrs` | 'Zawrs' (Wait on Reservation Set). |  |
    /// | `zba` | 'Zba' (Address Generation Instructions). |  |
    /// | `zbb` | 'Zbb' (Basic Bit-Manipulation). |  |
    /// | `zbc` | 'Zbc' (Carry-Less Multiplication). | `zbkc` |
    /// | `zbkb` | 'Zbkb' (Bitmanip instructions for Cryptography). |  |
    /// | `zbkc` | 'Zbkc' (Carry-less multiply instructions for Cryptography). |  |
    /// | `zbkx` | 'Zbkx' (Crossbar permutation instructions). |  |
    /// | `zbs` | 'Zbs' (Single-Bit Instructions). |  |
    /// | `zca` | 'Zca' (part of the C extension, excluding compressed floating point loads/stores). |  |
    /// | `zcb` | 'Zcb' (Compressed basic bit manipulation instructions). | `zca` |
    /// | `zcmop` | 'Zcmop' (Compressed May-Be-Operations). | `zca` |
    /// | `zdinx` | 'Zdinx' (Double in Integer). | `zfinx`, `zicsr` |
    /// | `zfa` | 'Zfa' (Additional Floating-Point). | `f`, `zicsr` |
    /// | `zfbfmin` | 'Zfbfmin' (Scalar BF16 Converts). | `f`, `zicsr` |
    /// | `zfh` | 'Zfh' (Half-Precision Floating-Point). | `f`, `zfhmin`, `zicsr` |
    /// | `zfhmin` | 'Zfhmin' (Half-Precision Floating-Point Minimal). | `f`, `zicsr` |
    /// | `zfinx` | 'Zfinx' (Float in Integer). | `zicsr` |
    /// | `zhinx` | 'Zhinx' (Half Float in Integer). | `zfinx`, `zhinxmin`, `zicsr` |
    /// | `zhinxmin` | 'Zhinxmin' (Half Float in Integer Minimal). | `zfinx`, `zicsr` |
    /// | `zic64b` | 'Zic64b' (Cache Block Size Is 64 Bytes). |  |
    /// | `zicbom` | 'Zicbom' (Cache-Block Management Instructions). |  |
    /// | `zicbop` | 'Zicbop' (Cache-Block Prefetch Instructions). |  |
    /// | `zicboz` | 'Zicboz' (Cache-Block Zero Instructions). |  |
    /// | `ziccamoa` | 'Ziccamoa' (Main Memory Supports All Atomics in A). |  |
    /// | `ziccif` | 'Ziccif' (Main Memory Supports Instruction Fetch with Atomicity Requirement). |  |
    /// | `zicclsm` | 'Zicclsm' (Main Memory Supports Misaligned Loads/Stores). |  |
    /// | `ziccrse` | 'Ziccrse' (Main Memory Supports Forward Progress on LR/SC Sequences). |  |
    /// | `zicntr` | 'Zicntr' (Base Counters and Timers). | `zicsr` |
    /// | `zicond` | 'Zicond' (Integer Conditional Operations). |  |
    /// | `zicsr` | 'Zicsr' (CSRs). |  |
    /// | `zifencei` | 'Zifencei' (fence.i). |  |
    /// | `zihintntl` | 'Zihintntl' (Non-Temporal Locality Hints). |  |
    /// | `zihintpause` | 'Zihintpause' (Pause Hint). |  |
    /// | `zihpm` | 'Zihpm' (Hardware Performance Counters). | `zicsr` |
    /// | `zimop` | 'Zimop' (May-Be-Operations). |  |
    /// | `zk` | 'Zk' (Standard scalar cryptography extension). | `zbkb`, `zbkc`, `zbkx`, `zkn`, `zknd`, `zkne`, `zknh`, `zkr`, `zkt` |
    /// | `zkn` | 'Zkn' (NIST Algorithm Suite). | `zbkb`, `zbkc`, `zbkx`, `zknd`, `zkne`, `zknh` |
    /// | `zknd` | 'Zknd' (NIST Suite: AES Decryption). |  |
    /// | `zkne` | 'Zkne' (NIST Suite: AES Encryption). |  |
    /// | `zknh` | 'Zknh' (NIST Suite: Hash Function Instructions). |  |
    /// | `zkr` | 'Zkr' (Entropy Source Extension). |  |
    /// | `zks` | 'Zks' (ShangMi Algorithm Suite). | `zbkb`, `zbkc`, `zbkx`, `zksed`, `zksh` |
    /// | `zksed` | 'Zksed' (ShangMi Suite: SM4 Block Cipher Instructions). |  |
    /// | `zksh` | 'Zksh' (ShangMi Suite: SM3 Hash Function Instructions). |  |
    /// | `zkt` | 'Zkt' (Data Independent Execution Latency). |  |
    /// | `ztso` | 'Ztso' (Memory Model |  |
    /// | `zvbb` | 'Zvbb' (Vector basic bit-manipulation instructions). | `zicsr`, `zve32x`, `zvkb`, `zvl32b` |
    /// | `zvbc` | 'Zvbc' (Vector Carryless Multiplication). | `zicsr`, `zve32x`, `zve64x`, `zvl32b`, `zvl64b` |
    /// | `zve32f` | 'Zve32f' (Vector Extensions for Embedded Processors with maximal 32 EEW and F extension). | `f`, `zicsr`, `zve32x`, `zvl32b` |
    /// | `zve32x` | 'Zve32x' (Vector Extensions for Embedded Processors with maximal 32 EEW). | `zicsr`, `zvl32b` |
    /// | `zve64d` | 'Zve64d' (Vector Extensions for Embedded Processors with maximal 64 EEW, F and D extension). | `d`, `f`, `zicsr`, `zve32f`, `zve32x`, `zve64f`, `zve64x`, `zvl32b`, `zvl64b` |
    /// | `zve64f` | 'Zve64f' (Vector Extensions for Embedded Processors with maximal 64 EEW and F extension). | `f`, `zicsr`, `zve32f`, `zve32x`, `zve64x`, `zvl32b`, `zvl64b` |
    /// | `zve64x` | 'Zve64x' (Vector Extensions for Embedded Processors with maximal 64 EEW). | `zicsr`, `zve32x`, `zvl32b`, `zvl64b` |
    /// | `zvfbfmin` | 'Zvfbfmin' (Vector BF16 Converts). | `f`, `zicsr`, `zve32f`, `zve32x`, `zvl32b` |
    /// | `zvfbfwma` | 'Zvfbfwma' (Vector BF16 widening mul-add). | `f`, `zfbfmin`, `zicsr`, `zve32f`, `zve32x`, `zvfbfmin`, `zvl32b` |
    /// | `zvfh` | 'Zvfh' (Vector Half-Precision Floating-Point). | `f`, `zfhmin`, `zicsr`, `zve32f`, `zve32x`, `zvfhmin`, `zvl32b` |
    /// | `zvfhmin` | 'Zvfhmin' (Vector Half-Precision Floating-Point Minimal). | `f`, `zicsr`, `zve32f`, `zve32x`, `zvl32b` |
    /// | `zvkb` | 'Zvkb' (Vector Bit-manipulation used in Cryptography). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvkg` | 'Zvkg' (Vector GCM instructions for Cryptography). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvkn` | 'Zvkn' (shorthand for 'Zvkned', 'Zvknhb', 'Zvkb', and 'Zvkt'). | `zicsr`, `zve32x`, `zve64x`, `zvkb`, `zvkned`, `zvknha`, `zvknhb`, `zvkt`, `zvl32b`, `zvl64b` |
    /// | `zvknc` | 'Zvknc' (shorthand for 'Zvknc' and 'Zvbc'). | `zicsr`, `zvbc`, `zve32x`, `zve64x`, `zvkb`, `zvkn`, `zvkned`, `zvknha`, `zvknhb`, `zvkt`, `zvl32b`, `zvl64b` |
    /// | `zvkned` | 'Zvkned' (Vector AES Encryption & Decryption (Single Round)). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvkng` | 'Zvkng' (shorthand for 'Zvkn' and 'Zvkg'). | `zicsr`, `zve32x`, `zve64x`, `zvkb`, `zvkg`, `zvkn`, `zvkned`, `zvknha`, `zvknhb`, `zvkt`, `zvl32b`, `zvl64b` |
    /// | `zvknha` | 'Zvknha' (Vector SHA-2 (SHA-256 only)). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvknhb` | 'Zvknhb' (Vector SHA-2 (SHA-256 and SHA-512)). | `zicsr`, `zve32x`, `zve64x`, `zvknha`, `zvl32b`, `zvl64b` |
    /// | `zvks` | 'Zvks' (shorthand for 'Zvksed', 'Zvksh', 'Zvkb', and 'Zvkt'). | `zicsr`, `zve32x`, `zvkb`, `zvksed`, `zvksh`, `zvkt`, `zvl32b` |
    /// | `zvksc` | 'Zvksc' (shorthand for 'Zvks' and 'Zvbc'). | `zicsr`, `zvbc`, `zve32x`, `zve64x`, `zvkb`, `zvks`, `zvksed`, `zvksh`, `zvkt`, `zvl32b`, `zvl64b` |
    /// | `zvksed` | 'Zvksed' (SM4 Block Cipher Instructions). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvksg` | 'Zvksg' (shorthand for 'Zvks' and 'Zvkg'). | `zicsr`, `zve32x`, `zvkb`, `zvkg`, `zvks`, `zvksed`, `zvksh`, `zvkt`, `zvl32b` |
    /// | `zvksh` | 'Zvksh' (SM3 Hash Function Instructions). | `zicsr`, `zve32x`, `zvl32b` |
    /// | `zvkt` | 'Zvkt' (Vector Data-Independent Execution Latency). |  |
    /// | `zvl1024b` | 'Zvl1024b' (Minimum Vector Length 1024). | `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `zvl128b` | 'Zvl128b' (Minimum Vector Length 128). | `zvl32b`, `zvl64b` |
    /// | `zvl16384b` | 'Zvl16384b' (Minimum Vector Length 16384). | `zvl1024b`, `zvl128b`, `zvl2048b`, `zvl256b`, `zvl32b`, `zvl4096b`, `zvl512b`, `zvl64b`, `zvl8192b` |
    /// | `zvl2048b` | 'Zvl2048b' (Minimum Vector Length 2048). | `zvl1024b`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `zvl256b` | 'Zvl256b' (Minimum Vector Length 256). | `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `zvl32768b` | 'Zvl32768b' (Minimum Vector Length 32768). | `zvl1024b`, `zvl128b`, `zvl16384b`, `zvl2048b`, `zvl256b`, `zvl32b`, `zvl4096b`, `zvl512b`, `zvl64b`, `zvl8192b` |
    /// | `zvl32b` | 'Zvl32b' (Minimum Vector Length 32). |  |
    /// | `zvl4096b` | 'Zvl4096b' (Minimum Vector Length 4096). | `zvl1024b`, `zvl128b`, `zvl2048b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `zvl512b` | 'Zvl512b' (Minimum Vector Length 512). | `zvl128b`, `zvl256b`, `zvl32b`, `zvl64b` |
    /// | `zvl64b` | 'Zvl64b' (Minimum Vector Length 64). | `zvl32b` |
    /// | `zvl65536b` | 'Zvl65536b' (Minimum Vector Length 65536). | `zvl1024b`, `zvl128b`, `zvl16384b`, `zvl2048b`, `zvl256b`, `zvl32768b`, `zvl32b`, `zvl4096b`, `zvl512b`, `zvl64b`, `zvl8192b` |
    /// | `zvl8192b` | 'Zvl8192b' (Minimum Vector Length 8192). | `zvl1024b`, `zvl128b`, `zvl2048b`, `zvl256b`, `zvl32b`, `zvl4096b`, `zvl512b`, `zvl64b` |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `an-erbium` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-45-series` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-a25` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-a45` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-ax25` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-ax45` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-ax45mpv` | `a`, `c`, `d`, `f`, `m`, `v`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `andes-n45` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `andes-nx45` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `et-soc1` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `generic` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `generic-ooo` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `generic-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `generic-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `mips-p8700` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zca`, `zicsr`, `zifencei` |
    /// | `rocket` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `rocket-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `rocket-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `rp2350-hazard3` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbkb`, `zbs`, `zca`, `zcb`, `zicsr`, `zifencei` |
    /// | `sifive-7-series` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e20` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e21` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e24` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e31` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e34` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-e76` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-p450` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbs`, `zca`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zkt` |
    /// | `sifive-p470` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbs`, `zca`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvkb`, `zvkg`, `zvkn`, `zvknc`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-p550` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-p670` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbs`, `zca`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvkb`, `zvkg`, `zvkn`, `zvknc`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-p870-d` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zama16b`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkr`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkg`, `zvkn`, `zvknc`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-s21` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-s51` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-s54` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-s76` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei`, `zihintpause` |
    /// | `sifive-u54` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-u74` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `sifive-x160` | `a`, `b`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zicbom`, `zicbop`, `zicboz`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-x180` | `a`, `b`, `c`, `d`, `f`, `m`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `ziccrse`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `sifive-x280` | `a`, `c`, `d`, `f`, `m`, `v`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zca`, `zfh`, `zfhmin`, `zicsr`, `zifencei`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfh`, `zvfhmin`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `sifive-x390` | `a`, `b`, `c`, `d`, `f`, `m`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkr`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkt`, `zvl1024b`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `spacemit-a100` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `unaligned-scalar-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkg`, `zvkn`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl1024b`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl512b`, `zvl64b` |
    /// | `spacemit-x100` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `unaligned-scalar-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkg`, `zvkn`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvks`, `zvksc`, `zvksed`, `zvksg`, `zvksh`, `zvkt`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl64b` |
    /// | `spacemit-x60` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintpause`, `zihpm`, `zkt`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfh`, `zvfhmin`, `zvkt`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl64b` |
    /// | `syntacore-scr1-base` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr1-max` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr3-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr3-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr4-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr4-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr5-rv32` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr5-rv64` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zca`, `zicsr`, `zifencei` |
    /// | `syntacore-scr7` | `a`, `c`, `d`, `f`, `m`, `v`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbc`, `zbkb`, `zbkc`, `zbkx`, `zbs`, `zca`, `zicsr`, `zifencei`, `zkn`, `zknd`, `zkne`, `zknh`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `tt-ascalon-x` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `unaligned-scalar-mem`, `unaligned-vector-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkr`, `zkt`, `zvbb`, `zvbc`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvkb`, `zvkg`, `zvkn`, `zvkned`, `zvkng`, `zvknha`, `zvknhb`, `zvkt`, `zvl128b`, `zvl256b`, `zvl32b`, `zvl64b` |
    /// | `veyron-v1` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zicbom`, `zicbop`, `zicboz`, `zicntr`, `zicsr`, `zifencei`, `zihintpause`, `zihpm` |
    /// | `xiangshan-kunminghu` | `a`, `b`, `c`, `d`, `f`, `m`, `supm`, `v`, `za128rs`, `za64rs`, `zaamo`, `zacas`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkb`, `zbkc`, `zbkx`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkn`, `zknd`, `zkne`, `zknh`, `zks`, `zksed`, `zksh`, `zkt`, `zvbb`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfh`, `zvfhmin`, `zvkb`, `zvkt`, `zvl128b`, `zvl32b`, `zvl64b` |
    /// | `xiangshan-nanhu` | `a`, `c`, `d`, `f`, `m`, `zaamo`, `zalrsc`, `zba`, `zbb`, `zbc`, `zbkb`, `zbkc`, `zbkx`, `zbs`, `zca`, `zicbom`, `zicboz`, `zicsr`, `zifencei`, `zkn`, `zknd`, `zkne`, `zknh`, `zksed`, `zksh` |
    /// | `xt-c910v2` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `zicclsm`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt` |
    /// | `xt-c920v2` | `a`, `b`, `c`, `d`, `f`, `m`, `unaligned-scalar-mem`, `v`, `za128rs`, `za64rs`, `zaamo`, `zalrsc`, `zawrs`, `zba`, `zbb`, `zbc`, `zbkc`, `zbs`, `zca`, `zcb`, `zcmop`, `zfa`, `zfbfmin`, `zfh`, `zfhmin`, `zic64b`, `zicbom`, `zicbop`, `zicboz`, `ziccamoa`, `ziccif`, `ziccrse`, `zicntr`, `zicond`, `zicsr`, `zifencei`, `zihintntl`, `zihintpause`, `zihpm`, `zimop`, `zkt`, `zve32f`, `zve32x`, `zve64d`, `zve64f`, `zve64x`, `zvfbfmin`, `zvfbfwma`, `zvfh`, `zvfhmin`, `zvl128b`, `zvl32b`, `zvl64b` |
    pub mod cpus {}
}
/// s390x documentation
pub mod s390x {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `backchain` | Store the address of the caller's frame into the callee's stack frame. |  |
    /// | `concurrent-functions` | Assume that the concurrent-functions facility is installed. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `deflate-conversion` | Assume that the deflate-conversion facility is installed. |  |
    /// | `enhanced-sort` | Assume that the enhanced-sort facility is installed. |  |
    /// | `guarded-storage` | Assume that the guarded-storage facility is installed. |  |
    /// | `high-word` | Assume that the high-word facility is installed. |  |
    /// | `message-security-assist-extension12` | Assume that the message-security-assist extension facility 12 is installed. |  |
    /// | `message-security-assist-extension3` | Assume that the message-security-assist extension facility 3 is installed. |  |
    /// | `message-security-assist-extension4` | Assume that the message-security-assist extension facility 4 is installed. |  |
    /// | `message-security-assist-extension5` | Assume that the message-security-assist extension facility 5 is installed. |  |
    /// | `message-security-assist-extension8` | Assume that the message-security-assist extension facility 8 is installed. | `message-security-assist-extension3` |
    /// | `message-security-assist-extension9` | Assume that the message-security-assist extension facility 9 is installed. | `message-security-assist-extension3`, `message-security-assist-extension4` |
    /// | `miscellaneous-extensions-2` | Assume that the miscellaneous-extensions facility 2 is installed. |  |
    /// | `miscellaneous-extensions-3` | Assume that the miscellaneous-extensions facility 3 is installed. |  |
    /// | `miscellaneous-extensions-4` | Assume that the miscellaneous-extensions facility 4 is installed. |  |
    /// | `nnp-assist` | Assume that the NNP-assist facility is installed. | `vector` |
    /// | `transactional-execution` | Assume that the transactional-execution facility is installed. |  |
    /// | `vector` | Assume that the vectory facility is installed. |  |
    /// | `vector-enhancements-1` | Assume that the vector enhancements facility 1 is installed. | `vector` |
    /// | `vector-enhancements-2` | Assume that the vector enhancements facility 2 is installed. | `vector`, `vector-enhancements-1` |
    /// | `vector-enhancements-3` | Assume that the vector enhancements facility 3 is installed. | `vector`, `vector-enhancements-1`, `vector-enhancements-2` |
    /// | `vector-packed-decimal` | Assume that the vector packed decimal facility is installed. | `vector` |
    /// | `vector-packed-decimal-enhancement` | Assume that the vector packed decimal enhancement facility is installed. | `vector`, `vector-packed-decimal` |
    /// | `vector-packed-decimal-enhancement-2` | Assume that the vector packed decimal enhancement facility 2 is installed. | `vector`, `vector-packed-decimal`, `vector-packed-decimal-enhancement` |
    /// | `vector-packed-decimal-enhancement-3` | Assume that the vector packed decimal enhancement facility 3 is installed. | `vector`, `vector-packed-decimal`, `vector-packed-decimal-enhancement`, `vector-packed-decimal-enhancement-2` |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `arch10` | `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `transactional-execution` |
    /// | `arch11` | `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `transactional-execution`, `vector` |
    /// | `arch12` | `guarded-storage`, `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `message-security-assist-extension8`, `miscellaneous-extensions-2`, `transactional-execution`, `vector`, `vector-enhancements-1`, `vector-packed-decimal` |
    /// | `arch13` | `deflate-conversion`, `enhanced-sort`, `guarded-storage`, `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `message-security-assist-extension8`, `message-security-assist-extension9`, `miscellaneous-extensions-2`, `miscellaneous-extensions-3`, `transactional-execution`, `vector`, `vector-enhancements-1`, `vector-enhancements-2`, `vector-packed-decimal`, `vector-packed-decimal-enhancement` |
    /// | `arch14` | `deflate-conversion`, `enhanced-sort`, `guarded-storage`, `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `message-security-assist-extension8`, `message-security-assist-extension9`, `miscellaneous-extensions-2`, `miscellaneous-extensions-3`, `nnp-assist`, `transactional-execution`, `vector`, `vector-enhancements-1`, `vector-enhancements-2`, `vector-packed-decimal`, `vector-packed-decimal-enhancement`, `vector-packed-decimal-enhancement-2` |
    /// | `arch15` | `concurrent-functions`, `deflate-conversion`, `enhanced-sort`, `guarded-storage`, `high-word`, `message-security-assist-extension12`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `message-security-assist-extension8`, `message-security-assist-extension9`, `miscellaneous-extensions-2`, `miscellaneous-extensions-3`, `miscellaneous-extensions-4`, `nnp-assist`, `transactional-execution`, `vector`, `vector-enhancements-1`, `vector-enhancements-2`, `vector-enhancements-3`, `vector-packed-decimal`, `vector-packed-decimal-enhancement`, `vector-packed-decimal-enhancement-2`, `vector-packed-decimal-enhancement-3` |
    /// | `arch8` |  |
    /// | `arch9` | `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4` |
    /// | `generic` |  |
    /// | `z10` |  |
    /// | `z13` | `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `transactional-execution`, `vector` |
    /// | `z14` | `guarded-storage`, `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `message-security-assist-extension8`, `miscellaneous-extensions-2`, `transactional-execution`, `vector`, `vector-enhancements-1`, `vector-packed-decimal` |
    /// | `z15` | `deflate-conversion`, `enhanced-sort`, `guarded-storage`, `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `message-security-assist-extension8`, `message-security-assist-extension9`, `miscellaneous-extensions-2`, `miscellaneous-extensions-3`, `transactional-execution`, `vector`, `vector-enhancements-1`, `vector-enhancements-2`, `vector-packed-decimal`, `vector-packed-decimal-enhancement` |
    /// | `z16` | `deflate-conversion`, `enhanced-sort`, `guarded-storage`, `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `message-security-assist-extension8`, `message-security-assist-extension9`, `miscellaneous-extensions-2`, `miscellaneous-extensions-3`, `nnp-assist`, `transactional-execution`, `vector`, `vector-enhancements-1`, `vector-enhancements-2`, `vector-packed-decimal`, `vector-packed-decimal-enhancement`, `vector-packed-decimal-enhancement-2` |
    /// | `z17` | `concurrent-functions`, `deflate-conversion`, `enhanced-sort`, `guarded-storage`, `high-word`, `message-security-assist-extension12`, `message-security-assist-extension3`, `message-security-assist-extension4`, `message-security-assist-extension5`, `message-security-assist-extension8`, `message-security-assist-extension9`, `miscellaneous-extensions-2`, `miscellaneous-extensions-3`, `miscellaneous-extensions-4`, `nnp-assist`, `transactional-execution`, `vector`, `vector-enhancements-1`, `vector-enhancements-2`, `vector-enhancements-3`, `vector-packed-decimal`, `vector-packed-decimal-enhancement`, `vector-packed-decimal-enhancement-2`, `vector-packed-decimal-enhancement-3` |
    /// | `z196` | `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4` |
    /// | `zEC12` | `high-word`, `message-security-assist-extension3`, `message-security-assist-extension4`, `transactional-execution` |
    pub mod cpus {}
}
/// sparc documentation
pub mod sparc {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `leoncasa` | Enable CASA instruction for LEON3 and LEON4 processors. |  |
    /// | `v8plus` | Enable V8+ mode, allowing use of 64-bit V9 instructions in 32-bit code. |  |
    /// | `v9` | Enable SPARC-V9 instructions. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `at697e` | `v8plus` |
    /// | `at697f` | `v8plus` |
    /// | `f934` | `v8plus` |
    /// | `generic` | `v8plus` |
    /// | `gr712rc` | `leoncasa`, `v8plus` |
    /// | `gr740` | `leoncasa`, `v8plus` |
    /// | `hypersparc` | `v8plus` |
    /// | `leon2` | `v8plus` |
    /// | `leon3` | `leoncasa`, `v8plus` |
    /// | `leon4` | `leoncasa`, `v8plus` |
    /// | `ma2080` | `leoncasa`, `v8plus` |
    /// | `ma2085` | `leoncasa`, `v8plus` |
    /// | `ma2100` | `leoncasa`, `v8plus` |
    /// | `ma2150` | `leoncasa`, `v8plus` |
    /// | `ma2155` | `leoncasa`, `v8plus` |
    /// | `ma2450` | `leoncasa`, `v8plus` |
    /// | `ma2455` | `leoncasa`, `v8plus` |
    /// | `ma2480` | `leoncasa`, `v8plus` |
    /// | `ma2485` | `leoncasa`, `v8plus` |
    /// | `ma2x5x` | `leoncasa`, `v8plus` |
    /// | `ma2x8x` | `leoncasa`, `v8plus` |
    /// | `myriad2` | `leoncasa`, `v8plus` |
    /// | `myriad2.1` | `leoncasa`, `v8plus` |
    /// | `myriad2.2` | `leoncasa`, `v8plus` |
    /// | `myriad2.3` | `leoncasa`, `v8plus` |
    /// | `niagara` | `v8plus`, `v9` |
    /// | `niagara2` | `v8plus`, `v9` |
    /// | `niagara3` | `v8plus`, `v9` |
    /// | `niagara4` | `v8plus`, `v9` |
    /// | `sparclet` | `v8plus` |
    /// | `sparclite` | `v8plus` |
    /// | `sparclite86x` | `v8plus` |
    /// | `supersparc` | `v8plus` |
    /// | `tsc701` | `v8plus` |
    /// | `ultrasparc` | `v8plus`, `v9` |
    /// | `ultrasparc3` | `v8plus`, `v9` |
    /// | `ut699` | `v8plus` |
    /// | `v7` | `v8plus` |
    /// | `v8` | `v8plus` |
    /// | `v9` | `v8plus`, `v9` |
    pub mod cpus {}
}
/// sparc64 documentation
pub mod sparc64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `leoncasa` | Enable CASA instruction for LEON3 and LEON4 processors. |  |
    /// | `v8plus` | Enable V8+ mode, allowing use of 64-bit V9 instructions in 32-bit code. |  |
    /// | `v9` | Enable SPARC-V9 instructions. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `at697e` |  |
    /// | `at697f` |  |
    /// | `f934` |  |
    /// | `generic` |  |
    /// | `gr712rc` | `leoncasa` |
    /// | `gr740` | `leoncasa` |
    /// | `hypersparc` |  |
    /// | `leon2` |  |
    /// | `leon3` | `leoncasa` |
    /// | `leon4` | `leoncasa` |
    /// | `ma2080` | `leoncasa` |
    /// | `ma2085` | `leoncasa` |
    /// | `ma2100` | `leoncasa` |
    /// | `ma2150` | `leoncasa` |
    /// | `ma2155` | `leoncasa` |
    /// | `ma2450` | `leoncasa` |
    /// | `ma2455` | `leoncasa` |
    /// | `ma2480` | `leoncasa` |
    /// | `ma2485` | `leoncasa` |
    /// | `ma2x5x` | `leoncasa` |
    /// | `ma2x8x` | `leoncasa` |
    /// | `myriad2` | `leoncasa` |
    /// | `myriad2.1` | `leoncasa` |
    /// | `myriad2.2` | `leoncasa` |
    /// | `myriad2.3` | `leoncasa` |
    /// | `niagara` | `v9` |
    /// | `niagara2` | `v9` |
    /// | `niagara3` | `v9` |
    /// | `niagara4` | `v9` |
    /// | `sparclet` |  |
    /// | `sparclite` |  |
    /// | `sparclite86x` |  |
    /// | `supersparc` |  |
    /// | `tsc701` |  |
    /// | `ultrasparc` | `v9` |
    /// | `ultrasparc3` | `v9` |
    /// | `ut699` |  |
    /// | `v7` |  |
    /// | `v8` |  |
    /// | `v9` | `v9` |
    pub mod cpus {}
}
/// wasm32 documentation
pub mod wasm32 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `atomics` | Enable Atomics. |  |
    /// | `bulk-memory` | Enable bulk memory operations. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `exception-handling` | Enable Wasm exception handling. |  |
    /// | `extended-const` | Enable extended const expressions. |  |
    /// | `gc` | Enable wasm gc. | `reference-types` |
    /// | `multivalue` | Enable multivalue blocks, instructions, and functions. |  |
    /// | `mutable-globals` | Enable mutable globals. |  |
    /// | `nontrapping-fptoint` | Enable non-trapping float-to-int conversion operators. |  |
    /// | `reference-types` | Enable reference types. |  |
    /// | `relaxed-simd` | Enable relaxed-simd instructions. | `simd128` |
    /// | `sign-ext` | Enable sign extension operators. |  |
    /// | `simd128` | Enable 128-bit SIMD. |  |
    /// | `tail-call` | Enable tail call instructions. |  |
    /// | `wide-arithmetic` | Enable wide-arithmetic instructions. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `bleeding-edge` | `atomics`, `bulk-memory`, `exception-handling`, `extended-const`, `gc`, `multivalue`, `mutable-globals`, `nontrapping-fptoint`, `reference-types`, `relaxed-simd`, `sign-ext`, `simd128`, `tail-call` |
    /// | `generic` | `bulk-memory`, `multivalue`, `mutable-globals`, `nontrapping-fptoint`, `reference-types`, `sign-ext` |
    /// | `lime1` | `extended-const`, `multivalue`, `mutable-globals`, `nontrapping-fptoint`, `sign-ext` |
    /// | `mvp` |  |
    pub mod cpus {}
}
/// wasm64 documentation
pub mod wasm64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `atomics` | Enable Atomics. |  |
    /// | `bulk-memory` | Enable bulk memory operations. |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `exception-handling` | Enable Wasm exception handling. |  |
    /// | `extended-const` | Enable extended const expressions. |  |
    /// | `gc` | Enable wasm gc. | `reference-types` |
    /// | `multivalue` | Enable multivalue blocks, instructions, and functions. |  |
    /// | `mutable-globals` | Enable mutable globals. |  |
    /// | `nontrapping-fptoint` | Enable non-trapping float-to-int conversion operators. |  |
    /// | `reference-types` | Enable reference types. |  |
    /// | `relaxed-simd` | Enable relaxed-simd instructions. | `simd128` |
    /// | `sign-ext` | Enable sign extension operators. |  |
    /// | `simd128` | Enable 128-bit SIMD. |  |
    /// | `tail-call` | Enable tail call instructions. |  |
    /// | `wide-arithmetic` | Enable wide-arithmetic instructions. |  |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `bleeding-edge` | `atomics`, `bulk-memory`, `exception-handling`, `extended-const`, `gc`, `multivalue`, `mutable-globals`, `nontrapping-fptoint`, `reference-types`, `relaxed-simd`, `sign-ext`, `simd128`, `tail-call` |
    /// | `generic` | `bulk-memory`, `multivalue`, `mutable-globals`, `nontrapping-fptoint`, `reference-types`, `sign-ext` |
    /// | `lime1` | `bulk-memory`, `extended-const`, `multivalue`, `mutable-globals`, `nontrapping-fptoint`, `sign-ext` |
    /// | `mvp` | `bulk-memory`, `mutable-globals`, `nontrapping-fptoint`, `sign-ext` |
    pub mod cpus {}
}
/// x86 documentation
pub mod x86 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `adx` | Support ADX instructions. |  |
    /// | `aes` | Enable AES instructions. | `sse`, `sse2` |
    /// | `amx-avx512` | Support AMX-AVX512 instructions. | `amx-tile` |
    /// | `amx-bf16` | Support AMX-BF16 instructions. | `amx-tile` |
    /// | `amx-complex` | Support AMX-COMPLEX instructions. | `amx-tile` |
    /// | `amx-fp16` | Support AMX amx-fp16 instructions. | `amx-tile` |
    /// | `amx-fp8` | Support AMX-FP8 instructions. | `amx-tile` |
    /// | `amx-int8` | Support AMX-INT8 instructions. | `amx-tile` |
    /// | `amx-movrs` | Support AMX-MOVRS instructions. | `amx-tile` |
    /// | `amx-tile` | Support AMX-TILE instructions. |  |
    /// | `apxf` | Support extended general purpose register. |  |
    /// | `avx` | Enable AVX instructions. | `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx10.1` | Support AVX10.1 instruction. | `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx10.2` | Support AVX10.2 instruction. | `avx`, `avx10.1`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx2` | Enable AVX2 instructions. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512bf16` | Support bfloat16 floating point. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512bitalg` | Enable AVX-512 Bit Algorithms. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512bw` | Enable AVX-512 Byte and Word Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512cd` | Enable AVX-512 Conflict Detection Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512dq` | Enable AVX-512 Doubleword and Quadword Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512f` | Enable AVX-512 instructions. | `avx`, `avx2`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512fp16` | Support 16-bit floating point. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512ifma` | Enable AVX-512 Integer Fused Multiple-Add. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vbmi` | Enable AVX-512 Vector Byte Manipulation Instructions. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vbmi2` | Enable AVX-512 further Vector Byte Manipulation Instructions. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vl` | Enable AVX-512 Vector Length eXtensions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vnni` | Enable AVX-512 Vector Neural Network Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vp2intersect` | Enable AVX-512 vp2intersect. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vpopcntdq` | Enable AVX-512 Population Count Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxifma` | Enable AVX-IFMA. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxneconvert` | Support AVX-NE-CONVERT instructions. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxvnni` | Support AVX_VNNI encoding. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxvnniint16` | Enable AVX-VNNI-INT16. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxvnniint8` | Enable AVX-VNNI-INT8. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `bmi1` | Support BMI instructions. |  |
    /// | `bmi2` | Support BMI2 instructions. |  |
    /// | `clflushopt` | Flush A Cache Line Optimized. |  |
    /// | `cmpxchg16b` | 64-bit with cmpxchg16b (this is true for most x86-64 chips, but not the first AMD chips). |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `ermsb` | REP MOVS/STOS are fast. |  |
    /// | `f16c` | Support 16-bit floating point conversion instructions. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `fma` | Enable three-operand fused multiple-add. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `fma4` | Enable four-operand fused multiple-add. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3` |
    /// | `fxsr` | Support fxsave/fxrestore instructions. |  |
    /// | `gfni` | Enable Galois Field Arithmetic Instructions. | `sse`, `sse2` |
    /// | `kl` | Support Key Locker kl Instructions. | `sse`, `sse2` |
    /// | `lahfsahf` | Support LAHF and SAHF instructions in 64-bit mode. |  |
    /// | `lzcnt` | Support LZCNT instruction. |  |
    /// | `movbe` | Support MOVBE instruction. |  |
    /// | `movrs` | Enable MOVRS. |  |
    /// | `pclmulqdq` | Enable packed carry-less multiplication instructions. | `sse`, `sse2` |
    /// | `popcnt` | Support POPCNT instruction. |  |
    /// | `prfchw` | Support PRFCHW instructions. |  |
    /// | `rdrand` | Support RDRAND instruction. |  |
    /// | `rdseed` | Support RDSEED instruction. |  |
    /// | `rtm` | Support RTM instructions. |  |
    /// | `sha` | Enable SHA instructions. | `sse`, `sse2` |
    /// | `sha512` | Support SHA512 instructions. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `sm3` | Support SM3 instructions. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `sm4` | Support SM4 instructions. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `sse` | Enable SSE instructions. |  |
    /// | `sse2` | Enable SSE2 instructions. | `sse` |
    /// | `sse3` | Enable SSE3 instructions. | `sse`, `sse2` |
    /// | `sse4.1` | Enable SSE 4.1 instructions. | `sse`, `sse2`, `sse3`, `ssse3` |
    /// | `sse4.2` | Enable SSE 4.2 instructions. | `sse`, `sse2`, `sse3`, `sse4.1`, `ssse3` |
    /// | `sse4a` | Support SSE 4a instructions. | `sse`, `sse2`, `sse3` |
    /// | `ssse3` | Enable SSSE3 instructions. | `sse`, `sse2`, `sse3` |
    /// | `tbm` | Enable TBM instructions. |  |
    /// | `vaes` | Promote selected AES instructions to AVX512/AVX registers. | `aes`, `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `vpclmulqdq` | Enable vpclmulqdq instructions. | `avx`, `pclmulqdq`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `widekl` | Support Key Locker wide Instructions. | `kl`, `sse`, `sse2` |
    /// | `x87` | Enable X87 float instructions. |  |
    /// | `xop` | Enable XOP instructions. | `avx`, `fma4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3` |
    /// | `xsave` | Support xsave instructions. |  |
    /// | `xsavec` | Support xsavec instructions. | `xsave` |
    /// | `xsaveopt` | Support xsaveopt instructions. | `xsave` |
    /// | `xsaves` | Support xsaves instructions. | `xsave` |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `alderlake` | `adx`, `aes`, `avx`, `avx2`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `amdfam10` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `lzcnt`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4a`, `x87` |
    /// | `arrowlake` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `arrowlake-s` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `arrowlake_s` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `athlon` | `prfchw`, `x87` |
    /// | `athlon-4` | `fxsr`, `prfchw`, `sse`, `x87` |
    /// | `athlon-fx` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon-mp` | `fxsr`, `prfchw`, `sse`, `x87` |
    /// | `athlon-tbird` | `prfchw`, `x87` |
    /// | `athlon-xp` | `fxsr`, `prfchw`, `sse`, `x87` |
    /// | `athlon64` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon64-sse3` | `cmpxchg16b`, `fxsr`, `prfchw`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `atom` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `sse`, `sse2`, `sse3`, `ssse3`, `x87` |
    /// | `atom_sse4_2` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `atom_sse4_2_movbe` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `barcelona` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `lzcnt`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4a`, `x87` |
    /// | `bdver1` | `aes`, `avx`, `cmpxchg16b`, `fma4`, `fxsr`, `lahfsahf`, `lzcnt`, `pclmulqdq`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xop`, `xsave` |
    /// | `bdver2` | `aes`, `avx`, `bmi1`, `cmpxchg16b`, `f16c`, `fma`, `fma4`, `fxsr`, `lahfsahf`, `lzcnt`, `pclmulqdq`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `tbm`, `x87`, `xop`, `xsave` |
    /// | `bdver3` | `aes`, `avx`, `bmi1`, `cmpxchg16b`, `f16c`, `fma`, `fma4`, `fxsr`, `lahfsahf`, `lzcnt`, `pclmulqdq`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `tbm`, `x87`, `xop`, `xsave`, `xsaveopt` |
    /// | `bdver4` | `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fma4`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `tbm`, `x87`, `xop`, `xsave`, `xsaveopt` |
    /// | `bonnell` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `sse`, `sse2`, `sse3`, `ssse3`, `x87` |
    /// | `broadwell` | `adx`, `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `btver1` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `lzcnt`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4a`, `ssse3`, `x87` |
    /// | `btver2` | `aes`, `avx`, `bmi1`, `cmpxchg16b`, `f16c`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `c3` | `prfchw`, `x87` |
    /// | `c3-2` | `fxsr`, `sse`, `x87` |
    /// | `c86-4g-m4` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `c86-4g-m6` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `c86-4g-m7` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `c86-4g-m8` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `cannonlake` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vl`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `cascadelake` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `avx512vnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `clearwaterforest` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `cooperlake` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `avx512vnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `core-avx-i` | `avx`, `cmpxchg16b`, `f16c`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core-avx2` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core2` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `sse`, `sse2`, `sse3`, `ssse3`, `x87` |
    /// | `core_2_duo_sse4_1` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `sse`, `sse2`, `sse3`, `sse4.1`, `ssse3`, `x87` |
    /// | `core_2_duo_ssse3` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `sse`, `sse2`, `sse3`, `ssse3`, `x87` |
    /// | `core_2nd_gen_avx` | `avx`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_3rd_gen_avx` | `avx`, `cmpxchg16b`, `f16c`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_4th_gen_avx` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_4th_gen_avx_tsx` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_5th_gen_avx` | `adx`, `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_5th_gen_avx_tsx` | `adx`, `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_aes_pclmulqdq` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `core_i7_sse4_2` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `corei7` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `corei7-avx` | `avx`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `diamondrapids` | `adx`, `aes`, `amx-avx512`, `amx-bf16`, `amx-complex`, `amx-fp16`, `amx-fp8`, `amx-int8`, `amx-movrs`, `amx-tile`, `avx`, `avx10.1`, `avx10.2`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `movrs`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `emeraldrapids` | `adx`, `aes`, `amx-bf16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `generic` | `x87` |
    /// | `geode` | `prfchw`, `x87` |
    /// | `goldmont` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `goldmont-plus` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `goldmont_plus` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `gracemont` | `adx`, `aes`, `avx`, `avx2`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `grandridge` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `graniterapids` | `adx`, `aes`, `amx-bf16`, `amx-fp16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `graniterapids-d` | `adx`, `aes`, `amx-bf16`, `amx-complex`, `amx-fp16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `graniterapids_d` | `adx`, `aes`, `amx-bf16`, `amx-complex`, `amx-fp16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `haswell` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `i386` | `x87` |
    /// | `i486` | `x87` |
    /// | `i586` | `x87` |
    /// | `i686` | `x87` |
    /// | `icelake-client` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `icelake-server` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `icelake_client` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `icelake_server` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `ivybridge` | `avx`, `cmpxchg16b`, `f16c`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `k6` | `x87` |
    /// | `k6-2` | `prfchw`, `x87` |
    /// | `k6-3` | `prfchw`, `x87` |
    /// | `k8` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `k8-sse3` | `cmpxchg16b`, `fxsr`, `prfchw`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `knl` | `adx`, `aes`, `avx`, `avx2`, `avx512cd`, `avx512f`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `knm` | `adx`, `aes`, `avx`, `avx2`, `avx512cd`, `avx512f`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `lakemont` |  |
    /// | `lunarlake` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `meteorlake` | `adx`, `aes`, `avx`, `avx2`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `mic_avx512` | `adx`, `aes`, `avx`, `avx2`, `avx512cd`, `avx512f`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `nehalem` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `nocona` | `cmpxchg16b`, `fxsr`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `novalake` | `adx`, `aes`, `avx`, `avx10.1`, `avx10.2`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `movrs`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `opteron` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `opteron-sse3` | `cmpxchg16b`, `fxsr`, `prfchw`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `pantherlake` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `penryn` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `sse`, `sse2`, `sse3`, `sse4.1`, `ssse3`, `x87` |
    /// | `pentium` | `x87` |
    /// | `pentium-m` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium-mmx` | `x87` |
    /// | `pentium2` | `fxsr`, `x87` |
    /// | `pentium3` | `fxsr`, `sse`, `x87` |
    /// | `pentium3m` | `fxsr`, `sse`, `x87` |
    /// | `pentium4` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium4m` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_4` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_4_sse3` | `fxsr`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `pentium_ii` | `fxsr`, `x87` |
    /// | `pentium_iii` | `fxsr`, `sse`, `x87` |
    /// | `pentium_iii_no_xmm_regs` | `fxsr`, `sse`, `x87` |
    /// | `pentium_m` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_mmx` | `x87` |
    /// | `pentium_pro` | `x87` |
    /// | `pentiumpro` | `x87` |
    /// | `prescott` | `fxsr`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `raptorlake` | `adx`, `aes`, `avx`, `avx2`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `rocketlake` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `sandybridge` | `avx`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `sapphirerapids` | `adx`, `aes`, `amx-bf16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `sierraforest` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `silvermont` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `skx` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `skylake` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `skylake-avx512` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `skylake_avx512` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `slm` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `tigerlake` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vp2intersect`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `tremont` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `gfni`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `westmere` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `wildcatlake` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `winchip-c6` | `x87` |
    /// | `winchip2` | `prfchw`, `x87` |
    /// | `x86-64` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `x86-64-v2` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `x86-64-v3` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave` |
    /// | `x86-64-v4` | `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave` |
    /// | `yonah` | `fxsr`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `znver1` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver2` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver3` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver4` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver5` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vp2intersect`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver6` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vp2intersect`, `avx512vpopcntdq`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    pub mod cpus {}
}
/// x86_64 documentation
pub mod x86_64 {
    /// | Feature | Description | Also Enables<sup>†</sup> |
    /// | ------- | ----------- | ------------------------ |
    /// | `adx` | Support ADX instructions. |  |
    /// | `aes` | Enable AES instructions. | `sse`, `sse2` |
    /// | `amx-avx512` | Support AMX-AVX512 instructions. | `amx-tile` |
    /// | `amx-bf16` | Support AMX-BF16 instructions. | `amx-tile` |
    /// | `amx-complex` | Support AMX-COMPLEX instructions. | `amx-tile` |
    /// | `amx-fp16` | Support AMX amx-fp16 instructions. | `amx-tile` |
    /// | `amx-fp8` | Support AMX-FP8 instructions. | `amx-tile` |
    /// | `amx-int8` | Support AMX-INT8 instructions. | `amx-tile` |
    /// | `amx-movrs` | Support AMX-MOVRS instructions. | `amx-tile` |
    /// | `amx-tile` | Support AMX-TILE instructions. |  |
    /// | `apxf` | Support extended general purpose register. |  |
    /// | `avx` | Enable AVX instructions. | `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx10.1` | Support AVX10.1 instruction. | `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx10.2` | Support AVX10.2 instruction. | `avx`, `avx10.1`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx2` | Enable AVX2 instructions. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512bf16` | Support bfloat16 floating point. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512bitalg` | Enable AVX-512 Bit Algorithms. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512bw` | Enable AVX-512 Byte and Word Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512cd` | Enable AVX-512 Conflict Detection Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512dq` | Enable AVX-512 Doubleword and Quadword Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512f` | Enable AVX-512 instructions. | `avx`, `avx2`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512fp16` | Support 16-bit floating point. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512ifma` | Enable AVX-512 Integer Fused Multiple-Add. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vbmi` | Enable AVX-512 Vector Byte Manipulation Instructions. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vbmi2` | Enable AVX-512 further Vector Byte Manipulation Instructions. | `avx`, `avx2`, `avx512bw`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vl` | Enable AVX-512 Vector Length eXtensions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vnni` | Enable AVX-512 Vector Neural Network Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vp2intersect` | Enable AVX-512 vp2intersect. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avx512vpopcntdq` | Enable AVX-512 Population Count Instructions. | `avx`, `avx2`, `avx512f`, `f16c`, `fma`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxifma` | Enable AVX-IFMA. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxneconvert` | Support AVX-NE-CONVERT instructions. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxvnni` | Support AVX_VNNI encoding. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxvnniint16` | Enable AVX-VNNI-INT16. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `avxvnniint8` | Enable AVX-VNNI-INT8. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `bmi1` | Support BMI instructions. |  |
    /// | `bmi2` | Support BMI2 instructions. |  |
    /// | `clflushopt` | Flush A Cache Line Optimized. |  |
    /// | `cmpxchg16b` | 64-bit with cmpxchg16b (this is true for most x86-64 chips, but not the first AMD chips). |  |
    /// | `crt-static` | Enables C Run-time Libraries to be statically linked. |  |
    /// | `ermsb` | REP MOVS/STOS are fast. |  |
    /// | `f16c` | Support 16-bit floating point conversion instructions. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `fma` | Enable three-operand fused multiple-add. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `fma4` | Enable four-operand fused multiple-add. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3` |
    /// | `fxsr` | Support fxsave/fxrestore instructions. |  |
    /// | `gfni` | Enable Galois Field Arithmetic Instructions. | `sse`, `sse2` |
    /// | `kl` | Support Key Locker kl Instructions. | `sse`, `sse2` |
    /// | `lahfsahf` | Support LAHF and SAHF instructions in 64-bit mode. |  |
    /// | `lzcnt` | Support LZCNT instruction. |  |
    /// | `movbe` | Support MOVBE instruction. |  |
    /// | `movrs` | Enable MOVRS. |  |
    /// | `pclmulqdq` | Enable packed carry-less multiplication instructions. | `sse`, `sse2` |
    /// | `popcnt` | Support POPCNT instruction. |  |
    /// | `prfchw` | Support PRFCHW instructions. |  |
    /// | `rdrand` | Support RDRAND instruction. |  |
    /// | `rdseed` | Support RDSEED instruction. |  |
    /// | `rtm` | Support RTM instructions. |  |
    /// | `sha` | Enable SHA instructions. | `sse`, `sse2` |
    /// | `sha512` | Support SHA512 instructions. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `sm3` | Support SM3 instructions. | `avx`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `sm4` | Support SM4 instructions. | `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `sse` | Enable SSE instructions. |  |
    /// | `sse2` | Enable SSE2 instructions. | `sse` |
    /// | `sse3` | Enable SSE3 instructions. | `sse`, `sse2` |
    /// | `sse4.1` | Enable SSE 4.1 instructions. | `sse`, `sse2`, `sse3`, `ssse3` |
    /// | `sse4.2` | Enable SSE 4.2 instructions. | `sse`, `sse2`, `sse3`, `sse4.1`, `ssse3` |
    /// | `sse4a` | Support SSE 4a instructions. | `sse`, `sse2`, `sse3` |
    /// | `ssse3` | Enable SSSE3 instructions. | `sse`, `sse2`, `sse3` |
    /// | `tbm` | Enable TBM instructions. |  |
    /// | `vaes` | Promote selected AES instructions to AVX512/AVX registers. | `aes`, `avx`, `avx2`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `vpclmulqdq` | Enable vpclmulqdq instructions. | `avx`, `pclmulqdq`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3` |
    /// | `widekl` | Support Key Locker wide Instructions. | `kl`, `sse`, `sse2` |
    /// | `x87` | Enable X87 float instructions. |  |
    /// | `xop` | Enable XOP instructions. | `avx`, `fma4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3` |
    /// | `xsave` | Support xsave instructions. |  |
    /// | `xsavec` | Support xsavec instructions. | `xsave` |
    /// | `xsaveopt` | Support xsaveopt instructions. | `xsave` |
    /// | `xsaves` | Support xsaves instructions. | `xsave` |
    ///
    /// <sup>†</sup> This is often empirical, rather than specified in any standard, i.e. all available CPUs with a particular feature also have another feature.
    pub mod feature {}

    /// | CPU | Enabled Features |
    /// | --- | -------- |
    /// | `alderlake` | `adx`, `aes`, `avx`, `avx2`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `amdfam10` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `lzcnt`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4a`, `x87` |
    /// | `arrowlake` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `arrowlake-s` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `arrowlake_s` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `athlon` | `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon-4` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon-fx` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon-mp` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon-tbird` | `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon-xp` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon64` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `athlon64-sse3` | `cmpxchg16b`, `fxsr`, `prfchw`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `atom` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `sse`, `sse2`, `sse3`, `ssse3`, `x87` |
    /// | `atom_sse4_2` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `atom_sse4_2_movbe` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `barcelona` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `lzcnt`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4a`, `x87` |
    /// | `bdver1` | `aes`, `avx`, `cmpxchg16b`, `fma4`, `fxsr`, `lahfsahf`, `lzcnt`, `pclmulqdq`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xop`, `xsave` |
    /// | `bdver2` | `aes`, `avx`, `bmi1`, `cmpxchg16b`, `f16c`, `fma`, `fma4`, `fxsr`, `lahfsahf`, `lzcnt`, `pclmulqdq`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `tbm`, `x87`, `xop`, `xsave` |
    /// | `bdver3` | `aes`, `avx`, `bmi1`, `cmpxchg16b`, `f16c`, `fma`, `fma4`, `fxsr`, `lahfsahf`, `lzcnt`, `pclmulqdq`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `tbm`, `x87`, `xop`, `xsave`, `xsaveopt` |
    /// | `bdver4` | `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fma4`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `tbm`, `x87`, `xop`, `xsave`, `xsaveopt` |
    /// | `bonnell` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `sse`, `sse2`, `sse3`, `ssse3`, `x87` |
    /// | `broadwell` | `adx`, `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `btver1` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `lzcnt`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4a`, `ssse3`, `x87` |
    /// | `btver2` | `aes`, `avx`, `bmi1`, `cmpxchg16b`, `f16c`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `c3` | `prfchw`, `sse`, `sse2`, `x87` |
    /// | `c3-2` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `c86-4g-m4` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `c86-4g-m6` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `c86-4g-m7` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `c86-4g-m8` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `cannonlake` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vl`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `cascadelake` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `avx512vnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `clearwaterforest` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `cooperlake` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `avx512vnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `core-avx-i` | `avx`, `cmpxchg16b`, `f16c`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core-avx2` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core2` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `sse`, `sse2`, `sse3`, `ssse3`, `x87` |
    /// | `core_2_duo_sse4_1` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `sse`, `sse2`, `sse3`, `sse4.1`, `ssse3`, `x87` |
    /// | `core_2_duo_ssse3` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `sse`, `sse2`, `sse3`, `ssse3`, `x87` |
    /// | `core_2nd_gen_avx` | `avx`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_3rd_gen_avx` | `avx`, `cmpxchg16b`, `f16c`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_4th_gen_avx` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_4th_gen_avx_tsx` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_5th_gen_avx` | `adx`, `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_5th_gen_avx_tsx` | `adx`, `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `core_aes_pclmulqdq` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `core_i7_sse4_2` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `corei7` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `corei7-avx` | `avx`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `diamondrapids` | `adx`, `aes`, `amx-avx512`, `amx-bf16`, `amx-complex`, `amx-fp16`, `amx-fp8`, `amx-int8`, `amx-movrs`, `amx-tile`, `avx`, `avx10.1`, `avx10.2`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `movrs`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `emeraldrapids` | `adx`, `aes`, `amx-bf16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `generic` | `sse`, `sse2`, `x87` |
    /// | `geode` | `prfchw`, `sse`, `sse2`, `x87` |
    /// | `goldmont` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `goldmont-plus` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `goldmont_plus` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `gracemont` | `adx`, `aes`, `avx`, `avx2`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `grandridge` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `graniterapids` | `adx`, `aes`, `amx-bf16`, `amx-fp16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `graniterapids-d` | `adx`, `aes`, `amx-bf16`, `amx-complex`, `amx-fp16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `graniterapids_d` | `adx`, `aes`, `amx-bf16`, `amx-complex`, `amx-fp16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `haswell` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `i386` | `sse`, `sse2`, `x87` |
    /// | `i486` | `sse`, `sse2`, `x87` |
    /// | `i586` | `sse`, `sse2`, `x87` |
    /// | `i686` | `sse`, `sse2`, `x87` |
    /// | `icelake-client` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `icelake-server` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `icelake_client` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `icelake_server` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `ivybridge` | `avx`, `cmpxchg16b`, `f16c`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `k6` | `sse`, `sse2`, `x87` |
    /// | `k6-2` | `prfchw`, `sse`, `sse2`, `x87` |
    /// | `k6-3` | `prfchw`, `sse`, `sse2`, `x87` |
    /// | `k8` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `k8-sse3` | `cmpxchg16b`, `fxsr`, `prfchw`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `knl` | `adx`, `aes`, `avx`, `avx2`, `avx512cd`, `avx512f`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `knm` | `adx`, `aes`, `avx`, `avx2`, `avx512cd`, `avx512f`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `lakemont` | `sse`, `sse2` |
    /// | `lunarlake` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `meteorlake` | `adx`, `aes`, `avx`, `avx2`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `mic_avx512` | `adx`, `aes`, `avx`, `avx2`, `avx512cd`, `avx512f`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `nehalem` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `nocona` | `cmpxchg16b`, `fxsr`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `novalake` | `adx`, `aes`, `avx`, `avx10.1`, `avx10.2`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `movrs`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `opteron` | `fxsr`, `prfchw`, `sse`, `sse2`, `x87` |
    /// | `opteron-sse3` | `cmpxchg16b`, `fxsr`, `prfchw`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `pantherlake` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `penryn` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `sse`, `sse2`, `sse3`, `sse4.1`, `ssse3`, `x87` |
    /// | `pentium` | `sse`, `sse2`, `x87` |
    /// | `pentium-m` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium-mmx` | `sse`, `sse2`, `x87` |
    /// | `pentium2` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium3` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium3m` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium4` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium4m` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_4` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_4_sse3` | `fxsr`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `pentium_ii` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_iii` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_iii_no_xmm_regs` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_m` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `pentium_mmx` | `sse`, `sse2`, `x87` |
    /// | `pentium_pro` | `sse`, `sse2`, `x87` |
    /// | `pentiumpro` | `sse`, `sse2`, `x87` |
    /// | `prescott` | `fxsr`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `raptorlake` | `adx`, `aes`, `avx`, `avx2`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `rocketlake` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `sandybridge` | `avx`, `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsaveopt` |
    /// | `sapphirerapids` | `adx`, `aes`, `amx-bf16`, `amx-int8`, `amx-tile`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `sierraforest` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `kl`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `widekl`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `silvermont` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `skx` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `skylake` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `skylake-avx512` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `skylake_avx512` | `adx`, `aes`, `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `slm` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `tigerlake` | `adx`, `aes`, `avx`, `avx2`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vp2intersect`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `ermsb`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `tremont` | `aes`, `clflushopt`, `cmpxchg16b`, `fxsr`, `gfni`, `lahfsahf`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `westmere` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `pclmulqdq`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `wildcatlake` | `adx`, `aes`, `avx`, `avx2`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint16`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sha512`, `sm3`, `sm4`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `winchip-c6` | `sse`, `sse2`, `x87` |
    /// | `winchip2` | `prfchw`, `sse`, `sse2`, `x87` |
    /// | `x86-64` | `fxsr`, `sse`, `sse2`, `x87` |
    /// | `x86-64-v2` | `cmpxchg16b`, `fxsr`, `lahfsahf`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87` |
    /// | `x86-64-v3` | `avx`, `avx2`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave` |
    /// | `x86-64-v4` | `avx`, `avx2`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512vl`, `bmi1`, `bmi2`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `popcnt`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `ssse3`, `x87`, `xsave` |
    /// | `yonah` | `fxsr`, `sse`, `sse2`, `sse3`, `x87` |
    /// | `znver1` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver2` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver3` | `adx`, `aes`, `avx`, `avx2`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver4` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vpopcntdq`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver5` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vp2intersect`, `avx512vpopcntdq`, `avxvnni`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    /// | `znver6` | `adx`, `aes`, `avx`, `avx2`, `avx512bf16`, `avx512bitalg`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512f`, `avx512fp16`, `avx512ifma`, `avx512vbmi`, `avx512vbmi2`, `avx512vl`, `avx512vnni`, `avx512vp2intersect`, `avx512vpopcntdq`, `avxifma`, `avxneconvert`, `avxvnni`, `avxvnniint8`, `bmi1`, `bmi2`, `clflushopt`, `cmpxchg16b`, `f16c`, `fma`, `fxsr`, `gfni`, `lahfsahf`, `lzcnt`, `movbe`, `pclmulqdq`, `popcnt`, `prfchw`, `rdrand`, `rdseed`, `sha`, `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`, `vaes`, `vpclmulqdq`, `x87`, `xsave`, `xsavec`, `xsaveopt`, `xsaves` |
    pub mod cpus {}
}
