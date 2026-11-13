#!/usr/bin/env bash
set -euo pipefail

echo "Launching frontend + backend"

cd "$(dirname "$0")"

(cd ./frontend && npm run build)

cargo run -- test
