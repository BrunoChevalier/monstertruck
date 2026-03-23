---
phase: 31-deferred-ayam-port-completion
plan: 2
tags: [tessellation, trim, degenerate, robustness]
key-files:
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/tests/tessellation/degenerate_trim_test.rs
  - monstertruck-meshing/tests/tessellation/main.rs
  - monstertruck-meshing/src/tessellation/mod.rs
decisions:
  - "Degenerate wires (collapsed to < 3 points) return None from try_new_inner; callers with allow_fallback=true skip them via filter_map instead of aborting the entire face"
  - "is_degenerate_loop uses UV_CLOSURE_TOLERANCE^2 as the area threshold for the shoelace formula"
  - "catch_unwind wraps the entire trimming_tessellation body to guard against CDT panics on pathological inputs"
metrics:
  tests_added: 7
  tests_passed: 59
  tests_failed: 0
  lines_added_impl: ~80
  lines_added_test: 334
---

## What was built

### Implementation (monstertruck-meshing/src/tessellation/triangulation.rs)
- `remove_collapsed_edges()`: Filters consecutive duplicate 3D points (zero-length edges) from boundary loops before UV parameter search.
- `is_degenerate_loop()`: Detects near-zero-area UV loops using the shoelace formula; loops below `UV_CLOSURE_TOLERANCE^2` are skipped.
- Degenerate loop filtering in `PolyBoundary::new`: Removes degenerate loops from `closed` before CDT insertion, with a log warning.
- Empty piece handling: `PolyBoundary::new` skips empty `PolyBoundaryPiece` entries (from collapsed wires).
- Self-touching boundary guard in `insert_to`: Explicitly skips zero-length CDT constraints when two logical boundary points map to the same vertex handle.
- Empty boundary guard in `cdt_trimming_tessellation`: Falls back to untrimmed tessellation when all boundary loops were filtered out.
- `catch_unwind` fallback in `trimming_tessellation`: Wraps CDT tessellation; on panic, returns a simple 2-triangle quad covering the UV bounding box.
- `fallback_uv_quad()`: Produces the fallback mesh from the UV AABB.
- `shell_create_polygon` and `cshell_tessellation`: When `allow_fallback=true`, individual degenerate wires are skipped via `filter_map` instead of aborting the entire face.

### Tests (monstertruck-meshing/tests/tessellation/degenerate_trim_test.rs)
- `near_zero_area_trim_loop_no_panic`: Thin-sliver face tessellates without panic.
- `degenerate_hole_preserves_face_mesh`: Face with near-zero-area inner hole still produces valid mesh.
- `multiple_degenerate_holes_preserves_face_mesh`: Multiple degenerate inner holes are all filtered.
- `self_touching_trim_boundary_no_panic`: Bowtie boundary (figure-8) tessellates without panic.
- `collapsed_edge_in_trim_wire_no_panic`: Wire with zero-length edge tessellates without panic.
- `watertight_boundary_after_degenerate_trim`: Adjacent faces with shared edge produce watertight output.
- `robust_vs_regular_degenerate_trim`: Robust triangulation recovers at least as many faces as regular.

## Deviations

- RED tests passed against existing code because the CDT's `can_add_constraint` and `spade_round` mechanisms already handle many degenerate cases gracefully. The implementation adds explicit, named guards for robustness and maintainability rather than relying on implicit CDT behavior.

## Verification

1. `cargo nextest run -p monstertruck-meshing --test tessellation` -- all 21 tests pass
2. `cargo nextest run -p monstertruck-meshing` -- all 59 tests pass, no regressions
3. `cargo clippy -p monstertruck-meshing --all-targets -- -W warnings` -- clean
4. `solid_is_closed` test passes (baseline watertightness preserved)
