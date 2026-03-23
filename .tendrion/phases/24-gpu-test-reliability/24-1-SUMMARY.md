---
phase: 24-gpu-test-reliability
plan: 1
tags: [gpu, camera, proptest, degenerate-inputs, projection-matrix]
key-files:
  - monstertruck-gpu/src/camera.rs
  - monstertruck-gpu/tests/camera.rs
  - monstertruck-math/src/lib.rs
decisions:
  - "Fixed transposed ortho/perspective/frustum matrices in monstertruck-math (root cause of proptest failures)"
  - "Added epsilon guards for degenerate zero-span bounding boxes in camera fitting functions"
metrics:
  tests-added: 4
  tests-total: 6
  proptest-cases-verified: 1000
  deviations: 2
---

## What was built

### Files modified

- **monstertruck-gpu/tests/camera.rs**: Added 4 dedicated unit tests for degenerate point cloud edge cases (all-zero and all-identical points) for both `parallel_view_fitting` and `perspective_view_fitting`.

- **monstertruck-gpu/src/camera.rs**: Added epsilon guards in `parallel_view_fitting` (lines 373-381) and `perspective_view_fitting` (lines 460-481) to handle degenerate point clouds where the bounding box has zero span in one or more dimensions. The epsilon is scale-relative (1e-10 * max(distance_from_origin, 1.0)) to avoid affecting non-degenerate inputs.

- **monstertruck-math/src/lib.rs**: Fixed transposed `ortho()`, `perspective()`, and `frustum()` projection matrix functions. These functions were passing values to `Matrix4::new` in row-major order, but `Matrix4::new` expects column-major order. The translation/clip components were in row 3 instead of column 3, causing incorrect w-component computation during `transform_point` perspective divide. This was the root cause of proptest failures for all input distributions (not just degenerate ones).

## Deviations from plan

1. **Pre-existing build errors** (auto-fix, bug): `monstertruck-gpu/tests/msaa.rs` and `monstertruck-gpu/tests/bindgroup.rs` have compilation errors unrelated to camera code. Used `--test camera` flag to isolate camera tests.

2. **Transposed projection matrices** (auto-fix, bug): The `ortho()`, `perspective()`, and `frustum()` functions in `monstertruck-math/src/lib.rs` produced transposed matrices due to passing arguments in row-major order to the column-major `Matrix4::new`. This caused `transform_point()` to compute incorrect w values, breaking all proptest assertions (not just degenerate cases). Fixed by reordering arguments to correct column-major layout.

## Verification

- `cargo nextest run -p monstertruck-gpu --test camera` -- 6/6 pass (4 unit + 2 proptest)
- `PROPTEST_CASES=1000 cargo nextest run -p monstertruck-gpu --test camera` -- 6/6 pass (2 consecutive runs)
- `cargo nextest run -p monstertruck-math` -- 42/42 pass
- `cargo nextest run -p monstertruck-render --test render` -- 2/2 pass
- `cargo nextest run -p monstertruck-core -p monstertruck-math -p monstertruck-geometry` -- 410/410 pass
- `cargo clippy -p monstertruck-gpu --tests -p monstertruck-math` -- clean (no new warnings)
- All existing `prop_assume!` guards remain unchanged; no new exclusion filters added
