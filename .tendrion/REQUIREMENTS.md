# Requirements: monstertruck

**Created:** 2026-03-08
**Source:** Domain research + feature selection

## Requirements

### Core Stabilization

- [ ] **CORE-01**: Fix IntersectionCurve unimplemented!() arms — 9 arms in monstertruck-modeling/src/geometry.rs gate the boolean-to-modeling pipeline; any workflow producing intersection curves that feeds back into modeling operations panics at runtime
- [ ] **CORE-02**: Audit and reduce unwrap() density in monstertruck-solid and monstertruck-meshing — 978 unwrap() calls across 155 files; highest density in solid and meshing where geometric invariants may not hold for degenerate inputs
- [ ] **CORE-03**: Replace deprecated proc-macro-error in monstertruck-derive — upstream proc-macro-error crate is deprecated; needs replacement with proc-macro-error2 or compile_error!
- [ ] **CORE-04**: Add benchmarking infrastructure — criterion or divan for performance regression detection in NURBS evaluation, tessellation, and boolean operations

### Numerical Robustness

- [ ] **ROBUST-01**: Adaptive tolerance framework — replace global TOLERANCE = 1.0e-6 with per-operation tolerance propagation tracking to prevent error accumulation across chained operations (boolean -> fillet -> tessellation)
- [ ] **ROBUST-02**: Newton solver fallbacks — add Levenberg-Marquardt or bisection fallbacks for near-singular Jacobians in the Newton solver
- [ ] **ROBUST-03**: Tessellation crack-filling at trimmed surface boundaries — independent trim curve approximation produces visible seams between adjacent faces; implement boundary-aware stitching using topological adjacency
- [ ] **ROBUST-04**: Boolean operation hardening — handle tangent face, coincident face, and pole degeneration edge cases in monstertruck-solid/src/transversal/
- [ ] **ROBUST-05**: Add fuzzing targets — cargo-fuzz targets for NURBS evaluation, knot vector manipulation, and STEP parsing

### Feature Completeness

- [ ] **FEAT-01**: STEP boolean export — enable export of shapes created by set operations; currently blocked with "Shapes created by set operations cannot be output yet"
- [ ] **FEAT-02**: Chamfer operations — flat-cut equivalent of existing fillet support; standard in mechanical CAD workflows
- [ ] **FEAT-03**: Shell/offset operations — hollow-out and surface offset; required for mechanical part design workflows
- [ ] **FEAT-04**: T-spline completion — resolve validation TODOs in t_spline/t_nurcc.rs and t_spline/t_mesh.rs for connection parity checks and zero knot intervals
- [ ] **FEAT-05**: Draft/taper operations — draft angle and taper for injection-molded part design workflows

### Strategic Evolution

- [ ] **EVOLVE-01**: cgmath to nalgebra migration — create monstertruck-math adapter crate, migrate monstertruck-core first, propagate outward; cgmath 0.18 unmaintained since 2021
- [ ] **EVOLVE-02**: RwLock for read-heavy topology traversal — replace Arc<Mutex<_>> with RwLock for read-heavy workloads like tessellation to reduce contention
- [ ] **EVOLVE-03**: GPU compute tessellation — explore WebGPU compute shaders for adaptive NURBS subdivision on GPU

## Out of Scope

All researched features were selected. No features explicitly excluded.

---
*Generated: 2026-03-08 via /td:new-project*
