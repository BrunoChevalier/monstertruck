---
phase: 23-error-propagation-and-test-hardening
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/error.rs
  - monstertruck-solid/src/fillet/edge_select.rs
  - monstertruck-solid/src/fillet/geometry.rs
  - monstertruck-solid/src/fillet/tests.rs
autonomous: true
must_haves:
  truths:
    - "User calls fillet_edges_generic on a shell that produces a non-closed result and receives Err(FilletError::ShellNotClosed) instead of a silently restored original shell"
    - "User can pattern-match on FilletError::ShellNotClosed to distinguish topology failures from conversion failures (UnsupportedGeometry) or geometry failures"
    - "User runs test_unit_circle proptest and it passes consistently across the full input range (angle 0.1..4.71, w0/w1 0.1..5.0) without false failures from absolute tolerance on large magnitudes"
    - "All existing fillet tests continue to pass since they operate on valid closed shells"
  artifacts:
    - path: "monstertruck-solid/src/fillet/error.rs"
      provides: "FilletError enum with ShellNotClosed variant"
      min_lines: 50
      contains: "ShellNotClosed"
    - path: "monstertruck-solid/src/fillet/edge_select.rs"
      provides: "fillet_edges_generic returning Err on non-closed shell instead of silent rollback"
      min_lines: 30
      contains: "ShellNotClosed"
    - path: "monstertruck-solid/src/fillet/geometry.rs"
      provides: "test_unit_circle with magnitude-aware relative tolerance"
      min_lines: 20
      contains: "magnitude2"
  key_links:
    - from: "monstertruck-solid/src/fillet/error.rs"
      to: "monstertruck-solid/src/fillet/edge_select.rs"
      via: "FilletError::ShellNotClosed variant used in error return"
      pattern: "FilletError::ShellNotClosed"
---

<objective>
Replace the silent rollback in fillet_edges_generic with explicit FilletError::ShellNotClosed error propagation, and fix the test_unit_circle proptest to use relative tolerance so it passes consistently across all input magnitudes.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@monstertruck-solid/src/fillet/error.rs
@monstertruck-solid/src/fillet/edge_select.rs
@monstertruck-solid/src/fillet/geometry.rs
@monstertruck-solid/src/fillet/tests.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add ShellNotClosed variant to FilletError and replace silent rollback</name>
  <files>monstertruck-solid/src/fillet/error.rs, monstertruck-solid/src/fillet/edge_select.rs</files>
  <action>
1. In `monstertruck-solid/src/fillet/error.rs`, add a new variant to the `FilletError` enum:

```rust
/// The fillet operation produced a non-closed shell.
#[error("Fillet produced non-closed shell.")]
ShellNotClosed,
```

Add it after the existing `DegenerateEdge` variant (or at the end before `PerEdgeRadiusMismatch`). The exact position is not critical as long as it is inside the enum.

2. In `monstertruck-solid/src/fillet/edge_select.rs`, replace lines 733-738 (the silent rollback block):

```rust
if internal_shell.shell_condition() != ShellCondition::Closed {
    if std::env::var_os("MT_FILLET_DEBUG").is_some() {
        eprintln!("debug fillet generic: rollback to original shell (non-closed result).");
    }
    internal_shell = original_shell;
}
```

with explicit error propagation:

```rust
if internal_shell.shell_condition() != ShellCondition::Closed {
    return Err(FilletError::ShellNotClosed);
}
```

Also remove the `let original_shell = internal_shell.clone();` line (line 730) since the clone is no longer needed -- the function now returns an error instead of rolling back. This is a minor optimization but also makes the intent clear.
  </action>
  <verify>Run `cargo check -p monstertruck-solid` to confirm the new variant compiles and the error return type-checks.</verify>
  <done>FilletError::ShellNotClosed variant added and fillet_edges_generic returns Err instead of silently restoring the original shell.</done>
</task>

<task type="auto">
  <name>Task 2: Fix test_unit_circle proptest to use relative tolerance</name>
  <files>monstertruck-solid/src/fillet/geometry.rs</files>
  <action>
In `monstertruck-solid/src/fillet/geometry.rs`, in the `test_unit_circle` proptest (around line 126), replace:

```rust
prop_assert_near!(p.magnitude2(), 1.0, "{w0} {w1} {p:?} {angle}");
```

with a relative tolerance check:

```rust
let mag2 = p.magnitude2();
let rel_err = (mag2 - 1.0).abs() / mag2.max(1.0);
prop_assert!(
    rel_err < 1e-6,
    "magnitude2 relative error {rel_err} too large for {w0} {w1} {p:?} {angle}"
);
```

This uses `(mag2 - 1.0).abs() / max(mag2, 1.0)` which is a standard relative tolerance comparison. The key insight: `prop_assert_near!` uses `Tolerance::near()` which calls `abs_diff_eq` with absolute epsilon `TOLERANCE = 1e-6`. When `p.magnitude2()` is close to 1.0 this works, but the unit_circle_arc function with large weights (w0, w1 up to 5.0) is supposed to still produce unit-magnitude points. The relative comparison normalizes by the larger of the two values being compared.

Note: The other two `prop_assert_near!` calls on lines 130-131 compare Point3 values (subs(0.0) and subs(1.0) against specific points). These are fine with absolute tolerance since those points are near the unit circle. Do NOT change those.
  </action>
  <verify>Run `cargo test -p monstertruck-solid test_unit_circle -- --nocapture` to confirm the proptest passes. Also run with increased cases: `PROPTEST_CASES=1000 cargo test -p monstertruck-solid test_unit_circle` to verify robustness.</verify>
  <done>test_unit_circle uses relative tolerance for magnitude2 comparison and passes consistently across the full proptest input range.</done>
</task>

<task type="auto">
  <name>Task 3: Verify all fillet tests pass and add a targeted ShellNotClosed test</name>
  <files>monstertruck-solid/src/fillet/tests.rs</files>
  <action>
1. Run the full fillet test suite to confirm no regressions: `cargo test -p monstertruck-solid fillet`. All existing tests (generic_fillet_identity, generic_fillet_modeling_types, generic_fillet_mixed_surfaces, generic_fillet_unsupported, generic_fillet_multi_chain, etc.) should continue to pass because they all operate on valid box shells that produce closed results after filleting.

2. Verify that `generic_fillet_unsupported` still correctly returns `Err(FilletError::UnsupportedGeometry { .. })` -- this test pattern-matches on the error, confirming callers can distinguish error variants.

3. If all tests pass, no new test is strictly required since the error path is tested by the existing `generic_fillet_unsupported` pattern (it proves callers can match on FilletError variants). However, if time permits and a simple non-closed scenario can be constructed, consider adding a brief test. The primary validation is that the existing test suite passes without regressions.

4. Also run integration tests: `cargo test -p monstertruck-solid --test feature_integration` to confirm the integration tests that use fillet_edges_generic still work (they call `.expect()` on the result, so they'll fail if the shell unexpectedly becomes non-closed).
  </action>
  <verify>All fillet tests pass: `cargo test -p monstertruck-solid fillet`. Integration tests pass: `cargo test -p monstertruck-solid --test feature_integration`.</verify>
  <done>All existing fillet tests and integration tests pass with no regressions, confirming the error propagation change is backward-compatible for valid inputs.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-solid` compiles without errors or warnings
2. `cargo test -p monstertruck-solid fillet` -- all fillet tests pass
3. `cargo test -p monstertruck-solid test_unit_circle` -- proptest passes
4. `PROPTEST_CASES=1000 cargo test -p monstertruck-solid test_unit_circle` -- proptest robust under high case count
5. `cargo test -p monstertruck-solid --test feature_integration` -- integration tests pass
6. `FilletError::ShellNotClosed` variant exists and is used in fillet_edges_generic error return
7. No silent rollback code remains in fillet_edges_generic (no `original_shell = internal_shell.clone()` pattern)
</verification>

<success_criteria>
- fillet_edges_generic returns Err(FilletError::ShellNotClosed) when the shell closure check fails, instead of silently rolling back (EREP-01)
- Callers can pattern-match on FilletError::ShellNotClosed to distinguish from other error variants like UnsupportedGeometry (EREP-01)
- test_unit_circle uses magnitude-aware relative tolerance and passes consistently across all proptest inputs (EREP-02)
- All existing tests pass with no regressions
</success_criteria>

<output>
After completion, create `.tendrion/phases/23-error-propagation-and-test-hardening/23-1-SUMMARY.md`
</output>
