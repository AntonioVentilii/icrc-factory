#!/usr/bin/env bash
set -euo pipefail

#scripts/setup cargo-binstall candid-extractor didc

cargo build

WASM="target/wasm32-unknown-unknown/release/icrc_factory.wasm"

test -e "$WASM" || cargo build --manifest-path="src/icrc-factory/Cargo.toml" \
  --target wasm32-unknown-unknown \
  --release --package "icrc-factory"

candid-extractor "$WASM" >"src/icrc-factory/icrc-factory.did"

dfx generate
