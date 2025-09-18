#!/bin/bash
echo "Testing specific entity condition test..."
cd /home/ashfaqf/playground/opencsd/openscenario-rs
cargo test test_by_entity_condition_speed 2>&1 | head -20