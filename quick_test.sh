#!/bin/bash
echo "Running quick library test..."
cargo test --lib basic_entity_condition_test 2>&1 | head -20