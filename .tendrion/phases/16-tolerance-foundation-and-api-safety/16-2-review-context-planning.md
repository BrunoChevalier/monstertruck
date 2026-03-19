# Review Context: Plan 16-2 (Planning Review)

## Round Info
- **Round:** 1 of 3
- **Review type:** planning
- **Phase:** 16 - Tolerance Foundation and API Safety

## Plan Under Review

### 16-2-PLAN.md

---
phase: 16-tolerance-foundation-and-api-safety
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/tests/surface_types_test.rs
  - monstertruck-geometry/tests/try_surface_constructors_test.rs
  - monstertruck-geometry/tests/try_gordon_skin_test.rs
  - monstertruck-modeling/src/builder.rs
autonomous: true
must_haves:
  truths:
    - "User cannot construct GordonOptions with struct literal from outside monstertruck-geometry crate"
    - "User cannot construct SkinOptions with struct literal from outside monstertruck-geometry crate"
    - "User cannot construct SweepRailOptions with struct literal from outside monstertruck-geometry crate"
    - "User cannot construct Birail1Options with struct literal from outside monstertruck-geometry crate"
    - "User cannot construct Birail2Options with struct literal from outside monstertruck-geometry crate"
    - "User constructs all option structs via Default::default() and field setters, and all existing tests pass"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/surface_options.rs"
      provides: "All five option structs with #[non_exhaustive] attribute"
      min_lines: 60
      contains: "#[non_exhaustive]"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/surface_options.rs"
      to: "monstertruck-modeling/src/builder.rs"
      via: "Option struct imports must still compile with non_exhaustive"
      pattern: "SweepRailOptions"
    - from: "monstertruck-geometry/src/nurbs/surface_options.rs"
      to: "monstertruck-geometry/tests/try_surface_constructors_test.rs"
      via: "Test struct literal construction updated to use Default + field override"
      pattern: "Default::default"

**Objective:** Add #[non_exhaustive] to all five surface constructor option structs (GordonOptions, SkinOptions, SweepRailOptions, Birail1Options, Birail2Options) and update all downstream struct literal construction sites to use Default::default() with field overrides via the `..` syntax.

**Tasks:**
1. Task 1: Add #[non_exhaustive] to option structs (surface_options.rs)
2. Task 2: Fix downstream crate struct literal construction (builder.rs)
3. Task 3: Verify cross-crate protection and update doc examples

**Verification:**
1. cargo check -p monstertruck-geometry passes
2. cargo check -p monstertruck-modeling passes
3. cargo check (full workspace) passes
4. cargo test -p monstertruck-geometry passes
5. cargo test -p monstertruck-modeling passes
6. All five structs have #[non_exhaustive] attribute
7. No struct literal construction of these types exists outside monstertruck-geometry
8. Doc examples compile and demonstrate correct construction pattern

**Success Criteria:**
- TOLAPI-02 is fully satisfied
- All downstream code compiles and tests pass
- Future field additions will not cause breaking changes for external consumers

---

## Sibling Plans

| Plan ID | Wave | Objective |
|---------|------|-----------|
| 16-1 | 1 | Centralize shared tolerance constants in monstertruck-core with documented defaults |
| 16-3 | 2 | Refactor all deprecated surface constructor methods to delegate to their try_* counterparts |

Full sibling plans can be read from .tendrion/phases/16-tolerance-foundation-and-api-safety/{sibling_plan_id}-PLAN.md if cross-plan analysis is needed.

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
