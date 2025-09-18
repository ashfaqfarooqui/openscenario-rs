#!/bin/bash

echo "Testing example compilation..."

# Test debug_catalog_resolution example
echo "Testing debug_catalog_resolution..."
cargo check --example debug_catalog_resolution 2>&1 | head -20

echo ""
echo "Testing parse example..."
cargo check --example parse 2>&1 | head -20

echo ""
echo "Testing full test suite..."
cargo test 2>&1 | head -30