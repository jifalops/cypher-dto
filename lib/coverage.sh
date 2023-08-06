#!/bin/bash
cargo llvm-cov nextest --html
pushd ../target/llvm-cov/html > /dev/null
python3 -m http.server 14532
popd > /dev/null
