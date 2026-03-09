#!/bin/bash
# P1 Phase 1: Run all 4 groups in parallel (scaled down for faster execution)

set -e
cd /tmp/bio-world/bioworld_mvp

TICKS=1500
UNIVERSES=8
OUTPUT_BASE="/tmp/bio-world/p1_experiments"

echo "═══════════════════════════════════════════════════════════"
echo "  P1 Phase 1: Causal Validation (Parallel)"
echo "  Generations: $TICKS, Universes: $UNIVERSES per seed"
echo "═══════════════════════════════════════════════════════════"

# Create output directories
mkdir -p $OUTPUT_BASE/{ctrl,p1a,p1b,p1c}

# Function to run a group
run_group() {
    local group=$1
    local seed=$2
    local output=$3
    
    echo "Starting $group (seed $seed)..."
    ./target/release/p1_experiment \
        --group $group \
        --seed $seed \
        --ticks $TICKS \
        --universes $UNIVERSES \
        --output-dir "$output" > "$output/run.log" 2>&1
    echo "✓ $group complete"
}

# Launch all 4 groups in parallel
run_group CTRL 101 "$OUTPUT_BASE/ctrl" &
PID_CTRL=$!
run_group P1A 201 "$OUTPUT_BASE/p1a" &
PID_P1A=$!
run_group P1B 301 "$OUTPUT_BASE/p1b" &
PID_P1B=$!
run_group P1C 401 "$OUTPUT_BASE/p1c" &
PID_P1C=$!

echo "All 4 groups launched in parallel..."
echo "PIDs: CTRL=$PID_CTRL, P1A=$PID_P1A, P1B=$PID_P1B, P1C=$PID_P1C"

# Wait for all to complete
wait $PID_CTRL
wait $PID_P1A
wait $PID_P1B
wait $PID_P1C

echo ""
echo "═══════════════════════════════════════════════════════════"
echo "  P1 Phase 1 Complete!"
echo "═══════════════════════════════════════════════════════════"

# Show results
for group in ctrl p1a p1b p1c; do
    echo ""
    echo "[$group]"
    if [ -f "$OUTPUT_BASE/$group/group_summary.json" ]; then
        cat "$OUTPUT_BASE/$group/group_summary.json" | head -20
    else
        echo "  No summary file"
    fi
done
