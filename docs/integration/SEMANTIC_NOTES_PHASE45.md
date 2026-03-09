# Semantic Notes Phase 4.5

## Runtime argument semantics
- `--ticks <N>`: number of simulation ticks (generations) attempted per universe run.
- `--universes <U>`: number of independent universe runs (seeded replicates) for the condition.

## File-level semantics
- `per_run.csv`: one row = one universe (one seed replicate) summary for a single condition.
- `memory.csv` (under `runs/sentinel/<condition>/u*/memory.csv`): one file = one universe tick-by-tick telemetry for that condition.

## Minimum statistical unit for core-condition comparison
- Primary comparison unit: **per-run row** (one universe replicate) from `per_run.csv`.
- Condition-level summaries are computed as aggregate statistics across per-run rows (e.g., mean over seeds).

## Survival-time note used in Phase 4.5 summary
- `survival_time` in the Phase 4.5 metric summary is computed as count of ticks with `population > 0` in each universe's `population.csv`, then averaged across seeds.
