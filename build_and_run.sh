#!/usr/bin/env bash
set -euo pipefail

echo "Launching frontend + backend"
echo "Running from ${PWD}"

(cd ./frontend && npm run build)

cargo run -- test
