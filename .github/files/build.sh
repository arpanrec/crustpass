#!/usr/bin/env bash
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive
sudo apt-get update &&
    sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu libc6-dev-arm64-cross

rustup target add aarch64-unknown-linux-gnu &&
    env CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
        CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
        CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
        cargo build --target aarch64-unknown-linux-gnu --release

mkdir -p /tmp/secretsquirrel && rm -rf /tmp/secretsquirrel/aarch64*

mv ./target/aarch64-unknown-linux-gnu/release/secretsquirrel /tmp/secretsquirrel/aarch64-secretsquirrel

cargo build --target x86_64-unknown-linux-gnu --release

mv ./target/x86_64-unknown-linux-gnu/release/secretsquirrel /tmp/secretsquirrel/x86_64-secretsquirrel
