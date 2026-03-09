# Open Questions (Atlas Sync)

## blocker
- [field] Confirm whether `lineage_diversity` remains normalized (`distinct_lineages / population`) for adjudication, or should be interpreted as absolute lineage count.
- [runnable] Confirm whether atlas requires immediate minimal rerun including `no_L1` and `L3_overpowered_direct` before GO/HOLD decision.
- [evidence] Need a dedicated invariant-style test for shuffled-vs-real sampling bandwidth equivalence (current proof is operational + observational).
- [interpretation] Need atlas-specified decision threshold for `L3_real_p001` vs `L3_shuffled_p001` deltas.

## semantic mismatch
1. `collapse_event_count` is currently in-run extinction event count; confirm if terminal-only collapse should be used in judgment.
2. `survival_time` in Phase 4.5 summary is defined as ticks with `population > 0`; confirm this matches atlas review semantics.

## next-phase proposal
1. If requested by atlas, perform only minimal rerun (missing conditions and/or missing field closure), no new conditions.
2. Add a tiny checker script that validates required_now column presence and non-placeholder status per condition.
3. Add a narrow test for shuffled-control equivalence on sampling pipeline counts.
