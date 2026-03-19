---
phase: 16
title: tolerance-foundation-and-api-safety
status: complete
plans_total: 3
plans_complete: 3
tdd_compliance: 0% (strict mode, missing REFACTOR commits; RED/GREEN cycles present)
deviations_auto_fix: 43
deviations_approval_needed: 0
---

## What Was Built

**Plan 16-1 (TOLAPI-01):** Created `monstertruck-core/src/tolerance_constants.rs` exporting six centralized tolerance constants (SNAP_TOLERANCE, VERTEX_MERGE_TOLERANCE, TESSELLATION_TOLERANCE, PERIODIC_CLOSURE_RATIO, G1_ANGLE_TOLERANCE, G2_CURVATURE_TOLERANCE) with full doc comments. Added `pub mod tolerance_constants` to `lib.rs`. Updated four call sites in monstertruck-solid (fillet/integrate.rs, transversal/loops_store/mod.rs, transversal/integrate/mod.rs) to import from the new module. Added 8 unit tests in `monstertruck-core/tests/tolerance_constants.rs`.

**Plan 16-2 (TOLAPI-02):** Added `#[non_exhaustive]` to all five surface constructor option structs in `monstertruck-geometry/src/nurbs/surface_options.rs` (GordonOptions, SkinOptions, SweepRailOptions, Birail1Options, Birail2Options). Updated integration test construction in `try_surface_constructors_test.rs` (7 sites) and `monstertruck-modeling/src/builder.rs` (1 site) to `Default::default()` + field mutation pattern.

**Plan 16-3 (TOLAPI-03):** Replaced full algorithm bodies of all five deprecated surface constructors (`skin()`, `gordon()`, `sweep_rail()`, `birail1()`, `birail2()`) with thin delegation wrappers in `bspline_surface.rs`. Removed 225 lines of duplicated logic. Added 6 characterization tests in `monstertruck-geometry/tests/deprecated_delegation_test.rs` verifying deprecated methods produce identical output to try_* methods.

## Requirement Coverage

| Requirement | Plan | Status |
|-------------|------|--------|
| TOLAPI-01 | 16-1 | Covered — tolerance_constants.rs exists with all 6 constants; 8 tests pass |
| TOLAPI-02 | 16-2 | Covered — all 5 option structs have `#[non_exhaustive]`; 46 tests pass |
| TOLAPI-03 | 16-3 | Covered — gordon() delegates to try_gordon() (body: 3 lines); 234 tests pass |

## Test Results

- monstertruck-core: 57 tests passing (8 new tolerance_constants tests)
- monstertruck-geometry: 234 tests passing (6 new delegation characterization tests)
- monstertruck-modeling: 98 tests passing
- monstertruck-solid: 127/134 pass; 7 pre-existing failures unrelated to phase changes

## Deviations

- Total auto-fix deviations (workspace): 43 (from DEVIATIONS.md)
- Approval-needed: 0
- Phase-specific logged deviations:
  - Plan 16-2: TDD exemption for `#[non_exhaustive]` (compile-time attribute, no runtime behavior)
  - Plan 16-3: RED tests passed immediately (deprecated and try_* methods already produced identical output)

## TDD Compliance

TDD level: strict. Compliance: 0% per td-tools (all 3 cycles missing REFACTOR commit). RED/GREEN phases were executed; REFACTOR commits were not separately tagged. Deviations logged in SUMMARY.md files for plans 16-2 and 16-3.

## Decisions Made

- Centralized constant values chosen to exactly preserve prior hardcoded expressions (10.0 * TOLERANCE, 100.0 * TOLERANCE, 0.01, 0.0175, 0.10) to ensure zero behavioral change.
- `#[non_exhaustive]` applied to all five option structs (not just gordon) for consistency and future-proofing.
- All five deprecated constructors refactored to delegation (not just gordon()) per plan scope expansion.
