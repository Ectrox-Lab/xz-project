# Phase 4 Results Index

## Run set
- Runner: `bioworld_mvp` sentinel mode CLI
- Actual config used: `ticks=2000`, `universes(seeds)=4` per condition
- Notes: this is the same validated small-scale config from Phase 3 for reproducible packaging.

## Condition result table
| condition | seeds | config | per-run csv | status |
|---|---:|---|---|---|
| baseline_full | 4 | ticks=2000, seeds=4 | `bioworld_mvp/runs/sentinel/baseline_full/per_run.csv` | success |
| no_L2 | 4 | ticks=2000, seeds=4 | `bioworld_mvp/runs/sentinel/no_L2/per_run.csv` | success |
| L3_off | 4 | ticks=2000, seeds=4 | `bioworld_mvp/runs/sentinel/L3_off/per_run.csv` | success |
| L3_real_p001 | 4 | ticks=2000, seeds=4 | `bioworld_mvp/runs/sentinel/L3_real_p001/per_run.csv` | success |
| L3_shuffled_p001 | 4 | ticks=2000, seeds=4 | `bioworld_mvp/runs/sentinel/L3_shuffled_p001/per_run.csv` | success |
| no_L1 | 0 (this round) | n/a | n/a | partial (not executed this round) |
| L3_overpowered_direct | 0 (this round) | n/a | n/a | partial (not executed this round) |

## Per-condition tick-level outputs
For each successful condition, tick-level telemetry is under:
- `bioworld_mvp/runs/sentinel/<condition>/u*/memory.csv`

## Result files generated/updated in Phase 4 packaging
- `docs/integration/PHASE4_RESULTS_INDEX.md`
- `docs/integration/PHASE4_DATA_DICTIONARY.md`
- `docs/integration/PHASE4_EVIDENCE_MEMO.md`
- `docs/integration/PHASE4_COMMAND_REPLAY.md`
- `docs/integration/IMPLEMENTATION_NOTES_PHASE4.md`
- `docs/integration/status-sync.json`
- `docs/integration/open-questions.md`
