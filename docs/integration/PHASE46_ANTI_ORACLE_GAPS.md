# Phase 4.6 Anti-Oracle Gaps

## Existing anti-god-mode evidence
- Access guard rejects:
  - cell direct archive access
  - archive write to cell memory
  - archive write to cell strategy
  - over-probability archive sampling requests
- Runtime guarded sampling path validates lineage->archive request before sampling.
- Overpowered control mode is explicitly separated in sentinel branch (`oracle_direct`) rather than hidden in normal modes.

## Missing stronger evidence
- Integration-level proof that forbidden direct archive injection remains unreachable across all modes (not only unit-level guard tests).
- Invariant test for shuffled-vs-real bandwidth equivalence.

## Blocker classification
- **Blocker**: missing atlas interpretation threshold for real-vs-shuffled adjudication.
- **Non-blocker**: absence of optional mode reruns (`no_L1`, `L3_overpowered_direct`) for this minimal phase4.6 closure.
- **Non-blocker (but recommended)**: lack of formal equivalence invariant test; current operational evidence is usable for triage but not final proof-grade.
