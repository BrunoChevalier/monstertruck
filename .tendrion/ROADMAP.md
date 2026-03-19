# Roadmap: monstertruck

## Milestones

- **v0.2.0** -- Phases 1-4
- **v0.3.0** -- Phases 5-8
- **v0.4.0** -- Phases 9-12
- **v0.5.0** -- Phases 13-15
- **v0.5.1** -- Phases 16-20

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
- [ ] **Phase 16: Tolerance Foundation and API Safety** - Centralize tolerance constants, add non_exhaustive to option structs, refactor deprecated delegations
- [ ] **Phase 17: Curve Intersection Engine** - Implement shared curve-curve intersection module for Gordon grid computation and trim intersection
- [ ] **Phase 18: Gordon Surface Variants** - Auto-intersect and verified-grid Gordon constructors using the curve intersection engine
- [ ] **Phase 19: Trim Tessellation Robustness** - Fallback boundary projection and tolerance-derived tessellation constants
- [ ] **Phase 20: Fixture Corpus and Migration Documentation** - Expand test fixtures across all surface types and add migration guidance docs

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
**Goal**: All tolerance constants are centralized in monstertruck-core and surface constructor option structs are safe for future extension
**Depends on**: Phase 15
**Requirements**: TOLAPI-01, TOLAPI-02, TOLAPI-03
**Success Criteria** (what must be TRUE):
  1. A `tolerance_constants` module in monstertruck-core exports SNAP_TOLERANCE, VERTEX_MERGE_TOLERANCE, TESSELLATION_TOLERANCE, PERIODIC_CLOSURE_RATIO, G1_ANGLE_TOLERANCE, and G2_CURVATURE_TOLERANCE with defaults that preserve existing behavior
  2. All surface constructor option structs (GordonOptions, SkinOptions, SweepRailOptions, Birail1Options, Birail2Options) have `#[non_exhaustive]` and downstream code still compiles
  3. The deprecated `gordon()` function delegates to `try_gordon()` with no independent implementation logic
  4. All existing tests pass without behavioral changes from tolerance centralization
**Plans**: TBD

### Phase 17: Curve Intersection Engine
**Goal**: A reusable curve-curve intersection module exists in monstertruck-geometry that downstream Gordon and trim code can call
**Depends on**: Phase 16
**Requirements**: CURVINT-01
**Success Criteria** (what must be TRUE):
  1. `monstertruck-geometry/src/nurbs/curve_intersect.rs` exports a public function that returns intersection parameters for two NURBS/B-spline curves
  2. Intersection results are accurate within SNAP_TOLERANCE from centralized constants
  3. The module handles degenerate cases (parallel curves, tangent intersections, self-intersections) without panics
**Plans**: TBD

### Phase 18: Gordon Surface Variants
**Goal**: Users can construct Gordon surfaces either by supplying a curve network (auto-intersection) or by providing pre-computed grid points with validation
**Depends on**: Phase 17
**Requirements**: GORDON-01, GORDON-02
**Success Criteria** (what must be TRUE):
  1. `try_gordon_from_network` computes intersection grid points from curve families using the curve intersection engine before compatibility normalization
  2. `try_gordon_verified` validates that caller-supplied grid points lie on both curve families within tolerance and snaps near-miss points
  3. Both variants produce surfaces equivalent to manual `try_gordon` calls with correctly computed grid points
  4. Gordon-specific network fixtures (near-miss grid points, nonuniform spacing) exercise both new variants
**Plans**: TBD

### Phase 19: Trim Tessellation Robustness
**Goal**: Trimmed face tessellation recovers from parameter search failures instead of silently dropping faces, and tessellation thresholds derive from centralized constants
**Depends on**: Phase 16
**Requirements**: TRIM-01, TRIM-02
**Success Criteria** (what must be TRUE):
  1. `PolyBoundaryPiece::try_new` falls back to UV interpolation from neighbors when parameter search fails, instead of returning None
  2. The hardcoded `1.0e-3` closure threshold and other tessellation magic constants are replaced with expressions derived from centralized tolerance constants
  3. Previously-dropped trimmed faces now tessellate successfully on regression fixtures
**Plans**: TBD

### Phase 20: Fixture Corpus and Migration Documentation
**Goal**: Comprehensive test fixtures cover pathological geometry cases across all surface types and migration docs guide users from deprecated to new APIs
**Depends on**: Phase 18, Phase 19
**Requirements**: FIXTURE-01, FIXTURE-02, FIXTURE-03, DOC-01
**Success Criteria** (what must be TRUE):
  1. Fixture corpus includes problematic rail/section combinations (inflection rails, converging rails, degenerate sections) with integration tests exercising surface constructors
  2. Fixture corpus includes near-degenerate NURBS cases (near-zero Jacobian, near-zero weight, collapsed control points) with tests verifying graceful handling
  3. Gordon-specific network fixtures (near-miss grid points, nonuniform spacing, high-degree curve families) are present and exercised by `try_gordon_from_network` and `try_gordon_verified`
  4. Doc comments on `try_*` functions and crate-level docs include before/after migration examples showing manual vs. automatic workflows
**Plans**: TBD

## Progress

**Execution Order:** Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Phase 5 -> Phase 6 -> Phase 7 -> Phase 8 -> Phase 9 -> Phase 10 -> Phase 11 -> Phase 12 -> Phase 13 -> Phase 14 -> Phase 15 -> Phase 16 -> Phase 17 -> Phase 18 -> Phase 19 -> Phase 20

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
| 16. Tolerance Foundation and API Safety | v0.5.1 | 0/TBD | Not started | - |
| 17. Curve Intersection Engine | v0.5.1 | 0/TBD | Not started | - |
| 18. Gordon Surface Variants | v0.5.1 | 0/TBD | Not started | - |
| 19. Trim Tessellation Robustness | v0.5.1 | 0/TBD | Not started | - |
| 20. Fixture Corpus and Migration Documentation | v0.5.1 | 0/TBD | Not started | - |

---

*Roadmap created: 2026-03-08*
*Updated: 2026-03-16 (v0.3.0 milestone added)*
*Updated: 2026-03-18 (v0.4.0 milestone added)*
*Updated: 2026-03-19 (v0.5.0 milestone added)*
*Updated: 2026-03-19 (v0.5.1 milestone added)*
