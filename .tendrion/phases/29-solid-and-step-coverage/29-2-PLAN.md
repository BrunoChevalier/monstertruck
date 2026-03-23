---
phase: 29-solid-and-step-coverage
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-step/tests/roundtrip_coverage.rs
autonomous: true
must_haves:
  truths:
    - "cargo nextest run -p monstertruck-step passes with all new tests green"
    - "A cube solid exported to STEP and re-imported produces a shell with matching bounding box geometry"
    - "A cylinder solid round-trips through STEP format with preserved geometry"
    - "A boolean result solid round-trips through STEP format and parses successfully"
    - "Multiple shapes exported via StepModels round-trip correctly"
    - "STEP round-trip preserves shell closedness (Closed shells remain Closed)"
  artifacts:
    - path: "monstertruck-step/tests/roundtrip_coverage.rs"
      provides: "STEP import/export round-trip tests with geometry comparison"
      min_lines: 150
      contains: "CompleteStepDisplay"
  key_links:
    - from: "monstertruck-step/tests/roundtrip_coverage.rs"
      to: "monstertruck-step/src/save/mod.rs"
      via: "Export via CompleteStepDisplay and StepModel::from"
      pattern: "CompleteStepDisplay::new"
    - from: "monstertruck-step/tests/roundtrip_coverage.rs"
      to: "monstertruck-step/src/load/mod.rs"
      via: "Import via Table::from_step and to_compressed_shell"
      pattern: "Table::from_step"
---

<objective>
Add meaningful test coverage to monstertruck-step with round-trip tests that create solids, export them to STEP format, re-import them, and compare geometry (bounding boxes, face counts, shell conditions) to verify the round-trip preserves the shape.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-step/src/lib.rs
@monstertruck-step/src/save/mod.rs
@monstertruck-step/src/load/mod.rs
@monstertruck-step/tests/io/ioi.rs
@monstertruck-step/tests/io/oi.rs
@monstertruck-step/Cargo.toml
@monstertruck-step/examples/shape-to-step.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: STEP round-trip tests for primitive solids</name>
  <files>monstertruck-step/tests/roundtrip_coverage.rs</files>
  <action>
Create a new integration test file `roundtrip_coverage.rs` in `monstertruck-step/tests/` that exercises full STEP round-trip: create solid -> compress -> export to STEP string -> parse STEP -> import back to CompressedShell -> compare geometry.

The existing `tests/io/ioi.rs` tests round-trip for STEP files from disk resources. This new file tests round-trip starting from programmatically-created solids, which is the use case described in the phase requirements.

Structure and imports:
```rust
use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;
use monstertruck_step::load::*;
use monstertruck_step::save::*;
use monstertruck_topology::compress::*;
use monstertruck_topology::shell::ShellCondition;
```

Helper functions:

1. `make_cube(origin: Point3, side: f64) -> Solid` -- builds a cube via builder chain

2. `roundtrip_shell(shell: &Shell) -> CompressedShell<Point3, Curve, Surface>` -- compresses shell, exports to STEP string via `CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string()`, then re-imports via `Table::from_step(&step_string).unwrap()`, iterates `table.shell.values()`, calls `table.to_compressed_shell(&step_shell).unwrap()`, returns the first shell.

3. `roundtrip_solid(solid: &Solid) -> Vec<CompressedShell<Point3, Curve, Surface>>` -- same pattern but via CompressedSolid, returns all shells.

4. `bounding_box_matches(original: &Shell, reimported: &CompressedShell<Point3, Curve, Surface>, tolerance: f64) -> bool` -- compares bounding boxes of original (via compress + triangulation) and reimported (via triangulation). Uses `triangulation(0.05).to_polygon().bounding_box()`. Checks that min/max coordinates match within tolerance.

Tests to create:

1. `roundtrip_cube` -- Build a unit cube, round-trip via `roundtrip_solid`. Verify: STEP string contains "CLOSED_SHELL", re-imported shell has 6 faces, bounding box matches within tolerance 0.1, re-imported shell triangulation produces Closed polygon shell condition.

2. `roundtrip_cube_offset` -- Build a cube at (1.0, 2.0, 3.0) with side 2.0. Round-trip and verify bounding box is approximately [(1,2,3), (3,4,5)].

3. `roundtrip_compressed_solid` -- Build a cube, compress to CompressedSolid via `solid.compress()`, export via `StepModel::from(&compressed)`, re-import, verify face count matches.

4. `roundtrip_boolean_result` -- Build two overlapping cubes, compute `monstertruck_solid::or(&cube1, &cube2, 0.05)`, round-trip the result. Verify re-import produces valid shells with matching approximate bounding box.

5. `roundtrip_step_string_valid` -- Build a cube, export to STEP string. Verify the STEP string contains expected entities: "MANIFOLD_SOLID_BREP" or "CLOSED_SHELL", "B_SPLINE_SURFACE_WITH_KNOTS" or "PLANE", "CARTESIAN_POINT", "EDGE_CURVE". Parse with `ruststep::parser::parse` and verify success.

6. `roundtrip_preserves_closedness` -- Build a cube (closed shell), round-trip, verify the re-imported shell's triangulation polygon has `ShellCondition::Closed`. Use the same polygon closedness check as `ioi.rs`: triangulate, `put_together_same_attrs`, `remove_degenerate_faces`, check condition.

7. `roundtrip_multiple_shapes` -- Build two cubes with different sizes, compress both to CompressedSolid, export via `StepModels` (collect into `StepModels::from_iter`), verify the STEP string parses and contains multiple shells.

8. `roundtrip_from_resource_file` -- Load a known-good shape from `resources/shape/cube.json` as CompressedSolid, export to STEP, re-import, verify face count and bounding box match.

Note: The `Table::from_step` returns `Option<Table>`. The `to_compressed_shell` returns `Result`. Handle these appropriately with `.unwrap()` in tests.

Follow the pattern from `tests/io/ioi.rs` for the re-import and polygon validation:
```rust
let cshell = table.to_compressed_shell(&step_shell).unwrap();
let bdb = cshell.triangulation(0.01).to_polygon().bounding_box();
let diag = bdb.max() - bdb.min();
let r = diag.x.min(diag.y).min(diag.z);
let mut poly = cshell.triangulation(0.01 * r).to_polygon();
poly.put_together_same_attrs(TOLERANCE * 50.0).remove_degenerate_faces();
assert_eq!(poly.shell_condition(), ShellCondition::Closed);
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-step roundtrip_coverage` and verify all tests pass.</verify>
  <done>STEP round-trip coverage tests created and passing.</done>
</task>

<task type="auto">
  <name>Task 2: Verify all tests pass together</name>
  <files>monstertruck-step/tests/roundtrip_coverage.rs</files>
  <action>
Run the full test suite for monstertruck-step to verify no regressions:

```bash
cargo nextest run -p monstertruck-step
```

If any tests fail:
- Check for compilation errors in the new test file and fix imports/types
- Check for runtime failures and adjust tolerances or expectations
- Verify the `Table::from_step` API returns `Option` or `Result` and handle accordingly
- Check if `to_compressed_shell` requires specific feature flags (the `load` feature is default-enabled)

Common issues to watch for:
- The `Table::from_step` method may return `Option<Table>` -- use `.unwrap()`
- The `to_compressed_shell` returns `Result` -- use `.unwrap()`
- The `TOLERANCE` constant comes from `monstertruck_meshing::prelude::*` (re-exported from monstertruck-core)
- Triangulation and polygon methods need `monstertruck_meshing` in dev-dependencies (already present)
- The boolean ops need `monstertruck_solid` in dev-dependencies (already present)
  </action>
  <verify>Run `cargo nextest run -p monstertruck-step` and confirm zero failures.</verify>
  <done>Full monstertruck-step test suite passes with no regressions.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-step roundtrip_coverage` -- all round-trip tests pass
2. `cargo nextest run -p monstertruck-step` -- all existing + new tests pass (no regressions)
3. STEP round-trip tests verify: export produces valid STEP string, re-import succeeds, geometry (bounding boxes) matches within tolerance
4. Tests cover cube, offset cube, boolean result, resource file shapes
5. Shell closedness is preserved through the round-trip
</verification>

<success_criteria>
- monstertruck-step has round-trip tests that write a solid to STEP format and re-read it with geometry comparison
- Tests verify bounding box preservation, face count preservation, and shell condition preservation
- Tests cover programmatically-created solids (not just resource files)
- All new tests pass via `cargo nextest run -p monstertruck-step`
- No regressions in existing tests
</success_criteria>

<output>
After completion, create `.tendrion/phases/29-solid-and-step-coverage/29-2-SUMMARY.md`
</output>
