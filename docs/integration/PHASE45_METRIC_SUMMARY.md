# Phase 4.5 Metric Summary (Core 5 Conditions)

Config used for this summary:
- `ticks=2000`
- `universes(seeds)=4`

| condition | seeds | survival_time (mean ticks with pop>0) | lineage_diversity (mean) | top1_lineage_share (mean) | strategy_entropy (mean) | collapse_event_count (mean) |
|---|---:|---:|---:|---:|---:|---:|
| baseline_full | 4 | 2000.0000 | 0.037664 | 0.368274 | 0.638603 | 0.000000 |
| no_L2 | 4 | 2000.0000 | 0.031592 | 0.516849 | 0.000000 | 0.000000 |
| L3_off | 4 | 2000.0000 | 0.033238 | 0.536538 | 0.576921 | 0.000000 |
| L3_real_p001 | 4 | 2000.0000 | 0.038901 | 0.439641 | 0.627200 | 0.000000 |
| L3_shuffled_p001 | 4 | 2000.0000 | 0.036578 | 0.499134 | 0.564802 | 0.000000 |

## Notes
- All requested metrics are computed and present.
- `collapse_event_count` is all-zero in this run because no collapse/extinction event occurred in these 5 conditions at this scale.
