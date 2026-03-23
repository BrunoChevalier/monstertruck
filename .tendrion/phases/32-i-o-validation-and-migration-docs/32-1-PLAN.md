---
phase: 32-i-o-validation-and-migration-docs
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-step/tests/step_export_validation.rs
  - monstertruck-mesh/tests/obj_export_validation.rs
  - monstertruck-mesh/tests/stl_export_validation.rs
autonomous: true
must_haves:
  truths:
    - "User runs `cargo nextest run -p monstertruck-step --test step_export_validation` and all STEP export round-trip tests pass"
    - "User runs `cargo nextest run -p monstertruck-mesh --test obj_export_validation` and all OBJ format validation tests pass"
    - "User runs `cargo nextest run -p monstertruck-mesh --test stl_export_validation` and all STL format validation tests pass"
    - "STEP round-trip tests verify bounding box coordinates match within tolerance after write-read cycle for sphere and torus geometries"
    - "OBJ export tests verify vertex count and face count in output text match the source mesh"
    - "STL export tests verify valid binary header, correct face count, and consistent normal orientation"
  artifacts:
    - path: "monstertruck-step/tests/step_export_validation.rs"
      provides: "STEP export round-trip fidelity tests for additional geometries beyond Phase 29 cube/cylinder coverage"
      min_lines: 80
      contains: "bounding_box"
    - path: "monstertruck-mesh/tests/obj_export_validation.rs"
      provides: "OBJ export format validation tests verifying vertex/face formatting and mesh vertex counts"
      min_lines: 60
      contains: "vertex_count"
    - path: "monstertruck-mesh/tests/stl_export_validation.rs"
      provides: "STL export format validation tests for binary/ASCII format and face normal orientation"
      min_lines: 60
      contains: "normal"
  key_links:
    - from: "monstertruck-step/tests/step_export_validation.rs"
      to: "monstertruck-step/src/save/mod.rs"
      via: "Uses CompleteStepDisplay and StepModel for STEP export"
      pattern: "CompleteStepDisplay"
    - from: "monstertruck-mesh/tests/obj_export_validation.rs"
      to: "monstertruck-mesh/src/obj.rs"
      via: "Uses obj::write and obj::read for round-trip validation"
      pattern: "obj::write"
    - from: "monstertruck-mesh/tests/stl_export_validation.rs"
      to: "monstertruck-mesh/src/stl.rs"
      via: "Uses stl::write and StlReader for format validation"
      pattern: "stl::write"
---

<objective>
Add comprehensive I/O validation tests for STEP, OBJ, and STL export pipelines. STEP tests extend Phase 29's coverage with additional geometries (sphere, torus) and explicit bounding-box tolerance checks. OBJ tests verify correct vertex/face formatting and mesh vertex count preservation. STL tests verify binary/ASCII format validity and consistent face normal orientation.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-step/tests/roundtrip_coverage.rs
@monstertruck-mesh/tests/obj-io.rs
@monstertruck-mesh/tests/stl-io.rs
@monstertruck-step/Cargo.toml
@monstertruck-mesh/Cargo.toml
</context>

<tasks>

<task type="auto">
  <name>Task 1: STEP export validation tests for additional geometries</name>
  <files>monstertruck-step/tests/step_export_validation.rs</files>
  <action>
Create `monstertruck-step/tests/step_export_validation.rs` with STEP export round-trip fidelity tests that complement the existing `roundtrip_coverage.rs` tests from Phase 29.

The test file should:

1. Import the same dependencies as `roundtrip_coverage.rs`:
   - `monstertruck_meshing::prelude::*`
   - `monstertruck_modeling::*`
   - `monstertruck_step::load::*` and `monstertruck_step::save::*`
   - `monstertruck_topology::shell::ShellCondition`

2. Reuse the same `roundtrip_shell` and `bounding_box_matches` helper pattern from `roundtrip_coverage.rs` (copy the helpers into this file).

3. Add these validation tests:

**`export_sphere_roundtrip_bbox`**: Build a sphere solid using `builder::revolve` (revolve a semicircular arc around the Y axis). Export to STEP, reimport, and assert bounding box matches within tolerance 0.15. Verify the STEP string contains "CLOSED_SHELL".

**`export_torus_roundtrip_bbox`**: Build a torus solid by creating a circular cross-section and revolving it around an offset axis with `builder::revolve`. Export to STEP, reimport, and assert bounding box matches within tolerance 0.15.

**`export_extrusion_roundtrip_bbox`**: Build an L-shaped profile (a wire with 6 edges forming an L) and extrude it. Export to STEP, reimport, and verify bounding box preservation and face count >= 8.

**`export_step_entities_complete`**: Build a simple cube, export to STEP string, and verify the output contains ALL required STEP entities: CARTESIAN_POINT, EDGE_CURVE, VERTEX_POINT, FACE_BOUND, ADVANCED_FACE, CLOSED_SHELL, and the ISO header line.

Note: The sphere construction may need `builder::vertex` + `builder::line` to create an arc, or use `builder::revolve` on a wire. Consult the existing test patterns in `roundtrip_coverage.rs` for how to build shapes. If revolve is complex, use simpler construction: e.g., extrude a circle edge to make a cylinder-like shape, or construct via vertex/edge/face/solid builder chain.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-step --test step_export_validation` and all tests pass. Run `cargo clippy -p monstertruck-step --tests -- -W warnings` with no new warnings.</verify>
  <done>STEP export validation tests created with bounding-box-matching round-trip tests for additional geometries beyond Phase 29 coverage.</done>
</task>

<task type="auto">
  <name>Task 2: OBJ export format validation tests</name>
  <files>monstertruck-mesh/tests/obj_export_validation.rs</files>
  <action>
Create `monstertruck-mesh/tests/obj_export_validation.rs` with tests that validate OBJ export format correctness and geometry integrity beyond the existing round-trip equality tests in `obj-io.rs`.

The test file should:

1. Import `monstertruck_mesh::*`.

2. Add these validation tests:

**`obj_export_vertex_count_matches`**: Create a cube PolygonMesh with 8 positions (reuse the cube positions from `obj-io.rs` pattern). Write to OBJ via `obj::write`. Parse the output as a string and count lines starting with "v " (vertex lines). Assert the count equals the source mesh position count (8). Count lines starting with "f " (face lines) and assert it matches the source face count.

**`obj_export_vertex_format_valid`**: Create a mesh, write to OBJ. Parse each "v " line and verify it has exactly 3 floating-point coordinates. Parse each "f " line and verify all vertex indices are within the valid range [1, vertex_count].

**`obj_export_with_normals_format`**: Create a mesh with normals (reuse cube::NORMALS pattern). Write to OBJ. Verify "vn " lines exist in the output and each has exactly 3 float coordinates. Verify "f " lines use the `v//vn` format with valid indices.

**`obj_export_with_texcoords_format`**: Create a mesh with UV coordinates (reuse cube::UV_COORDS pattern). Write to OBJ. Verify "vt " lines exist and each has exactly 2 float coordinates. Verify "f " lines use the `v/vt` format.

**`obj_export_roundtrip_vertex_count_preserved`**: Create a mesh with known vertex count, write to OBJ, read back, and assert `mesh.positions().len()` is preserved through the cycle.

Each test should use `String::from_utf8(gened_obj).unwrap()` to convert the OBJ bytes to string for line-by-line parsing.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-mesh --test obj_export_validation` and all tests pass. Run `cargo clippy -p monstertruck-mesh --tests -- -W warnings` with no new warnings.</verify>
  <done>OBJ export format validation tests created verifying vertex/face formatting and mesh vertex count preservation.</done>
</task>

<task type="auto">
  <name>Task 3: STL export format validation tests</name>
  <files>monstertruck-mesh/tests/stl_export_validation.rs</files>
  <action>
Create `monstertruck-mesh/tests/stl_export_validation.rs` with tests that validate STL export format correctness beyond the existing round-trip tests in `stl-io.rs`.

The test file should:

1. Import:
   - `monstertruck_mesh::*`
   - `stl::{IntoStlIterator, StlFace, StlReader, StlType}`

2. Add these validation tests:

**`stl_binary_header_valid`**: Create a known set of StlFace entries (e.g., 3 triangle faces forming a tetrahedron). Write to binary STL via `stl::write(..., StlType::Binary)`. Verify: the output length is exactly 84 + (50 * face_count) bytes (80 header + 4 byte face count + 50 bytes per face). Read the face count from bytes 80..84 as a little-endian u32 and assert it matches the input face count.

**`stl_ascii_format_valid`**: Create 3 StlFace entries, write to ASCII STL. Convert to string. Verify the output starts with "solid" and ends with "endsolid". Count occurrences of "facet normal" and assert it equals the face count. Count occurrences of "vertex" (excluding "vertex" within other words -- use line-based counting of lines containing "vertex" that don't contain "facet") and assert it equals 3 * face_count.

**`stl_normal_orientation_consistent`**: Create a closed tetrahedron with 4 faces where normals point outward. Write to binary STL, read back. For each face, compute the cross product of (v1-v0) x (v2-v0) and verify it has positive dot product with the stored normal (i.e., the stored normal points in the same general direction as the geometric normal). This verifies consistent face normal orientation.

**`stl_binary_ascii_roundtrip_equivalent`**: Create a set of faces, write to both binary and ASCII STL. Read both back via StlReader. Assert the face data (vertices and normals) match between the two formats within floating-point tolerance.

Use `monstertruck_core::assert_near` for floating-point comparisons where needed.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-mesh --test stl_export_validation` and all tests pass. Run `cargo clippy -p monstertruck-mesh --tests -- -W warnings` with no new warnings.</verify>
  <done>STL export format validation tests created verifying binary/ASCII format validity and consistent face normal orientation.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-step --test step_export_validation` -- all tests pass
2. `cargo nextest run -p monstertruck-mesh --test obj_export_validation` -- all tests pass
3. `cargo nextest run -p monstertruck-mesh --test stl_export_validation` -- all tests pass
4. `cargo nextest run -p monstertruck-step --test roundtrip_coverage` -- existing tests still pass (no regressions)
5. `cargo nextest run -p monstertruck-mesh --test obj-io --test stl-io` -- existing tests still pass
6. `cargo clippy -p monstertruck-step -p monstertruck-mesh --tests -- -W warnings` -- no new warnings
</verification>

<success_criteria>
- STEP export round-trip tests verify bounding boxes match within tolerance for geometries beyond basic cube/cylinder (IO-01)
- OBJ export tests verify correct vertex/face formatting and mesh vertex counts match the source tessellation (IO-02)
- STL export tests verify valid binary/ASCII format output and consistent face normal orientation (IO-02)
- All new tests pass alongside existing I/O tests with no regressions
</success_criteria>

<output>
After completion, create `.tendrion/phases/32-i-o-validation-and-migration-docs/32-1-SUMMARY.md`
</output>
