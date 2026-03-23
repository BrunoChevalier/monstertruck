---
phase: 27-topology-coverage
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-topology/tests/edge_wire_vertex_ops.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo nextest run -p monstertruck-topology and all new edge/wire/vertex tests pass"
    - "Edge creation with try_new validates same-vertex rejection"
    - "Edge inversion, absolute_clone, and orientation are tested"
    - "Edge splitting via cut_with_parameter produces two connected sub-edges"
    - "Wire continuity, closure, simplicity, and inversion are tested"
    - "Wire split_off, append, swap_edge_into_wire operations are tested"
    - "Vertex creation, cloning, id uniqueness, set_point synchronization are tested"
  artifacts:
    - path: "monstertruck-topology/tests/edge_wire_vertex_ops.rs"
      provides: "Comprehensive integration tests for edge, wire, and vertex operations"
      min_lines: 250
      contains: "fn test_edge_"
  key_links:
    - from: "monstertruck-topology/tests/edge_wire_vertex_ops.rs"
      to: "monstertruck-topology/src/edge.rs"
      via: "test imports and exercises Edge public API"
      pattern: "Edge::new"
    - from: "monstertruck-topology/tests/edge_wire_vertex_ops.rs"
      to: "monstertruck-topology/src/wire.rs"
      via: "test imports and exercises Wire public API"
      pattern: "Wire::from"
---

<objective>
Create comprehensive tests for edge creation, splitting, and merging operations, wire construction and traversal, and vertex operations to increase monstertruck-topology test coverage toward 50%.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-topology/src/edge.rs
@monstertruck-topology/src/wire.rs
@monstertruck-topology/src/vertex.rs
@monstertruck-topology/src/lib.rs
@monstertruck-topology/src/errors.rs
@monstertruck-topology/tests/euler_operators.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create vertex and edge operation tests</name>
  <files>monstertruck-topology/tests/edge_wire_vertex_ops.rs</files>
  <action>
Create a new integration test file `monstertruck-topology/tests/edge_wire_vertex_ops.rs` with the following test functions. Use `monstertruck_topology::*` and `monstertruck_topology::errors::Error` imports. All tests use unit geometry types `()` for P, C, S unless geometric operations (cut/concat) are needed.

**Vertex tests:**
- `test_vertex_creation_and_identity`: Create vertices with `Vertex::new(())`, verify different vertices have different ids, cloned vertices share id. Use `Vertex::news(&[(); 4])` for batch creation.
- `test_vertex_point_synchronization`: Create vertex, clone it, set_point on one, verify both reflect the change. Use `Vertex::new(42)` and `set_point(99)`.
- `test_vertex_count`: Create vertex, verify count()==1, clone it, verify count()==2, drop clone, verify count()==1.
- `test_vertex_mapped`: Create `Vertex::new(5)`, map with `|x| *x * 2`, verify mapped vertex has point 10.
- `test_vertex_display_formats`: Create `Vertex::new([1,2])`, test all `VertexDisplayFormat` variants (Full, IDTuple, PointTuple, AsPoint) produce non-empty debug output.

**Edge tests:**
- `test_edge_try_new_same_vertex`: Verify `Edge::try_new(&v, &v, ())` returns `Err(Error::SameVertex)`.
- `test_edge_new_different_vertices`: Create edge with two different vertices, verify front/back match.
- `test_edge_inverse_and_orientation`: Create edge, verify `orientation() == true`, create inverse, verify `orientation() == false`, verify `front`/`back` swapped, verify `id()` same, verify `is_same()` returns true.
- `test_edge_absolute_endpoints`: Create edge, invert it, verify `absolute_front()` and `absolute_back()` unchanged while `front()` and `back()` are swapped.
- `test_edge_absolute_clone`: Create edge, inverse it, absolute_clone it, verify the clone has `orientation() == true` and `is_same()` with original.
- `test_edge_curve_get_set`: Create `Edge::new(&v0, &v1, 42)`, verify `curve() == 42`, call `set_curve(99)`, clone the edge and verify both see 99.
- `test_edge_count`: Create edge, verify count()==1, clone, verify count()==2, drop, verify count()==1.
- `test_edge_equality_vs_is_same`: Two edges with same vertices but separate `Edge::new` calls have different ids. Cloned edges are equal. Inversed edges are not equal but `is_same()`.
- `test_edge_debug_new`: In debug mode, `Edge::debug_new` panics on same vertex (wraps in `std::panic::catch_unwind`).
- `test_edge_display_formats`: Create edge, test `EdgeDisplayFormat::AsCurve`, `VerticesTuple`, `VerticesTupleStruct`, `Full`, `VerticesTupleAndID`, `VerticesTupleAndCurve` all produce non-empty debug strings.

**Edge with geometry (using euler_operators::Segment pattern):**
For cut/concat tests, define a minimal `Segment` struct as in `tests/euler_operators.rs` or use simple tuple types `(i32, i32)` implementing required traits. Since the Segment is already defined in euler_operators.rs, create a local helper instead.

- `test_edge_oriented_curve`: Create `Edge::new(&v0, &v1, (0, 1))` where `(i32, i32)` implements `Invertible` (invert swaps tuple). Create inverse edge, verify `oriented_curve()` is `(1, 0)` while `curve()` is still `(0, 1)`. Note: This requires implementing `Invertible` for the test type. If trait bounds prevent this, test with `()` types and just exercise the method existence.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-topology --test edge_wire_vertex_ops` and verify all tests pass.</verify>
  <done>Vertex and edge operation tests created and passing.</done>
</task>

<task type="auto">
  <name>Task 2: Create wire operation tests</name>
  <files>monstertruck-topology/tests/edge_wire_vertex_ops.rs</files>
  <action>
Add wire operation tests to the same file:

**Wire construction and property tests:**
- `test_wire_new_empty`: Create `Wire::<(), ()>::new()`, verify is_empty, is_continuous, is_cyclic, is_simple all return true for empty wire.
- `test_wire_from_vec`: Create wire from `Vec<Edge>`, verify len, front_vertex, back_vertex, ends_vertices.
- `test_wire_with_capacity`: Create `Wire::with_capacity(10)`, verify it is empty.
- `test_wire_push_front_back`: Build wire incrementally with push_front and push_back, verify vertex order.
- `test_wire_is_continuous`: Create continuous wire (v0->v1->v2), verify `is_continuous() == true`. Create discontinuous wire (v0->v1, v3->v4), verify `is_continuous() == false`.
- `test_wire_is_cyclic`: Create non-cyclic wire (v0->v1->v2), verify `is_cyclic() == false`. Add closing edge (v2->v0), verify `is_cyclic() == true`.
- `test_wire_is_closed`: Verify closed = continuous AND cyclic. Create continuous non-cyclic wire, verify not closed. Create cyclic but discontinuous wire, verify not closed. Create proper closed wire.
- `test_wire_is_simple`: Create simple closed wire (triangle). Create non-simple wire where a vertex is shared by more than 2 edges (figure-8 shape). Verify simple/non-simple correctly identified.
- `test_wire_disjoint_wires`: Create two wires with disjoint vertices, verify `Wire::disjoint_wires` returns true. Create two wires sharing a vertex, verify returns false.

**Wire manipulation tests:**
- `test_wire_invert`: Create wire v0->v1->v2->v3, invert it, verify vertex order is now v3->v2->v1->v0.
- `test_wire_inverse`: Same as invert but using `inverse()` (returns new wire, doesn't mutate).
- `test_wire_split_off`: Create wire with 6 edges, split_off at index 4, verify lengths (4 and 2), append back together, verify matches original.
- `test_wire_swap_edge_into_wire`: Create wire with 3 edges, swap middle edge with a 2-edge wire having same endpoints, verify new wire has 4 edges.
- `test_wire_swap_edge_into_wire_failure`: Attempt swap where endpoints don't match, verify returns false and wire unchanged.
- `test_wire_vertex_iter`: Create closed triangular wire, iterate vertices, verify 3 vertices returned (cyclic wire doesn't repeat start).
- `test_wire_vertex_iter_open`: Create open wire v0->v1->v2, iterate vertices, verify 3 vertices returned (open wire includes both endpoints).
- `test_wire_edge_iter`: Create wire with 3 edges, verify edge_iter yields 3 edges matching originals.
- `test_wire_display_formats`: Create wire, test `WireDisplayFormat::EdgesListTuple`, `EdgesList`, `VerticesList` all produce non-empty debug output.
- `test_wire_clone_eq`: Create wire, clone it, verify equality. Modify clone by pushing an edge, verify no longer equal.
- `test_wire_macro`: Use the `wire![]` macro to create a wire, verify it works identically to `Wire::from_iter`.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-topology --test edge_wire_vertex_ops` and verify all tests pass.</verify>
  <done>Wire operation tests created and passing, covering continuity, closure, simplicity, inversion, split, swap, and iteration.</done>
</task>

<task type="auto">
  <name>Task 3: Run full test suite and verify coverage direction</name>
  <files>monstertruck-topology/tests/edge_wire_vertex_ops.rs</files>
  <action>
Run the complete monstertruck-topology test suite to verify no regressions:
```
cargo nextest run -p monstertruck-topology --no-fail-fast
```

Also run clippy to ensure no warnings:
```
cargo clippy -p monstertruck-topology --all-targets -- -W warnings
```

Fix any compilation errors or test failures. Ensure all existing tests plus new tests pass cleanly.
  </action>
  <verify>All tests pass with `cargo nextest run -p monstertruck-topology`. No clippy warnings.</verify>
  <done>Full test suite passes with no regressions and no clippy warnings.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-topology --test edge_wire_vertex_ops` passes all tests
2. `cargo nextest run -p monstertruck-topology --no-fail-fast` passes all tests including existing ones
3. `cargo clippy -p monstertruck-topology --all-targets -- -W warnings` produces no warnings
4. Test file contains at least 250 lines covering vertex, edge, and wire operations
</verification>

<success_criteria>
- Edge creation (new, try_new, debug_new, new_unchecked) has dedicated test cases
- Edge splitting operations (inverse, invert, absolute_clone) have dedicated test cases
- Wire construction from edges has dedicated test cases
- Wire property checks (continuous, cyclic, closed, simple) have dedicated test cases
- Wire manipulation (invert, split_off, swap_edge_into_wire, append) has dedicated test cases
- Vertex creation, identity, and synchronization have dedicated test cases
</success_criteria>

<output>
After completion, create `.tendrion/phases/27-topology-coverage/27-1-SUMMARY.md`
</output>
