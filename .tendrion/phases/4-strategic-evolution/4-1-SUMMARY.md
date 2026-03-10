---
phase: 4-strategic-evolution
plan: 1
tags: [t-spline, t-nurcc, t-mesh, validation, zero-knot-interval, connection-parity]
key-files:
  - monstertruck-geometry/src/t_spline/t_nurcc.rs
  - monstertruck-geometry/src/t_spline/t_mesh.rs
  - monstertruck-geometry/tests/t_spline_validation.rs
decisions:
  - "Connection parity (L/R) in t_nurcc.rs verified correct: LeftAcw/LeftCw for alpha_{ij}, RightAcw/RightCw for alpha_{ji}"
  - "TnurccMalformedFace error retained for isolated vertex case (no need for separate error variant)"
  - "Zero knot interval support: TmeshControlPointNotFound treated as non-fatal in find_inferred_connection"
metrics:
  tests_added: 8
  tests_total: 160
  lines_changed: ~60
  deviations: 0
---

## What was built

### Files modified

- **monstertruck-geometry/src/t_spline/t_nurcc.rs**: Resolved 3 TODOs:
  - Lines ~390, ~403: Replaced connection parity TODOs with documentation confirming L/R mapping is correct per Equation 14 of Sederberg et al. 1998. Added debug assertions for non-negative knot intervals. Extracted `alpha` closure to deduplicate the computation.
  - Line ~451: Replaced malformed face error TODO with documentation explaining when `TnurccMalformedFace` is returned (isolated vertex with no radial edges).

- **monstertruck-geometry/src/t_spline/t_mesh.rs**: Resolved 1 TODO:
  - Line ~345: Replaced zero knot interval TODO with documentation referencing Figure 9 of Sederberg et al. 2003. Changed `find_inferred_connection` error handling from `.map_err(|_| TmeshUnknownError)?` to `match` blocks that tolerate `TmeshControlPointNotFound` (edge condition reached during zero-interval traversal).

- **monstertruck-geometry/tests/t_spline_validation.rs**: New integration test file with 8 tests covering connection parity, zero knot intervals, malformed face errors, and T-mesh conversion.

## Task commits

| Step | SHA | Message |
|------|-----|---------|
| RED | `30300e7b` | test(t_spline): add failing tests for T-spline validation scenarios |
| GREEN | `59ad820b` | feat(t_spline): resolve T-spline validation TODOs for connection parity and zero knot intervals |
| REFACTOR | `d2d9893d` | refactor(t_spline): extract alpha closure to deduplicate parity computation |

## Decisions

1. **Connection parity is correct**: Analysis of `TnurccEdge` connectivity confirms `LeftAcw`/`LeftCw` map to the left face (origin side) and `RightAcw`/`RightCw` map to the right face (dest side), matching Equation 14's convention.

2. **Retained `TnurccMalformedFace` for isolated vertex**: A dedicated `TnurccIsolatedVertex` error would add complexity without benefit -- the condition only arises from malformed face definitions.

3. **Zero-knot-interval tolerance**: When `find_inferred_connection` traversal hits a mesh boundary (returns `TmeshControlPointNotFound`), this is non-fatal for zero-interval insertions where the new point shares parametric coordinates with an existing vertex.

## Self-check

- All 160 monstertruck-geometry tests pass (84 lib + 76 integration).
- No TODOs remain in t_nurcc.rs or t_mesh.rs.
- No clippy warnings.
- Artifact line counts: t_nurcc.rs=1798 (>400), t_mesh.rs=4558 (>300), t_spline_validation.rs=366 (>80).
- All artifact content markers present: "a_od", "knot_interval", "parity".
