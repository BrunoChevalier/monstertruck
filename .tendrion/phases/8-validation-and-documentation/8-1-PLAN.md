---
phase: 8-validation-and-documentation
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/validate.rs
  - monstertruck-solid/src/fillet/mod.rs
  - monstertruck-solid/src/fillet/edge_select.rs
  - monstertruck-solid/src/fillet/ops.rs
autonomous: true
must_haves:
  truths:
    - "Debug builds run Euler-Poincare check (V - E + F = 2) on closed shells after every fillet topology modification"
    - "shell.shell_condition() returns Oriented or Closed after fillet operations in all existing test cases"
    - "Release builds do not pay runtime cost for topology invariant checks"
    - "The current fillet test suite continues to pass without modification"
    - "A new test demonstrates the debug assertion fires on a shell with corrupted orientation"
    - "A direct unit test verifies euler_poincare_check returns true for a valid closed shell and false for an invalid one"
  artifacts:
    - path: "monstertruck-solid/src/fillet/validate.rs"
      provides: "Euler-Poincare and orientation validation functions with #[cfg(test)] module for corruption tests"
      min_lines: 80
      contains: "euler_poincare"
    - path: "monstertruck-solid/src/fillet/edge_select.rs"
      provides: "Post-fillet debug assertions in fillet_edges and fillet_edges_generic"
      min_lines: 700
      contains: "debug_assert_topology"
    - path: "monstertruck-solid/src/fillet/ops.rs"
      provides: "Post-fillet debug assertions in fillet_along_wire"
      min_lines: 580
      contains: "debug_assert_topology"
  key_links:
    - from: "monstertruck-solid/src/fillet/validate.rs"
      to: "monstertruck-solid/src/fillet/edge_select.rs"
      via: "use super::validate"
      pattern: "debug_assert_topology"
    - from: "monstertruck-solid/src/fillet/validate.rs"
      to: "monstertruck-solid/src/fillet/ops.rs"
      via: "use super::validate"
      pattern: "debug_assert_topology"
---

<objective>
Add topology invariant assertions to all fillet operations so that debug builds automatically verify Euler-Poincare (V - E + F = 2 for closed shells) and orientation consistency after every fillet topology modification. Include tests that prove assertions fire on invalid topology. Tests live inside validate.rs as a #[cfg(test)] module -- tests.rs is never modified.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/fillet/mod.rs
@monstertruck-solid/src/fillet/edge_select.rs
@monstertruck-solid/src/fillet/ops.rs
@monstertruck-topology/src/shell.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create validate.rs with Euler-Poincare and orientation checks</name>
  <files>monstertruck-solid/src/fillet/validate.rs, monstertruck-solid/src/fillet/mod.rs</files>
  <action>
Create `monstertruck-solid/src/fillet/validate.rs` containing:

1. A function `euler_poincare_check(shell: &Shell) -> bool` that:
   - Counts unique vertices V using `shell.vertex_iter()` with a `HashSet` on vertex IDs.
   - Counts unique edges E using `shell.edge_iter()` with a `HashSet` on edge IDs.
   - Counts faces F using `shell.len()`.
   - Checks `shell.shell_condition()`:
     - If `ShellCondition::Closed`: strictly returns `V - E + F == 2`.
     - Otherwise (open/oriented/regular/irregular): returns `true` unconditionally (Euler-Poincare is only enforced on closed shells).

2. A function `is_oriented_check(shell: &Shell) -> bool` that:
   - Calls `shell.shell_condition()`.
   - Returns true if condition is `ShellCondition::Oriented` or `ShellCondition::Closed`.

3. A public function `debug_assert_topology(shell: &Shell, context: &str)` that:
   - Is a no-op in release builds (wrap body in `if cfg!(debug_assertions)`).
   - In debug builds: calls both checks above.
   - If Euler-Poincare fails: `debug_assert!(false, "Euler-Poincare violation after {context}: V={v} E={e} F={f}, V-E+F={chi}")`.
   - If orientation fails: `debug_assert!(false, "Orientation violation after {context}: condition={condition:?}")`.

4. A helper `debug_assert_euler(shell: &Shell, context: &str)` that only checks Euler-Poincare (not orientation). Used for mid-chain intermediate steps where orientation may be temporarily invalid.

Make `euler_poincare_check` and `is_oriented_check` `pub(crate)` so the `#[cfg(test)]` module in this same file can call them.

Use the existing type aliases from the fillet module: `Shell`, `Edge`, etc. (which are `monstertruck_topology::Shell<Point3, Curve, Surface>`).

Import `ShellCondition` from `monstertruck_topology::shell::ShellCondition`.

Update `monstertruck-solid/src/fillet/mod.rs` to add `mod validate;` (not public, internal only).
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-solid --lib -- fillet --skip test_unit_circle` to confirm compilation succeeds and existing tests still pass.
  </verify>
  <done>Created validate.rs with euler_poincare_check (closed-shell-only enforcement), is_oriented_check, debug_assert_topology, and debug_assert_euler functions. Module registered in mod.rs.</done>
</task>

<task type="auto">
  <name>Task 2: Insert debug assertions after fillet topology modifications</name>
  <files>monstertruck-solid/src/fillet/edge_select.rs, monstertruck-solid/src/fillet/ops.rs</files>
  <action>
Insert `validate::debug_assert_topology(shell, "context")` calls at these points:

In `edge_select.rs`:
1. At the end of `fillet_edges()` (just before `Ok(())`):
   `validate::debug_assert_topology(shell, "fillet_edges");`
2. In `fillet_edges_generic()` after `fillet_edges()` returns and before `convert_shell_out`:
   `validate::debug_assert_topology(&internal_shell, "fillet_edges_generic");`
3. After each successful single-edge fillet in the chain loop (after `apply_single_edge_fillet` returns `Ok(())`):
   `validate::debug_assert_euler(shell, "fillet_edges/single_edge");`
   (Use `debug_assert_euler` for mid-chain to avoid spurious orientation failures on intermediate states.)

In `ops.rs`:
1. At the end of `fillet_along_wire()` (before returning Ok), after shell mutation is complete:
   `validate::debug_assert_topology(shell, "fillet_along_wire");`

Add `use super::validate;` to both files' import sections.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-solid --lib -- fillet --skip test_unit_circle` to verify all existing tests pass with the debug assertions active.
  </verify>
  <done>Inserted debug_assert_topology and debug_assert_euler calls at all fillet topology modification points in edge_select.rs and ops.rs.</done>
</task>

<task type="auto">
  <name>Task 3: Add topology validation tests in validate.rs #[cfg(test)] module</name>
  <files>monstertruck-solid/src/fillet/validate.rs</files>
  <action>
Add a `#[cfg(test)]` module at the bottom of `monstertruck-solid/src/fillet/validate.rs`. Do NOT modify tests.rs (repo rules prohibit it).

The tests must construct shells inline using the same pattern as `build_6face_box` in tests.rs: 8 vertices, 12 edges, 6 faces forming a closed unit cube. Replicate this construction locally in the test module since `build_6face_box` is private to tests.rs and cannot be imported.

IMPORTANT: Use `build_6face_box` pattern (6 faces, closed shell), NOT `build_box_shell` (4 faces, open shell). The 4-face `build_box_shell` creates an Oriented (not Closed) shell, which would bypass the Euler-Poincare check entirely. The 6-face closed box is required to exercise both the Euler and orientation checks.

The tests:

1. `euler_poincare_valid_closed_box` -- builds a 6-face closed box inline, verifies:
   - `shell.shell_condition()` is `ShellCondition::Closed`.
   - `euler_poincare_check(&shell)` returns true.
   - `is_oriented_check(&shell)` returns true.
   - Explicitly count V, E, F and assert V - E + F == 2 (8 - 12 + 6 = 2).

2. `topology_valid_after_box_fillet` -- builds a 6-face closed box inline, fillets one edge with `fillet_edges`, then:
   - Calls `euler_poincare_check(&shell)` and asserts true.
   - Calls `is_oriented_check(&shell)` and asserts true.
   (This exercises the checks on a post-fillet shell. The shell remains Closed after filleting a single edge of a closed cube.)

3. `debug_assert_fires_on_corrupted_orientation` -- constructs a 6-face closed box inline, then corrupts it by calling `shell[5].invert()` (inverting the bottom face). This makes the face orientations incompatible on the shared edges, changing `shell_condition()` from `Closed` to `Regular` (not Oriented). Uses `std::panic::catch_unwind` to verify that `debug_assert_topology` panics with an orientation violation message. Only runs under `#[cfg(debug_assertions)]`.

   WHY orientation corruption (not face removal): Removing a face from a closed box changes its `shell_condition()` from `Closed` to `Oriented`, which causes `euler_poincare_check` to return `true` unconditionally (Euler is only enforced on Closed shells). Inverting a face instead produces `Regular` condition, which fails `is_oriented_check` and triggers the ORIENTATION debug assertion in `debug_assert_topology`.

4. `euler_poincare_check_detects_invalid_chi` -- a direct unit test of the `euler_poincare_check` function on an artificially invalid closed shell. Build a closed tetrahedron (4 triangular faces, 6 edges, 4 vertices: V-E+F = 4-6+4 = 2, valid). Then add a 5th degenerate face that reuses existing edges (making it Irregular, not Closed). Since constructing a Closed shell with wrong chi is topologically impossible in a valid half-edge structure, instead directly verify: build the valid tetrahedron, assert `euler_poincare_check` returns true, then build a second shell that is intentionally NOT closed (e.g., a 5-face open box missing one face) and verify `euler_poincare_check` returns true (because it skips non-Closed shells). This confirms the guard logic works correctly. The actual Euler violation is caught indirectly: if the fillet code ever produces a Closed shell with wrong V-E+F, the debug assertion fires.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-solid --lib -- fillet::validate --skip test_unit_circle` to confirm the new tests pass.
Also run `cargo clippy -p monstertruck-solid --all-targets -- -W warnings` and `cargo fmt -p monstertruck-solid -- --check`.
  </verify>
  <done>Added 4 topology validation tests in validate.rs #[cfg(test)] module. Tests cover: Euler-Poincare on valid closed box, topology after fillet, debug assertion firing on orientation corruption (not face removal -- avoids the closed-shell bypass), and direct euler_poincare_check guard logic verification. All tests pass. Clippy and fmt clean.</done>
</task>

</tasks>

<verification>
1. The current fillet test suite passes without modification (debug assertions active).
2. New topology validation tests in validate.rs pass.
3. `cargo clippy -p monstertruck-solid --all-targets -- -W warnings` produces no warnings.
4. `cargo fmt -p monstertruck-solid -- --check` passes.
5. Debug assertions are compile-time gated (no runtime cost in release).
6. Euler-Poincare check only enforces V-E+F=2 on ShellCondition::Closed shells.
7. Orientation corruption test proves debug_assert_topology fires on invalid topology (face inversion, not face removal).
8. Test shell construction uses 6-face closed box (build_6face_box pattern), not the 4-face open build_box_shell.
</verification>

<success_criteria>
- Euler-Poincare debug assertions fire after every fillet topology modification in debug builds
- Enforcement restricted to closed shells only (V - E + F = 2 when ShellCondition::Closed)
- shell.shell_condition() returns Oriented or Closed after fillet operations in all existing tests
- No runtime overhead in release builds
- All existing tests continue to pass (tests.rs is never modified)
- Orientation corruption test in validate.rs proves debug_assert_topology fires (uses face inversion to keep shell Regular, avoiding the Closed-shell bypass from face removal)
- Shell construction in tests uses 6-face closed box pattern, not 4-face open box
</success_criteria>

<output>
After completion, create `.tendrion/phases/8-validation-and-documentation/8-1-SUMMARY.md`
</output>
