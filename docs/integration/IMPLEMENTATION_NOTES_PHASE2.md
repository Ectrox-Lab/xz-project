# Phase 2 Implementation Notes (Instrumentation Closure + Sentinel Runnable Conditions)

## 1) Files changed this round
- `bioworld_mvp/src/main.rs`
- `bioworld_mvp/src/bio_world/experiments/experiment_runner.rs`
- `bioworld_mvp/src/bio_world/engine/world.rs`
- `docs/integration/status-sync.json`
- `docs/integration/open-questions.md`
- `docs/integration/IMPLEMENTATION_NOTES_PHASE2.md`

## 2) Sentinel condition switches and code paths
Sentinel mode is selected via CLI `--sentinel-mode <name>` (or all modes with `--run-sentinel-suite`).

Modes:
- `baseline_full`: full L1/L2/L3 path
- `no_L1`: disables `CellMemory` write/decay/cost branch
- `no_L2`: disables lineage inheritance (child lineage reset)
- `L3_off`: disables archive queue sampling path
- `L3_real_p001`: normal archive sampling at `p=0.01`
- `L3_shuffled_p001`: same sample frequency/bandwidth as real but sampled record context is shuffled before lesson compression
- `L3_overpowered_direct`: explicit negative-control mode enabling direct strategy override (`oracle_direct`) for newborn lineage

## 3) CSV contract columns and origin
Contract fields are emitted in both:
1) per-tick `runs/sentinel/<mode>/u*/memory.csv`
2) per-run `runs/sentinel/<mode>/per_run.csv`

Columns:
- `archive_sample_attempts`: tick/run counters around lineage sampling attempts
- `archive_sample_successes`: successful sample count
- `archive_influenced_births`: births with archive/overpowered influence
- `lineage_diversity`: distinct_lineages/population (tick average in per-run)
- `top1_lineage_share`: largest lineage ratio (tick average in per-run)
- `strategy_entropy`: entropy over lineage strategy labels (tick average in per-run)
- `collapse_event_count`: extinction event counter

## 4) Real-value vs placeholder fields
- Real-valued: all contract columns are emitted from runtime computation.
- Potentially zero-by-design in short runs: sampling counters and influenced births (depends on stochastic path and run length).
- TODO: add deterministic seed-level assertion tests for non-zero sampling under extended ticks.

## 5) Most missing tests now
- Condition parity tests proving `L3_real_p001` and `L3_shuffled_p001` share identical sample-rate budget.
- Explicit regression tests for each sentinel mode outputting `per_run.csv`.
- Overpowered mode impact guardrails (expected divergence threshold checks).
