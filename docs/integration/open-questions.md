# Open Questions (Atlas Sync)

## Unresolved interface items
1. `CI` and `r` source-of-truth formulas for PR-3 continuity probe are not yet surfaced in this repo.
2. Expected sentinel-run summary schema (single file vs per-condition tree) needs final alignment.

## Semantic mismatches to resolve
- Current Bio-World emits memory instrumentation in `memory.csv`; confirm Atlas expects per-tick or per-run aggregation.
- Confirm whether `lineage_diversity` should remain normalized (`distinct_lineages / population`) or absolute count.

## Falsification blockers
- L3 shuffled control path is still pending (PR-2).
- Overpowered direct-control negative control is still pending (PR-2).

## Upstream patch requests
- Atlas should provide canonical continuity probe columns and units for direct compatibility.
