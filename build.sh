#!/bin/bash
set -e # exit immediately if a command exits with non-zero status

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/dummy_splitfee_repo.wasm ./res
