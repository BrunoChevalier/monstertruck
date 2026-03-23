# `monstertruck`

**M**ultifarious **O**mnificence, **N**omenclature **S**tandardized, **T**erminology **E**nhanced & **R**efactored **Truck** – a **Ru**st **C**ad **K**ernel.

## Overview

`monstertruck` is an open-source, Rust-based shape processing kernel. It is a heavily fortified, feature-expanded fork of the original [`truck`](https://github.com/ricosjp/truck) project.

The underlying philosophy of this kernel rests on three foundational pillars:

- **Modern Tooling**

  We are building a next-generation CAD kernel in Rust with first-class WebGPU support.

- **Classical Techniques, Reborn**

  This a Rust-native implementation of classic Boundary Representation (B-rep) and NURBS from the ground up.

- **Ship of Theseus-like Architecture**

  Instead of repeating the mistakes of monolithic CAD kernel architectures, we abandoned the idea of a single, massive library or app. Instead, we modularized the kernel into a collection of small, highly optimized, and interchangeable crates – much like [the ship of Theseus](https://en.wikipedia.org/wiki/Ship_of_Theseus).

## Why Was This Forked?

Getting PRs accepted upstream was proving to be a challenge, so we spun up `monstertruck` to keep development moving.

This fork exists to accomplish two main goals:

1. **Supercharge the functionality:** We are actively adding and enhancing features, tools, and operations that go beyond the original scope (hence the _Multifarious Omnificence_). This includes merging `truck` PRs that we deem useful (but you are welcome to open PRs against `monstertruck` directly, ofc!).

2. **Fix the ergonomics:** The original codebase suffered from unconventional phrasing, non-idiomatic naming conventions, and occasionally confusing translations.
   We have overhauled the project using idiomatic Rust naming conventions and standard, industry-recognized CAD terminology.
   Our goal is to make the codebase highly inclusive, readable, and accessible – whether you are a non-native English speaker or a seasoned CAD veteran.

## Usage

### Running the Examples

All examples are located under the `examples` directory within each respective crate. They use standard Cargo syntax for execution.

To test-drive `monstertruck` and render your first object, run the following commands:

```bash
# Clone the required submodules
git submodule update --init

# Run the basic rotation example
cargo run --example rotate-objects
```

## Architecture & Crate Ecosystem

The `monstertruck` kernel is split into independent crates so you only need to pull in what you need (and also to help with build times).

### Core & Geometry

- [`monstertruck-math`](monstertruck-math/) – Math abstraction layer with nalgebra backend, polynomial solvers, and optional `cgmath64` compatibility shim.
- [`monstertruck-core`](monstertruck-core/) – Core types and traits for linear algebra, curves, surfaces, and tolerances.
- [`monstertruck-derive`](monstertruck-derive/) – Derive macros for geometric traits.
- [`monstertruck-traits`](monstertruck-traits/) – Geometric trait definitions.
- [`monstertruck-geometry`](monstertruck-geometry/) – Geometric primitives: knot vectors, B-splines, NURBS, and T-splines.

### Topology & Modeling

- [`monstertruck-topology`](monstertruck-topology/) – Topological data structures: vertices, edges, wires, faces, shells, and solids.
- [`monstertruck-modeling`](monstertruck-modeling/) – Integrated geometric and topological modeling algorithms. The `solid-ops` feature re-exports shell, offset, and draft operations from `monstertruck-solid`.
- [`monstertruck-solid`](monstertruck-solid/) – Boolean operations, fillets, chamfers, shell/offset, draft/taper, and shape healing for solids.
- [`monstertruck-assembly`](monstertruck-assembly/) – Assembly data structures using a directed acyclic graph (DAG).

### Meshing & Rendering

- [`monstertruck-mesh`](monstertruck-mesh/) – Polygon mesh data structures and algorithms.
- [`monstertruck-meshing`](monstertruck-meshing/) – Tessellation and meshing algorithms for B-rep shapes.
- [`monstertruck-gpu`](monstertruck-gpu/) – Graphics utility crate built on `wgpu`, including a WebGPU compute-shader tessellator for NURBS surfaces.
- [`monstertruck-render`](monstertruck-render/) – Shape and polygon mesh visualization.

### I/O & Bindings

- [`monstertruck-step`](monstertruck-step/) – STEP file import and export, including boolean-result shape export.
- [`monstertruck-wasm`](monstertruck-wasm/) – WebAssembly/JavaScript bindings.

## Development

### Running Tests

This project uses [cargo-nextest](https://nexte.st/) for test execution with per-test timeouts:

```bash
cargo nextest run -p monstertruck-geometry --lib
```

### Benchmarks

Criterion benchmarks are available for the core crates:

```bash
cargo bench -p monstertruck-geometry
cargo bench -p monstertruck-meshing
cargo bench -p monstertruck-solid
```

To verify benchmarks compile without running them:

```bash
cargo test --benches
```

### Fuzzing

Fuzz targets are available for NURBS evaluation, knot vector manipulation, and STEP parsing using `cargo-fuzz` (requires nightly):

```bash
cd monstertruck-geometry && cargo +nightly fuzz run nurbs_eval -- -max_total_time=60
cd monstertruck-geometry && cargo +nightly fuzz run knot_vector -- -max_total_time=60
cd monstertruck-step && cargo +nightly fuzz run step_parse -- -max_total_time=60
```

### Numerical Robustness

The `monstertruck-core` crate provides:

- **`OperationTolerance`** -- per-operation tolerance propagation to track accumulated error across chained geometric operations.
- **`solve_robust` / `solve_robust_1d`** -- Newton solver variants with Levenberg-Marquardt and bisection fallbacks for near-singular Jacobians.

The `monstertruck-meshing` crate includes boundary-aware vertex stitching during tessellation to eliminate seams between adjacent trimmed faces.

### Phase 28 -- Modeling Coverage

- **monstertruck-modeling builder round-trip tests** -- Added 23 tests covering extrude, revolve, sweep_rail, homotopy, and transformation builder APIs, plus primitive construction tests (rect, circle, cuboid).
- **monstertruck-modeling text and geometry enum tests** -- Added 17 tests covering TextOptions configuration, edge cases, and Curve/Surface enum variant construction.
- **Total:** 40 new modeling tests, coverage target 45%+.

### Phase 27 -- Topology Coverage

- **monstertruck-topology vertex/edge/wire tests** -- Added 39 unit tests covering edge creation, splitting via `cut_with_parameter`, wire construction, and concatenation.
- **monstertruck-topology face/shell/solid tests** -- Added 68 unit tests covering face boundary traversal, shell connectivity, orientation validation, and solid construction.
- **Total:** 107 new topology tests, coverage target 50%+.

### Phase 26 -- Core and Traits Coverage

- **monstertruck-core unit tests** -- Added 96 unit tests covering tolerance traits (`Tolerance`, `OperationTolerance`), bounding box operations, `Id` and `EntryMap` collections, derivative utilities, and `cgmath` extension methods. Coverage increased from ~40% toward 55%+.
- **monstertruck-traits unit tests** -- Added 80 unit tests covering `ParametricCurve`, `ParametricSurface`, `BoundedCurve`, `BoundedSurface`, `Invertible`, `Transformed`, and `SearchParameter` trait implementations. Coverage grew from 0% to meaningful levels.

### Phase 25 -- Clippy & Dependency Hygiene

- **vtkio upgrade** -- Updated `vtkio` from 0.6 to 0.7.0-rc2, eliminating deprecated `nom` v3.2.1 and `quick-xml` v0.22.0 transitive dependencies.
- **Clippy clean** -- Fixed 4 clippy warnings (unnecessary qualification in `stl.rs`, dead code in `validate.rs`). `cargo clippy --workspace -- -D warnings` now exits 0.

### Milestone v0.5.3 Summary (Phase 24) -- GPU Test Reliability

- **Projection matrix fixes (Phase 24)** -- Fixed transposed projection matrices (`ortho`, `perspective`, `frustum`) in `monstertruck-math`. Added epsilon guards for degenerate point clouds in camera fitting functions (`monstertruck-gpu`). 4 new unit tests.
- **GPU test graceful degradation (Phase 24)** -- `monstertruck-gpu` render tests (`bindgroup`, `msaa`, `wgsl-utils`) now skip gracefully when no GPU device is available via `try_init_device` and `os_alt_try_exec_test` helpers.

### Milestone v0.5.2 Summary (Phases 21--23) -- Fillet Conversion Pipeline Fix

Milestone v0.5.2 hardened the fillet conversion pipeline end-to-end: preserving edge identity through NURBS conversion (Phase 21), improving conversion fidelity with cubic interpolation and exact revolved surface support (Phase 22), and replacing silent error swallowing with explicit error propagation (Phase 23).

- **Edge identity preservation (Phase 21)** -- `ensure_cuttable_edge` now uses `set_curve()` instead of `Edge::new()` when converting `IntersectionCurve` edges to NURBS, preserving arc identity. Endpoint matching tolerance widened from `TOLERANCE` (1e-6) to `SNAP_TOLERANCE` (1e-5) for boolean-origin edges.
- **Conversion fidelity (Phase 22)** -- `sample_curve_to_nurbs`, `sample_surface_to_nurbs`, and fillet output paths upgraded from degree-1 to degree-3 cubic interpolation. Exact `RevolutedCurve` to `NurbsSurface` conversion via rational circle arc tensor product. Endpoint snapping in `convert_shell_in`/`convert_shell_out` preserves vertex-edge coincidence.
- **Explicit error propagation (Phase 23)** -- `FilletError::ShellNotClosed` variant replaces silent rollback in `fillet_edges_generic`. Tests updated to expect `Err(ShellNotClosed)` instead of silent no-ops. `generic_fillet_unsupported` corrected to expect `NonManifoldEdge` (adjacency check fires before geometry conversion on single-face shells).

### Milestone v0.5.1 Summary (Phases 16--20)

Milestone v0.5.1 focused on API hardening, robustness infrastructure, and migration documentation for the surface construction pipeline.

- **Tolerance centralization (Phase 16)** -- Consolidated 6 tolerance constants into `monstertruck-core::tolerance_constants`. Marked all surface option structs `#[non_exhaustive]`. Deduplicated deprecated constructors by delegating to `try_*` counterparts.
- **Curve-curve intersection (Phase 17)** -- New `monstertruck-geometry::nurbs::curve_intersect` module with `find_intersections` and `find_self_intersections` APIs using subdivision + Newton-Raphson refinement.
- **Gordon surface from network (Phase 18)** -- `try_gordon_from_network` auto-computes grid points from curve families. `try_gordon_verified` validates caller-supplied grid points with snap-distance control via `GordonOptions.grid_tolerance`.
- **Tessellation robustness (Phase 19)** -- Centralized tessellation tolerances. `PolyBoundaryPiece::try_new_with_fallback` prevents face drops via UV interpolation fallback.
- **Fixture corpus and migration documentation (Phase 20)** -- Expanded test corpus with 7 pathological geometry fixtures (inflection rails, converging rails, degenerate sections, near-zero Jacobian/weight, collapsed control points, weight spikes) and 6 Gordon-specific network fixtures (near-miss grid points, nonuniform spacing, high-degree curves). Added migration doc comments on all 7 `try_*` surface constructor functions with before/after examples and a crate-level migration guide with quick reference table. 25+ new integration and smoke tests.

### Earlier Changes (Phase 15 -- Font Stress Testing & Performance)

- **`try_gordon_from_network`** -- New constructor on `BsplineSurface` that auto-computes intersection grid points from u/v curve families using the curve intersection engine, eliminating the need for callers to supply grid points manually.
- **`try_gordon_verified`** -- Validated Gordon surface constructor that checks caller-supplied grid points against the input curves, snapping near-miss points within `GordonOptions.grid_tolerance`.
- **`GordonOptions.grid_tolerance`** -- New field controlling the snap distance for grid point validation.
- **New diagnostics** -- `CurveNetworkDiagnostic::IntersectionCountMismatch` and `GridPointNotOnCurve` variants for detailed error reporting.
- **Builder wrappers** -- `builder::try_gordon_from_network` and `builder::try_gordon_verified` in `monstertruck-modeling` with topology assembly. 19 tests total covering crossing networks, near-miss snapping, nonuniform spacing, and error cases.

### Earlier Changes (Phase 17 -- Curve-Curve Intersection)

- **Curve intersection module** -- New `monstertruck-geometry::nurbs::curve_intersect` module implementing curve-curve and self-intersection detection via subdivision + Newton-Raphson refinement. Public API: `find_intersections(curve_a, curve_b)` and `find_self_intersections(curve)`, returning `CurveIntersection` results with parameter values (`t0`, `t1`) and intersection `point`. Uses `SNAP_TOLERANCE` from centralized constants. 12 tests cover crossing, tangent, multiple intersections, self-intersection, parallel curves, and degenerate cases.

### Earlier Changes (Phase 16 -- Tolerance Centralization & API Hardening)

- **Centralized tolerance constants** -- New `monstertruck-core::tolerance_constants` module consolidates 6 numeric tolerance constants (`SNAP_TOLERANCE`, `VERTEX_MERGE_TOLERANCE`, `TESSELLATION_TOLERANCE`, `PERIODIC_CLOSURE_RATIO`, `G1_ANGLE_TOLERANCE`, `G2_CURVATURE_TOLERANCE`) previously scattered as magic numbers across `monstertruck-solid`. All call sites now import from the central module.
- **`#[non_exhaustive]` option structs** -- All 5 surface constructor option structs (`GordonOptions`, `SkinOptions`, `SweepRailOptions`, `Birail1Options`, `Birail2Options`) in `monstertruck-geometry` are now marked `#[non_exhaustive]`, allowing future field additions without breaking downstream crate builds.
- **Deprecated constructor deduplication** -- The 5 deprecated surface constructors (`gordon`, `skin`, `sweep_rail`, `birail1`, `birail2`) now delegate to their `try_*` counterparts, removing ~225 lines of duplicated logic.

### Earlier Changes (Phases 9--15)

<details>
<summary>Click to expand earlier changelog</summary>

#### Phase 15 -- Font Stress Testing & Performance
- Font stress corpus with 11 pathological geometry fixtures and 16 regression tests.
- Criterion benchmark suite for profile pipeline throughput.

#### Phase 14 -- Profile Solid Pipeline
- `revolve_from_planar_profile` and `sweep_from_planar_profile` in `monstertruck-modeling`.
- `mixed_profile_face` for combining font glyph wires with sketch loops.
- `validate_solid` with `ValidationReport` for Euler-Poincare and tessellation checks.

#### Phase 13 -- v0.5.0 API Polish & Surface Operations
- Typed option structs (`SweepRailOptions`, `Birail1Options`, `Birail2Options`, `GordonOptions`, `SkinOptions`) and `CurveNetworkDiagnostic` error reporting.
- Fallible `try_*` surface constructors returning `Result` instead of panicking.
- Patch split/extract: `split_at_u`, `split_at_v`, `sub_patch` on `BsplineSurface` and `NurbsSurface`.

#### Phase 12 -- v0.4.0 Milestone Completion
- 11 end-to-end font pipeline integration tests.
- Ayam port plan finalized with deferred items annotated.

#### Phase 11 -- Surface Constructors
- `sweep_multi_rail` and `sweep_periodic` surface constructors.
- 5 typed builder wrappers in `monstertruck-modeling::builder`.

#### Phase 10 -- NURBS Fixture Corpus & Surface Healing
- 10 reusable NURBS fixtures covering degenerate surfaces.
- `heal_surface_shell` for detecting and repairing degenerate NURBS surfaces.

#### Phase 9 -- Boolean Repair & Tolerance Foundation
- Centralized tolerance policy in `monstertruck-core::tolerance`.
- Boolean face classification hardening, 3-stage shell healing, diagnostic logging.
- Boolean shell welding fix and `Matrix4::from_translation` correction.

</details>

### Earlier Changes (Phase 8)

- **Topology validation module** -- New `monstertruck-solid::fillet::validate` module with Euler-Poincare characteristic checks and face orientation validation. Debug assertions (`debug_assert_topology`, `debug_assert_euler`) run automatically after fillet modifications in debug builds with zero runtime cost in release builds.
- **Fillet implementation plan v0.3.0** -- Updated test inventory, API surface documentation, and known-limitations tracking.

### Earlier Changes (Phase 7)

- **Fillet integration mode** -- New `FilletMode` enum (`KeepSeparateFace` / `IntegrateVisual`) in `FilletOptions` controls whether fillet surfaces remain distinct faces or are merged into adjacent geometry. `IntegrateVisual` produces seamless results with G1/G2 continuity annotations.
- **Continuity annotations** -- `FilletResult` now carries per-edge continuity classifications. The new `fillet_annotated()` API and helpers (`annotate_fillet_edges`, `classify_edge_continuity`, `ensure_seamless_vertices`) let downstream consumers query join quality.
- **Extended fillet options** -- `FilletOptions` gained `mode`, `extend_mode`, and `corner_mode` fields, all backward-compatible via `Default`.

### Earlier Changes (Phase 6)

- **Fillet seam averaging fix** -- `fillet_along_wire` now dehomogenizes `Vector4` control points before averaging at seam boundaries, producing correct C0-continuous joins between fillet patches (TOPO-02).
- **Intersection curve edge handling** -- New `ensure_cuttable_edge()` converts `IntersectionCurve` boundary edges to NURBS approximations before `cut_face_by_bezier` runs, preventing topology surgery failures on boolean-result geometry (TOPO-01).

### Earlier Changes (Phase 5 -- Polynomial Solvers)

- **Polynomial solvers** -- `monstertruck-math` now includes quadratic, cubic, and quartic polynomial solvers (ported from `matext4cgmath`), eliminating the legacy `cgmath` transitive dependency. These solvers are wired into `monstertruck-geometry` curve intersection call sites (hyperbola and parabola specifieds).
- **Namespace collision fix** -- Resolved the `polynomial` module name collision between `monstertruck-math` (solvers) and `monstertruck-traits` (evaluation traits) via explicit re-export precedence in the geometry crate.

### Earlier Changes (Phase 4)

- **nalgebra math backend** -- The new `monstertruck-math` crate provides a nalgebra-backed linear algebra layer. `monstertruck-core` now uses nalgebra internally; a `cgmath64` compatibility shim is available for downstream crates that have not yet migrated.
- **RwLock concurrency** -- Topology types (`Vertex`, `Edge`, `Face`) in `monstertruck-topology` have been migrated from `parking_lot::Mutex` to `parking_lot::RwLock`, reducing contention for concurrent read-heavy workloads.
- **GPU compute tessellation** -- `monstertruck-gpu` includes a prototype `GpuTessellator` that evaluates NURBS surfaces on the GPU via a WGSL compute shader and WebGPU, with integration tests and GPU-vs-CPU benchmarks.
- **T-spline validation fixes** -- `monstertruck-geometry` gained zero-knot-interval support and connection-parity verification for T-spline/T-NURCc meshes, along with 8 new validation tests.

## Dependency Graph

![dependencies](./dependencies.svg)

## License

Apache License 2.0
