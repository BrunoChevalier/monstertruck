# Roadmap: monstertruck

## Milestones

- **v0.2.0** -- Phases 1-4
- **v0.3.0** -- Phases 5-8
- **v0.4.0** -- Phases 9-12
- **v0.5.0** -- Phases 13-15
- **v0.5.1** -- Phases 16-20
- **v0.5.2** -- Phases 21-23
- **v0.5.3** -- Phases 24-32

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
- [x] **Phase 23: Error Propagation and Test Hardening** - Replace silent fillet rollback with explicit errors and fix proptest tolerance
- [x] **Phase 24: GPU Test Reliability** - Fix proptest camera failures and add graceful GPU hardware skip
- [x] **Phase 25: Clippy and Dependency Hygiene** - Eliminate clippy warnings in monstertruck-step and update deprecated nom/quick-xml
- [x] **Phase 26: Core and Traits Coverage** - Add tests for monstertruck-core tolerance infrastructure and monstertruck-traits curve/surface impls
- [ ] **Phase 27: Topology Coverage** - Add edge, wire, face, and shell operation tests to reach 50%+ coverage
- [ ] **Phase 28: Modeling Coverage** - Add builder, profile, and text module tests to reach 45%+ coverage
- [ ] **Phase 29: Solid and STEP Coverage** - Add boolean/fillet/healing tests and STEP round-trip tests for zero-coverage crates
- [ ] **Phase 30: New Surface Constructors** - Implement ruled surface, loft surface, and expanded geometry healing
- [ ] **Phase 31: Deferred Ayam Port Completion** - Intersection-grid Gordon variants and trim tessellation robustness improvements
- [ ] **Phase 32: I/O Validation and Migration Docs** - STEP and OBJ/STL round-trip fidelity tests and manual workflow migration guidance

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
**Plans**: Archived — see `.tendrion/milestones/v0.5.2-ROADMAP.md`

### Phase 22: Conversion Fidelity
**Plans**: Archived — see `.tendrion/milestones/v0.5.2-ROADMAP.md`

### Phase 23: Error Propagation and Test Hardening
**Plans**: Archived — see `.tendrion/milestones/v0.5.2-ROADMAP.md`

### Phase 24: GPU Test Reliability
**Goal**: GPU camera proptests pass on all inputs and render tests skip gracefully when no GPU hardware is available
**Depends on**: None
**Requirements**: RELY-01, RELY-02
**Success Criteria** (what must be TRUE):
  1. Running `cargo test` in monstertruck-rendimpl produces zero proptest failures for camera perspective and parallel view fitting
  2. Degenerate all-zero-point inputs are handled with explicit early returns or clamped defaults instead of NaN propagation
  3. GPU/render tests on a headless CI machine (no GPU) exit with skip status rather than failure
  4. `cargo test --workspace` shows no GPU-related test failures regardless of hardware availability

### Phase 25: Clippy and Dependency Hygiene
**Goal**: monstertruck-step compiles with zero clippy warnings and all deprecated dependencies are updated to maintained versions
**Depends on**: None
**Requirements**: RELY-03, RELY-04
**Success Criteria** (what must be TRUE):
  1. `cargo clippy -p monstertruck-step -- -D warnings` exits with code 0
  2. nom dependency is updated from v3.2.1 to a supported version (v7+) and all STEP parsers compile
  3. quick-xml dependency is updated from v0.22.0 to a supported version (v0.30+) and all XML-based I/O compiles
  4. `cargo clippy --workspace -- -D warnings` shows no new warnings introduced by the dependency updates

### Phase 26: Core and Traits Coverage
**Goal**: monstertruck-core and monstertruck-traits have meaningful test suites covering their public APIs
**Depends on**: None
**Requirements**: COV-05, COV-06
**Success Criteria** (what must be TRUE):
  1. monstertruck-core test coverage reaches 55% or higher as measured by cargo-tarpaulin
  2. Tolerance infrastructure functions (comparison, epsilon management) have dedicated unit tests
  3. monstertruck-traits has at least one test per public trait method for curve and surface trait implementations
  4. `cargo test -p monstertruck-core -p monstertruck-traits` passes with all new tests green

### Phase 27: Topology Coverage
**Goal**: monstertruck-topology test coverage exceeds 50% with tests exercising all major topological operations
**Depends on**: Phase 26
**Requirements**: COV-03
**Success Criteria** (what must be TRUE):
  1. monstertruck-topology test coverage reaches 50% or higher as measured by cargo-tarpaulin
  2. Edge creation, splitting, and merging operations have dedicated test cases
  3. Wire construction and face boundary traversal operations have dedicated test cases
  4. Shell connectivity and orientation validation operations have dedicated test cases

### Phase 28: Modeling Coverage
**Goal**: monstertruck-modeling test coverage exceeds 45% with tests for builder patterns, profile operations, and text modules
**Depends on**: Phase 27
**Requirements**: COV-04
**Success Criteria** (what must be TRUE):
  1. monstertruck-modeling test coverage reaches 45% or higher as measured by cargo-tarpaulin
  2. Builder API methods (extrude, revolve, sweep) each have at least one round-trip construction test
  3. Profile combination and validation paths have test coverage
  4. Text/glyph module public functions have at least one test each

### Phase 29: Solid and STEP Coverage
**Goal**: monstertruck-solid and monstertruck-step have meaningful test coverage where previously there was none
**Depends on**: Phase 25, Phase 28
**Requirements**: COV-01, COV-02
**Success Criteria** (what must be TRUE):
  1. monstertruck-solid has unit tests covering boolean union/intersection/difference operations
  2. monstertruck-solid has unit tests covering fillet pipeline entry points and healing module functions
  3. monstertruck-step has round-trip tests that write a solid to STEP format and re-read it with geometry comparison
  4. `cargo test -p monstertruck-solid -p monstertruck-step` passes with all new tests green

### Phase 30: New Surface Constructors
**Goal**: Users can create ruled surfaces, loft surfaces, and apply expanded geometry healing through the modeling API
**Depends on**: Phase 26
**Requirements**: CAD-01, CAD-02, CAD-03
**Success Criteria** (what must be TRUE):
  1. A ruled surface between two boundary curves can be constructed via a public API and the resulting surface evaluates correctly at parameter boundaries
  2. A loft surface through three or more cross-section profiles can be constructed with configurable interpolation and the result is G1-continuous between sections
  3. Geometry healing can detect and repair gaps between adjacent edges within a configurable tolerance
  4. Edge-curve consistency checking identifies and reports mismatches between topological edges and their underlying curve geometry
  5. All new constructors are accessible through monstertruck-modeling builder APIs

### Phase 31: Deferred Ayam Port Completion
**Goal**: Intersection-grid driven Gordon surface construction and improved trim tessellation robustness complete the deferred Ayam port items
**Depends on**: Phase 30
**Requirements**: PORT-01, PORT-02
**Success Criteria** (what must be TRUE):
  1. Gordon surface constructor accepts a family of curves and automatically computes intersection grid points without user-supplied parameters
  2. Intersection-grid Gordon surfaces produce valid B-rep topology that passes shell validation
  3. Trim tessellation handles degenerate trimming boundary cases (near-zero-area loops, self-touching boundaries) without panics
  4. Trimmed surface tessellation output is watertight at boundary edges within the configured tolerance

### Phase 32: I/O Validation and Migration Docs
**Goal**: STEP and mesh export pipelines have round-trip validation tests and manual workflow users have migration guidance
**Depends on**: Phase 29
**Requirements**: IO-01, IO-02, DOC-01
**Success Criteria** (what must be TRUE):
  1. STEP export round-trip tests verify that geometry bounding boxes match within tolerance after a write-read cycle
  2. OBJ export tests verify correct vertex/face formatting and that mesh vertex counts match the source tessellation
  3. STL export tests verify valid binary/ASCII format output and consistent face normal orientation
  4. Migration guidance document exists covering new API patterns, deprecated function replacements, and version upgrade steps

## Progress

**Execution Order:** Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Phase 5 -> Phase 6 -> Phase 7 -> Phase 8 -> Phase 9 -> Phase 10 -> Phase 11 -> Phase 12 -> Phase 13 -> Phase 14 -> Phase 15 -> Phase 16 -> Phase 17 -> Phase 18 -> Phase 19 -> Phase 20 -> Phase 21 -> Phase 22 -> Phase 23 -> Phase 24 -> Phase 25 -> Phase 26 -> Phase 27 -> Phase 28 -> Phase 29 -> Phase 30 -> Phase 31 -> Phase 32

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
| 23. Error Propagation and Test Hardening | v0.5.2 | 1/1 | ✓ Complete | 2026-03-22 |
| 24. GPU Test Reliability | v0.5.3 | 2/2 | ✓ Complete | 2026-03-23 |
| 25. Clippy and Dependency Hygiene | v0.5.3 | 2/2 | ✓ Complete | 2026-03-23 |
| 26. Core and Traits Coverage | v0.5.3 | 2/2 | ✓ Complete | 2026-03-23 |
| 27. Topology Coverage | v0.5.3 | 0/TBD | Not started | - |
| 28. Modeling Coverage | v0.5.3 | 0/TBD | Not started | - |
| 29. Solid and STEP Coverage | v0.5.3 | 0/TBD | Not started | - |
| 30. New Surface Constructors | v0.5.3 | 0/TBD | Not started | - |
| 31. Deferred Ayam Port Completion | v0.5.3 | 0/TBD | Not started | - |
| 32. I/O Validation and Migration Docs | v0.5.3 | 0/TBD | Not started | - |

---

*Roadmap created: 2026-03-08*
*Updated: 2026-03-16 (v0.3.0 milestone added)*
*Updated: 2026-03-18 (v0.4.0 milestone added)*
*Updated: 2026-03-19 (v0.5.0 milestone added)*
*Updated: 2026-03-19 (v0.5.1 milestone added)*
*Updated: 2026-03-22 (v0.5.2 milestone added)*
*Updated: 2026-03-23 (v0.5.3 milestone added)*
