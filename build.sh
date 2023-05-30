#!/bin/bash
cd "$(dirname "$0")"

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/release/terra-and-caelus.wasm 

cp index.html ./dist/index.html
cp -R ./assets ./dist/assets