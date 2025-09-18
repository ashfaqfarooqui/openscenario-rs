#!/bin/bash
cd /home/ashfaqf/playground/opencsd/openscenario-rs
rustc --edition 2021 -L target/debug/deps test_simple.rs --extern openscenario_rs=target/debug/libopenscenario_rs.rlib 2>&1