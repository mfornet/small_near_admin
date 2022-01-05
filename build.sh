#!/bin/bash
env RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/small_admin.wasm res/
ls -la res/small_admin.wasm
