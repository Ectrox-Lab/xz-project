# Open Questions (Atlas Sync)

## blocker
- [field] Confirm whether `lineage_diversity` remains normalized (`distinct_lineages / population`) or should be absolute count in acceptance checks.
- [runnable] Optional conditions `no_L1` and `L3_overpowered_direct` are not included in this Phase 4 packaged result set.
- [evidence] Need stronger integration-level proof for "direct archive injection unreachable" across all sentinel modes.
- [interpretation] Need atlas-defined effect-size/decision threshold for interpreting `L3_real_p001` vs `L3_shuffled_p001`.

## semantic mismatch
1. `collapse_event_count` currently counts in-run extinction events; confirm if only terminal collapse should count.
2. Strategy entropy is averaged per tick per run; confirm if atlas expects terminal-only value.

## next-phase proposal
1. Run `no_L1` and `L3_overpowered_direct` under same `ticks=2000,seeds=4` for completion parity.
2. Add a scripted comparison artifact (CSV/markdown) automatically generated after run completion.
3. Add integration regression asserting forbidden direct archive->cell injection paths are unreachable in normal modes.
