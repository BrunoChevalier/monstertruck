# Roadmap: monstertruck

## Milestones

- **v0.2.0** -- Phases 1-4

## Phases

- [x] **Phase 1: Core Stabilization** - Fix critical panics, reduce unwrap density, replace deprecated deps, add benchmarking
- [x] **Phase 2: Numerical Robustness** - Adaptive tolerances, solver fallbacks, tessellation stitching, boolean hardening, fuzzing
- [x] **Phase 3: Feature Completeness** - STEP boolean export, chamfer, shell/offset, and draft/taper operations
- [x] **Phase 4: Strategic Evolution** - cgmath-to-nalgebra migration, RwLock concurrency, GPU tessellation, T-spline completion

## Phase Details

### Phase 1: Core Stabilization
**Goal**: Boolean-to-modeling pipeline no longer panics, unwrap density is materially reduced, deprecated dependencies are replaced, and performance regressions are detectable
**Depends on**: None
**Requirements**: CORE-01, CORE-02, CORE-03, CORE-04
**Success Criteria** (what must be TRUE):
  1. All 9 IntersectionCurve unimplemented!() arms in monstertruck-modeling/src/geometry.rs are replaced with working implementations and exercised by tests
  2. unwrap() calls in monstertruck-solid and monstertruck-meshing are reduced by at least 50%, with remaining calls annotated with SAFETY comments
  3. monstertruck-derive compiles without proc-macro-error; the deprecated crate is removed from the dependency tree
  4. Running `cargo bench` executes criterion or divan benchmarks covering NURBS evaluation, tessellation, and boolean operations
**Plans**: TBD

### Phase 2: Numerical Robustness
**Goal**: Chained geometric operations (boolean, fillet, tessellation) produce correct results without tolerance drift, solver failures, or visible seams
**Depends on**: Phase 1
**Requirements**: ROBUST-01, ROBUST-02, ROBUST-03, ROBUST-04, ROBUST-05
**Success Criteria** (what must be TRUE):
  1. Per-operation tolerance propagation is in place: a boolean followed by fillet followed by tessellation carries accumulated tolerance metadata through the pipeline
  2. The Newton solver falls back to Levenberg-Marquardt or bisection when the Jacobian is near-singular, and previously-divergent test cases converge
  3. Adjacent trimmed faces share boundary vertices after tessellation -- no visible seams in rendered output for the standard test models
  4. Boolean operations on tangent-face, coincident-face, and pole-degeneration inputs produce valid topology without panics
  5. cargo-fuzz targets exist for NURBS evaluation, knot vector manipulation, and STEP parsing, and a 60-second fuzz run completes without crashes
**Plans**: TBD

### Phase 3: Feature Completeness
**Goal**: Users can export boolean-operated shapes to STEP, apply chamfers, shell/offset, and draft/taper to solid bodies
**Depends on**: Phase 1, Phase 2
**Requirements**: FEAT-01, FEAT-02, FEAT-03, FEAT-05
**Success Criteria** (what must be TRUE):
  1. A shape created by boolean union/difference/intersection can be written to a STEP file and re-imported with matching topology
  2. Chamfer operations produce flat-cut edges on solid bodies and the result passes topological validity checks
  3. Shell operations hollow out a solid body to a specified wall thickness, and offset operations produce a valid offset surface
  4. Draft/taper operations apply a specified draft angle to faces of a solid body, producing valid B-rep output
**Plans**: TBD

### Phase 4: Strategic Evolution
**Goal**: The codebase runs on nalgebra instead of cgmath, read-heavy topology traversal has reduced lock contention, GPU-accelerated tessellation is prototyped, and T-spline validation is complete
**Depends on**: Phase 1
**Requirements**: FEAT-04, EVOLVE-01, EVOLVE-02, EVOLVE-03
**Success Criteria** (what must be TRUE):
  1. A monstertruck-math adapter crate exists, monstertruck-core compiles against nalgebra, and cgmath is removed from at least the foundation layer
  2. Topology traversal in tessellation uses RwLock instead of Mutex, and a benchmark demonstrates reduced contention under concurrent read workloads
  3. A WebGPU compute shader prototype performs adaptive NURBS subdivision, producing tessellation output matching the CPU path within tolerance
  4. T-spline connection parity checks and zero knot interval validation in t_spline/t_nurcc.rs and t_spline/t_mesh.rs pass without TODOs remaining
**Plans**: TBD

## Progress

**Execution Order:** Phase 1 -> Phase 2 -> Phase 3 -> Phase 4

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Core Stabilization | v0.2.0 | 4/4 | ✓ Complete | 2026-03-09 |
| 2. Numerical Robustness | v0.2.0 | 4/4 | ✓ Complete | 2026-03-09 |
| 3. Feature Completeness | v0.2.0 | 5/5 | ✓ Complete | 2026-03-10 |
| 4. Strategic Evolution | v0.2.0 | 4/4 | ✓ Complete | 2026-03-15 |

---

*Roadmap created: 2026-03-08*
