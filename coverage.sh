#!/bin/bash
set -e
cargo llvm-cov test --html
python3 -m http.server --directory target/llvm-cov/html
