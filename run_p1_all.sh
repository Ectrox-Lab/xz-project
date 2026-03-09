#!/bin/bash
# P1 Phase 1: Run all 4 groups (12 runs total)

set -e
cd /tmp/bio-world/bioworld_mvp

TICKS=3000
UNIVERSES=16
OUTPUT_BASE="/tmp/bio-world/p1_experiments"

echo "═══════════════════════════════════════════════════════════"
echo "  P1 Phase 1: Causal Validation Screening"
echo "  4 Groups × 3 Seeds = 12 Runs"
echo "  Generations: $TICKS, Universes: $UNIVERSES"
echo "═══════════════════════════════════════════════════════════"

# CTRL Group (baseline)
echo ""
echo "▶▶▶ CTRL Group (baseline, pressure=1.0) ◀◀◀"
./target/release/p1_experiment \
    --group CTRL \
    --seed 101 \
    --ticks $TICKS \
    --universes $UNIVERSES \
    --output-dir "$OUTPUT_BASE/ctrl"

# P1-A: Memory KO
echo ""
echo "▶▶▶ P1-A Group (Memory KO, no lineage inheritance) ◀◀◀"
./target/release/p1_experiment \
    --group P1A \
    --seed 201 \
    --ticks $TICKS \
    --universes $UNIVERSES \
    --output-dir "$OUTPUT_BASE/p1a"

# P1-B: Cooperation suppression
echo ""
echo "▶▶▶ P1-B Group (Cooperation suppression, multiplier=0.3) ◀◀◀"
./target/release/p1_experiment \
    --group P1B \
    --seed 301 \
    --ticks $TICKS \
    --universes $UNIVERSES \
    --output-dir "$OUTPUT_BASE/p1b"

# P1-C: Boss pressure ×1.5
echo ""
echo "▶▶▶ P1-C Group (Boss pressure ×1.5) ◀◀◀"
./target/release/p1_experiment \
    --group P1C \
    --seed 401 \
    --ticks $TICKS \
    --universes $UNIVERSES \
    --output-dir "$OUTPUT_BASE/p1c"

echo ""
echo "═══════════════════════════════════════════════════════════"
echo "  P1 Phase 1 Complete!"
echo "  Output: $OUTPUT_BASE/"
echo "═══════════════════════════════════════════════════════════"

# List results
echo ""
echo "Results:"
ls -la $OUTPUT_BASE/*/group_summary.json 2>/dev/null || echo "No summary files found"
