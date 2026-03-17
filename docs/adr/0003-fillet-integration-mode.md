# ADR-0003: Fillet integration mode with continuity annotations

- **Status:** Accepted
- **Date:** 2026-03-16

## Context

After Phase 6 hardened fillet topology surgery (ADR-0002), fillet surfaces were always created as separate faces stitched at seam boundaries. For visualization and downstream meshing workflows, users often need fillet geometry that blends seamlessly into adjacent faces with explicit continuity guarantees rather than discrete face boundaries.

Additionally, there was no mechanism for downstream consumers to query the geometric continuity class (G0, G1, G2) at fillet-to-face join edges, making it impossible to automate quality checks or adaptive meshing decisions.

## Decision

1. **Introduce `FilletMode` enum** with two variants:
   - `KeepSeparateFace` (default) -- preserves existing behavior where fillet surfaces are distinct topological faces.
   - `IntegrateVisual` -- merges fillet surfaces into adjacent geometry, enforcing seamless vertex positions and annotating edge continuity.

2. **Extend `FilletOptions`** with three new fields (`mode`, `extend_mode`, `corner_mode`), all defaulting to backward-compatible values via `Default` so existing call sites are unaffected.

3. **Add `FilletResult` with continuity annotations** -- The new `fillet_annotated()` API returns a `FilletResult` carrying per-edge G1/G2 continuity classifications. Helper functions `annotate_fillet_edges()`, `classify_edge_continuity()`, and `ensure_seamless_vertices()` implement the annotation and enforcement logic.

## Alternatives Considered

- **Post-processing pass instead of integrated mode:** Rejected because splitting annotation from construction would require re-traversing topology and matching edges after the fact, which is fragile and slower than annotating during construction.
- **Single boolean flag instead of enum:** Rejected because future modes (e.g., `IntegrateAnalytic` for exact G2 surface merging) are anticipated; an enum is extensible without breaking changes.

## Consequences

- Existing code using default `FilletOptions` is unaffected (`KeepSeparateFace` is the default).
- The `IntegrateVisual` mode produces topology with fewer faces, which simplifies downstream meshing but means individual fillet surfaces can no longer be selected as separate entities.
- Continuity annotations add a small per-edge cost but enable automated quality validation without geometric recomputation.
- Five new integration tests validate both modes and continuity classification correctness.

## Status Update (Phase 8)

Phase 8 added debug-only topology validation (`fillet::validate` module) that runs after fillet operations in both `KeepSeparateFace` and `IntegrateVisual` modes. Euler-Poincare and orientation assertions now guard the post-fillet shell state, catching topology corruption early in debug builds. This complements the continuity annotations from this ADR by validating structural integrity in addition to geometric smoothness.
