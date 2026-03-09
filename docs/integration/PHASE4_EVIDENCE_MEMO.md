# Phase 4 Evidence Memo

## 1) Anti-god-mode guard/assert/test evidence
- Guard validation exists for forbidden paths in `MemoryAccessGuard`:
  - cell -> archive forbidden
  - archive write -> cell memory forbidden
  - archive write -> cell strategy forbidden
  - sampling probability bounds enforced
- Runtime assertions in world loop still check forbidden request forms through guard paths.
- Unit tests exist for guard rejection and sampling-probability enforcement.

## 2) Which conditions use real archive content
- `baseline_full` and `L3_real_p001` use archive records as-is before lesson compression.
- `L3_off` disables archive path.
- `no_L2` disables lineage inheritance/sampling flow.

## 3) How `L3_shuffled_p001` is shuffled
- Sampling flow remains active and uses same sampling policy.
- After selecting a sampled record, context fields are perturbed (e.g., lineage/event label) before compression to lesson.
- This preserves channel existence while breaking original archive context alignment.

## 4) Why shuffled vs real keeps bandwidth comparable
- Both modes use the same attempt gate, same sampling probability policy (`p=0.01`), and same lesson cap constraints.
- Differences are introduced only at content-mapping step before lesson compression.

## 5) Remaining weak evidence
- Need stronger integration-level regression that proves no direct archive injection path is reachable across all sentinel modes.
- Need threshold-based interpretation criteria from atlas for real-vs-shuffled deltas.
