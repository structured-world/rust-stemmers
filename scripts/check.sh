#!/usr/bin/env bash
# One reproducible release gate; only this checkout's build outputs are cleaned.
set -euo pipefail
python3 -B scripts/snowball.py check "$@"
cargo build --all-targets
cargo nextest run --all-targets
cargo test --doc
python3 -B scripts/check-package.py
cargo clean
