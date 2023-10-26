#!/bin/bash
set -e -x

git submodule update --init --recursive

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/near_deposit_withdraw.wasm ./res/

cd aurora-deposit-withdraw
forge build
