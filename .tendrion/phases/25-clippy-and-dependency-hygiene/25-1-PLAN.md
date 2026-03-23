---
phase: 25-clippy-and-dependency-hygiene
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - Cargo.toml
  - Cargo.lock
  - monstertruck-meshing/src/vtk.rs
  - monstertruck-meshing/tests/vtk.rs
  - monstertruck-step/examples/step-to-mesh.rs
autonomous: true
must_haves:
  truths:
    - "cargo tree --workspace -i nom@3.2.1 returns no results (nom v3 is eliminated)"
    - "cargo tree --workspace -i quick-xml@0.22.0 returns no results (quick-xml v0.22 is eliminated)"
    - "cargo clippy --workspace 2>&1 shows no future-incompat warnings for nom or quick-xml"
    - "cargo nextest run -p monstertruck-meshing passes all tests including VTK tests"
    - "cargo nextest run --workspace compiles without errors from the vtkio update"
  artifacts:
    - path: "Cargo.toml"
      provides: "Updated workspace vtkio dependency version"
      min_lines: 90
      contains: "vtkio"
    - path: "monstertruck-meshing/src/vtk.rs"
      provides: "VTK conversion code compatible with updated vtkio API"
      min_lines: 100
      contains: "vtkio"
  key_links:
    - from: "Cargo.toml"
      to: "monstertruck-meshing/src/vtk.rs"
      via: "workspace dependency resolution"
      pattern: "vtkio"
    - from: "monstertruck-meshing/src/vtk.rs"
      to: "monstertruck-meshing/tests/vtk.rs"
      via: "test coverage of VTK conversion"
      pattern: "vtkio::model"
---

<objective>
Update the vtkio dependency from v0.6.3 to v0.7.0-rc2 to eliminate the deprecated transitive dependencies nom v3.2.1 and quick-xml v0.22.0 that are flagged for rejection by future Rust versions. Adapt all code using the vtkio API to any breaking changes in the new version.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@Cargo.toml
@monstertruck-meshing/Cargo.toml
@monstertruck-meshing/src/vtk.rs
@monstertruck-meshing/tests/vtk.rs
@monstertruck-step/examples/step-to-mesh.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Update vtkio dependency and fix compilation</name>
  <files>Cargo.toml, Cargo.lock, monstertruck-meshing/src/vtk.rs, monstertruck-meshing/tests/vtk.rs, monstertruck-step/examples/step-to-mesh.rs</files>
  <action>
1. In the workspace `Cargo.toml`, update the vtkio dependency from `"0.6"` to `"0.7.0-rc2"`.

2. Run `cargo clippy -p monstertruck-meshing` to identify any API breakage from the vtkio update.

3. The key areas that may need updates in `monstertruck-meshing/src/vtk.rs`:
   - The `vtkio::model` module path may have changed
   - `IOBuffer`, `DataSet`, `Piece`, `VertexNumbers`, `CellType`, `Attributes`, `DataArray`, `ElementType` types may have moved or been renamed
   - The `VertexNumbers::XML` variant format may have changed
   - `Piece::Inline(Box::new(...))` pattern may have changed

4. Fix any compilation errors in:
   - `monstertruck-meshing/src/vtk.rs` (main VTK conversion code)
   - `monstertruck-meshing/tests/vtk.rs` (VTK test code)
   - `monstertruck-step/examples/step-to-mesh.rs` (example using vtkio::model)

5. Run `cargo clippy --workspace` to confirm the entire workspace compiles without warnings.
  </action>
  <verify>
Run `cargo clippy --workspace` -- must exit with code 0.
Run `cargo tree --workspace -i nom@3.2.1 2>&1` -- should show "no crate found" or empty output (nom v3 eliminated).
Run `cargo tree --workspace -i quick-xml@0.22.0 2>&1` -- should show "no crate found" or empty output (quick-xml v0.22 eliminated).
  </verify>
  <done>vtkio updated to v0.7.0-rc2, nom v3.2.1 and quick-xml v0.22.0 eliminated from dependency tree, workspace compiles cleanly.</done>
</task>

<task type="auto">
  <name>Task 2: Run full test suite to verify no regressions</name>
  <files>monstertruck-meshing/src/vtk.rs, monstertruck-meshing/tests/vtk.rs</files>
  <action>
1. Run `cargo nextest run -p monstertruck-meshing` to verify all meshing tests pass, especially the VTK-related tests.

2. If any VTK tests fail due to API changes (e.g., different serialization format, changed field names, changed enum variants), fix the test expectations to match the new vtkio API while preserving the test's intent.

3. Run `cargo nextest run --workspace` (excluding GPU-dependent tests if needed) to verify no regressions from the dependency update across the entire workspace.

4. Verify future-incompat warnings are gone: `cargo clippy --workspace 2>&1 | grep future-incompat` should produce no output.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-meshing` -- all tests pass.
Run `cargo nextest run --workspace` -- no new failures introduced.
Run `cargo clippy --workspace 2>&1 | grep "future-incompat"` -- no output.
  </verify>
  <done>Full test suite passes with updated vtkio dependency, no regressions, no future-incompat warnings.</done>
</task>

</tasks>

<verification>
1. `cargo tree --workspace -i nom@3.2.1` returns empty/error (nom v3 gone)
2. `cargo tree --workspace -i quick-xml@0.22.0` returns empty/error (quick-xml v0.22 gone)
3. `cargo clippy --workspace` exits with code 0
4. `cargo nextest run --workspace` shows no new test failures
5. `cargo clippy --workspace 2>&1` shows no future-incompat warnings for nom or quick-xml
</verification>

<success_criteria>
- RELY-04 satisfied: nom v3.2.1 and quick-xml v0.22.0 eliminated via vtkio update
- All VTK-related code compiles and tests pass with the new dependency version
- No regressions introduced across the workspace
</success_criteria>

<output>
After completion, create `.tendrion/phases/25-clippy-and-dependency-hygiene/25-1-SUMMARY.md`
</output>
