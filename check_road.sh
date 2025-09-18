#!/bin/bash
cd /home/ashfaqf/playground/opencsd/openscenario-rs
cargo check --lib 2>&1 | grep "road.rs" | head -10