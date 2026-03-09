# Phase 4.5 Shuffled Control Note (`L3_real_p001` vs `L3_shuffled_p001`)

## What is shuffled
In `L3_shuffled_p001`, the sampled archive record's context mapping is altered before compression to lesson:
- sampled record lineage ID is randomized
- sampled event label is replaced with shuffled marker

## What is kept unchanged
- Same sampling gate and policy path (`p=0.01` policy)
- Same attempt opportunity in reproduction flow
- Same lesson cap constraints (`MAX_DISTILLED_LESSONS`)
- Same archive write/read pipeline outside the context-remap step

## How equivalence is maintained (rate / lesson count / bandwidth intent)
- Both real and shuffled modes pass through the same sampling counter path (`archive_sample_attempts`).
- Both count successful samples through the same success path (`archive_sample_successes`).
- Both push at most one lesson per sampled event under the same lesson-cap logic.

## Current weakest evidence point
- We do not yet have a formal invariant test proving strict per-seed equality of sampling bandwidth between real and shuffled; current evidence is observational from run outputs/counters rather than a dedicated regression assertion.
