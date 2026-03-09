#!/usr/bin/env bash
set -euo pipefail
SEED="${1:-1}"
TICKS="${2:-10000}"
UNIVERSES="${3:-16}"

cargo run --release -- --seed "$SEED" --ticks "$TICKS" --universes "$UNIVERSES"
python audit/check_shortcuts.py
