#!/bin/bash

# Number of times to run the program
NUM_RUNS=5

for i in $(seq 1 $NUM_RUNS); do
  echo "===== Run $i of $NUM_RUNS ====="
  ./target/debug/simple_rule_self_play
  echo "===== Completed Run $i of $NUM_RUNS ====="
  
  # Add a small delay between runs
  sleep 1
done

echo "All runs completed successfully!"
