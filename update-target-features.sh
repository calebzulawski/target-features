#!/bin/sh
set -eu

# Requires `rustup component add rustc-dev --toolchain nightly`.
cargo +nightly run --manifest-path list-target-features/Cargo.toml
mv rustc-version.md target-features/
mv database.rs target-features/src/database/generated.rs
rustfmt +nightly target-features/src/database/generated.rs
