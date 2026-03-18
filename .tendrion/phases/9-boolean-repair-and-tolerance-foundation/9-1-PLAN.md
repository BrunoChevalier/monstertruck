---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-core/src/tolerance.rs
  - monstertruck-solid/src/fillet/edge_select.rs
  - monstertruck-core/tests/tolerance_policy.rs
autonomous: true
must_haves:
  truths:
    - "monstertruck-core tolerance module has a module-level doc comment explaining the tolerance policy"
    - "monstertruck-solid fillet/edge_select.rs imports TOLERANCE from monstertruck-core instead of hardcoding 1.0e-6"
    - "A tolerance_policy integration test in monstertruck-core pins TOLERANCE == 1.0e-6 and verifies OperationTolerance::from_global()"
    - "cargo clippy --all-targets passes without new warnings"
  artifacts:
    - path: "monstertruck-core/src/tolerance.rs"
      provides: "Tolerance policy documentation and canonical constants"
      min_lines: 120
      contains: "Numeric Tolerance Policy"
    - path: "monstertruck-solid/src/fillet/edge_select.rs"
      provides: "Fillet edge selection using shared TOLERANCE constant instead of hardcoded 1.0e-6"
      min_lines: 60
      contains: "use monstertruck_core::tolerance::TOLERANCE"
    - path: "monstertruck-core/tests/tolerance_policy.rs"
      provides: "Regression tests pinning TOLERANCE value and documenting canonical import"
      min_lines: 15
      contains: "tolerance_value_is_1e_minus_6"
  key_links:
    - from: "monstertruck-solid/src/fillet/edge_select.rs"
      to: "monstertruck-core/src/tolerance.rs"
      via: "TOLERANCE constant import replaces hardcoded magic number"
      pattern: "use monstertruck_core::tolerance::TOLERANCE"
---

<objective>
Establish a documented numeric tolerance policy in monstertruck-core and eliminate the hardcoded 1.0e-6 in monstertruck-solid's fillet edge_select. Scope is limited to monstertruck-core and monstertruck-solid fillet files; other crates with hardcoded tolerances (monstertruck-geometry, monstertruck-meshing, etc.) are out of scope for this plan.
</objective>

<execution_context>
@AGENTS.md
</execution_context>

<context>
@monstertruck-core/src/tolerance.rs
@monstertruck-solid/src/fillet/edge_select.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Document tolerance policy and replace hardcoded value in fillet</name>
  <files>monstertruck-core/src/tolerance.rs, monstertruck-solid/src/fillet/edge_select.rs</files>
  <action>
**Part A: Add policy documentation to tolerance module.**

Add a module-level doc comment to the very top of `monstertruck-core/src/tolerance.rs`, before the existing `use` statements:

```rust
//! # Numeric Tolerance Policy
//!
//! All crates in the monstertruck workspace SHOULD import [`TOLERANCE`] and [`TOLERANCE2`]
//! from this module rather than hardcoding `1.0e-6`. This ensures a single source of truth
//! for the geometric coincidence threshold.
//!
//! ## Constants
//!
//! - [`TOLERANCE`] (`1.0e-6`): General geometric coincidence threshold. Use when comparing
//!   whether two points, distances, or parameter values are "the same."
//! - [`TOLERANCE2`] (`TOLERANCE * TOLERANCE`): Squared-order tolerance for squared-distance
//!   comparisons, avoiding an unnecessary `sqrt`.
//!
//! ## Traits
//!
//! - [`Tolerance`]: Provides `.near()` (within [`TOLERANCE`]) and `.near2()` (within
//!   [`TOLERANCE2`]) methods on any type implementing `AbsDiffEq<Epsilon = f64>`.
//! - [`Origin`]: Extends [`Tolerance`] with `.so_small()` and `.so_small2()` for
//!   near-zero checks.
//!
//! ## Per-operation tracking
//!
//! - [`OperationTolerance`]: Tracks accumulated numeric error across chained operations
//!   (boolean -> fillet -> tessellation). Use `OperationTolerance::from_global()` to
//!   start a pipeline from the canonical TOLERANCE.
//!
//! ## When to use a local constant instead
//!
//! Domain-specific constants that coincidentally share the same numeric magnitude
//! (e.g., finite-difference step sizes in `t_mesh.rs`, STEP file format values,
//! fillet continuity angle thresholds) may remain as local constants with a comment
//! explaining why they are not [`TOLERANCE`].
```

Do NOT change any existing code, constants, traits, macros, or tests in the file. Only add the doc comment block at the top.

**Part B: Replace hardcoded tolerance in fillet edge_select.**

In `monstertruck-solid/src/fillet/edge_select.rs`:
1. Add `use monstertruck_core::tolerance::TOLERANCE;` after the existing import block (after line 8, before `type Result<T>`).
2. Replace line 67: `let tolerance = 1.0e-6;` with `let tolerance = TOLERANCE;`.

No other changes. The local variable name `tolerance` and all downstream usage remain identical.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core --lib` to verify no tolerance module regressions. Run `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` to verify fillet tests pass. Run `cargo clippy -p monstertruck-core -p monstertruck-solid --all-targets` to check for warnings.</verify>
  <done>Tolerance policy documented in monstertruck-core; hardcoded 1.0e-6 in fillet/edge_select.rs replaced with TOLERANCE import.</done>
</task>

<task type="auto">
  <name>Task 2: Add tolerance policy regression tests</name>
  <files>monstertruck-core/tests/tolerance_policy.rs</files>
  <action>
Create a new integration test file `monstertruck-core/tests/tolerance_policy.rs`:

```rust
//! Tolerance policy regression tests.
//!
//! These tests pin the canonical tolerance values and verify the import path
//! that all workspace crates should use. If TOLERANCE needs to change, update
//! these tests intentionally after assessing downstream impact.

use monstertruck_core::tolerance::{OperationTolerance, Tolerance, TOLERANCE, TOLERANCE2};

#[test]
fn tolerance_value_is_1e_minus_6() {
    assert_eq!(TOLERANCE, 1.0e-6, "TOLERANCE must be 1.0e-6");
}

#[test]
fn tolerance2_is_tolerance_squared() {
    assert_eq!(TOLERANCE2, TOLERANCE * TOLERANCE);
}

#[test]
fn operation_tolerance_from_global_uses_tolerance() {
    let op = OperationTolerance::from_global();
    assert_eq!(op.base(), TOLERANCE);
    assert_eq!(op.accumulated_error(), 0.0);
    assert_eq!(op.operation_count(), 0);
}

#[test]
fn near_trait_uses_tolerance() {
    let a: f64 = 1.0;
    let b: f64 = 1.0 + TOLERANCE * 0.5;
    assert!(a.near(&b), "Values within TOLERANCE should be near");

    let c: f64 = 1.0 + TOLERANCE * 2.0;
    assert!(!a.near(&c), "Values beyond TOLERANCE should not be near");
}
```

This test file ensures:
- The numeric value of TOLERANCE is pinned and cannot drift silently.
- OperationTolerance integrates with the global constant.
- The Tolerance trait's `.near()` threshold aligns with TOLERANCE.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core -E 'test(tolerance_policy)' --no-fail-fast` to verify all 4 tests pass.</verify>
  <done>Tolerance policy regression tests created and passing in monstertruck-core.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-core --lib` passes (no regressions in existing tolerance code).
2. `cargo nextest run -p monstertruck-core -E 'test(tolerance_policy)'` passes (new policy tests green).
3. `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` passes (fillet edge_select still works).
4. `cargo clippy --all-targets -- -W warnings` produces no new warnings.
5. No hardcoded `1.0e-6` remains in `monstertruck-solid/src/fillet/edge_select.rs`.
6. `monstertruck-core/src/tolerance.rs` starts with a module-level `//! # Numeric Tolerance Policy` doc comment.
</verification>

<success_criteria>
- Tolerance policy is documented in monstertruck-core/src/tolerance.rs module doc
- monstertruck-solid fillet/edge_select.rs uses TOLERANCE import instead of hardcoded 1.0e-6
- Regression tests pin TOLERANCE == 1.0e-6 and verify OperationTolerance integration
- All existing tests continue to pass
- Scope correctly limited to monstertruck-core and monstertruck-solid fillet files only
- Addresses requirement TEST-02 for the crates in scope
</success_criteria>

<output>
After completion, create `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-1-SUMMARY.md`
</output>
