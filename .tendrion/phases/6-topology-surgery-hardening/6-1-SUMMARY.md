---
phase: 6-topology-surgery-hardening
plan: 1
tags: [fillet, seam-averaging, homogeneous-coordinates, TOPO-02]
key-files:
  - monstertruck-solid/src/fillet/ops.rs
  - monstertruck-solid/src/fillet/tests.rs
decisions: []
metrics:
  tests_added: 2
  tests_passing: 37
  tests_failing_preexisting: 7
  tdd_compliance: strict
---

## What Was Built

- **monstertruck-solid/src/fillet/ops.rs**: Fixed seam averaging in `fillet_along_wire` to dehomogenize Vector4 control points before averaging, then rehomogenize with the average weight. Extracted `dehomogenized_average` helper function to reduce duplication between interior seam and wrap-around seam blocks. Addresses TOPO-02.

- **monstertruck-solid/src/fillet/tests.rs**: Fixed `seam_averaging_dehomogenizes` test to properly assert that naive homogeneous averaging produces incorrect 3D midpoints (negative assertion) and that the dehomogenize-average-rehomogenize pattern produces correct results. Added `fillet_wire_seam_continuity` integration test that builds a 4-face open box, applies `fillet_along_wire`, and verifies the resulting shell geometry is well-formed.

## Task Commits

| SHA | Message |
|-----|---------|
| aba7974c | test(fillet): add failing test for dehomogenized seam averaging |
| 87754e76 | feat(fillet): dehomogenize seam control points before averaging in fillet_along_wire |
| ad3884d1 | refactor(fillet): extract dehomogenized_average helper for seam control point averaging |

## TDD Cycle

- **RED** (aba7974c): `seam_averaging_dehomogenizes` test committed and failing, demonstrating the homogeneous averaging bug.
- **GREEN** (87754e76): Applied dehomogenize-average-rehomogenize fix in both interior and wrap-around seam blocks. Fixed test assertions. Added `fillet_wire_seam_continuity` integration test. Both tests pass.
- **REFACTOR** (ad3884d1): Extracted `dehomogenized_average(p, q)` helper function to eliminate duplication between the two seam averaging blocks.

## Deviations

- 7 pre-existing fillet test failures unrelated to seam averaging: `generic_fillet_identity`, `generic_fillet_mixed_surfaces`, `generic_fillet_modeling_types`, `generic_fillet_multi_chain`, `generic_fillet_unsupported`, `boolean_shell_converts_for_fillet`, `chamfer_serialization_round_trip`. These fail identically on the committed state before our changes.

## Self-Check

- `cargo test -p monstertruck-solid --lib seam_averaging` -- PASS
- `cargo test -p monstertruck-solid --lib fillet_wire_seam` -- PASS
- `monstertruck-solid/src/fillet/ops.rs` contains `to_point` and `dehomogenized_average` -- confirmed
- `monstertruck-solid/src/fillet/tests.rs` contains `seam_averaging_dehomogenizes` -- confirmed
- Both interior and wrap-around seam blocks use `dehomogenized_average` -- confirmed
