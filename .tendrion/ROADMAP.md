# Roadmap: monstertruck

## Milestones

- **v0.2.0** -- Phases 1-4
- **v0.3.0** -- Phases 5-8
- **v0.4.0** -- Phases 9-12

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
- [ ] **Phase 12: Font Pipeline and Final Documentation** - End-to-end text profile tests with real fonts and updated Ayam port plan

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
**Success Criteria** (what must be TRUE):
  1. The v0.3.0 criteria 2 and 4 gaps (boolean result face handling) pass their original verification checks without manual workarounds
  2. A shared tolerance constants module exists and is imported by truck-shapeops, truck-modeling, and truck-meshalgo
  3. Running `cargo test -p truck-shapeops` passes with no boolean-related test failures
  4. Tolerance constants are documented with rationale for each value choice
**Plans**: TBD

### Phase 10: Test Infrastructure and Healing Hooks
**Goal**: A fixture corpus of problematic geometries exists for regression testing and truck-shapeops can heal topology after surface construction
**Depends on**: Phase 9
**Requirements**: BOOL-02, TEST-01
**Success Criteria** (what must be TRUE):
  1. Fixture files for problematic rail/section combos, near-degenerate NURBS, and representative font glyphs are loadable via test helpers
  2. truck-shapeops exposes healing hooks that detect and repair topology gaps introduced by sweep_rail, birail, and gordon constructors
  3. At least 3 degenerate-geometry fixtures trigger healing code paths and produce valid topology
  4. Running `cargo test` with the new fixtures produces no panics or timeouts
**Plans**: TBD

### Phase 11: Surface Constructors
**Goal**: Users can create multi-rail sweeps, periodic sweeps, and birail/gordon surfaces through typed builder APIs in truck-modeling
**Depends on**: Phase 10
**Requirements**: SURF-01, SURF-02
**Success Criteria** (what must be TRUE):
  1. `SweepBuilder::multi_rail()` and `SweepBuilder::periodic()` produce valid BSplineSurface results for 3+ rail curves
  2. Builder wrappers for sweep_rail, birail, and gordon in truck-modeling return typed Result errors instead of panicking on invalid input
  3. Generated surfaces pass Euler-Poincare topology checks when converted to solids
  4. At least one periodic sweep test demonstrates closed-surface continuity at the wrap seam
**Plans**: TBD

### Phase 12: Font Pipeline and Final Documentation
**Goal**: Text profile creation from real fonts works end-to-end with hole preservation and the Ayam port plan reflects current status
**Depends on**: Phase 11
**Requirements**: FONT-01, DOC-02
**Success Criteria** (what must be TRUE):
  1. End-to-end tests load a real font fixture, generate glyph profiles for characters with holes (e.g., "B", "O"), and verify inner loop preservation
  2. Generated text profiles produce valid Wire topology suitable for extrusion
  3. AYAM_PORT_PLAN.md has all completed items checked, deprecated items marked, and remaining work documented
**Plans**: TBD

## Progress

**Execution Order:** Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Phase 5 -> Phase 6 -> Phase 7 -> Phase 8 -> Phase 9 -> Phase 10 -> Phase 11 -> Phase 12

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
| 12. Font Pipeline and Final Documentation | v0.4.0 | 0/TBD | Not started | - |

---

*Roadmap created: 2026-03-08*
*Updated: 2026-03-16 (v0.3.0 milestone added)*
*Updated: 2026-03-18 (v0.4.0 milestone added)*
