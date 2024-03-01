#!/bin/bash
set -e
cargo doc --no-deps
python3 -m http.server --directory target/doc/
