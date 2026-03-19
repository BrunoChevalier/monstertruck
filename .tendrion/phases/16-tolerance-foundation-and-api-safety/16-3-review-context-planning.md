# Review Context: Plan 16-3 (Planning Review, Round 1)

## Plan Under Review

File: `.tendrion/phases/16-tolerance-foundation-and-api-safety/16-3-PLAN.md`

---
phase: 16-tolerance-foundation-and-api-safety
plan: 3
type: execute
wave: 2
depends_on: ["16-2"]
files_modified:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
autonomous: true
must_haves:
  truths:
    - "User calls deprecated gordon() and gets identical results to try_gordon() with default options"
    - "User calls deprecated skin() and gets identical results to try_skin() with default options"
    - "User calls deprecated sweep_rail() and gets identical results to try_sweep_rail() with matching options"
    - "User calls deprecated birail1() and gets identical results to try_birail1() with matching options"
    - "User calls deprecated birail2() and gets identical results to try_birail2() with matching options"
    - "Each deprecated method body is under 10 lines (delegation only)"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      provides: "Deprecated methods refactored to delegate to try_* variants"
      min_lines: 2500
      contains: "try_gordon"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/tests/try_gordon_skin_test.rs"
      via: "deprecated_gordon_still_works and deprecated_skin_still_works tests"
      pattern: "deprecated_gordon_still_works"

### Objective

Refactor all deprecated surface constructor methods (gordon, skin, sweep_rail, birail1, birail2) to delegate to their try_* counterparts instead of maintaining independent algorithm implementations, eliminating ~300 lines of duplicated logic.

### Tasks

**Task 1: Refactor gordon() and skin() to delegate**
- Files: monstertruck-geometry/src/nurbs/bspline_surface.rs
- Replace full implementations of deprecated gordon() and skin() with thin delegation wrappers calling try_gordon() and try_skin() respectively
- Verify: Run cargo test for deprecated_gordon_still_works and deprecated_skin_still_works

**Task 2: Refactor sweep_rail(), birail1(), birail2() to delegate**
- Files: monstertruck-geometry/src/nurbs/bspline_surface.rs
- Replace full implementations with thin delegation wrappers calling try_sweep_rail(), try_birail1(), try_birail2()
- Forward n_sections parameter to options struct
- Verify: Run cargo test for sweep_rail and birail tests

**Task 3: Verify full test suite and check line reduction**
- Run full test suite across monstertruck-geometry, monstertruck-modeling, monstertruck-solid
- Verify deprecated methods are thin wrappers (under 10 lines each)
- Check #[allow(deprecated)] only on test functions
- Verify no other files call deprecated methods directly

### Verification

1. cargo test -p monstertruck-geometry passes
2. cargo test -p monstertruck-modeling passes
3. cargo test -p monstertruck-solid passes
4. deprecated_gordon_still_works test passes
5. deprecated_skin_still_works test passes
6. Each deprecated method body is under 10 lines
7. No duplicated algorithm logic between deprecated and try_* methods
8. #[allow(deprecated)] only appears on test functions

### Success Criteria

- TOLAPI-03 is fully satisfied
- ~250-300 lines of duplicated algorithm code removed
- All existing tests pass with identical behavior
- Deprecated methods panic with informative messages on error cases

---

## Sibling Plans

| Plan ID | Wave | Objective |
|---------|------|-----------|
| 16-1 | 1 | Centralize shared tolerance constants in monstertruck-core with documented defaults |
| 16-2 | 1 | Add #[non_exhaustive] to all five surface constructor option structs and update downstream struct literal construction sites |

Full sibling plans can be read from `.tendrion/phases/16-tolerance-foundation-and-api-safety/{sibling_plan_id}-PLAN.md` if cross-plan analysis is needed.

---

## Roadmap: Phase 16

### Phase 16: Tolerance Foundation and API Safety
**Goal**: All tolerance constants are centralized in monstertruck-core and surface constructor option structs are safe for future extension
**Depends on**: Phase 15
**Requirements**: TOLAPI-01, TOLAPI-02, TOLAPI-03
**Success Criteria** (what must be TRUE):
  1. A `tolerance_constants` module in monstertruck-core exports SNAP_TOLERANCE, VERTEX_MERGE_TOLERANCE, TESSELLATION_TOLERANCE, PERIODIC_CLOSURE_RATIO, G1_ANGLE_TOLERANCE, and G2_CURVATURE_TOLERANCE with defaults that preserve existing behavior
  2. All surface constructor option structs (GordonOptions, SkinOptions, SweepRailOptions, Birail1Options, Birail2Options) have `#[non_exhaustive]` and downstream code still compiles
  3. The deprecated `gordon()` function delegates to `try_gordon()` with no independent implementation logic
  4. All existing tests pass without behavioral changes from tolerance centralization

---

## Review Parameters

- **Round:** 1 of 3
- **Review Type:** planning
- **Phase:** 16
- **Plan ID:** 16-3
