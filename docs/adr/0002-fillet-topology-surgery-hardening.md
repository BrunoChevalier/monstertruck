# ADR-0002: Harden fillet topology surgery for non-trivial edge types

- **Status:** Accepted
- **Date:** 2026-03-16

## Context

Fillet operations in `monstertruck-solid` encountered two classes of topology failures when operating on solids produced by boolean operations:

1. **TOPO-02 -- Incorrect seam averaging:** `fillet_along_wire` averaged `Vector4` (homogeneous) control points directly when stitching adjacent fillet surface patches at shared seam edges. Because the weight component participated in the arithmetic, the resulting seam positions were shifted away from the true geometric midpoint, breaking C0 continuity.

2. **TOPO-01 -- Uncuttable intersection-curve edges:** Boolean operations produce boundary edges backed by `IntersectionCurve` geometry. The `cut_face_by_bezier` routine assumed all edges were NURBS-representable. When it encountered an `IntersectionCurve` edge, it could not split the face, causing the fillet to fail.

## Decision

1. **Dehomogenized seam averaging:** Extract a `dehomogenized_average` helper that divides each `Vector4` control point by its weight, averages in Cartesian space, and re-homogenizes. `fillet_along_wire` calls this helper instead of raw vector averaging at seam boundaries.

2. **Pre-cut edge conversion:** Introduce `ensure_cuttable_edge()` in `monstertruck-solid::fillet::convert`. Before `cut_face_by_bezier` executes, each boundary edge is inspected; `IntersectionCurve` edges are approximated as NURBS curves (via the existing `NurbsCurve::try_from` path) and swapped in-place so the cutter always operates on representable geometry.

## Alternatives Considered

- **Upgrade `cut_face_by_bezier` to handle `IntersectionCurve` natively:** Rejected because the intersection-curve representation lacks the knot structure needed for splitting; converting to NURBS is the standard B-rep kernel approach.
- **Skip seam averaging entirely and accept G0 gaps:** Rejected because downstream meshing and rendering depend on at least C0 continuity at patch boundaries.

## Consequences

- Fillets now succeed on solids produced by boolean operations, expanding the set of modelable shapes.
- The NURBS approximation in `ensure_cuttable_edge` introduces a bounded approximation error controlled by the existing operation tolerance.
- Additional boolean-fillet integration tests are currently ignored pending fixes to upstream boolean operations; they should be un-ignored once those fixes land.

## Status Update (Phase 7)

The topology surgery hardening from this ADR forms the foundation for Phase 7's integration mode work. The `IntegrateVisual` fillet mode (see [ADR-0003](0003-fillet-integration-mode.md)) builds on the dehomogenized seam averaging and pre-cut edge conversion established here, extending them with G1/G2 continuity classification and seamless vertex enforcement.
