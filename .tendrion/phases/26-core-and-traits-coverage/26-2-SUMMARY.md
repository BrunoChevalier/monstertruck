---
phase: 26-core-and-traits-coverage
plan: 2
tags: [testing, traits, coverage]
key-files:
  - monstertruck-traits/tests/curve_traits.rs
  - monstertruck-traits/tests/surface_traits.rs
  - monstertruck-traits/tests/search_parameter_tests.rs
  - monstertruck-traits/tests/invertible_transformed.rs
decisions: []
metrics:
  tests_added: 80
  tests_total: 94
  test_files_created: 4
---

## What Was Built

Four test files covering all public trait methods in `monstertruck-traits`:

- **monstertruck-traits/tests/curve_traits.rs** (24 tests): `ParametricCurve` methods (evaluate, derivative, derivative_2, derivative_n, derivatives, deprecated aliases), `BoundedCurve` (range_tuple, front, back), reference/Box delegation, `CurveCollector` (is_singleton, into Option), `ConcatError` (Display formatting, point_map).

- **monstertruck-traits/tests/surface_traits.rs** (27 tests): `ParametricSurface` methods (evaluate, derivative_u/v/uu/uv/vv, derivative_mn, derivatives, deprecated aliases, parameter_range, try_range_tuple, u_period, v_period), `ParametricSurface3D` (normal perpendicularity, unit length, normal_uder/normal_vder finite difference), `BoundedSurface` (range_tuple), reference/Box delegation.

- **monstertruck-traits/tests/search_parameter_tests.rs** (16 tests): `D1::DIM`, `D2::DIM`, `SearchParameterHint1D` (Parameter/Range/None round-trips, From impls), `SearchParameterHint2D` (Parameter/Range/None round-trips, From impls), PartialEq, Debug.

- **monstertruck-traits/tests/invertible_transformed.rs** (13 tests): `Invertible` for `(usize,usize)`, `Vec<P>`, `Box<T>` (invert/inverse), `Transformed` for Point3/Matrix4, Point3/Matrix3, Point2/Matrix3, Box<Point3>/Matrix4 (transform_by/transformed), `ToSameGeometry` compile-time check.

## Verification

- `cargo nextest run -p monstertruck-traits --features polynomial` -- 93 tests passed
- `cargo nextest run -p monstertruck-traits --features polynomial,derive` -- 94 tests passed
- `cargo nextest run -p monstertruck-core -p monstertruck-traits --features polynomial` -- 277 tests passed
- No warnings in any test file.

## Deviations

- Tests pass immediately in RED phase because they cover existing trait implementations (not new code). Logged as auto-fix deviation.
