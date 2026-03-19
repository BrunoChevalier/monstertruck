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

## Status Update (Phase 8)

Phase 8 added formal topology invariant validation to the fillet pipeline via `monstertruck-solid::fillet::validate`. The new module provides:

- **`euler_poincare_check`** -- Verifies V - E + F = 2 for closed shells after fillet modifications.
- **`is_oriented_check`** -- Confirms face orientation consistency (Oriented or Closed condition).
- **`debug_assert_topology` / `debug_assert_euler`** -- Debug-only assertions that fire on Euler-Poincare or orientation violations, providing early detection of the topology corruption classes described in this ADR.

These checks are integrated into fillet operations (`ops.rs`, `edge_select.rs`) and run automatically in debug builds with zero cost in release builds. Four new tests validate the assertions against closed boxes, tetrahedra, open shells, and deliberately corrupted orientations.

## Status Update (Phase 9)

Phase 9 replaced the hardcoded `1.0e-6` in `edge_select.rs` with the canonical `TOLERANCE` constant from `monstertruck-core`, aligning fillet edge selection with the project-wide tolerance policy established in this phase. Gap-fix work also hardened the boolean pipeline: `weld_compressed_shell` was corrected and coincident face detection no longer self-compares, improving stability of boolean-produced solids that feed into fillet operations. The topology surgery hardening from this ADR remains unchanged.

## Status Update (Phase 10)

No changes to fillet topology surgery. Phase 10 added a NURBS fixture corpus with degenerate surface cases and surface healing hooks (`heal_surface_shell`) for sweep/birail/gordon constructors in `monstertruck-solid::healing`. The healing pipeline complements this ADR's pre-cut edge conversion by ensuring surfaces are well-formed before they enter topology operations. Integration tests validate that healed surfaces produce valid closed shells.

## Status Update (Phase 11)

No changes to fillet topology surgery. Phase 11 added `sweep_multi_rail` and `sweep_periodic` surface constructors in `monstertruck-geometry` and typed builder wrappers in `monstertruck-modeling`. The new builders use Euler-Poincare topology checks consistent with the validation framework established in Phase 8, but the fillet surgery code itself was unaffected.

## Status Update (Phase 12 -- v0.4.0 Final)

No changes to fillet topology surgery. Phase 12 added end-to-end font pipeline integration tests and finalized the Ayam port plan. The fillet surgery code was unaffected. This ADR remains Accepted with no anticipated changes for v0.4.0.

## Status Update (Phase 13 -- v0.5.0 API Polish)

No changes to fillet topology surgery. Phase 13 added typed option structs, fallible `try_*` surface constructors, and patch split/extract methods in `monstertruck-geometry`, plus option-struct builder functions in `monstertruck-modeling`. The fillet surgery code was unaffected.

## Status Update (Phase 14 -- Profile Solid Pipeline)

No changes to fillet topology surgery. Phase 14 added profile revolve/sweep, mixed profile faces, and solid validation (`validate_solid` with `ValidationReport`) in `monstertruck-modeling`. The Euler-Poincare checks in `validate_solid` complement the fillet validation from Phase 8 but operate at the solid level rather than the fillet-specific pipeline. The fillet surgery code was unaffected.
