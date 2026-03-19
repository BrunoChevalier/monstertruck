---
phase: 16-tolerance-foundation-and-api-safety
plan: 1
tags: [tolerance, constants, refactor, core]
key-files:
  - monstertruck-core/src/tolerance_constants.rs
  - monstertruck-core/src/lib.rs
  - monstertruck-core/src/tolerance.rs
  - monstertruck-solid/src/fillet/integrate.rs
  - monstertruck-solid/src/transversal/loops_store/mod.rs
  - monstertruck-solid/src/transversal/integrate/mod.rs
  - monstertruck-core/tests/tolerance_constants.rs
decisions: []
metrics:
  tests_added: 8
  tests_passing: 57 (monstertruck-core), 108 (monstertruck-solid, 6 pre-existing failures excluded)
  files_created: 2
  files_modified: 5
---

## What was built

- **monstertruck-core/src/tolerance_constants.rs**: New module exporting six centralized tolerance constants (SNAP_TOLERANCE, VERTEX_MERGE_TOLERANCE, TESSELLATION_TOLERANCE, PERIODIC_CLOSURE_RATIO, G1_ANGLE_TOLERANCE, G2_CURVATURE_TOLERANCE) with full doc comments.
- **monstertruck-core/src/lib.rs**: Added `pub mod tolerance_constants` declaration.
- **monstertruck-core/src/tolerance.rs**: Added cross-reference documentation pointing to the new tolerance_constants module.
- **monstertruck-core/tests/tolerance_constants.rs**: 8 unit tests verifying all constant values.
- **monstertruck-solid/src/fillet/integrate.rs**: Removed local G1_ANGLE_TOLERANCE and G2_CURVATURE_TOLERANCE constants; replaced with centralized imports.
- **monstertruck-solid/src/transversal/loops_store/mod.rs**: Replaced three hardcoded tolerance expressions (10.0 * TOLERANCE, 100.0 * TOLERANCE) with SNAP_TOLERANCE and VERTEX_MERGE_TOLERANCE imports.
- **monstertruck-solid/src/transversal/integrate/mod.rs**: Replaced `100.0 * TOLERANCE` with VERTEX_MERGE_TOLERANCE import.

## Deviations

- 6 pre-existing test failures in monstertruck-solid fillet tests (generic_fillet_*, test_unit_circle) -- not caused by this plan.

## Self-check

- All 57 monstertruck-core tests pass (including 8 new tolerance_constants tests).
- All 108 non-pre-existing monstertruck-solid tests pass.
- No hardcoded `10.0 * TOLERANCE` or `100.0 * TOLERANCE` in refactored files.
- No local G1/G2 constants remain in fillet/integrate.rs.
- Clippy clean on lib targets (pre-existing warnings only).
- Doc cross-reference link resolves correctly.
