---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 3
type: execute
wave: 2
depends_on: ["9-1", "9-2"]
files_modified:
  - monstertruck-solid/src/transversal/integrate/tests.rs
  - monstertruck-solid/src/transversal/integrate/mod.rs
  - monstertruck-solid/src/transversal/loops_store/mod.rs
autonomous: true
must_haves:
  truths:
    - "Boolean AND of overlapping cubes produces a solid with exactly one closed boundary shell and volume ~0.125"
    - "Boolean OR of overlapping cubes produces a solid with a closed shell and volume ~1.875"
    - "Boolean difference of overlapping cubes produces a solid with volume ~0.875"
    - "Chained boolean operations (AND followed by OR) produce valid closed-shell topology"
    - "The boolean pipeline references TOLERANCE via import rather than inline magic numbers"
    - "cargo clippy --all-targets -- -W warnings passes without warnings"
    - "Integration tests in boolean_edge_cases pass alongside new unit tests"
  artifacts:
    - path: "monstertruck-solid/src/transversal/integrate/tests.rs"
      provides: "Extended boolean tests with topology validation, volume assertions, and chained-boolean test"
      min_lines: 80
      contains: "chained_boolean"
    - path: "monstertruck-solid/src/transversal/integrate/mod.rs"
      provides: "Boolean pipeline with documented tolerance usage"
      min_lines: 500
      contains: "TOLERANCE"
    - path: "monstertruck-solid/src/transversal/loops_store/mod.rs"
      provides: "Loop store with documented tolerance multiplier rationale"
      min_lines: 800
      contains: "TOLERANCE"
  key_links:
    - from: "monstertruck-core/src/tolerance.rs"
      to: "monstertruck-solid/src/transversal/integrate/mod.rs"
      via: "TOLERANCE constant used in boolean pipeline tolerance checks"
      pattern: "TOLERANCE"
    - from: "monstertruck-solid/src/transversal/integrate/mod.rs"
      to: "monstertruck-solid/src/transversal/integrate/tests.rs"
      via: "Unit tests exercise the boolean pipeline end-to-end"
      pattern: "crate::and"
---

<objective>
Validate the boolean repairs and tolerance unification with end-to-end integration tests including topology assertions, volume checks, and a chained-boolean test. Add documentation comments to tolerance multipliers in the boolean pipeline. Run both unit tests and the boolean_edge_cases integration regression suite.
</objective>

<execution_context>
@AGENTS.md
</execution_context>

<context>
@monstertruck-solid/src/transversal/integrate/mod.rs
@monstertruck-solid/src/transversal/integrate/tests.rs
@monstertruck-solid/src/transversal/loops_store/mod.rs
@monstertruck-solid/tests/boolean_edge_cases.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add topology validation and chained-boolean tests</name>
  <files>monstertruck-solid/src/transversal/integrate/tests.rs, monstertruck-solid/src/transversal/loops_store/mod.rs</files>
  <action>
Add new tests to `monstertruck-solid/src/transversal/integrate/tests.rs`. The file currently has `adjacent_cubes_or` and `punched_cube` tests. Add the following after the existing tests.

First, add `use monstertruck_topology::shell::ShellCondition;` to the imports if not already present.

1. **Strengthen existing `adjacent_cubes_or` test** — add topology assertions at the end (before the closing brace):
```rust
    let shell = &solid.boundaries()[0];
    assert_eq!(shell.shell_condition(), ShellCondition::Closed,
        "adjacent_cubes_or: shell should be closed");
    assert!(shell.singular_vertices().is_empty(),
        "adjacent_cubes_or: no singular vertices");
```

2. **New test: `overlapping_cubes_and_topology`**:
```rust
#[test]
fn overlapping_cubes_and_topology() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(0.5, 0.5, 0.5));
    let w = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&w, Vector3::unit_y());
    let cube2: Solid = builder::extrude(&f, Vector3::unit_z());

    let result = crate::and(&cube, &cube2, 0.05);
    assert!(result.is_ok(), "Overlapping cubes AND should succeed: {result:?}");
    let solid = result.unwrap();
    assert_eq!(solid.boundaries().len(), 1);
    let shell = &solid.boundaries()[0];
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
    assert!(shell.singular_vertices().is_empty());

    let poly = solid.triangulation(0.01).to_polygon();
    // AND of two unit cubes offset by (0.5, 0.5, 0.5): intersection is 0.5^3 = 0.125
    assert_near!(poly.volume(), 0.125);
}
```

3. **New test: `overlapping_cubes_or_topology`**:
```rust
#[test]
fn overlapping_cubes_or_topology() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(0.5, 0.5, 0.5));
    let w = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&w, Vector3::unit_y());
    let cube2: Solid = builder::extrude(&f, Vector3::unit_z());

    let result = crate::or(&cube, &cube2, 0.05);
    assert!(result.is_ok(), "Overlapping cubes OR should succeed: {result:?}");
    let solid = result.unwrap();
    assert_eq!(solid.boundaries().len(), 1);
    let shell = &solid.boundaries()[0];
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
    assert!(shell.singular_vertices().is_empty());

    let poly = solid.triangulation(0.01).to_polygon();
    // OR volume = 2*1.0 - 0.125 = 1.875
    assert_near!(poly.volume(), 1.875);
}
```

4. **New test: `overlapping_cubes_difference_topology`**:
```rust
#[test]
fn overlapping_cubes_difference_topology() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(0.5, 0.5, 0.5));
    let w = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&w, Vector3::unit_y());
    let cube2: Solid = builder::extrude(&f, Vector3::unit_z());

    let result = crate::difference(&cube, &cube2, 0.05);
    assert!(result.is_ok(), "Overlapping cubes difference should succeed: {result:?}");
    let solid = result.unwrap();
    assert_eq!(solid.boundaries().len(), 1);
    let shell = &solid.boundaries()[0];
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
    assert!(shell.singular_vertices().is_empty());

    let poly = solid.triangulation(0.01).to_polygon();
    // Difference volume = 1.0 - 0.125 = 0.875
    assert_near!(poly.volume(), 0.875);
}
```

5. **New test: `chained_boolean_and_then_or`** — This directly addresses the must-have for chained boolean topology:
```rust
#[test]
fn chained_boolean_and_then_or() {
    // Create three cubes: base, cutter, and adder.
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let base: Solid = builder::extrude(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(0.5, 0.5, 0.5));
    let w = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&w, Vector3::unit_y());
    let cutter: Solid = builder::extrude(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(-0.5, 0.0, 0.0));
    let w = builder::extrude(&v, Vector3::unit_x() * 0.5);
    let f = builder::extrude(&w, Vector3::unit_y());
    let adder: Solid = builder::extrude(&f, Vector3::unit_z());

    // Step 1: AND base with cutter -> intersection volume = 0.125
    let intersection = crate::and(&base, &cutter, 0.05);
    assert!(intersection.is_ok(), "AND step should succeed: {intersection:?}");
    let intersection = intersection.unwrap();

    // Step 2: OR intersection with adder -> union of 0.125 intersection + 0.5 adder cube
    let result = crate::or(&intersection, &adder, 0.05);
    assert!(result.is_ok(), "Chained OR step should succeed: {result:?}");
    let solid = result.unwrap();

    // Topology check: result must have closed boundary shells.
    for (i, shell) in solid.boundaries().iter().enumerate() {
        assert_eq!(
            shell.shell_condition(),
            ShellCondition::Closed,
            "chained boolean: boundary shell[{i}] should be closed"
        );
        assert!(
            shell.singular_vertices().is_empty(),
            "chained boolean: boundary shell[{i}] has singular vertices"
        );
    }

    // Volume sanity: the result has positive volume.
    let poly = solid.triangulation(0.01).to_polygon();
    assert!(poly.volume() > 0.0, "Chained boolean result should have positive volume");
}
```

Also, in `monstertruck-solid/src/transversal/loops_store/mod.rs`, add documentation comments for the tolerance multipliers at lines 805 and 807:
- Before line 805 (`let snap_tol = f64::max(snap_tol, 10.0 * TOLERANCE);`) add comment:
  `// Snap tolerance floor: mesh vertex snapping needs at least 10x TOLERANCE.`
- Before line 807 (`let vertex_merge_tol = 100.0 * TOLERANCE;`) add comment:
  `// Vertex merge tolerance: 100x TOLERANCE for merging nearby vertices during loop construction.`
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --lib -E 'test(overlapping_cubes) | test(chained_boolean) | test(adjacent_cubes_or)' --no-fail-fast` to verify all new and modified tests pass. Run `cargo nextest run -p monstertruck-solid --test boolean_edge_cases --no-fail-fast` to verify integration regression suite passes.</verify>
  <done>Topology validation tests added for AND, OR, difference, and chained boolean operations; tolerance multipliers in loops_store documented; boolean_edge_cases integration tests verified.</done>
</task>

<task type="auto">
  <name>Task 2: Document tolerance usage in boolean pipeline and run full verification</name>
  <files>monstertruck-solid/src/transversal/integrate/mod.rs</files>
  <action>
Add documentation comments to the tolerance-related lines in the boolean pipeline:

1. In `process_one_pair_of_shells`:
   - Before the line `if tol < TOLERANCE {`: add comment `// Operation tolerance must be at least the global geometric coincidence threshold.`
   - Before the line `let poly_tol = f64::max(tol * 0.25, 2.0 * TOLERANCE);`: add comment `// Triangulation tolerance: 25% of operation tol, floored at 2x TOLERANCE for mesh stability.`

2. In `try_cap_shell_with_existing_surfaces`:
   - Before the `f64::max(10.0 * TOLERANCE, tol)` usage: add comment `// Mesh triangulation for capping uses at least 10x TOLERANCE for vertex snap reliability.`

3. Run the full verification suite:
   - `cargo nextest run -p monstertruck-core --lib --no-fail-fast` — tolerance module tests
   - `cargo nextest run -p monstertruck-solid --no-fail-fast` — all solid tests: unit tests AND integration tests (boolean_edge_cases, feature_integration, unwrap_safety)
   - `cargo clippy --all-targets -- -W warnings` — full workspace lint with warnings enabled
   - `cargo fmt --all -- --check` — formatting

Fix any issues found. Constrain fixes to ONLY the files declared in this plan's `files_modified` list: `monstertruck-solid/src/transversal/integrate/tests.rs`, `monstertruck-solid/src/transversal/integrate/mod.rs`, and `monstertruck-solid/src/transversal/loops_store/mod.rs`. If a test fails due to a bug in a file outside this scope, document the failure and move on — do not modify files outside scope.
  </action>
  <verify>All four verification commands pass with zero failures and zero warnings. In particular: `cargo nextest run -p monstertruck-solid --test boolean_edge_cases --no-fail-fast` passes all edge case tests.</verify>
  <done>Boolean pipeline tolerance usage documented; full workspace verification passed including boolean_edge_cases integration regression suite.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-core --lib` passes.
2. `cargo nextest run -p monstertruck-solid --no-fail-fast` passes including all new topology unit tests AND the boolean_edge_cases integration test suite.
3. `cargo nextest run -p monstertruck-solid --test boolean_edge_cases --no-fail-fast` passes all edge case integration tests (tangent_face_and, tangent_face_or, coincident_face_and, coincident_face_or, pole_degeneration_sphere_and, pole_degeneration_sphere_difference, regression_standard_boolean).
4. `cargo clippy --all-targets -- -W warnings` produces no warnings.
5. `cargo fmt --all -- --check` passes.
6. New tests verify shell_condition == Closed and singular_vertices.is_empty() for all boolean results.
7. Volume assertions confirm geometric correctness of AND (~0.125), OR (~1.875), and difference (~0.875).
8. Chained boolean test (AND then OR) produces valid closed-shell topology.
9. Tolerance multipliers in loops_store and integrate are documented.
</verification>

<success_criteria>
- Boolean AND/OR/difference on overlapping cubes produce closed shells with correct volumes
- Chained boolean operations produce valid closed-shell topology (dedicated test)
- Tolerance usage in the boolean pipeline is documented with rationale comments
- Full package test suite passes (--lib AND --test boolean_edge_cases)
- Full workspace passes clippy (--all-targets -- -W warnings) and fmt checks
- Addresses requirements BOOL-01 (validation) and TEST-02 (tolerance documentation in pipeline)
</success_criteria>

<output>
After completion, create `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-3-SUMMARY.md`
</output>
