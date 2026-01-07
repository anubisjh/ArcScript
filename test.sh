#!/bin/bash
# Quick test script for ArcScript

echo "=== ArcScript Test Suite ==="
echo ""

# Test 1: Basic arithmetic
echo "Test 1: Basic arithmetic"
cargo run <<EOF
var x = 10 + 5;
var y = x * 2;
EOF
echo ""

# Test 2: Run example files
echo "Test 2: Running example files"
for file in examples/*.arc; do
    echo "Running $file..."
    cargo run "$file"
    echo ""
done

# Test 3: Unit tests
echo "Test 3: Running unit tests"
cargo test

echo "=== All tests complete ==="
