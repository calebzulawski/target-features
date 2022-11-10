#!/bin/sh
cargo +nightly run --manifest-path list-target-features/Cargo.toml
mv target-features.txt target-features/target-features.txt
