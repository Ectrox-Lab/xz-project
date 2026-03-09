# Phase 4.6 Mini Scale-up Attempt (10000/16)

## Requested target
- Conditions: `baseline_full`, `no_L2`, `L3_real_p001`, `L3_shuffled_p001`
- Config: `--ticks 10000 --universes 16`

## What was attempted
- Executed release runner with all 4 conditions at target config.
- Also attempted parallelized per-condition execution to reduce wall-clock time.

## Outcome
- Runs were computationally heavy in this environment and did not complete cleanly for full closure in one cycle.
- Partial universe telemetry directories were generated, but `per_run.csv` was not finalized for the four conditions in this attempt.

## Minimal fix added
- Added a formal invariant-style test to lock that `L3_real_p001` and `L3_shuffled_p001` share the same access envelope toggles (`L1/L2/L3/on-overpowered`), differing only by shuffled content flag.
- Added a runtime optimization for sentinel runs to skip full archive JSON persistence (not needed for sentinel metric closure) to reduce I/O overhead.

## Next minimal action
- Re-run the same four conditions at `10000/16` in a lower-contention window and require successful `per_run.csv` generation before adjudication.
