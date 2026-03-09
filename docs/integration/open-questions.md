# Open Questions (Atlas Sync)

## blocker
- [runnable] Optional sentinel modes `no_L1` and `L3_overpowered_direct` were not executed in this phase pass.
- [semantic] Need acceptance threshold definition for `L3_real_p001` vs `L3_shuffled_p001` (metric + effect size).
- [evidence] Need stronger end-to-end regression proof for "no direct archive injection reachable" across all sentinel modes.
- [variance] Small-run variance remains high at `ticks=2000, seeds=4`; requires larger repeat to stabilize interpretation.

## semantic mismatch
1. Confirm whether `lineage_diversity` should stay normalized (`distinct_lineages / population`) or become absolute count.
2. Confirm whether `collapse_event_count` should represent all in-run collapse events or terminal collapse only.

## next-phase proposal
1. Execute optional modes (`no_L1`, `L3_overpowered_direct`) and append to phase summary.
2. Add automated comparative report generation for real vs shuffled conditions.
3. Add integration-level anti-god-mode audit tests spanning all sentinel modes.
