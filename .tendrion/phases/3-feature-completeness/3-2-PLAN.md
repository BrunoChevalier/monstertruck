---
phase: 3-feature-completeness
plan: 2
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/tests.rs
  - monstertruck-solid/src/fillet/params.rs
autonomous: true
must_haves:
  truths:
    - "User creates a chamfer on cube edges using FilletProfile::Chamfer and the result passes topological validity checks"
    - "User applies chamfer via fillet_edges with FilletProfile::Chamfer and gets a valid closed shell"
    - "Chamfer operations produce flat-cut edges (ruled surface) between adjacent faces"
    - "Chamfer works with variable radius and per-edge radius specifications"
    - "The chamfered solid can be serialized to JSON and deserialized back"
  artifacts:
    - path: "monstertruck-solid/src/fillet/tests.rs"
      provides: "Comprehensive chamfer tests covering edge cases and validity"
      min_lines: 1300
      contains: "chamfer_cube_edge_valid_topology"
  key_links:
    - from: "monstertruck-solid/src/fillet/params.rs"
      to: "monstertruck-solid/src/fillet/geometry.rs"
      via: "FilletProfile::Chamfer selects chamfer_fillet_surface geometry"
      pattern: "FilletProfile::Chamfer"
    - from: "monstertruck-modeling/src/lib.rs"
      to: "monstertruck-solid/src/fillet/mod.rs"
      via: "Re-exports chamfer API through modeling crate's fillet feature"
      pattern: "fillet_edges_generic"
---

<objective>
Verify and harden chamfer operations to ensure they produce topologically valid flat-cut edges on solid bodies. Chamfer is already implemented as FilletProfile::Chamfer but needs comprehensive validation tests proving it meets the phase requirement for flat-cut edge operations.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/fillet/mod.rs
@monstertruck-solid/src/fillet/params.rs
@monstertruck-solid/src/fillet/geometry.rs
@monstertruck-solid/src/fillet/ops.rs
@monstertruck-solid/src/fillet/tests.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write TDD tests proving chamfer topological validity on primitive solids</name>
  <files>monstertruck-solid/src/fillet/tests.rs</files>
  <action>
**TDD Red/Green phase**: Write targeted tests that explicitly verify chamfer operations meet the phase requirement: "Chamfer operations produce flat-cut edges on solid bodies and the result passes topological validity checks."

Add the following tests to `monstertruck-solid/src/fillet/tests.rs`:

1. `chamfer_cube_edge_valid_topology`:
   - Create a unit cube using `monstertruck_modeling::builder`
   - Apply chamfer (FilletProfile::Chamfer) to one edge using `fillet_edges_generic`
   - Assert the resulting shell has `ShellCondition::Closed`
   - Assert `shell.singular_vertices().is_empty()`
   - Assert `shell.extract_boundaries().is_empty()`
   - Assert the face count increased by 1 (chamfer face added, replacing the edge)

2. `chamfer_cube_multiple_edges`:
   - Create a unit cube
   - Apply chamfer to 2-3 non-adjacent edges
   - Assert same topological validity checks
   - Verify the chamfer faces are ruled (linear in one direction) -- check surface type is BsplineSurface

3. `chamfer_variable_radius`:
   - Create a unit cube
   - Apply chamfer with `RadiusSpec::Variable` (e.g., linear from 0.05 to 0.15)
   - Assert topological validity

4. `chamfer_per_edge_radius`:
   - Apply chamfer with `RadiusSpec::PerEdge` on two edges with different radii
   - Assert topological validity

5. `chamfer_serialization_round_trip`:
   - Create chamfered solid, compress it, serialize to JSON, deserialize, extract
   - Assert the extracted solid is topologically valid

Run all tests. Investigate and fix any failures that arise (may need to adjust tolerance or division count). The existing chamfer tests in the file (around lines 1183, 1260, 1295, 1387) already exercise some paths -- review them and ensure the new tests cover the specific success criteria.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-solid -E 'test(fillet::tests::chamfer)'` and confirm all chamfer tests pass.
Run `cargo nextest run -p monstertruck-solid -E 'test(fillet)'` to confirm no regressions in existing fillet tests.
  </verify>
  <done>Chamfer operations verified to produce topologically valid closed shells with flat-cut edges. Tests cover single edge, multiple edges, variable radius, per-edge radius, and serialization round-trip.</done>
</task>

<task type="auto">
  <name>Task 2: Verify chamfer API surface and enhance documentation</name>
  <files>monstertruck-solid/src/fillet/params.rs</files>
  <action>
1. Verify the public API chain works:
   - `FilletProfile::Chamfer` is already in `params.rs` and re-exported from `monstertruck-solid::fillet`
   - `FilletProfile` is already re-exported from `monstertruck-modeling` (confirmed at line 130)
   - No code changes needed for re-exports

2. Enhance documentation in `monstertruck-solid/src/fillet/params.rs`:
   - Add a more detailed doc comment to `FilletProfile::Chamfer`:
     ```rust
     /// Flat ruled surface (chamfer/bevel).
     ///
     /// Creates a flat cut between two adjacent faces, replacing the shared edge
     /// with a ruled surface. Unlike [`Round`](Self::Round), which creates a
     /// circular arc cross-section, `Chamfer` creates a straight-line transition.
     /// Use with [`FilletOptions::with_profile`].
     ```

3. Verify the docs render correctly:
   `cargo doc -p monstertruck-solid --no-deps`
  </action>
  <verify>
Run `cargo doc -p monstertruck-solid --no-deps` and verify `FilletProfile::Chamfer` doc appears.
Run `cargo nextest run -p monstertruck-modeling --features fillet` to confirm no build errors.
  </verify>
  <done>Chamfer API documentation enhanced. FilletProfile::Chamfer accessible from monstertruck-modeling confirmed.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-solid -E 'test(fillet::tests::chamfer)'` -- all chamfer-specific tests pass
2. `cargo nextest run -p monstertruck-solid -E 'test(fillet)'` -- no regressions in fillet tests
3. `cargo nextest run -p monstertruck-modeling --features fillet` -- modeling crate builds with fillet feature
4. Chamfered solid shells are topologically valid (closed, no singular vertices, no boundaries)
5. Chamfer produces flat-cut (ruled) surfaces between adjacent faces
</verification>

<success_criteria>
- Chamfer operations produce flat-cut edges on solid bodies and the result passes topological validity checks
- FilletProfile::Chamfer is accessible from the modeling crate API
- Tests cover single edge, multiple edges, variable radius, per-edge radius, and serialization
</success_criteria>

<output>
After completion, create `.tendrion/phases/3-feature-completeness/3-2-SUMMARY.md`
</output>
