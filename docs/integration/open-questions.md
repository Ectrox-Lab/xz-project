# Open Questions (Atlas Sync)

## blocker
- [field] Confirm final adjudication semantics for `lineage_diversity` (normalized ratio vs absolute count).
- [runnable] Core minimal rerun is complete; confirm whether atlas still requires optional `no_L1` and `L3_overpowered_direct` reruns in this cycle.
- [evidence] Need decision on whether observational equivalence evidence is sufficient, or invariant test patch is required now.
- [interpretation] Provide threshold rule for interpreting `L3_real_p001` vs `L3_shuffled_p001` deltas.

## semantic mismatch
1. `collapse_event_count` remains zero across core 5000/8 reruns; confirm expected handling when collapse signal is absent.
2. `survival_time` is measured as ticks with `population > 0`; confirm this is acceptable for atlas side comparison.

## next-phase proposal
1. If atlas requests stronger proof, add only a tiny invariant test around shuffled-vs-real sampling/bandwidth counters.
2. If atlas requests completeness, run optional `no_L1` and `L3_overpowered_direct` with the same 5000/8 config and append summaries.
3. Keep changes minimal: no new modes, no refactor, no long-run expansion.
