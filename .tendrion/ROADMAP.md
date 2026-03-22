# Roadmap: monstertruck

## Milestones

- **v0.2.0** -- Phases 1-4
- **v0.3.0** -- Phases 5-8
- **v0.4.0** -- Phases 9-12
- **v0.5.0** -- Phases 13-15
- **v0.5.1** -- Phases 16-20
- **v0.5.2** -- Phases 21-23

## Phases

- [x] **Phase 1: Core Stabilization** - Fix critical panics, reduce unwrap density, replace deprecated deps, add benchmarking
- [x] **Phase 2: Numerical Robustness** - Adaptive tolerances, solver fallbacks, tessellation stitching, boolean hardening, fuzzing
- [x] **Phase 3: Feature Completeness** - STEP boolean export, chamfer, shell/offset, and draft/taper operations
- [x] **Phase 4: Strategic Evolution** - cgmath-to-nalgebra migration, RwLock concurrency, GPU tessellation, T-spline completion
- [x] **Phase 5: Solver Port** - Port polynomial solvers from matext4cgmath to monstertruck-math to fix build breakage
- [x] **Phase 6: Topology Surgery Hardening** - Harden cut_face_by_bezier for boolean-result faces and fix seam averaging bug
- [x] **Phase 7: Integration Mode** - Implement FilletMode with G1/G2 continuity annotations and fillet option extensions
- [x] **Phase 8: Validation and Documentation** - Add topology invariant checks and update fillet implementation plan
- [x] **Phase 9: Boolean Repair and Tolerance Foundation** - Fix boolean op bugs from v0.3.0 verification and establish shared numeric tolerance policy
- [x] **Phase 10: Test Infrastructure and Healing Hooks** - Build fixture corpus and add topological healing hooks for new surface constructors
- [x] **Phase 11: Surface Constructors** - Implement multi-rail and periodic sweep variants with builder-level wrappers in truck-modeling
- [x] **Phase 12: Font Pipeline and Final Documentation** - End-to-end text profile tests with real fonts and updated Ayam port plan
- [x] **Phase 13: API Polish and Surface Operations** - Typed option structs for surface constructors, curve network diagnostics, and patch split/extract
- [x] **Phase 14: Profile Solid Pipeline** - Revolve/sweep solid creation, mixed glyph-profile combinations, and profile validation
- [x] **Phase 15: Font Stress Testing and Performance** - Pathological font corpus and large-text pipeline benchmarks
- [x] **Phase 16: Tolerance Foundation and API Safety** - Centralize tolerance constants, add non_exhaustive to option structs, refactor deprecated delegations
- [x] **Phase 17: Curve Intersection Engine** - Implement shared curve-curve intersection module for Gordon grid computation and trim intersection
- [x] **Phase 18: Gordon Surface Variants** - Auto-intersect and verified-grid Gordon constructors using the curve intersection engine
- [x] **Phase 19: Trim Tessellation Robustness** - Fallback boundary projection and tolerance-derived tessellation constants
- [x] **Phase 20: Fixture Corpus and Migration Documentation** - Expand test fixtures across all surface types and add migration guidance docs
- [x] **Phase 21: Edge Identity and Topology Repair** - Fix edge identity preservation in ensure_cuttable_edge and widen conversion tolerance for boolean-origin edges
- [x] **Phase 22: Conversion Fidelity** - Degree-3 cubic interpolation, endpoint snapping, and exact RevolutedCurve conversion to eliminate geometric loss
- [ ] **Phase 23: Error Propagation and Test Hardening** - Replace silent fillet rollback with explicit errors and fix proptest tolerance

## Phase Details

### Phase 1: Core Stabilization
**Goal**: Critical runtime panics are eliminated and the codebase has benchmarking infrastructure
**Depends on**: None
**Requirements**: CORE-01, CORE-02, CORE-03, CORE-04
**Plans**: Archived — see `.tendrion/milestones/v0.2.0-ROADMAP.md`

### Phase 2: Numerical Robustness
**Goal**: Numerical operations are resilient to edge cases with adaptive tolerances and solver fallbacks
**Depends on**: Phase 1
**Requirements**: ROBUST-01, ROBUST-02, ROBUST-03, ROBUST-04, ROBUST-05
**Plans**: Archived — see `.tendrion/milestones/v0.2.0-ROADMAP.md`

### Phase 3: Feature Completeness
**Goal**: Core CAD operations (boolean export, chamfer, shell, draft, T-splines) are functional
**Depends on**: Phase 2
**Requirements**: FEAT-01, FEAT-02, FEAT-03, FEAT-04, FEAT-05
**Plans**: Archived — see `.tendrion/milestones/v0.2.0-ROADMAP.md`

### Phase 4: Strategic Evolution
**Goal**: The math foundation is modernized and concurrency/GPU infrastructure is in place
**Depends on**: Phase 3
**Requirements**: EVOLVE-01, EVOLVE-02, EVOLVE-03
**Plans**: Archived — see `.tendrion/milestones/v0.2.0-ROADMAP.md`

### Phase 5: Solver Port
**Goal**: The crate compiles cleanly with polynomial solvers hosted in monstertruck-math, unblocking all downstream fillet work
**Depends on**: None
**Requirements**: BUILD-01
**Plans**: Archived — see `.tendrion/milestones/v0.3.0-ROADMAP.md`

### Phase 6: Topology Surgery Hardening
**Goal**: Fillet operations on boolean-result faces complete without panics or geometry corruption from IntersectionCurve edges or seam averaging
**Depends on**: Phase 5
**Requirements**: TOPO-01, TOPO-02
**Plans**: Archived — see `.tendrion/milestones/v0.3.0-ROADMAP.md`

### Phase 7: Integration Mode
**Goal**: Users can select FilletMode::IntegrateVisual to produce fillet faces with G1/G2 continuity annotations and seamless tessellation
**Depends on**: Phase 6
**Requirements**: INTEG-01, INTEG-02
**Plans**: Archived — see `.tendrion/milestones/v0.3.0-ROADMAP.md`

### Phase 8: Validation and Documentation
**Goal**: Topology modifications are guarded by invariant assertions and the fillet implementation plan reflects final v0.3.0 status
**Depends on**: Phase 7
**Requirements**: TOPO-03, DOC-01
**Plans**: Archived — see `.tendrion/milestones/v0.3.0-ROADMAP.md`

### Phase 9: Boolean Repair and Tolerance Foundation
**Goal**: Boolean operations on complex faces produce correct topology and all crates share a consistent numeric tolerance policy
**Depends on**: None
**Requirements**: BOOL-01, TEST-02
**Plans**: Archived — see `.tendrion/milestones/v0.4.0-ROADMAP.md`

### Phase 10: Test Infrastructure and Healing Hooks
**Goal**: A fixture corpus of problematic geometries exists for regression testing and truck-shapeops can heal topology after surface construction
**Depends on**: Phase 9
**Requirements**: BOOL-02, TEST-01
**Plans**: Archived — see `.tendrion/milestones/v0.4.0-ROADMAP.md`

### Phase 11: Surface Constructors
**Goal**: Users can create multi-rail sweeps, periodic sweeps, and birail/gordon surfaces through typed builder APIs in truck-modeling
**Depends on**: Phase 10
**Requirements**: SURF-01, SURF-02
**Plans**: Archived — see `.tendrion/milestones/v0.4.0-ROADMAP.md`

### Phase 12: Font Pipeline and Final Documentation
**Goal**: Text profile creation from real fonts works end-to-end with hole preservation and the Ayam port plan reflects current status
**Depends on**: Phase 11
**Requirements**: FONT-01, DOC-02
**Plans**: Archived — see `.tendrion/milestones/v0.4.0-ROADMAP.md`

### Phase 13: API Polish and Surface Operations
**Plans**: Archived — see `.tendrion/milestones/v0.5.0-ROADMAP.md`

### Phase 14: Profile Solid Pipeline
**Plans**: Archived — see `.tendrion/milestones/v0.5.0-ROADMAP.md`

### Phase 15: Font Stress Testing and Performance
**Plans**: Archived — see `.tendrion/milestones/v0.5.0-ROADMAP.md`

### Phase 16: Tolerance Foundation and API Safety
**Plans**: Archived — see `.tendrion/milestones/v0.5.1-ROADMAP.md`

### Phase 17: Curve Intersection Engine
**Plans**: Archived — see `.tendrion/milestones/v0.5.1-ROADMAP.md`

### Phase 18: Gordon Surface Variants
**Plans**: Archived — see `.tendrion/milestones/v0.5.1-ROADMAP.md`

### Phase 19: Trim Tessellation Robustness
**Plans**: Archived — see `.tendrion/milestones/v0.5.1-ROADMAP.md`

### Phase 20: Fixture Corpus and Migration Documentation
**Plans**: Archived — see `.tendrion/milestones/v0.5.1-ROADMAP.md`

### Phase 21: Edge Identity and Topology Repair
**Goal**: Edge identity is preserved through the fillet conversion pipeline, and boolean-origin edges match endpoints within appropriate tolerances
**Depends on**: Phase 20
**Requirements**: ETOPO-01, ETOPO-02
**Success Criteria** (what must be TRUE):
  1. `ensure_cuttable_edge` mutates the existing Edge's curve in-place rather than creating a new Edge, so `edge.is_same()` returns true before and after conversion
  2. Edges originating from boolean operations with IntersectionCurve geometry pass endpoint matching in `convert_shell_in` without false-negative rejections
  3. `cut_face_by_bezier` boundary replacement correctly locates and replaces edges that were converted by `ensure_cuttable_edge`, with no stale edges remaining in the boundary
**Plans**: TBD

### Phase 22: Conversion Fidelity
**Goal**: The fillet conversion pipeline preserves geometric fidelity through degree-3 cubic interpolation, exact RevolutedCurve conversion, and endpoint snapping
**Depends on**: Phase 21
**Requirements**: FCONV-01, FCONV-02, FCONV-03
**Success Criteria** (what must be TRUE):
  1. `sample_curve_to_nurbs` and `sample_surface_to_nurbs` produce degree-3 NURBS with C1-continuous normals instead of degree-1 piecewise-linear approximations
  2. Converted curve endpoints exactly match their source vertex positions (within machine epsilon), preserving `ShellCondition::Closed` through the conversion round-trip
  3. `RevolutedCurve` surfaces convert to `NurbsSurface` via rational circle arc tensor product without falling back to the sampling path
  4. Fillet output paths (`From<ParameterCurveLinear>` and `From<FilletIntersectionCurve>`) produce degree-3 curves consistent with the input conversion quality
**Plans**: TBD

### Phase 23: Error Propagation and Test Hardening
**Goal**: Fillet failures surface explicit errors instead of silently restoring the original shell, and proptest tolerances match the mathematical properties being validated
**Depends on**: Phase 22
**Requirements**: EREP-01, EREP-02
**Success Criteria** (what must be TRUE):
  1. `fillet_edges_generic` returns `Err(FilletError)` with a descriptive variant when the shell closure check fails, instead of silently rolling back to the unfilleted shell
  2. Callers of `fillet_edges_generic` can pattern-match on the error variant to distinguish conversion failures from topology failures
  3. `test_unit_circle` proptest uses relative tolerance (comparing ratio to 1.0) rather than absolute `prop_assert_near!`, and passes consistently across the property test input space
**Plans**: TBD

## Progress

**Execution Order:** Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Phase 5 -> Phase 6 -> Phase 7 -> Phase 8 -> Phase 9 -> Phase 10 -> Phase 11 -> Phase 12 -> Phase 13 -> Phase 14 -> Phase 15 -> Phase 16 -> Phase 17 -> Phase 18 -> Phase 19 -> Phase 20 -> Phase 21 -> Phase 22 -> Phase 23

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Core Stabilization | v0.2.0 | 4/4 | ✓ Complete | 2026-03-09 |
| 2. Numerical Robustness | v0.2.0 | 4/4 | ✓ Complete | 2026-03-09 |
| 3. Feature Completeness | v0.2.0 | 5/5 | ✓ Complete | 2026-03-10 |
| 4. Strategic Evolution | v0.2.0 | 4/4 | ✓ Complete | 2026-03-15 |
| 5. Solver Port | v0.3.0 | 2/2 | ✓ Complete | 2026-03-16 |
| 6. Topology Surgery Hardening | v0.3.0 | 2/2 | ✓ Complete | 2026-03-16 |
| 7. Integration Mode | v0.3.0 | 2/2 | ✓ Complete | 2026-03-16 |
| 8. Validation and Documentation | v0.3.0 | 2/2 | ✓ Complete | 2026-03-17 |
| 9. Boolean Repair and Tolerance Foundation | v0.4.0 | 5/5 | ✓ Complete | 2026-03-19 |
| 10. Test Infrastructure and Healing Hooks | v0.4.0 | 3/3 | ✓ Complete | 2026-03-19 |
| 11. Surface Constructors | v0.4.0 | 2/2 | ✓ Complete | 2026-03-19 |
| 12. Font Pipeline and Final Documentation | v0.4.0 | 2/2 | ✓ Complete | 2026-03-19 |
| 13. API Polish and Surface Operations | v0.5.0 | 3/3 | ✓ Complete | 2026-03-19 |
| 14. Profile Solid Pipeline | v0.5.0 | 3/3 | ✓ Complete | 2026-03-19 |
| 15. Font Stress Testing and Performance | v0.5.0 | 2/2 | ✓ Complete | 2026-03-19 |
| 16. Tolerance Foundation and API Safety | v0.5.1 | 3/3 | ✓ Complete | 2026-03-19 |
| 17. Curve Intersection Engine | v0.5.1 | 1/1 | ✓ Complete | 2026-03-20 |
| 18. Gordon Surface Variants | v0.5.1 | 2/2 | ✓ Complete | 2026-03-20 |
| 19. Trim Tessellation Robustness | v0.5.1 | 2/2 | ✓ Complete | 2026-03-20 |
| 20. Fixture Corpus and Migration Documentation | v0.5.1 | 3/3 | ✓ Complete | 2026-03-20 |
| 21. Edge Identity and Topology Repair | v0.5.2 | 1/1 | ✓ Complete | 2026-03-22 |
| 22. Conversion Fidelity | v0.5.2 | 3/3 | ✓ Complete | 2026-03-22 |
| 23. Error Propagation and Test Hardening | v0.5.2 | 0/TBD | Not started | - |

---

*Roadmap created: 2026-03-08*
*Updated: 2026-03-16 (v0.3.0 milestone added)*
*Updated: 2026-03-18 (v0.4.0 milestone added)*
*Updated: 2026-03-19 (v0.5.0 milestone added)*
*Updated: 2026-03-19 (v0.5.1 milestone added)*
*Updated: 2026-03-22 (v0.5.2 milestone added)*
