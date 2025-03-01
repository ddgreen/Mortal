#!/bin/bash

# Build the binary without Python features
cargo build --bin simple_rule_vs_simple_rule --no-default-features

# Run the program
./target/debug/simple_rule_vs_simple_rule

echo "Self-play completed. Game logs are saved in the game_logs directory."
