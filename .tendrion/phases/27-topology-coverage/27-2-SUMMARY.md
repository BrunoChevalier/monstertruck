---
phase: 27-topology-coverage
plan: 2
tags: [topology, testing, face, shell, solid, coverage]
key-files:
  - monstertruck-topology/tests/face_shell_ops.rs
decisions: []
metrics:
  tests_added: 68
  test_file_lines: 1264
  total_package_tests: 119
  clippy_warnings_from_new_code: 0
---

## What was built

- **monstertruck-topology/tests/face_shell_ops.rs** (1264 lines, 68 tests): Comprehensive integration tests covering face, shell, and solid topological operations.

### Face tests (36 tests)
- Creation validation: `try_new` with empty wire, open wire, non-simple wire, non-disjoint wires, and valid cases.
- Identity and properties: id uniqueness, `is_same`, `count` (Arc refcount), surface get/set, orientation.
- Boundary traversal: `boundaries`, `absolute_boundaries`, `absolute_clone`, `into_boundaries`, `boundary_iters` (including inverted face), double-ended `BoundaryIter`, `edge_iter`, `vertex_iter`.
- Boundary modification: `add_boundary`, `try_add_boundary` error cases (EmptyWire, NotClosedWire, NotSimpleWire, NotDisjointWires).
- Cutting: `cut_by_edge` (success + failure cases), `cut_by_wire`.
- Adjacency: `border_on`, `border_wires`.
- Gluing: `glue_at_boundaries`.
- Display: all 6 `FaceDisplayFormat` variants.

### Shell tests (24 tests)
- Construction: `new`, `with_capacity`, `from` Vec, `collect` from iterator, `push`/Deref, `append`, `shell!` macro.
- Shell condition: all 4 variants (Irregular, Regular, Oriented, Closed) plus `BitAnd` combinations.
- Connectivity: single face, shared edge (connected), disconnected, `connected_components` (2 components + empty).
- Adjacency: `vertex_adjacency`, `face_adjacency`.
- Boundaries: `extract_boundaries`.
- Singular vertices: manifold (none) and wedge-of-spheres (present).
- Iteration: `face_iter`, `edge_iter`, `vertex_iter`.

### Solid tests (12 tests)
- Construction validation: valid tetrahedron, all 4 error cases (EmptyShell, NotConnected, NotClosedShell, NotManifold).
- Accessors: `boundaries`, `into_boundaries`, `face_iter`, `edge_iter`, `vertex_iter`.
- Mutation: `not` (orientation flip).
- Display: all 3 `SolidDisplayFormat` variants.

### Compress roundtrip (1 test)
- `compress` -> `extract` roundtrip preserves face count and boundary structure.

## Deviations

- Tests exercise existing public API, so all tests pass immediately (feature-already-exists case). TDD RED phase produces immediate GREEN. Logged as auto-fix deviation.

## Verification

- `cargo nextest run -p monstertruck-topology --test face_shell_ops`: 68/68 passed.
- `cargo nextest run -p monstertruck-topology --no-fail-fast`: 119 passed, 1 skipped, 0 failed.
- `cargo clippy -p monstertruck-topology --all-targets -- -W warnings`: no warnings from new test code.
