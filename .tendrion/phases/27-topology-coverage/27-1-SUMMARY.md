---
phase: 27-topology-coverage
plan: 1
tags: [topology, tests, coverage, vertex, edge, wire]
key-files:
  - monstertruck-topology/tests/edge_wire_vertex_ops.rs
decisions: []
metrics:
  tests_added: 37
  lines_added: 631
  deviations: 2
---

## What was built

- **monstertruck-topology/tests/edge_wire_vertex_ops.rs** (631 lines, 37 tests): Comprehensive integration tests for vertex, edge, and wire operations in the `monstertruck-topology` crate.

### Vertex tests (5)
- Creation, identity, and batch construction (`Vertex::new`, `Vertex::news`)
- Point synchronization across clones (`set_point`)
- Reference counting (`count`)
- Point mapping (`mapped`)
- Display format variants (`VertexDisplayFormat`)

### Edge tests (12)
- `try_new` same-vertex rejection (`Error::SameVertex`)
- Construction with different vertices
- Inverse and orientation
- Absolute endpoints invariance under inversion
- `absolute_clone` behavior
- Curve get/set with clone synchronization
- Reference counting
- Equality vs `is_same` semantics
- `debug_new` panic on same vertex
- Display format variants (`EdgeDisplayFormat`)
- `oriented_curve` with `Invertible` type `(usize, usize)`
- `ends` and `absolute_ends`

### Wire tests (20)
- Empty wire properties (continuous, cyclic, simple)
- Construction from `Vec`, `with_capacity`, `push_front`/`push_back`
- `is_continuous`, `is_cyclic`, `is_closed`, `is_simple`
- `disjoint_wires`
- `invert`, `inverse`
- `split_off` and `append` roundtrip
- `swap_edge_into_wire` success and failure paths
- `vertex_iter` for closed and open wires
- `edge_iter`
- Display format variants (`WireDisplayFormat`)
- Clone and equality
- `wire!` macro equivalence

## Deviations

1. TDD RED/GREEN cycle not applicable: plan adds tests for existing public API. Tests pass immediately.
2. Pre-existing compile error in untracked `face_shell_ops.rs` from parallel agent -- does not affect this plan.

## Verification

- `cargo nextest run -p monstertruck-topology --test edge_wire_vertex_ops`: 37/37 passed
- `cargo clippy -p monstertruck-topology --test edge_wire_vertex_ops -- -W warnings`: 0 warnings
- File exceeds 250-line minimum (631 lines)
