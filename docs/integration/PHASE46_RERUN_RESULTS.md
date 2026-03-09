# Phase 4.6 Minimal Rerun Results

## Commands actually executed
```bash
cd /workspace/bio-world/bioworld_mvp
cargo build --release
./target/release/bioworld_mvp --ticks 5000 --universes 8 --sentinel-mode baseline_full
./target/release/bioworld_mvp --ticks 5000 --universes 8 --sentinel-mode no_L2
./target/release/bioworld_mvp --ticks 5000 --universes 8 --sentinel-mode L3_real_p001
./target/release/bioworld_mvp --ticks 5000 --universes 8 --sentinel-mode L3_shuffled_p001
```

## Output paths
- `bioworld_mvp/runs/sentinel/baseline_full/per_run.csv`
- `bioworld_mvp/runs/sentinel/no_L2/per_run.csv`
- `bioworld_mvp/runs/sentinel/L3_real_p001/per_run.csv`
- `bioworld_mvp/runs/sentinel/L3_shuffled_p001/per_run.csv`
- per-seed telemetry: `bioworld_mvp/runs/sentinel/<condition>/u*/memory.csv`

## Universe count per condition
- baseline_full: 8
- no_L2: 8
- L3_real_p001: 8
- L3_shuffled_p001: 8

## Core metric summaries (mean over universes)
| condition | survival_time | lineage_diversity | top1_lineage_share | strategy_entropy | collapse_event_count |
|---|---:|---:|---:|---:|---:|
| baseline_full | 5000.0000 | 0.014088 | 0.644639 | 0.702674 | 0.000000 |
| no_L2 | 5000.0000 | 0.013529 | 0.542984 | 0.000000 | 0.000000 |
| L3_real_p001 | 5000.0000 | 0.014088 | 0.644639 | 0.702674 | 0.000000 |
| L3_shuffled_p001 | 5000.0000 | 0.015485 | 0.597776 | 0.660077 | 0.000000 |

## Notes
- `survival_time` = mean ticks with `population > 0` across universes.
- `collapse_event_count` stayed zero in this run, so collapse-sensitive conclusions remain limited.
