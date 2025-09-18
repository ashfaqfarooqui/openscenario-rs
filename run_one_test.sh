#!/bin/bash
cd /home/ashfaqf/playground/opencsd/openscenario-rs
cargo test test_by_entity_condition_basic --lib 2>&1 | head -30