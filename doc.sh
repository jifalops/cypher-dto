#!/bin/bash
set -e
PORT=${1:-8080}
cargo doc --no-deps
echo "================================="
echo "http://localhost:$PORT/cypher_dto"
echo "================================="
python3 -m http.server $PORT --directory target/doc/
