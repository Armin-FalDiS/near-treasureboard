#! /bin/bash
rm -rf res
mkdir res
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm res