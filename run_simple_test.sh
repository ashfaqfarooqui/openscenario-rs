#!/bin/bash
cd /home/ashfaqf/playground/opencsd/openscenario-rs
cargo test basic_entity_condition_test --lib 2>&1 | head -20