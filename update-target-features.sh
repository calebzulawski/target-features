#!/bin/sh
cargo +nightly run --manifest-path list-target-features/Cargo.toml
mv *.txt target-features/
