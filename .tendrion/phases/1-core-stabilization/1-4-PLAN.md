---
phase: 1-core-stabilization
plan: 4
type: execute
wave: 2
depends_on: ["1-1"]
files_modified:
  - Cargo.toml
  - monstertruck-geometry/Cargo.toml
  - monstertruck-geometry/benches/nurbs_eval.rs
  - monstertruck-meshing/Cargo.toml
  - monstertruck-meshing/benches/tessellation.rs
  - monstertruck-solid/Cargo.toml
  - monstertruck-solid/benches/boolean_ops.rs
  - .github/workflows/ci.yml
autonomous: true
must_haves:
  truths:
    - "Running cargo test --benches -p monstertruck-geometry compiles and verifies NURBS evaluation benchmarks"
    - "Running cargo test --benches -p monstertruck-meshing compiles and verifies tessellation benchmarks"
    - "Running cargo test --benches -p monstertruck-solid compiles and verifies boolean operation benchmarks"
    - "Benchmark results can be obtained via cargo bench for local regression detection"
    - "CI workflow includes a benchmark compilation check step"
  artifacts:
    - path: "monstertruck-geometry/benches/nurbs_eval.rs"
      provides: "Criterion benchmarks for NURBS curve and surface evaluation"
      min_lines: 30
      contains: "criterion_group"
    - path: "monstertruck-meshing/benches/tessellation.rs"
      provides: "Criterion benchmarks for mesh tessellation"
      min_lines: 30
      contains: "criterion_group"
    - path: "monstertruck-solid/benches/boolean_ops.rs"
      provides: "Criterion benchmarks for boolean operations"
      min_lines: 30
      contains: "criterion_group"
    - path: ".github/workflows/ci.yml"
      provides: "CI step that compiles benchmarks to detect regressions"
      min_lines: 5
      contains: "cargo test --benches"
  key_links:
    - from: "monstertruck-geometry/Cargo.toml"
      to: "Cargo.toml"
      via: "workspace criterion dependency"
      pattern: "criterion"
    - from: ".github/workflows/ci.yml"
      to: "monstertruck-geometry/benches/nurbs_eval.rs"
      via: "CI compiles benchmark targets"
      pattern: "cargo test --benches"
---

<objective>
Add criterion-based benchmarking infrastructure to three core crates (monstertruck-geometry, monstertruck-meshing, monstertruck-solid) with representative benchmarks for NURBS evaluation, tessellation, and boolean operations, plus CI integration for regression detection.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@Cargo.toml
@monstertruck-geometry/Cargo.toml
@monstertruck-meshing/Cargo.toml
@monstertruck-solid/Cargo.toml
@AGENTS.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add criterion workspace dependency and geometry benchmarks</name>
  <files>Cargo.toml, monstertruck-geometry/Cargo.toml, monstertruck-geometry/benches/nurbs_eval.rs</files>
  <action>
**Step 1: Add criterion to workspace dependencies in root Cargo.toml:**
Add under `[workspace.dependencies]`:
```toml
criterion = { version = "0.5", features = ["html_reports"] }
```

**Step 2: Add dev-dependency and bench target to monstertruck-geometry/Cargo.toml:**
Add:
```toml
[dev-dependencies]
criterion = { workspace = true }

[[bench]]
name = "nurbs_eval"
harness = false
```

**Step 3: Create monstertruck-geometry/benches/nurbs_eval.rs:**
Write benchmarks covering the three key NURBS evaluation hotpaths:

1. **BsplineCurve point evaluation** (`subs`): Create a degree-3 B-spline curve with 20+ control points, benchmark `subs(t)` at various parameter values.
2. **NurbsSurface point evaluation** (`subs`): Create a degree-3x3 NURBS surface with a moderate control point grid (e.g., 8x8), benchmark `subs(u, v)`.
3. **BsplineCurve derivative evaluation** (`der`): Benchmark first derivative computation on the same curve.

IMPORTANT: Use `KnotVector` (not `KnotVec` which is deprecated). Check `monstertruck-geometry/src/nurbs/mod.rs` line 152-154 for confirmation.

Use `monstertruck_geometry::prelude::*` for imports. Construct test geometry programmatically.

Example structure:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use monstertruck_geometry::prelude::*;

fn bench_bspline_curve_eval(c: &mut Criterion) {
    let n = 20usize;
    let degree = 3usize;
    let points: Vec<Point3> = (0..n).map(|i| {
        let t = i as f64 / (n - 1) as f64;
        Point3::new(t.cos(), t.sin(), t * 0.5)
    }).collect();
    let knots: Vec<f64> = std::iter::repeat_n(0.0, degree + 1)
        .chain((1..n - degree).map(|i| i as f64 / (n - degree) as f64))
        .chain(std::iter::repeat_n(1.0, degree + 1))
        .collect();
    let knot_vec = KnotVector::from(knots);
    let curve = BsplineCurve::new(knot_vec, points);

    c.bench_function("bspline_curve_subs", |b| {
        b.iter(|| black_box(curve.subs(black_box(0.5))))
    });
}

fn bench_bspline_curve_der(c: &mut Criterion) {
    // Same curve as above
    let n = 20usize;
    let degree = 3usize;
    let points: Vec<Point3> = (0..n).map(|i| {
        let t = i as f64 / (n - 1) as f64;
        Point3::new(t.cos(), t.sin(), t * 0.5)
    }).collect();
    let knots: Vec<f64> = std::iter::repeat_n(0.0, degree + 1)
        .chain((1..n - degree).map(|i| i as f64 / (n - degree) as f64))
        .chain(std::iter::repeat_n(1.0, degree + 1))
        .collect();
    let knot_vec = KnotVector::from(knots);
    let curve = BsplineCurve::new(knot_vec, points);

    c.bench_function("bspline_curve_der", |b| {
        b.iter(|| black_box(curve.der(black_box(0.5))))
    });
}

fn bench_nurbs_surface_eval(c: &mut Criterion) {
    // Create an 8x8 grid of control points for a degree 3x3 surface
    let n = 8usize;
    let degree = 3usize;
    // ... construct surface ...

    c.bench_function("nurbs_surface_subs", |b| {
        b.iter(|| black_box(surface.subs(black_box(0.5), black_box(0.5))))
    });
}

criterion_group!(benches, bench_bspline_curve_eval, bench_bspline_curve_der, bench_nurbs_surface_eval);
criterion_main!(benches);
```

Check the actual API by examining the prelude exports. The trait methods (`subs`, `der`) come from `ParametricCurve` trait. Verify correct method names.
  </action>
  <verify>Run `cargo test --benches -p monstertruck-geometry` to verify benchmarks compile and their test mode executes. Run `cargo clippy --all-targets -- -W warnings` to ensure no warnings.</verify>
  <done>Criterion dependency added to workspace. NURBS evaluation benchmarks created for monstertruck-geometry with curve eval, surface eval, and derivative benchmarks. Uses KnotVector (not deprecated KnotVec).</done>
</task>

<task type="auto">
  <name>Task 2: Add tessellation and boolean operation benchmarks</name>
  <files>monstertruck-meshing/Cargo.toml, monstertruck-meshing/benches/tessellation.rs, monstertruck-solid/Cargo.toml, monstertruck-solid/benches/boolean_ops.rs</files>
  <action>
**Tessellation benchmarks (monstertruck-meshing):**

Add to monstertruck-meshing/Cargo.toml:
```toml
[dev-dependencies]
criterion = { workspace = true }

[[bench]]
name = "tessellation"
harness = false
```

Create monstertruck-meshing/benches/tessellation.rs with benchmarks for:
1. **Polygon mesh tessellation**: Create a simple mesh programmatically, tessellate it, and measure time.
2. If test fixture JSON files are available (check for `resources/shape/*.json` files), deserialize a solid and benchmark its tessellation.

Check what public APIs are available in monstertruck-meshing for tessellation. Look at `monstertruck-meshing/src/tessellation/mod.rs` for the public API. The crate likely provides trait methods like `triangulation()` or `robust_triangulation()`.

Note: tessellation benchmarks will need to construct or deserialize geometry. Check if monstertruck-meshing re-exports topology types or if you need to add monstertruck-topology as a dev-dependency.

**Boolean operation benchmarks (monstertruck-solid):**

Add to monstertruck-solid/Cargo.toml:
```toml
[dev-dependencies]
criterion = { workspace = true }

[[bench]]
name = "boolean_ops"
harness = false
```

Create monstertruck-solid/benches/boolean_ops.rs with benchmarks for:
1. **Solid intersection**: Construct two overlapping box-like solids and benchmark their intersection.
2. **Solid union**: Same solids, benchmark union.

Check `monstertruck-solid/src/lib.rs` for the public boolean operation API. The `and()` and `or()` functions are in the `transversal::integrate` module and are re-exported.

For both crates: if constructing realistic test geometry is prohibitively complex for a benchmark harness, create simpler but representative geometry. The key is having the infrastructure in place -- the benchmarks can be enriched later with more complex geometry.

IMPORTANT: Use `KnotVector` (not deprecated `KnotVec`) in all benchmark code.

Both benchmark files should follow the standard criterion pattern:
```rust
use criterion::{criterion_group, criterion_main, Criterion};
// imports...

fn bench_function(c: &mut Criterion) { /* ... */ }

criterion_group!(benches, bench_function);
criterion_main!(benches);
```

Ensure all added dev-dependencies also include any needed internal crates (e.g., monstertruck-modeling for constructing test solids).
  </action>
  <verify>Run `cargo test --benches -p monstertruck-meshing` and `cargo test --benches -p monstertruck-solid` to verify compilation and basic execution. Run `cargo clippy --all-targets -- -W warnings` to ensure no warnings.</verify>
  <done>Tessellation and boolean operation benchmarks created with criterion infrastructure. All three crates have runnable benchmarks for performance regression detection.</done>
</task>

<task type="auto">
  <name>Task 3: Add CI workflow step for benchmark compilation</name>
  <files>.gitlab-ci.yml</files>
  <action>
Add a CI step that compiles and test-runs benchmarks to detect compilation regressions. This does NOT run full benchmarks (which would be slow), but verifies benchmarks compile and can execute in test mode.

**Step 1:** Check if `.gitlab-ci.yml` exists. The repo uses GitLab CI, NOT GitHub Actions.

**Step 2:** Add a stage/job to `.gitlab-ci.yml` that runs:
```yaml
bench-check:
  stage: test
  script:
    - cargo test --benches
```

If `.gitlab-ci.yml` does not exist, create a minimal one with the benchmark check job.

**Verification commands per repo policy:** Use `cargo test --benches` (not `cargo bench`) for CI. For local regression detection, developers can use `cargo bench` manually. The CI step only verifies compilation, not performance.
  </action>
  <verify>Verify the CI file exists and contains a `cargo test --benches` step. Verify YAML is well-formed.</verify>
  <done>GitLab CI updated with benchmark compilation check step. Benchmark regressions (compilation failures, API breaks) will be caught in CI.</done>
</task>

</tasks>

<verification>
1. `cargo test --benches -p monstertruck-geometry` compiles and runs
2. `cargo test --benches -p monstertruck-meshing` compiles and runs
3. `cargo test --benches -p monstertruck-solid` compiles and runs
4. `cargo clippy --all-targets -- -W warnings` produces no warnings
5. `criterion` appears in workspace dependencies in root Cargo.toml
6. Each benchmark crate has a `[[bench]]` section in Cargo.toml with `harness = false`
7. Benchmark files exist at the expected paths with `criterion_group!` and `criterion_main!` macros
8. CI workflow contains a `cargo test --benches` step
9. No deprecated `KnotVec` usage in benchmark code
</verification>

<success_criteria>
- Criterion benchmarking infrastructure is in place across three core crates
- NURBS evaluation, tessellation, and boolean operations each have at least one benchmark
- `cargo bench` can be used locally to detect performance regressions
- CI catches benchmark compilation failures via `cargo test --benches`
- Benchmarks use `KnotVector` (not deprecated `KnotVec`)
- All benchmarks are representative of real workloads (not trivial no-ops)
</success_criteria>

<output>
After completion, create `.tendrion/phases/1-core-stabilization/1-4-SUMMARY.md`
</output>
