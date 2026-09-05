# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-09-05
### Added
- Added support for Arm64EC, LoongArch32/64, NVPTX64, S390X, and SPARC/64 compilation targets

### Changed
- Redesigned the API around `TargetFeatures`. This release is not API-compatible with version 0.1
- Feature sets are now created using architecture-specific constants or the `target_features!` macro
- `TargetFeatures::enabled_for_target()` replaces `CURRENT_TARGET`
- Improved `suggested_simd_width` for additional architectures and target features
- Raised the minimum supported Rust version to 1.82

### Removed
- Removed support for describing other architectures and specific CPUs

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
