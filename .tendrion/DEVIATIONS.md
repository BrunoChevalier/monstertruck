# Deviations Log

**Summary:** Auto-fixes: 6 | Approval-needed: 0 | Total: 6

---
### [2026-03-08T22:36:30.656Z] [AUTO-FIX] Category: bug
**Type:** auto-fix
**Category:** bug
**Description:** monstertruck-solid lib tests have pre-existing compilation errors (7 errors in fillet/tests.rs) unrelated to IntersectionCurve changes. Cannot verify downstream solid tests as required by plan Task 3.
**Files changed:** none
**Status:** applied
### [2026-03-08T22:32:07.836Z] [AUTO-FIX] Category: dependency
**Type:** auto-fix
**Category:** dependency
**Description:** TDD RED phase adapted for proc-macro dependency swap: proc-macro crates cannot host tests that exercise their own macros. Using downstream monstertruck-modeling tests as the functional test surface. Writing a Cargo-level verification test instead.
**Files changed:** none
**Status:** applied
### [2026-03-08T22:37:18.775Z] [AUTO-FIX] Category: bug
**Type:** auto-fix
**Category:** bug
**Description:** Pre-existing compilation errors in monstertruck-solid/src/fillet/tests.rs and monstertruck-solid/src/healing/tests.rs prevent cargo test --lib from compiling. These errors exist on master and are not introduced by this plan. Tests verified via --test unwrap_safety and per-crate --lib runs that exclude the broken test modules.
**Files changed:** none
**Status:** applied

### [2026-03-09T14:30:25.731Z] [AUTO-FIX] Category: dependency
**Type:** auto-fix
**Category:** dependency
**Description:** Tasks 1 and 2 artifacts already exist: criterion workspace dep, all three Cargo.toml bench configs, and all three benchmark files (nurbs_eval.rs, tessellation.rs, boolean_ops.rs) were already in place. Only Task 3 (CI bench-check job) needed implementation.
**Files changed:** monstertruck-geometry/benches/nurbs_eval.rs, monstertruck-meshing/benches/tessellation.rs, monstertruck-solid/benches/boolean_ops.rs
**Status:** applied

### [2026-03-10T21:57:05.748Z] [AUTO-FIX] Category: design
**Type:** auto-fix
**Category:** design
**Description:** OffsetCurve/OffsetSurface impls for modeling types are behind #[cfg(test)] in shell_ops/tests.rs and unavailable to integration tests. Cannot make monstertruck-modeling a regular dep due to circular dependency. shell_then_step_export test constructs a hollow solid manually instead of calling shell_solid.
**Files changed:** monstertruck-solid/tests/feature_integration.rs
**Status:** applied

### [2026-03-10T21:59:16.566Z] [AUTO-FIX] Category: dependency
**Type:** auto-fix
**Category:** dependency
**Description:** Exempt: Integration tests for plans 3-1 through 3-4 pass immediately because all underlying features are already implemented. TDD RED phase not applicable for cross-feature integration tests validating existing functionality.
**Files changed:** monstertruck-solid/tests/feature_integration.rs
**Status:** applied
