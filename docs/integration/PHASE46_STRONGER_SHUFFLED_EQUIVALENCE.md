# Phase 4.6 Stronger Shuffled Equivalence Note

## What is the same between `L3_real_p001` and `L3_shuffled_p001`
- Same sampling gate location in reproduction flow.
- Same sampling policy probability path (`p=0.01` policy object).
- Same per-life sample cap logic and same lesson cap constraint.
- Same archive queue/read pipeline and same counter instrumentation fields.

## What is different
- In shuffled mode only, sampled record context is remapped before compression:
  - lineage id randomized
  - event label replaced with shuffled marker

## How channel equivalence is enforced today
- Both modes increment the same attempt/success counters in the same code path.
- Both apply a single compressed lesson per successful sampled record under identical caps.
- Both report the same bandwidth-related fields (`archive_sample_attempts`, `archive_sample_successes`, `archive_influenced_births`).

## Remaining gap to full guarantee
- There is still no dedicated invariant/regression test that enforces per-seed near-equality tolerance for bandwidth counters between real and shuffled.
- Current evidence is stronger than earlier phases (fresh 5000/8 rerun + same-path instrumentation) but still observational, not a formal invariant test.
