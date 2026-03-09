---
phase: 2-numerical-robustness
plan: 3
type: tdd
wave: 2
depends_on: ["2-1"]
files_modified:
  - monstertruck-meshing/src/tessellation/boundary_stitching.rs
  - monstertruck-meshing/src/tessellation/mod.rs
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/tests/tessellation/boundary_stitching.rs
  - monstertruck-meshing/tests/tessellation/main.rs
autonomous: true
must_haves:
  truths:
    - "User tessellates a solid with trimmed surfaces and adjacent faces share boundary vertices -- no visible seams in the rendered output"
    - "User tessellates the standard test models (cube, cylinder, fillet) and put_together_same_attrs produces a closed mesh"
    - "User calls boundary-aware tessellation and the vertex count along shared edges is consistent between adjacent faces"
    - "User tessellates with TessellationOptions and the boundary stitching respects the specified tolerance"
  artifacts:
    - path: "monstertruck-meshing/src/tessellation/boundary_stitching.rs"
      provides: "Boundary-aware stitching module that merges tessellation vertices along shared topological edges"
      min_lines: 100
      contains: "stitch_boundaries"
    - path: "monstertruck-meshing/tests/tessellation/boundary_stitching.rs"
      provides: "Tests verifying seam elimination on trimmed surface tessellation"
      min_lines: 80
      contains: "no_seams"
  key_links:
    - from: "monstertruck-meshing/src/tessellation/boundary_stitching.rs"
      to: "monstertruck-meshing/src/tessellation/triangulation.rs"
      via: "Stitching is invoked after per-face triangulation in shell_tessellation"
      pattern: "stitch_boundaries"
    - from: "monstertruck-meshing/src/tessellation/boundary_stitching.rs"
      to: "monstertruck-meshing/src/tessellation/mod.rs"
      via: "Module declaration and re-export"
      pattern: "boundary_stitching"
---

<objective>
Implement boundary-aware vertex stitching during tessellation so that adjacent trimmed faces share boundary vertices, eliminating visible seams in rendered output for standard test models.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-meshing/src/tessellation/mod.rs
@monstertruck-meshing/src/tessellation/triangulation.rs
@monstertruck-meshing/src/filters/optimizing.rs
@monstertruck-meshing/src/common/adjacency.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write failing tests for boundary stitching</name>
  <files>monstertruck-meshing/tests/tessellation/boundary_stitching.rs</files>
  <action>
Create test file `monstertruck-meshing/tests/tessellation/boundary_stitching.rs`:

1. **Test `no_seams_on_cube`**: Build a unit cube using `monstertruck_modeling::builder`, tessellate with `shell.triangulation(0.01)`, convert to polygon with `to_polygon()`, call `put_together_same_attrs(TOLERANCE)`, and verify `shell_condition() == ShellCondition::Closed`. This test should already pass with existing code (it's a regression guard).

2. **Test `no_seams_on_cylinder`**: Create a cylinder by revolving a line segment, tessellate, and verify after `put_together_same_attrs` the mesh is closed. This exercises trimmed surface boundaries.

3. **Test `boundary_vertices_shared_between_adjacent_faces`**: Tessellate a two-face shell (e.g., two adjacent planar faces sharing an edge). Extract the polyline edges along the shared boundary. Verify that the polyline vertices are identical (pointer-equal or coordinate-equal within TOLERANCE) between the two faces' tessellations. This is the core test that catches the seam bug.

4. **Test `stitched_tessellation_preserves_vertex_count`**: Tessellate a trimmed surface pair and verify the boundary stitching does not change the total vertex count significantly (it merges duplicates but should not add or remove geometric content).

5. **Test `stitch_respects_tolerance`**: Tessellate with a loose tolerance (0.1) and a tight tolerance (0.001), verify both produce seam-free results along shared edges.

Use `monstertruck_meshing::prelude::*`, `monstertruck_modeling::*`, and `monstertruck_topology::shell::ShellCondition`.
All tests should use `cargo nextest run` for verification.

**IMPORTANT:** Also add `mod boundary_stitching;` to `monstertruck-meshing/tests/tessellation/main.rs` so the test module is discovered by the test harness.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-meshing -E 'test(boundary_stitching)'` and confirm the tests that exercise stitching fail (tests 3-5 should fail if the stitching module doesn't exist yet; tests 1-2 may pass as regression guards).</verify>
  <done>Failing tests written for boundary stitching behavior.</done>
</task>

<task type="auto">
  <name>Task 2: Implement boundary-aware vertex stitching</name>
  <files>monstertruck-meshing/src/tessellation/boundary_stitching.rs, monstertruck-meshing/src/tessellation/mod.rs</files>
  <action>
Create `monstertruck-meshing/src/tessellation/boundary_stitching.rs`:

The core algorithm:

1. **Build edge-to-faces adjacency map**: After per-face tessellation, iterate over all faces in the shell. For each face, extract the polyline boundary edges. Each topological edge has an `id()` -- group faces that share the same edge ID.

2. **For each shared edge, align polyline vertices**:
   - Two adjacent faces produce two polyline approximations of the same edge curve.
   - One is the "canonical" polyline (from the first face encountered), the other is the "duplicate".
   - The duplicate's boundary vertices must be replaced with references to the canonical polyline's vertices.
   - Since both polylines approximate the same curve, vertices should be within tolerance. Match by nearest-point or by parameter ordering.

3. **Merge vertex positions**: For each pair of corresponding boundary vertices from adjacent faces:
   - If they are within `tolerance` of each other, replace both with their midpoint.
   - Update the polygon mesh vertex indices to point to the shared position.

4. **Integration point**: The stitching function should have signature:
```rust
pub fn stitch_boundaries(
    shell: &mut Shell<Point3, PolylineCurve, Option<PolygonMesh>>,
    tolerance: f64,
)
```
   - It modifies the tessellated shell in-place, ensuring shared edges use identical vertex positions.

5. **Alternative simpler approach**: Since `shell_tessellation` already builds a shared `edge_map` (see `triangulation.rs` line 73-86), the polyline edges are already shared between faces via `Edge::debug_new`. The seam problem occurs because the polygon mesh positions are computed independently per face from surface evaluation. The fix is:
   - After building the per-face polygon meshes, collect all boundary vertices (from polyline edges).
   - For each shared edge, ensure both faces' polygon meshes use the exact same position values for vertices on that edge.
   - This can be done by building a `HashMap<(EdgeId, usize), Point3>` mapping (edge, vertex_index) -> canonical position, and then rewriting each face's polygon positions to use the canonical values.

Register the module in `mod.rs` with `mod boundary_stitching;` and `pub use boundary_stitching::stitch_boundaries;`.

All code comments must end with a period.
Use functional style, SmallVec where appropriate.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-meshing -E 'test(boundary_stitching)'` and confirm all tests pass.</verify>
  <done>Boundary-aware vertex stitching implemented and integrated into tessellation pipeline.</done>
</task>

<task type="auto">
  <name>Task 3: Integrate stitching into tessellation pipeline and verify end-to-end</name>
  <files>monstertruck-meshing/src/tessellation/triangulation.rs, monstertruck-meshing/src/tessellation/mod.rs</files>
  <action>
Integrate boundary stitching into the existing tessellation flow:

1. In `shell_tessellation` (triangulation.rs), after all faces are tessellated and the meshed shell is assembled, call `stitch_boundaries(&mut result, tolerance)` before returning.

2. Similarly update `shell_tessellation_single_thread` (the wasm path) with the same stitching call.

3. Update `cshell_tessellation` for `CompressedShell` if the stitching approach is applicable (it may need a parallel implementation).

4. In `TessellationOptions`, consider whether stitching should be opt-in or always-on. Default to always-on since it fixes a correctness issue. Add `pub stitch_boundaries: bool` field with `default = true` if making it optional.

5. Run the full test suite to verify no regressions:
   - `cargo nextest run -p monstertruck-meshing` -- all tests pass.
   - `cargo nextest run -p monstertruck-solid` -- boolean operation tests still pass (they use tessellation internally).
   - Check that the existing `MeshableShape` doc test (cube tessellation -> put_together_same_attrs -> Closed) still passes.

Ensure all code comments end with a period.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-meshing -p monstertruck-solid` and confirm all tests pass including the new boundary stitching tests and existing regression tests.</verify>
  <done>Boundary stitching integrated into tessellation pipeline with end-to-end verification.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-meshing` -- all tests pass including boundary stitching tests.
2. `cargo nextest run -p monstertruck-solid` -- boolean operation tests pass (they use tessellation).
3. Tessellating a cube, cylinder, and filleted box produces meshes where `put_together_same_attrs` yields `ShellCondition::Closed`.
4. Adjacent trimmed faces share exact boundary vertex positions after tessellation.
5. No regressions in existing tessellation behavior.
</verification>

<success_criteria>
- ROBUST-03 complete: Adjacent trimmed faces share boundary vertices after tessellation with no visible seams
- Boundary stitching uses topological adjacency information to match shared edges
- Standard test models (cube, cylinder) produce closed meshes after tessellation and vertex merging
- All existing tessellation tests continue to pass
</success_criteria>

<output>
After completion, create `.tendrion/phases/2-numerical-robustness/2-3-SUMMARY.md`
</output>
