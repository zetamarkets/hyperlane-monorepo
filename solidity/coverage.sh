#!/bin/bash

# exit on error
set -e

if ! forge coverage \
    --report lcov \
    --report summary \
    --no-match-coverage "(test|mock|node_modules|script|Fast|TypedMemView)" \
    --no-match-test "Fork" \
    --ir-minimum; then # https://github.com/foundry-rs/foundry/issues/3357
  echo "forge coverage failed; falling back to hardhat coverage"
  yarn hardhat-esm coverage
fi
