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

### Recent Changes (Phase 9 -- Boolean Repair & Tolerance Foundation)

- **Tolerance policy** -- `monstertruck-core::tolerance` now has comprehensive documentation of the numeric tolerance policy (TOLERANCE, TOLERANCE2, Tolerance trait, OperationTolerance). The hardcoded `1.0e-6` in fillet `edge_select` was replaced with the canonical `TOLERANCE` constant. Regression tests pin tolerance values.
- **Boolean face classification hardening** -- `faces_classification::integrate_by_component` uses majority-edge scoring with `FxHashSet` and no longer panics on empty boundary components. Unknown-face classification falls back to a conservative default instead of returning an error.
- **3-stage shell healing** -- Boolean `heal_shell_if_needed` now uses a 3-stage fallback (healed, unhealed, original) and never returns `None` for non-empty shells.
- **Diagnostic logging** -- Coincident-face detection (`MT_BOOL_DEBUG_COINCIDENT`), healing diagnostics (`MT_BOOL_DEBUG_HEAL`), and dropped-boundary logging (`MT_BOOL_DEBUG_COMPONENTS`) are available as opt-in environment variables.
- **Tolerance documentation in boolean pipeline** -- Doc comments in `integrate/mod.rs` and `loops_store/mod.rs` explain tolerance multipliers (operation tolerance floor, triangulation tolerance, snap tolerance 10x, vertex merge 100x).
- **Topology validation tests** -- New tests for AND, OR, difference, and chained boolean operations with closed-shell and singular-vertex assertions.
- **Explicit tolerance imports in meshing** -- `monstertruck-meshing` now imports tolerance constants from `monstertruck-core` instead of using local magic numbers, aligning tessellation with the project-wide tolerance policy.
- **Boolean shell welding fix** -- `weld_compressed_shell` corrected to handle degenerate weld cases; coincident face detection no longer self-compares faces, eliminating false positives.
- **Matrix4 translation fix** -- `Matrix4::from_translation` in `monstertruck-math` corrected column placement of translation components.

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
