# Requirements: monstertruck

**Created:** 2026-03-08
**Source:** Domain research + feature selection

## Requirements

### Core Stabilization

- [x] **CORE-01**: Fix IntersectionCurve unimplemented!() arms — 9 arms in monstertruck-modeling/src/geometry.rs gate the boolean-to-modeling pipeline; any workflow producing intersection curves that feeds back into modeling operations panics at runtime
- [x] **CORE-02**: Audit and reduce unwrap() density in monstertruck-solid and monstertruck-meshing — 978 unwrap() calls across 155 files; highest density in solid and meshing where geometric invariants may not hold for degenerate inputs
- [x] **CORE-03**: Replace deprecated proc-macro-error in monstertruck-derive — upstream proc-macro-error crate is deprecated; needs replacement with proc-macro-error2 or compile_error!
- [x] **CORE-04**: Add benchmarking infrastructure — criterion or divan for performance regression detection in NURBS evaluation, tessellation, and boolean operations

### Numerical Robustness

- [x] **ROBUST-01**: Adaptive tolerance framework — replace global TOLERANCE = 1.0e-6 with per-operation tolerance propagation tracking to prevent error accumulation across chained operations (boolean -> fillet -> tessellation)
- [x] **ROBUST-02**: Newton solver fallbacks — add Levenberg-Marquardt or bisection fallbacks for near-singular Jacobians in the Newton solver
- [x] **ROBUST-03**: Tessellation crack-filling at trimmed surface boundaries — independent trim curve approximation produces visible seams between adjacent faces; implement boundary-aware stitching using topological adjacency
- [x] **ROBUST-04**: Boolean operation hardening — handle tangent face, coincident face, and pole degeneration edge cases in monstertruck-solid/src/transversal/
- [x] **ROBUST-05**: Add fuzzing targets — cargo-fuzz targets for NURBS evaluation, knot vector manipulation, and STEP parsing

### Feature Completeness

- [x] **FEAT-01**: STEP boolean export — enable export of shapes created by set operations; currently blocked with "Shapes created by set operations cannot be output yet"
- [x] **FEAT-02**: Chamfer operations — flat-cut equivalent of existing fillet support; standard in mechanical CAD workflows
- [x] **FEAT-03**: Shell/offset operations — hollow-out and surface offset; required for mechanical part design workflows
- [x] **FEAT-04**: T-spline completion — resolve validation TODOs in t_spline/t_nurcc.rs and t_spline/t_mesh.rs for connection parity checks and zero knot intervals
- [x] **FEAT-05**: Draft/taper operations — draft angle and taper for injection-molded part design workflows

### Strategic Evolution

- [x] **EVOLVE-01**: cgmath to nalgebra migration — create monstertruck-math adapter crate, migrate monstertruck-core first, propagate outward; cgmath 0.18 unmaintained since 2021
- [x] **EVOLVE-02**: RwLock for read-heavy topology traversal — replace Arc<Mutex<_>> with RwLock for read-heavy workloads like tessellation to reduce contention
- [x] **EVOLVE-03**: GPU compute tessellation — explore WebGPU compute shaders for adaptive NURBS subdivision on GPU

### Build Fix (v0.3.0)

- [x] **BUILD-01**: Port polynomial solvers (solve_quadratic, pre_solve_cubic, solve_cubic, pre_solve_quartic, solve_quartic) from matext4cgmath to monstertruck-math, using Algorithm 954 rescaling and Newton polishing for robustness

### Topology Hardening (v0.3.0)

- [x] **TOPO-01**: Harden cut_face_by_bezier via parameter-space projection for IntersectionCurve boundary edges, with NURBS approximation fallback when projection fails
- [x] **TOPO-02**: Fix homogeneous coordinate seam averaging bug in fillet_along_wire — dehomogenize Vector4 control points before averaging to produce correct 3D midpoints
- [x] **TOPO-03**: Add Euler-Poincare debug assertions and is_oriented() post-operation checks after fillet topology modifications

### Integration Mode (v0.3.0)

- [x] **INTEG-01**: Implement FilletMode with KeepSeparateFace (default, current behavior) and IntegrateVisual (G1/G2 continuity-annotated separate faces with seamless tessellation)
- [x] **INTEG-02**: Add extend_mode and corner_mode fields to FilletOptions

### Documentation (v0.3.0)

- [x] **DOC-01**: Update FILLET_IMPLEMENTATION_PLAN.md to reflect final v0.3.0 status — mark completed items, update Phase 6 description, remove deprecated sections

### Boolean Operations (v0.4.0)

- [ ] **BOOL-01**: Fix pre-existing boolean op bugs identified in v0.3.0 phase verification — criteria 2+4 gaps caused by boolean result face handling in truck-shapeops
- [ ] **BOOL-02**: Add topological integration and healing hooks in truck-shapeops for new surface constructors (sweep_rail, birail, gordon)

### Surface Constructors (v0.4.0)

- [ ] **SURF-01**: Implement multi-rail sweep and periodic sweep variants (port ay_npt_sweep, ay_npt_sweepperiodic algorithms)
- [ ] **SURF-02**: Add builder-level wrappers for sweep_rail, birail, and gordon in truck-modeling with typed error handling

### Font/Profile Pipeline (v0.4.0)

- [ ] **FONT-01**: End-to-end text profile creation tests with real-font fixtures including hole-preserving glyphs (validates Phase 5 done criteria from Ayam plan)

### Testing & Quality (v0.4.0)

- [ ] **TEST-01**: Create fixture corpus for problematic rail/section combinations, near-degenerate NURBS cases, and representative fonts/glyph sets
- [ ] **TEST-02**: Define numeric tolerance policy and shared constants across crates

### Documentation (v0.4.0)

- [ ] **DOC-02**: Update AYAM_PORT_PLAN.md to reflect current implementation status — mark deprecated items, document remaining work, verify all checkboxes

## Out of Scope

- Full corner-blend networks for arbitrary high-valence vertices
- Guaranteed class-A continuity targets beyond G1/G2 constraints
- Automatic feature recognition UI
- Literal NURBS surface merging (IntegrateIntoHost) — research confirms this is an anti-pattern
- relay_spheres convergence on high-curvature surfaces (unless it blocks test cases)
- Option structs for orientation/frame rules and interpolation modes (deferred)
- Gordon intersection-grid driven variants and invalid network diagnostics (deferred)
- Patch split/extract workflows (deferred)
- Solid creation by revolve/sweep — v2 font track (deferred)
- Font track milestones M1-M4 with real-font fixtures (deferred — FONT-01 covers basic coverage)
- Curated pathological geometry regression corpus (deferred)
- cargo test + cargo clippy verification gates (deferred)
- Migration guidance for manual workflow users (deferred)
- Tessellation robustness improvements — Phase 8 of Ayam plan (explicitly deferred, no regressions identified)

## Traceability

- v0.2.0: CORE-01 through EVOLVE-03
- v0.3.0: BUILD-01, TOPO-01 through TOPO-03, INTEG-01 through INTEG-02, DOC-01
- v0.4.0: BOOL-01 through BOOL-02, SURF-01 through SURF-02, FONT-01, TEST-01 through TEST-02, DOC-02

---
*Generated: 2026-03-08 via /td:new-project*
*Updated: 2026-03-16 via /td:new-milestone (v0.3.0)*
*Updated: 2026-03-18 via /td:new-milestone (v0.4.0)*
