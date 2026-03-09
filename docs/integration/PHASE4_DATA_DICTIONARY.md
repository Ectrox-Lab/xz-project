# Phase 4 Data Dictionary (required_now fields)

| field | computation source | expected range | when it can be 0 | placeholder possible? | current truth status |
|---|---|---|---|---|---|
| `archive_sample_attempts` | per-tick counter around L3 sampling attempt in reproduction path; aggregated in run summary and exported to `per_run.csv` | `>=0` | `no_L2`, `L3_off`, or zero births | no | true-value |
| `archive_sample_successes` | successful draws from archive sampler; aggregated/exported | `>=0` and `<= attempts` | same as above, or no records/probability misses | no | true-value |
| `archive_influenced_births` | increments when archive lesson is applied (and in overpowered mode branch) | `>=0` | no successful sample or mode disabled | no | true-value |
| `lineage_diversity` | `distinct_lineages / population` per tick, averaged per run | `[0,1]` in current impl | low diversity collapse or empty pop edge | no | true-value |
| `top1_lineage_share` | largest lineage count / population per tick, averaged | `[0,1]` | 0 at empty-pop edge | no | true-value |
| `strategy_entropy` | entropy over lineage preferred strategy distribution per tick, averaged | `>=0` | all cells same strategy (e.g., `no_L2`) | no | true-value |
| `collapse_event_count` | extinction event counter from world loop | integer `>=0` | no collapse occurred in run | no (zero can be valid) | true-value |

## Column presence validation
All five core condition `per_run.csv` files include all seven required_now columns.
