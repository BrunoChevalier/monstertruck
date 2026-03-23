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

## Status Update (Phase 9)

No changes to fillet integration mode logic. Phase 9 established a tolerance policy in `monstertruck-core` and hardened the boolean pipeline (face classification, shell healing, shell welding, coincident face detection). Meshing tolerance constants were also aligned with `monstertruck-core`. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 10)

No changes to fillet integration mode logic. Phase 10 introduced surface healing hooks and a NURBS fixture corpus for testing degenerate surfaces through sweep/birail/gordon constructors. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 11)

No changes to fillet integration mode logic. Phase 11 added multi-rail sweep and periodic sweep constructors in `monstertruck-geometry` and typed builder wrappers in `monstertruck-modeling`. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 12 -- v0.4.0 Final)

No changes to fillet integration mode logic. Phase 12 added end-to-end font pipeline integration tests and finalized the Ayam port plan. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted with no anticipated changes for v0.4.0.

## Status Update (Phase 13 -- v0.5.0 API Polish)

No changes to fillet integration mode logic. Phase 13 added typed option structs, fallible `try_*` surface constructors, and patch split/extract methods in `monstertruck-geometry`, plus option-struct builder functions in `monstertruck-modeling`. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 14 -- Profile Solid Pipeline)

No changes to fillet integration mode logic. Phase 14 added profile revolve/sweep functions, mixed profile face construction, and solid validation in `monstertruck-modeling`. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 15 -- Font Stress Testing & Performance)

No changes to fillet integration mode logic. Phase 15 added a font stress corpus with pathological geometry fixtures and Criterion performance benchmarks for the profile pipeline. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 16 -- Tolerance Centralization & API Hardening)

No changes to fillet integration mode logic. Phase 16 centralized tolerance constants in `monstertruck-core::tolerance_constants` and refactored import paths in `monstertruck-solid` modules. The `G1_ANGLE_TOLERANCE` and `G2_CURVATURE_TOLERANCE` constants used by continuity classification are now sourced from the central module. The fillet integration mode and continuity annotations from this ADR are functionally unaffected.

## Status Update (Phase 17 -- Curve-Curve Intersection)

No changes to fillet integration mode logic. Phase 17 added a curve-curve intersection module in `monstertruck-geometry`. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 18 -- Gordon Surface from Network)

No changes to fillet integration mode logic. Phase 18 added `try_gordon_from_network` and `try_gordon_verified` Gordon surface constructors and builder wrappers. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 19 -- Tessellation Robustness)

No changes to fillet integration mode logic. Phase 19 centralized tessellation tolerance constants and added fallback UV interpolation in `monstertruck-meshing`. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 20 -- Fixture Corpus & Migration Documentation, v0.5.1 Final)

No changes to fillet integration mode logic. Phase 20 expanded the pathological geometry fixture corpus, added migration doc comments on `try_*` surface constructor functions, and created a crate-level migration guide. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted. Milestone v0.5.1 is complete.

## Status Update (Phase 21 -- Fillet Edge Identity Fix)

No changes to fillet integration mode logic. Phase 21 fixed edge identity preservation in `ensure_cuttable_edge` (see [ADR-0002](0002-fillet-topology-surgery-hardening.md) Phase 21 update) and widened endpoint matching tolerance in `fillet::convert`. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 22 -- Conversion Fidelity Improvements)

No changes to fillet integration mode logic. Phase 22 upgraded curve/surface sampling to cubic interpolation, added exact `RevolutedCurve` conversion, and introduced endpoint snapping in shell conversion. The fillet integration mode and continuity annotations from this ADR are unaffected.

## Status Update (Phase 23 -- Error Propagation & Test Hardening, v0.5.2 Final)

No changes to fillet integration mode logic. Phase 23 replaced silent rollback in `fillet_edges_generic` with explicit `Err(FilletError::ShellNotClosed)` error propagation and hardened fillet test expectations. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted. Milestone v0.5.2 is complete.

## Status Update (Phase 24 -- GPU Test Reliability, v0.5.3)

No changes to fillet integration mode logic. Phase 24 fixed transposed projection matrices in `monstertruck-math` and added graceful GPU test degradation in `monstertruck-gpu`. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted.

## Status Update (Phase 25 -- Clippy & Dependency Hygiene)

No changes to fillet integration mode logic. Phase 25 upgraded `vtkio` from 0.6 to 0.7.0-rc2 and resolved 4 clippy warnings across the workspace. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted.

## Status Update (Phase 26 -- Core and Traits Coverage)

No changes to fillet integration mode logic. Phase 26 added 96 unit tests to `monstertruck-core` and 80 unit tests to `monstertruck-traits`, improving coverage of tolerance traits, bounding box, collections, derivatives, and geometric trait contracts. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted.

## Status Update (Phase 27 -- Topology Coverage)

No changes to fillet integration mode logic. Phase 27 added 107 unit tests to `monstertruck-topology` covering vertex/edge/wire and face/shell/solid operations. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted.

## Status Update (Phase 28 -- Modeling Coverage)

No changes to fillet integration mode logic. Phase 28 added 40 unit tests to `monstertruck-modeling` covering builder round-trip operations (extrude, revolve, sweep_rail, homotopy, transformations, primitives) and text/geometry enum APIs. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted.

## Status Update (Phase 29 -- Solid and STEP Coverage)

No changes to fillet integration mode logic. Phase 29 added 22 tests to `monstertruck-solid` (boolean ops, fillet pipeline, healing module) and 9 STEP round-trip tests to `monstertruck-step`. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted.

## Status Update (Phase 30 -- New Surface Constructors)

No changes to fillet integration mode logic. Phase 30 added ruled surface (`BsplineSurface::try_ruled`) and loft surface (`builder::try_loft`) constructors, and expanded healing with `check_edge_curve_consistency` and `detect_and_repair_gaps`. The fillet integration mode and continuity annotations from this ADR are unaffected. This ADR remains Accepted.
