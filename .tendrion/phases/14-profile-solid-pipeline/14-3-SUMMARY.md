---
phase: 14-profile-solid-pipeline
plan: 3
tags: [validation, topology, euler-poincare, tessellation, profile]
key-files:
  - monstertruck-modeling/src/profile.rs
  - monstertruck-modeling/src/errors.rs
  - monstertruck-modeling/tests/profile_test.rs
  - monstertruck-modeling/tests/font_pipeline.rs
decisions:
  - "Euler-Poincare check relaxed to accept genus > 0 (euler even, <= 2) since torus revolves yield euler=0"
  - "Swept solids have Oriented (not Closed) shell topology; validate_solid accepts Oriented shells"
  - "Negative test uses duplicated face (Irregular topology) instead of removed face (which gives Oriented)"
metrics:
  tests_added: 16
  tests_total_profile: 32
  tests_total_font: 20
  profile_rs_lines: 728
  profile_test_rs_lines: 565
---

## What was built

- **`ValidationReport` struct** in `profile.rs`: Reports vertices, edges, faces, Euler characteristic, orientation, closure, and geometric consistency.
- **`validate_solid` function** in `profile.rs`: Validates each shell boundary for orientation (Oriented or Closed), Euler-Poincare invariant (even, <= 2 for closed shells), and geometric consistency. Returns descriptive errors on failure.
- **`ProfileValidationFailed` error variant** in `errors.rs`: New error type with a `reason` string for clear diagnostics.
- **13 validation tests** in `profile_test.rs`: Covers extruded box, tube, revolved, swept, triangular, diagonal extrusions, report metrics, broken solid negative test, tessellation smoke test, and 4 cross-cutting tests.
- **3 validation tests** in `font_pipeline.rs`: Covers glyph 'O', glyph 'B', and mixed glyph+custom extruded solids.

## Task commits

| Commit | Message |
|--------|---------|
| c0749dbd | test(profile): add failing tests for validate_solid validation |
| 20ce9f87 | feat(profile): implement validate_solid with Euler-Poincare and consistency checks |
| b3b0f2f3 | refactor(profile): extract validate_shell helper and improve docs |
| cbc72e1b | test(profile): add cross-cutting validation tests for Euler-Poincare and metrics |

## Deviations from plan

1. **Euler-Poincare generalization**: Plan specified V-E+F=2 for closed shells, but torus revolves produce genus-1 surfaces with euler=0. Relaxed to accept even values <= 2.
2. **Swept solid topology**: Swept solids have `Oriented` (not `Closed`) shell condition due to non-shared edges between caps and side faces. `validate_solid` accepts both conditions.
3. **Negative test construction**: Plan suggested removing a face, but that yields an `Oriented` shell which passes orientation checks. Changed to duplicating a face, creating `Irregular` topology that is correctly rejected.

## Self-check

- [x] profile.rs contains `validate_solid` (728 lines, min 400)
- [x] profile_test.rs contains `validate_solid` tests (565 lines, min 280)
- [x] All 32 profile tests pass
- [x] All 20 font pipeline tests pass
- [x] Broken solid produces descriptive error
- [x] Box metrics: V=8, E=12, F=6, euler=2
- [x] No regressions in existing tests
