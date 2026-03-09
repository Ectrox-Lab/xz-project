# Phase 3 Blockers

## Conditions not executed in this run
- Optional conditions not executed in this pass:
  - `no_L1`
  - `L3_overpowered_direct`

## CSV issues
- No schema-missing issue found in required_now 7 columns.
- Some fields may be low-signal at short horizon (e.g., collapse count remained zero).

## Anti-god-mode evidence gaps
- Guard tests exist for forbidden paths, but dedicated integration regression for "direct archive injection unreachable in all sentinel modes" is still weak.
- Overpowered mode is implemented as explicit negative control, but comparative threshold tests vs normal modes are not yet codified.

## Needs atlas-hec-v2.1 semantic alignment
- Effect-size acceptance rule for `L3_real_p001` vs `L3_shuffled_p001`.
- Confirm final semantics for `lineage_diversity` (normalized vs absolute count).
- Confirm collapse metric interpretation (`collapse_event_count`) for acceptance checks.
