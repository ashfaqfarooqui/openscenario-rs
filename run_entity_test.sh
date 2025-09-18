#!/bin/bash
echo "Running entity condition test..."
cd /home/ashfaqf/playground/opencsd/openscenario-rs
cargo test entity_condition_integration_tests 2>&1 | head -30