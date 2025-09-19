#!/bin/bash
set -e

echo "Building..."
cargo build

echo "Checking formatting..."
cargo fmt -- --check

echo "Running Clippy..."
cargo clippy -- -D warnings

echo "Running tests..."
cargo test

echo "All checks passed!"
