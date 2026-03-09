# Atlas ↔ Bio-World Integration Contract

## Interface Version
- `interface_version`: `0.1.0-pr1`

## PR-1 instrumentation field names
The following snake_case fields are contract-stable for sentinel and sync work:

- `archive_sample_attempts`
- `archive_sample_successes`
- `archive_influenced_births`
- `lineage_diversity`
- `top1_lineage_share`
- `strategy_entropy`
- `collapse_event_count`

These fields are emitted in `runs/*/u*/memory.csv`.

## Anti-god-mode guard requirements
- Cell must not read archive directly.
- Archive must not write cell memory or inject cell strategy.
- Archive sampling probability must be bounded by `ARCHIVE_SAMPLE_PROBABILITY`.
- Cross-layer accesses must be auditable through `MemoryAccessGuard` logs/errors.


## Sentinel runnable condition names
- `baseline_full`
- `no_L1`
- `no_L2`
- `L3_off`
- `L3_real_p001`
- `L3_shuffled_p001`
- `L3_overpowered_direct`
