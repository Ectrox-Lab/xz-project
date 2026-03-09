# Open Questions (Atlas Sync)

## blocker
- [field] `per_run.csv` finalization is missing for the latest 10000/16 mini-scale-up attempt; confirm whether atlas accepts prior 5000/8 summaries as temporary fallback.
- [runnable] Re-run requirement: complete clean 10000/16 runs for `baseline_full`, `no_L2`, `L3_real_p001`, `L3_shuffled_p001`.
- [evidence] Need completed 10000/16 output set to tighten real-vs-shuffled interpretation confidence.
- [interpretation] Keep atlas threshold request open; adjudication should wait until clean 10000/16 closure.

## semantic mismatch
1. Confirm whether atlas wants strict 16/16 successful universes per condition or allows partial-universe retries with explicit tagging.
2. Confirm whether missing per_run finalization can be replaced by reconstructed summaries from per-seed telemetry (currently not done).

## next-phase proposal
1. Re-run the 4 conditions in a dedicated window with no concurrent heavy jobs.
2. Keep scope fixed (no new conditions, no new modules).
3. After clean rerun, refresh metric summary docs and status-sync in one pass.
