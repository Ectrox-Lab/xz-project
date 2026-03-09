#!/bin/bash
# P1 Phase 1: Causal Validation Screening (n=3 per group, 12 runs total)
# Groups: CTRL, P1-A (Memory KO), P1-B (Coop suppression), P1-C (Boss×1.5)

set -e

BINARY="./target/release/bioworld_mvp"
BASE_DIR="p1_experiments"
GENERATIONS=5000
UNIVERSES=32

# Seed mapping for reproducibility
# CTRL:  101, 102, 103
# P1-A:  201, 202, 203 (Memory KO)
# P1-B:  301, 302, 303 (Cooperation suppression)
# P1-C:  401, 402, 403 (Boss pressure ×1.5)

echo "═══════════════════════════════════════════════════════════════"
echo "  P1 Phase 1: Causal Validation Screening"
echo "  4 Groups × 3 Seeds = 12 Runs"
echo "═══════════════════════════════════════════════════════════════"

# CTRL Group (baseline)
echo ""
echo "▶ Running CTRL Group (baseline)..."
for seed in 101 102 103; do
    echo "  CTRL seed=$seed..."
    $BINARY \
        --mode experiment \
        --seed $seed \
        --generations $GENERATIONS \
        --universes $UNIVERSES \
        --output-dir "$BASE_DIR/ctrl/seed_${seed}" \
        --group CTRL \
        2>&1 | tail -5
done

# P1-A: Memory KO (disable L2/L3 memory inheritance)
echo ""
echo "▶ Running P1-A Group (Memory KO)..."
for seed in 201 202 203; do
    echo "  P1-A seed=$seed..."
    $BINARY \
        --mode experiment \
        --seed $seed \
        --generations $GENERATIONS \
        --universes $UNIVERSES \
        --output-dir "$BASE_DIR/p1a/seed_${seed}" \
        --group P1A \
        --disable-lineage-memory \
        --disable-archive-sampling \
        2>&1 | tail -5
done

# P1-B: Cooperation suppression (reduce cooperation reward)
echo ""
echo "▶ Running P1-B Group (Cooperation suppression)..."
for seed in 301 302 303; do
    echo "  P1-B seed=$seed..."
    $BINARY \
        --mode experiment \
        --seed $seed \
        --generations $GENERATIONS \
        --universes $UNIVERSES \
        --output-dir "$BASE_DIR/p1b/seed_${seed}" \
        --group P1B \
        --cooperation-multiplier 0.3 \
        2>&1 | tail -5
done

# P1-C: Boss pressure ×1.5 (increase environmental stress)
echo ""
echo "▶ Running P1-C Group (Boss pressure ×1.5)..."
for seed in 401 402 403; do
    echo "  P1-C seed=$seed..."
    $BINARY \
        --mode experiment \
        --seed $seed \
        --generations $GENERATIONS \
        --universes $UNIVERSES \
        --output-dir "$BASE_DIR/p1c/seed_${seed}" \
        --group P1C \
        --boss-pressure 1.5 \
        2>&1 | tail -5
done

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  P1 Phase 1 Complete!"
echo "  Data location: $BASE_DIR/"
echo "═══════════════════════════════════════════════════════════════"
