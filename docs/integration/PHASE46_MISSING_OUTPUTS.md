# Phase 4.6 Missing Outputs Audit

## Core condition output presence
| condition | per_run.csv | per-seed telemetry (`u0..u7/memory.csv`) | status |
|---|---|---|---|
| baseline_full | present | all 8 present | closed |
| no_L2 | present | all 8 present | closed |
| L3_real_p001 | present | all 8 present | closed |
| L3_shuffled_p001 | present | all 8 present | closed |

## Non-core condition output presence (not required this round)
| condition | status |
|---|---|
| no_L1 | not rerun in phase4.6 |
| L3_overpowered_direct | not rerun in phase4.6 |

## required_now 7 columns status
All four core `per_run.csv` files contain true-value columns:
- `archive_sample_attempts`
- `archive_sample_successes`
- `archive_influenced_births`
- `lineage_diversity`
- `top1_lineage_share`
- `strategy_entropy`
- `collapse_event_count`

## Placeholder / semantic-unstable columns
- No placeholder columns detected in required_now 7.
- Semantic stability still pending atlas confirmation for:
  - normalized vs absolute interpretation of `lineage_diversity`
  - interpretation threshold for real vs shuffled deltas

## Blockers from missing outputs
- **No blocker** from missing core files in phase4.6.
