# Roadmap: monstertruck

## Milestones

- **v0.2.0** -- Phases 1-4
- **v0.3.0** -- Phases 5-8
- **v0.4.0** -- Phases 9-12
- **v0.5.0** -- Phases 13-15

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
- [ ] **Phase 15: Font Stress Testing and Performance** - Pathological font corpus and large-text pipeline benchmarks

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
**Goal**: Surface constructors accept typed option structs instead of positional parameters, invalid curve networks produce actionable diagnostics, and patch split/extract operations are available
**Depends on**: None
**Requirements**: API-01, API-02, SURF-03
**Success Criteria** (what must be TRUE):
  1. All surface constructor functions (sweep_rail, birail, gordon, skin) accept dedicated option structs for orientation/frame rules and interpolation modes instead of positional parameters
  2. Passing an incompatible curve network to gordon or birail returns a Result::Err containing specific diagnostic information (e.g., knot mismatch location, curve count discrepancy) rather than a generic error
  3. A NURBS surface can be split at an arbitrary parameter value and the resulting sub-patches are valid, watertight surfaces that tessellate without cracks
  4. Extracting a rectangular sub-patch from a surface by parameter bounds produces a geometrically identical subset of the original surface
**Plans**: TBD

### Phase 14: Profile Solid Pipeline
**Goal**: Users can create solids from planar profiles via revolve and sweep operations, combine font glyph outlines with arbitrary sketch loops, and all profile-generated solids pass consistency checks
**Depends on**: Phase 13
**Requirements**: PROFILE-01, PROFILE-02, PROFILE-03
**Success Criteria** (what must be TRUE):
  1. A closed planar profile can be revolved around an axis to produce a valid solid with correct topology (e.g., a circle revolved 360 degrees produces a torus)
  2. A closed planar profile can be swept along a 3D guide curve to produce a valid solid whose cross-sections match the original profile
  3. A face can be constructed from a mixture of font glyph outlines and user-defined sketch loops, with correct winding and hole detection
  4. All profile-generated solids pass Euler-Poincare invariant checks and tessellate without cracks or missing faces
**Plans**: TBD

### Phase 15: Font Stress Testing and Performance
**Goal**: The profile pipeline is validated against pathological font geometry and benchmarked for throughput on large text inputs
**Depends on**: Phase 14
**Requirements**: FONT-03, FONT-04
**Success Criteria** (what must be TRUE):
  1. A curated stress corpus of at least 10 pathological font fixtures (small features, deeply nested contours, near-degenerate curves, self-touching outlines) exists and all fixtures produce valid geometry without panics
  2. Regression tests using the stress corpus run in CI and catch topology or tessellation failures
  3. Benchmark results exist for profile pipeline throughput on strings of 100+ characters, establishing a baseline for performance regression detection
**Plans**: TBD

## Progress

**Execution Order:** Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Phase 5 -> Phase 6 -> Phase 7 -> Phase 8 -> Phase 9 -> Phase 10 -> Phase 11 -> Phase 12 -> Phase 13 -> Phase 14 -> Phase 15

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
| 15. Font Stress Testing and Performance | v0.5.0 | 0/TBD | Not started | - |

---

*Roadmap created: 2026-03-08*
*Updated: 2026-03-16 (v0.3.0 milestone added)*
*Updated: 2026-03-18 (v0.4.0 milestone added)*
*Updated: 2026-03-19 (v0.5.0 milestone added)*
