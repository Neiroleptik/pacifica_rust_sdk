#!/bin/bash
set -e

echo "Building..."
cargo build

echo "Checking formatting..."
cargo fmt -- --check

echo "Running Clippy Lib (exclude bin)..."
cargo clippy --fix --lib -p  pacifica_rust_sdk

echo "Running tests..."
cargo test

echo "All checks passed!"
