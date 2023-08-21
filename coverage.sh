#!/bin/bash
set -e
PORT=${1:-8081}
cargo llvm-cov test --html
python3 -m http.server $PORT --directory target/llvm-cov/html
