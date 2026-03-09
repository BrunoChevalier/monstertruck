---
phase: 3-feature-completeness
plan: 3
tags: [shell, offset, solid, topology]
key-files:
  - monstertruck-solid/src/shell_ops/mod.rs
  - monstertruck-solid/src/shell_ops/shell_op.rs
  - monstertruck-solid/src/shell_ops/offset_op.rs
  - monstertruck-solid/src/shell_ops/tests.rs
  - monstertruck-solid/src/lib.rs
decisions:
  - "Used generic trait-based design (OffsetSurface, OffsetCurve) instead of concrete types due to circular dependency between monstertruck-solid and monstertruck-modeling"
  - "Trait implementations for modeling types placed in test module (dev-dependency only)"
  - "Vertex positions computed via plane intersection (3-plane system) for exact results on planar shells"
  - "Self-intersection detection via edge direction reversal check rather than geometric intersection tests"
metrics:
  tests_added: 7
  tests_passing: 7
  files_created: 4
  files_modified: 1
  total_lines: 505
---

## What was built

Shell (hollow-out) and surface offset operations for solid bodies in `monstertruck-solid`.

### Files created

- `monstertruck-solid/src/shell_ops/mod.rs` (33 lines) -- Module root with public API re-exports.
- `monstertruck-solid/src/shell_ops/offset_op.rs` (199 lines) -- `offset_shell` function and `OffsetSurface`/`OffsetCurve` traits. Compresses the shell, offsets surfaces accounting for face orientation, computes new vertex positions via 3-plane intersection, validates against self-intersection, and reconstructs via `Shell::extract`.
- `monstertruck-solid/src/shell_ops/shell_op.rs` (89 lines) -- `shell_solid` function. Creates a hollow solid by offsetting inward and inverting the inner shell's face normals.
- `monstertruck-solid/src/shell_ops/tests.rs` (218 lines) -- 7 tests covering topology validity, geometric correctness, serialization round-trip, excessive thickness rejection, face count preservation, and multiple thicknesses. Includes `OffsetSurface` and `OffsetCurve` trait implementations for the modeling crate's `Surface`/`Curve` types.

### Files modified

- `monstertruck-solid/src/lib.rs` -- Added `pub mod shell_ops` and re-exports.

## Tests

| Test | Description |
|------|-------------|
| `shell_cube_valid_topology` | Shells unit cube, verifies 2 closed shells with 6 faces each |
| `shell_cube_wall_thickness_geometric` | Verifies inner vertices are offset to [0.1, 0.9] range |
| `offset_shell_flat_surface` | Offsets cube shell inward, verifies face count and closure |
| `shell_serialization_round_trip` | Compress/JSON/extract round-trip of shelled solid |
| `shell_negative_thickness_error` | 0.3-cube with 0.2 thickness returns None |
| `offset_shell_preserves_face_count` | Multiple distances preserve face count |
| `shell_different_thicknesses` | Thicknesses 0.05--0.4 all produce valid hollow solids |

## Decisions

1. **Generic traits over concrete types**: Due to the circular dependency (monstertruck-modeling optionally depends on monstertruck-solid), the shell_ops module uses `OffsetSurface` and `OffsetCurve` traits instead of depending on the concrete `Surface`/`Curve` enums. Trait implementations are provided in the test module via the dev-dependency.

2. **Plane intersection for vertex computation**: For planar-face solids, vertex positions are computed as the exact intersection of three offset planes rather than averaging normals. This eliminates the diagonal-offset error that occurs with the average-normal approach at corners.

3. **Face orientation handling**: The offset distance is adjusted based on `CompressedFace::orientation`. When `orientation == false`, the geometric normal is opposite to the outward normal, so the effective offset distance is negated.

## Deviations

None. All tasks executed as planned.
