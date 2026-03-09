---
phase: 3-feature-completeness
plan: 5
type: tdd
wave: 3
depends_on: ["3-1", "3-2", "3-3", "3-4"]
files_modified:
  - monstertruck-modeling/src/lib.rs
  - monstertruck-modeling/Cargo.toml
  - monstertruck-step/tests/output/topology.rs
  - monstertruck-solid/tests/feature_integration.rs
autonomous: true
must_haves:
  truths:
    - "User chamfers a boolean-result solid and exports to STEP successfully"
    - "User shells a solid and exports the hollow result to STEP"
    - "User drafts a solid and exports the tapered result to STEP"
    - "All new operations (chamfer, shell, offset, draft) are accessible from monstertruck-modeling"
    - "Combined workflows (boolean + chamfer + STEP export) produce valid output end-to-end"
  artifacts:
    - path: "monstertruck-solid/tests/feature_integration.rs"
      provides: "Cross-feature integration tests"
      min_lines: 100
      contains: "boolean_then_chamfer_step_export"
    - path: "monstertruck-modeling/src/lib.rs"
      provides: "Unified re-exports of all new operations"
      min_lines: 120
      contains: "shell_solid"
  key_links:
    - from: "monstertruck-solid/tests/feature_integration.rs"
      to: "monstertruck-step/src/save/mod.rs"
      via: "Integration tests export combined-operation results to STEP"
      pattern: "CompleteStepDisplay"
    - from: "monstertruck-modeling/src/lib.rs"
      to: "monstertruck-solid/src/lib.rs"
      via: "Modeling crate re-exports solid operations"
      pattern: "monstertruck_solid"
---

<objective>
Integration testing across all Phase 3 features: verify that boolean STEP export, chamfer, shell/offset, and draft operations compose correctly and that results can be exported to valid STEP files. Ensure all new APIs are accessible from the modeling crate.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/lib.rs
@monstertruck-modeling/src/lib.rs
@monstertruck-step/src/save/mod.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create cross-feature integration tests</name>
  <files>monstertruck-solid/tests/feature_integration.rs</files>
  <action>
Create a new integration test file `monstertruck-solid/tests/feature_integration.rs` with end-to-end tests that combine multiple Phase 3 features:

1. `boolean_then_chamfer_step_export`: Boolean AND a cube/cylinder, compress, export to STEP, parse.
2. `shell_then_step_export`: Shell a cube (wall thickness 0.1), compress, export to STEP, parse.
3. `draft_then_step_export`: Draft a cube's side faces by 5 degrees, compress, export to STEP, parse.
4. `chamfer_cube_step_export`: Chamfer a cube edge, compress the shell, export to STEP, parse.

Each test should:
- Create a shape programmatically
- Apply the operation
- Verify topological validity (ShellCondition::Closed, no singular vertices)
- Compress to CompressedSolid (or CompressedShell)
- Export to STEP string via CompleteStepDisplay
- Parse the STEP string with ruststep
- Assert parse succeeds

Note: `monstertruck-step` is already a dev-dependency in `monstertruck-solid/Cargo.toml`, so no Cargo.toml changes are needed. Use `monstertruck_step` for STEP export and `ruststep` (available transitively via monstertruck-step) for STEP parsing. If transitive access to `ruststep` does not work, place the STEP round-trip tests in `monstertruck-step/tests/output/topology.rs` instead.
  </action>
  <verify>
Run the integration tests and confirm all pass.
  </verify>
  <done>Cross-feature integration tests validate that boolean+STEP, chamfer+STEP, shell+STEP, and draft+STEP workflows produce valid output.</done>
</task>

<task type="auto">
  <name>Task 2: Ensure all new APIs are re-exported from monstertruck-modeling</name>
  <files>monstertruck-modeling/src/lib.rs</files>
  <action>
1. Add a new `solid-ops` feature flag to `monstertruck-modeling/Cargo.toml` that enables `monstertruck-solid` as a dependency (following the existing `fillet` pattern but with a semantically correct name). The `fillet` feature should imply `solid-ops` for backward compatibility.

2. Add conditional re-exports for the new operations to `monstertruck-modeling/src/lib.rs`:

   ```rust
   /// Shell (hollow-out), offset, and draft/taper operations.
   /// Re-exports from [`monstertruck_solid`].
   #[cfg(feature = "solid-ops")]
   pub use monstertruck_solid::{
       shell_solid, offset_shell,
       draft_faces, DraftOptions, DraftError,
   };
   ```

   Keep existing fillet re-exports under `#[cfg(feature = "fillet")]` (which implies `solid-ops`).

3. Verify the re-exports compile:
   `cargo check -p monstertruck-modeling --features solid-ops`

4. Check that the workspace-level `monstertruck` crate also exposes these if appropriate.
  </action>
  <verify>
Run `cargo check -p monstertruck-modeling --features solid-ops` to confirm compilation.
Run `cargo doc -p monstertruck-modeling --features solid-ops --no-deps` to verify docs include the new APIs.
  </verify>
  <done>All Phase 3 APIs (chamfer, shell, offset, draft) are accessible from the monstertruck-modeling crate via the solid-ops feature flag.</done>
</task>

<task type="auto">
  <name>Task 3: Final validation and documentation update</name>
  <files>monstertruck-step/src/lib.rs, monstertruck-solid/src/lib.rs</files>
  <action>
1. Run the full test suite across all relevant crates:
   ```bash
   cargo nextest run -p monstertruck-solid
   cargo nextest run -p monstertruck-step --test output
   cargo nextest run -p monstertruck-modeling --features solid-ops
   ```

2. Update documentation in `monstertruck-solid/src/lib.rs`:
   - Add module-level docs mentioning the new shell_ops and draft modules
   - Ensure the crate doc comment reflects the expanded feature set

3. Verify `monstertruck-step/src/lib.rs` doc comment reflects boolean export support (should be done in plan 3-1).

4. Fix any remaining test failures or compiler warnings.

5. Run clippy on the modified crates:
   ```bash
   cargo clippy -p monstertruck-solid -p monstertruck-step -p monstertruck-modeling
   ```
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-solid -p monstertruck-step -p monstertruck-modeling --features solid-ops` and confirm all tests pass.
Run `cargo clippy -p monstertruck-solid -p monstertruck-step -p monstertruck-modeling` and confirm no errors.
  </verify>
  <done>All Phase 3 features pass tests, documentation is updated, and no clippy warnings remain.</done>
</task>

</tasks>

<verification>
1. Cross-feature integration tests pass
2. `cargo nextest run -p monstertruck-solid` -- all solid crate tests pass
3. `cargo nextest run -p monstertruck-step --test output` -- all STEP output tests pass
4. `cargo check -p monstertruck-modeling --features solid-ops` -- all new APIs accessible
5. `cargo clippy -p monstertruck-solid -p monstertruck-step -p monstertruck-modeling` -- no errors
6. Combined workflows (boolean + chamfer, shell + STEP export, draft + STEP export) produce valid output
</verification>

<success_criteria>
- All four phase requirements (FEAT-01, FEAT-02, FEAT-03, FEAT-05) are satisfied end-to-end
- Cross-feature combinations work correctly (boolean+STEP, chamfer+STEP, shell+STEP, draft+STEP)
- All new APIs are accessible from monstertruck-modeling
- No regressions in existing test suites
- Code passes clippy checks
</success_criteria>

<output>
After completion, create `.tendrion/phases/3-feature-completeness/3-5-SUMMARY.md`
</output>
