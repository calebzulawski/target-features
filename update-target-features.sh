#!/bin/sh
set -eu

# Requires `rustup component add rustc-dev --toolchain nightly`.
cargo +nightly run --manifest-path list-target-features/Cargo.toml
mv rustc-version.txt target-cpus.txt target-features.txt target-features/
mv docs.rs target-features/src/
