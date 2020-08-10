#!/usr/bin/env bash

set -x

cargo clean;
cargo build --release --target wasm32-unknown-unknown;
cp target/wasm32-unknown-unknown/release/rustycheckers.wasm demo/;
python3 -m http.server;

