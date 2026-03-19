---
phase: 13-api-polish-and-surface-operations
plan: 2
tags: [bspline-surface, nurbs-surface, split, sub-patch, tdd]
key-files:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/nurbs/nurbs_surface.rs
  - monstertruck-geometry/tests/bspsurface.rs
decisions: []
metrics:
  tests_added: 5
  tests_passing: 26
  deviations: 0
  tdd_cycles: 1
---

## What was built

- **`BsplineSurface::split_at_u`** -- non-mutating split at a u-parameter, returning `(left, right)` tuple.
- **`BsplineSurface::split_at_v`** -- non-mutating split at a v-parameter, returning `(bottom, top)` tuple.
- **`BsplineSurface::sub_patch`** -- extracts a rectangular sub-region `[u0, u1] x [v0, v1]` via successive splits.
- **`NurbsSurface::split_at_u`**, **`split_at_v`**, **`sub_patch`** -- delegation to inner `BsplineSurface`.

## Files modified

| File | Change |
|------|--------|
| `monstertruck-geometry/src/nurbs/bspline_surface.rs` | Added `split_at_u`, `split_at_v`, `sub_patch` methods with doc-comments and examples. |
| `monstertruck-geometry/src/nurbs/nurbs_surface.rs` | Added delegating `split_at_u`, `split_at_v`, `sub_patch` methods. |
| `monstertruck-geometry/tests/bspsurface.rs` | Added 5 integration tests: evaluation preservation for u-split, v-split, sub_patch, boundary splits, and full-domain sub_patch. |

## Task commits

| SHA | Message |
|-----|---------|
| `ee10fb2f` | `test(bspline-surface): add failing tests for split_at_u, split_at_v, sub_patch` |
| `30fde975` | `feat(bspline-surface): implement split_at_u, split_at_v, sub_patch on BsplineSurface` |
| `a91f1382` | `feat(nurbs-surface): delegate split_at_u, split_at_v, sub_patch on NurbsSurface` |

## Decisions made

None. Implementation followed the plan exactly.

## Deviations from plan

None.

## Self-check

- All 26 bspsurface integration tests pass.
- `cargo clippy -p monstertruck-geometry -- -W warnings` produces zero warnings.
- Both `BsplineSurface` and `NurbsSurface` expose the new API surface.
