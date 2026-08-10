# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-08-10
### Added
- Added Arm64EC, LoongArch32/64, NVPTX/64, S390X, and SPARC/64 architectures
- Added `Feature::can_detect_at_runtime` to report runtime detection availability
- Added architecture name and family queries

### Changed
- Split MIPS, PowerPC, RISC-V, WebAssembly, and x86 into 32- and 64-bit variants
- Made `Architecture` non-exhaustive
- Renamed `CURRENT_TARGET` to `BUILD_TARGET`
- Generated feature implications from rustc's target-feature graph

### Fixed
- Improved `suggested_simd_width`

## [0.1.6] - 2024-03-15
### Changed
- Updated feature list

## [0.1.5] - 2023-09-22
### Changed
- Updated feature list

## [0.1.4] - 2023-05-17
### Changed
- Updated feature list

## [0.1.3] - 2023-03-02
### Added
- Added new features
- Added documentation for features enabled by other features

## [0.1.2] - 2023-01-04
### Fixed
- Fixed crate always needing rebuild

## [0.1.1] - 2022-11-30
### Fixed
- Added missing features using new rustc nightly (https://github.com/rust-lang/rust/pull/104627)

## [0.1.0] - 2022-11-19
### Added
- Initial release

[Unreleased]: https://github.com/calebzulawski/target-features/compare/0.2.0...HEAD
[0.2.0]: https://github.com/calebzulawski/target-features/compare/0.1.6...0.2.0
[0.1.6]: https://github.com/calebzulawski/target-features/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/calebzulawski/target-features/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/calebzulawski/target-features/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/calebzulawski/target-features/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/calebzulawski/target-features/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/calebzulawski/target-features/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/calebzulawski/target-features/releases/tag/0.1.0
