#!/usr/bin/env bash

cargo build --release
sudo mv target/release/calculator /bin/rust_calculator
echo "calculator instaled!!"
echo 'run "rust_calculator" to open the aplication'
