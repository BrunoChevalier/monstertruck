---
phase: 27-topology-coverage
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-topology/tests/face_shell_ops.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo nextest run -p monstertruck-topology and all new face/shell tests pass"
    - "Face creation with try_new validates boundary wire conditions (empty, not closed, not simple)"
    - "Face inversion, boundary traversal, and boundary_iters are tested"
    - "Face cut_by_edge and glue_at_boundaries operations have dedicated tests"
    - "Shell connectivity, shell_condition (Irregular/Regular/Oriented/Closed) are tested"
    - "Shell extract_boundaries, connected_components, singular_vertices are tested"
    - "Shell vertex_adjacency and face_adjacency matrix operations are tested"
  artifacts:
    - path: "monstertruck-topology/tests/face_shell_ops.rs"
      provides: "Comprehensive integration tests for face, shell, and solid operations"
      min_lines: 300
      contains: "fn test_face_"
  key_links:
    - from: "monstertruck-topology/tests/face_shell_ops.rs"
      to: "monstertruck-topology/src/face.rs"
      via: "test imports and exercises Face public API"
      pattern: "Face::new"
    - from: "monstertruck-topology/tests/face_shell_ops.rs"
      to: "monstertruck-topology/src/shell.rs"
      via: "test imports and exercises Shell public API"
      pattern: "shell_condition"
---

<objective>
Create comprehensive tests for face boundary traversal, shell connectivity and orientation validation, and solid construction to increase monstertruck-topology test coverage to 50%+.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-topology/src/face.rs
@monstertruck-topology/src/shell.rs
@monstertruck-topology/src/solid.rs
@monstertruck-topology/src/lib.rs
@monstertruck-topology/src/errors.rs
@monstertruck-topology/src/compress.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create face operation tests</name>
  <files>monstertruck-topology/tests/face_shell_ops.rs</files>
  <action>
Create a new integration test file `monstertruck-topology/tests/face_shell_ops.rs` with the following test functions. Use `monstertruck_topology::*`, `monstertruck_topology::errors::Error`, and `monstertruck_topology::shell::ShellCondition` imports. All tests use unit geometry `()` unless otherwise stated.

**Face creation and validation tests:**
- `test_face_try_new_empty_wire`: Verify `Face::try_new(vec![Wire::new()], ())` returns `Err(Error::EmptyWire)`.
- `test_face_try_new_not_closed`: Create open wire (v0->v1), verify `Face::try_new` returns `Err(Error::NotClosedWire)`.
- `test_face_try_new_not_simple`: Create non-simple closed wire (figure-8: v0->v1->v2->v3->v1->v0), verify returns `Err(Error::NotSimpleWire)`.
- `test_face_try_new_success`: Create valid triangular boundary (v0->v1->v2->v0), verify `Face::try_new` returns Ok.
- `test_face_new_quad`: Create quadrilateral face (v0->v1->v2->v3->v0), verify boundaries() returns matching wire.
- `test_face_try_new_non_disjoint_wires`: Create two boundary wires sharing a vertex, verify returns `Err(Error::NotSimpleWire)` (from the disjoint_wires check).

**Face identity and properties:**
- `test_face_id_uniqueness`: Two faces from the same wire (but separate Face::new calls) have different ids. Cloned faces share id. Inverse face has same id.
- `test_face_is_same`: Create face, inverse it, verify `is_same` returns true. Create another face from same wire, verify `is_same` returns false.
- `test_face_count`: Create face, verify count()==1. Clone, verify count()==2. Drop, verify count()==1.
- `test_face_surface_get_set`: Create `Face::new(vec![wire], 42)`, verify `surface() == 42`. Call `set_surface(99)`, verify cloned face also sees 99.
- `test_face_orientation`: New face has `orientation() == true`. After `invert()`, `orientation() == false`.

**Face boundary traversal:**
- `test_face_boundaries`: Create triangular face, verify `boundaries()` returns 1 wire with 3 edges matching the input.
- `test_face_boundaries_inverted`: Create face, invert it, verify `boundaries()` returns the inverse wire.
- `test_face_absolute_boundaries`: Create face, invert it, verify `absolute_boundaries()` returns the original (non-inverted) wire.
- `test_face_absolute_clone`: Create face, invert it, take `absolute_clone()`, verify it has `orientation() == true` and `is_same()` with original.
- `test_face_into_boundaries`: Create face, consume with `into_boundaries()`, verify returned wires.
- `test_face_boundary_iters`: Create triangular face, get `boundary_iters()`, iterate through and verify edges match. Test with inverted face too.
- `test_face_boundary_iter_double_ended`: Create face with 4-edge boundary, verify `next()` and `next_back()` work as double-ended iterator.
- `test_face_edge_iter`: Create face with 2 boundary wires, verify `edge_iter()` yields all edges from both wires.
- `test_face_vertex_iter`: Create triangular face, verify `vertex_iter()` yields 3 vertices.
- `test_face_add_boundary`: Create face with 1 boundary, add a second disjoint boundary wire, verify `boundaries()` returns 2 wires.
- `test_face_try_add_boundary_errors`: Attempt adding empty wire (EmptyWire), open wire (NotClosedWire), non-simple wire (NotSimpleWire), non-disjoint wire (NotDisjointWires) -- verify each error.

**Face cutting and gluing:**
- `test_face_cut_by_edge`: Create quad face (v0->v1->v2->v3->v0), cut with edge v1->v3. Verify two resulting triangular faces have correct vertex sets.
- `test_face_cut_by_edge_fails_multiple_boundaries`: Create face with 2 boundaries, verify `cut_by_edge` returns None.
- `test_face_cut_by_edge_fails_missing_vertex`: Create quad face, cut with edge using a vertex not in the boundary, verify returns None.
- `test_face_cut_by_wire`: Create quad face with 5 vertices (v0..v4), cut with 2-edge wire v1->v4->v3. Verify two resulting faces have correct vertex sets.
- `test_face_border_on`: Create two faces sharing an edge, verify `border_on` returns true. Create two faces with no shared edges, verify returns false.
- `test_face_border_wires`: Create two adjacent faces, verify `border_wires` returns the shared edges as wires.
- `test_face_glue_at_boundaries`: Create two faces that share boundary edges (per the doctest pattern with 8 vertices), glue them, verify resulting face has expected boundary count and edge count.
- `test_face_display_formats`: Create face, test `FaceDisplayFormat::Full`, `BoundariesAndID`, `BoundariesAndSurface`, `LoopsListTuple`, `LoopsList`, `AsSurface` all produce non-empty debug strings.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-topology --test face_shell_ops` and verify all face tests pass.</verify>
  <done>Face operation tests created and passing, covering creation validation, boundary traversal, cutting, and gluing.</done>
</task>

<task type="auto">
  <name>Task 2: Create shell and solid operation tests</name>
  <files>monstertruck-topology/tests/face_shell_ops.rs</files>
  <action>
Add shell and solid operation tests to the same file:

**Shell construction tests:**
- `test_shell_new_empty`: Create `Shell::<(), (), ()>::new()`, verify is_empty, `is_connected()` returns true for empty shell.
- `test_shell_with_capacity`: Create `Shell::with_capacity(10)`, verify empty.
- `test_shell_from_vec`: Create shell from `Vec<Face>`, verify len and face_iter count.
- `test_shell_from_iter`: Create shell using `.collect::<Shell<_,_,_>>()` from face iterator.
- `test_shell_push_and_deref`: Push faces to shell, access via indexing (Deref to Vec), verify.
- `test_shell_append`: Create two shells, append one to the other, verify combined length.
- `test_shell_macro`: Use `shell![]` macro to create shell, verify it works.

**Shell condition tests (all 4 conditions):**
- `test_shell_condition_irregular`: Create shell where 3 faces share one edge (edge appears 3 times). Verify `shell_condition() == ShellCondition::Irregular`.
- `test_shell_condition_regular`: Create shell where edge orientations are incompatible (same direction on shared edge). Verify `shell_condition() == ShellCondition::Regular`.
- `test_shell_condition_oriented`: Create oriented but non-closed shell (some boundary edges). Verify `shell_condition() == ShellCondition::Oriented`.
- `test_shell_condition_closed`: Create fully closed shell (cube topology using the `cube()` pattern from solid.rs). Verify `shell_condition() == ShellCondition::Closed`.
- `test_shell_condition_bitand`: Test that `ShellCondition::Closed & ShellCondition::Oriented == ShellCondition::Oriented`, and other bitand combinations.

**Shell connectivity tests:**
- `test_shell_is_connected_single_face`: Single-face shell is connected.
- `test_shell_is_connected_shared_edge`: Two faces sharing an edge are connected.
- `test_shell_is_connected_disconnected`: Two faces with no shared vertices are not connected.
- `test_shell_connected_components`: Create shell with two disconnected groups of faces, verify `connected_components()` returns 2 components with correct sizes.
- `test_shell_connected_components_empty`: Empty shell returns no components.

**Shell adjacency tests:**
- `test_shell_vertex_adjacency`: Create 2-face shell, verify `vertex_adjacency()` maps contain correct adjacent vertices for each vertex.
- `test_shell_face_adjacency`: Create 4-face shell (as in the doctest), verify `face_adjacency()` maps show correct neighbor counts (e.g., central face adjacent to 3 others).

**Shell boundary and singular vertex tests:**
- `test_shell_extract_boundaries`: Create oriented shell with boundary edges, verify `extract_boundaries()` returns correct boundary wire(s).
- `test_shell_singular_vertices_none`: Create manifold shell (Mobius strip pattern or cube), verify `singular_vertices()` is empty.
- `test_shell_singular_vertices_present`: Create shell with a singular vertex (wedge of two spheres pattern from errors.rs doctest), verify `singular_vertices()` returns the singular vertex.

**Shell iteration tests:**
- `test_shell_face_iter`: Verify `face_iter()` yields correct count.
- `test_shell_edge_iter`: Verify `edge_iter()` yields all edges from all faces.
- `test_shell_vertex_iter`: Verify `vertex_iter()` yields front vertices of all edges.

**Solid tests:**
- `test_solid_new_valid`: Build a tetrahedron solid (4 triangular faces forming a closed shell), verify construction succeeds.
- `test_solid_try_new_empty_shell`: Verify `Solid::try_new(vec![Shell::new()])` returns `Err(Error::EmptyShell)`.
- `test_solid_try_new_not_connected`: Create disconnected shell, verify `Err(Error::NotConnected)`.
- `test_solid_try_new_not_closed`: Create oriented but not closed shell, verify `Err(Error::NotClosedShell)`.
- `test_solid_try_new_not_manifold`: Create wedge-of-spheres shell (singular vertex), verify `Err(Error::NotManifold)`.
- `test_solid_boundaries`: Build cube solid, verify `boundaries()` returns 1 shell.
- `test_solid_into_boundaries`: Build solid, consume with `into_boundaries()`, verify shell count.
- `test_solid_face_iter`: Build cube solid, verify `face_iter()` yields 6 faces.
- `test_solid_edge_iter`: Build cube solid, verify `edge_iter()` count (each edge appears in 2 faces, so 24 edge references).
- `test_solid_vertex_iter`: Build cube solid, verify `vertex_iter()` count.
- `test_solid_not`: Build cube solid, call `not()`, verify all face orientations flipped.
- `test_solid_display_formats`: Create solid, test `SolidDisplayFormat::Struct`, `ShellsListTuple`, `ShellsList` produce non-empty debug strings.

**Compress round-trip test (hits compress.rs):**
- `test_shell_compress_extract_roundtrip`: Build a shell, compress it, extract it back, verify face count matches and topology is equivalent (use the `same_topology` pattern or compare lengths).

Build a `tetrahedron()` helper function (4 vertices, 6 edges, 4 triangular faces forming a closed shell) and a `cube_shell()` helper function reusable across tests.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-topology --test face_shell_ops` and verify all tests pass.</verify>
  <done>Shell and solid operation tests created and passing, covering conditions, connectivity, adjacency, boundaries, singular vertices, and compress roundtrip.</done>
</task>

<task type="auto">
  <name>Task 3: Run full test suite, clippy, and verify coverage improvement</name>
  <files>monstertruck-topology/tests/face_shell_ops.rs</files>
  <action>
Run the complete monstertruck-topology test suite:
```
cargo nextest run -p monstertruck-topology --no-fail-fast
```

Run clippy:
```
cargo clippy -p monstertruck-topology --all-targets -- -W warnings
```

Fix any compilation errors, test failures, or clippy warnings.

Then optionally run coverage to verify improvement direction:
```
cargo tarpaulin -p monstertruck-topology --skip-clean --out Stdout 2>/dev/null | tail -5
```
(If tarpaulin is not available, this step is informational only -- the executor should note the test count increase as a proxy for coverage improvement.)

Ensure all existing tests plus new tests pass cleanly with no clippy warnings.
  </action>
  <verify>All tests pass with `cargo nextest run -p monstertruck-topology --no-fail-fast`. No clippy warnings. Test count has significantly increased.</verify>
  <done>Full test suite passes with no regressions, no clippy warnings, and significant test coverage improvement.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-topology --test face_shell_ops` passes all tests
2. `cargo nextest run -p monstertruck-topology --no-fail-fast` passes all tests including existing ones
3. `cargo clippy -p monstertruck-topology --all-targets -- -W warnings` produces no warnings
4. Test file contains at least 300 lines covering face, shell, and solid operations
5. All four ShellCondition variants are exercised in dedicated test cases
6. Shell connectivity and singular vertex detection are tested with both positive and negative cases
</verification>

<success_criteria>
- Face creation with validation of boundary wire conditions has dedicated test cases
- Face boundary traversal with BoundaryIter (double-ended) has dedicated test cases
- Face cutting (cut_by_edge, cut_by_wire) and gluing (glue_at_boundaries) have dedicated test cases
- Shell connectivity (is_connected, connected_components) has dedicated test cases
- Shell condition (Irregular, Regular, Oriented, Closed) has dedicated test cases for each variant
- Shell orientation validation (singular_vertices, extract_boundaries) has dedicated test cases
- Solid construction validation (all error cases) has dedicated test cases
- Compress/extract roundtrip is tested
</success_criteria>

<output>
After completion, create `.tendrion/phases/27-topology-coverage/27-2-SUMMARY.md`
</output>
