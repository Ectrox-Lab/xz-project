# Phase 3 Run Summary

## 1) Executed commands
Executed the five required core conditions via release runner:
- `baseline_full`
- `no_L2`
- `L3_off`
- `L3_real_p001`
- `L3_shuffled_p001`

(Exact command lines are listed in `PHASE3_COMMANDS.md`.)

## 2) Seeds per condition
Actual executed seeds (`universes`) per condition in this run: **4**.

## 3) CSV paths
- `bioworld_mvp/runs/sentinel/baseline_full/per_run.csv`
- `bioworld_mvp/runs/sentinel/no_L2/per_run.csv`
- `bioworld_mvp/runs/sentinel/L3_off/per_run.csv`
- `bioworld_mvp/runs/sentinel/L3_real_p001/per_run.csv`
- `bioworld_mvp/runs/sentinel/L3_shuffled_p001/per_run.csv`

Per-seed tick telemetry exists under each condition at:
- `bioworld_mvp/runs/sentinel/<condition>/u*/memory.csv`

## 4) required_now field generation check
All five `per_run.csv` files contain the required columns:
- `archive_sample_attempts`
- `archive_sample_successes`
- `archive_influenced_births`
- `lineage_diversity`
- `top1_lineage_share`
- `strategy_entropy`
- `collapse_event_count`

## 5) First-pass directional observations (small-scale)
Using mean values over 4 seeds:
- `no_L2` and `L3_off` show lower `lineage_diversity` and higher `top1_lineage_share` than `baseline_full`.
- `L3_real_p001` tracks `baseline_full` closely in this run.
- `L3_shuffled_p001` differs from `L3_real_p001` on diversity/share/entropy and has similar order-of-magnitude sampling counters.

## 6) Ambiguous points
- Sample size and runtime are still small (`ticks=2000`, `seeds=4`) for strong claims.
- `collapse_event_count` remained 0 across this run, so collapse sensitivity is not yet tested.
- `L3_real_p001` vs `L3_shuffled_p001` effect-size thresholds still need atlas-side acceptance criteria.
