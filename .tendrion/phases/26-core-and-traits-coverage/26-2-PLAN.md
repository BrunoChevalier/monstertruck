---
phase: 26-core-and-traits-coverage
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-traits/tests/curve_traits.rs
  - monstertruck-traits/tests/surface_traits.rs
  - monstertruck-traits/tests/search_parameter_tests.rs
  - monstertruck-traits/tests/invertible_transformed.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo nextest run -p monstertruck-traits --features polynomial and all new tests pass green"
    - "Every public trait method in ParametricCurve has at least one test exercising it"
    - "Every public trait method in ParametricSurface has at least one test exercising it"
    - "ParametricSurface3D normal, normal_uder, and normal_vder are tested"
    - "BoundedCurve front/back/range_tuple are tested"
    - "BoundedSurface range_tuple is tested"
    - "SearchParameter and SearchNearestParameter hint types are tested"
  artifacts:
    - path: "monstertruck-traits/tests/curve_traits.rs"
      provides: "Tests for ParametricCurve, BoundedCurve, Cut, CurveCollector, ConcatError, ParameterDivision1D"
      min_lines: 120
      contains: "ParametricCurve"
    - path: "monstertruck-traits/tests/surface_traits.rs"
      provides: "Tests for ParametricSurface, ParametricSurface3D, BoundedSurface, ParameterDivision2D"
      min_lines: 100
      contains: "ParametricSurface"
    - path: "monstertruck-traits/tests/search_parameter_tests.rs"
      provides: "Tests for SearchParameterHint1D, SearchParameterHint2D, D1, D2 dimension types"
      min_lines: 50
      contains: "SearchParameterHint"
    - path: "monstertruck-traits/tests/invertible_transformed.rs"
      provides: "Tests for Invertible and Transformed traits"
      min_lines: 40
      contains: "Invertible"
  key_links:
    - from: "monstertruck-traits/tests/curve_traits.rs"
      to: "monstertruck-traits/src/traits/curve.rs"
      via: "imports ParametricCurve, BoundedCurve, Cut, ConcatError, CurveCollector"
      pattern: "use monstertruck_traits"
    - from: "monstertruck-traits/tests/surface_traits.rs"
      to: "monstertruck-traits/src/traits/surface.rs"
      via: "imports ParametricSurface, ParametricSurface3D, BoundedSurface"
      pattern: "use monstertruck_traits"
    - from: "monstertruck-traits/tests/search_parameter_tests.rs"
      to: "monstertruck-traits/src/traits/search_parameter.rs"
      via: "imports SearchParameterHint1D, SearchParameterHint2D"
      pattern: "SearchParameterHint"
---

<objective>
Add tests for monstertruck-traits (currently at 0% coverage) covering curve and surface trait implementations, ensuring at least one test per public trait method for ParametricCurve, ParametricSurface, BoundedCurve, BoundedSurface, ParametricSurface3D, Invertible, Transformed, and SearchParameter hint types. Use the polynomial feature's PolynomialCurve and PolynomialSurface as concrete implementations.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-traits/src/traits/curve.rs
@monstertruck-traits/src/traits/surface.rs
@monstertruck-traits/src/traits/search_parameter.rs
@monstertruck-traits/src/traits/mod.rs
@monstertruck-traits/src/polynomial.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: ParametricCurve and BoundedCurve trait method tests</name>
  <files>monstertruck-traits/tests/curve_traits.rs</files>
  <action>
Create `monstertruck-traits/tests/curve_traits.rs` with `#![cfg(feature = "polynomial")]` gate. Use `PolynomialCurve` as the concrete implementation. Test every public trait method:

**ParametricCurve methods:**
1. `evaluate(t)` - verify against manual polynomial evaluation for several t values
2. `derivative(t)` - verify first derivative matches manual calculation
3. `derivative_2(t)` - verify second derivative matches manual calculation
4. `derivative_n(n, t)` - test for n=0,1,2,3 with known polynomial
5. `derivatives(n, t)` - verify returns CurveDerivatives with correct max_order and values
6. `subs(t)` - verify deprecated alias returns same as evaluate
7. `der(t)` - verify deprecated alias returns same as derivative
8. `der2(t)` - verify deprecated alias returns same as derivative_2
9. `der_n(n, t)` - verify deprecated alias
10. `ders(n, t)` - verify deprecated alias
11. `parameter_range()` - verify PolynomialCurve returns Included(-100, 100)
12. `try_range_tuple()` - verify returns Some((-100.0, 100.0))
13. `period()` - verify returns None for non-periodic curve

**BoundedCurve methods:**
14. `range_tuple()` - verify returns (-100.0, 100.0)
15. `front()` - verify evaluates at range start
16. `back()` - verify evaluates at range end

**ParametricCurve for &C delegation:**
17. Verify all methods work through a reference

**ParametricCurve for Box<C> delegation:**
18. Verify evaluate and derivative work through Box

**Cut trait tests (using (usize, usize) implementation):**
19. `cut(t)` - via the existing test utility `cut_random_test` or direct test

**CurveCollector tests:**
20. `CurveCollector::Singleton` is_singleton returns true
21. `CurveCollector::Curve(c)` is_singleton returns false
22. `From<CurveCollector<C>> for Option<C>` conversion

**ConcatError tests:**
23. `ConcatError::DisconnectedParameters` Display formatting
24. `ConcatError::DisconnectedPoints` Display formatting
25. `ConcatError::point_map` transforms points correctly

Use a simple polynomial: `P(t) = (t^2 + t + 1, 2t - 1)` as Point2 curve for most tests.

```rust
#![cfg(feature = "polynomial")]

use monstertruck_core::cgmath64::*;
use monstertruck_core::tolerance::*;
use monstertruck_traits::polynomial::PolynomialCurve;
use monstertruck_traits::*;
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-traits --features polynomial --test curve_traits` and verify all tests pass.</verify>
  <done>ParametricCurve and BoundedCurve trait method tests created and passing with at least one test per public method.</done>
</task>

<task type="auto">
  <name>Task 2: ParametricSurface and ParametricSurface3D trait method tests</name>
  <files>monstertruck-traits/tests/surface_traits.rs</files>
  <action>
Create `monstertruck-traits/tests/surface_traits.rs` with `#![cfg(feature = "polynomial")]` gate. Use `PolynomialSurface` as the concrete implementation. Test every public trait method:

**ParametricSurface methods:**
1. `evaluate(u, v)` - verify against manual tensor-product polynomial evaluation
2. `derivative_u(u, v)` - verify partial derivative w.r.t. u
3. `derivative_v(u, v)` - verify partial derivative w.r.t. v
4. `derivative_uu(u, v)` - verify second partial
5. `derivative_uv(u, v)` - verify mixed partial
6. `derivative_vv(u, v)` - verify second partial
7. `derivative_mn(m, n, u, v)` - test for (0,0), (1,0), (0,1), (1,1), (2,0), (0,2)
8. `derivatives(max_order, u, v)` - verify returns SurfaceDerivatives with correct values
9. `subs(u, v)` - deprecated alias matches evaluate
10. `uder(u, v)` - deprecated alias
11. `vder(u, v)` - deprecated alias
12. `uuder(u, v)` - deprecated alias
13. `uvder(u, v)` - deprecated alias
14. `vvder(u, v)` - deprecated alias
15. `der_mn(m, n, u, v)` - deprecated alias
16. `ders(max_order, u, v)` - deprecated alias
17. `parameter_range()` - verify returns appropriate range
18. `try_range_tuple()` - verify returns Some for bounded surface
19. `u_period()` - verify returns None
20. `v_period()` - verify returns None

**ParametricSurface3D methods:**
21. `normal(u, v)` - verify normal is perpendicular to both uder and vder
22. `normal(u, v)` - verify normal is unit length
23. `normal_uder(u, v)` - verify against finite difference approximation
24. `normal_vder(u, v)` - verify against finite difference approximation

**BoundedSurface methods:**
25. `range_tuple()` - verify returns bounded ranges

**Delegation tests:**
26. `&S` reference delegation works for evaluate
27. `Box<S>` delegation works for evaluate

Use a tensor product surface from two simple polynomials. Example:
```rust
let coef0 = vec![Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0)];
let coef1 = vec![Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0)];
let curve0 = PolynomialCurve::<Point3>(coef0);
let curve1 = PolynomialCurve::<Point3>(coef1);
let surface = PolynomialSurface::by_tensor(curve0, curve1);
```

```rust
#![cfg(feature = "polynomial")]

use monstertruck_core::cgmath64::*;
use monstertruck_core::tolerance::*;
use monstertruck_traits::polynomial::{PolynomialCurve, PolynomialSurface};
use monstertruck_traits::*;
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-traits --features polynomial --test surface_traits` and verify all tests pass.</verify>
  <done>ParametricSurface and ParametricSurface3D trait method tests created and passing with at least one test per public method.</done>
</task>

<task type="auto">
  <name>Task 3: SearchParameter hints and Invertible/Transformed trait tests</name>
  <files>monstertruck-traits/tests/search_parameter_tests.rs, monstertruck-traits/tests/invertible_transformed.rs</files>
  <action>
Create two test files:

**monstertruck-traits/tests/search_parameter_tests.rs:**
Test the search parameter hint types and dimension types:

1. `D1::DIM` equals 1
2. `D2::DIM` equals 2
3. `SearchParameterHint1D::Parameter(x)` round-trips from f64
4. `SearchParameterHint1D::Range(a, b)` round-trips from (f64, f64)
5. `SearchParameterHint1D::None` from Option::<f64>::None
6. `From<f64> for SearchParameterHint1D` creates Parameter variant
7. `From<(f64, f64)> for SearchParameterHint1D` creates Range variant
8. `From<Option<f64>> for SearchParameterHint1D` creates Parameter from Some, None from None
9. `SearchParameterHint2D::Parameter(u, v)` round-trips from (f64, f64)
10. `SearchParameterHint2D::Range(ur, vr)` round-trips from ((f64,f64),(f64,f64))
11. `SearchParameterHint2D::None` from Option::<(f64,f64)>::None
12. `From<(f64, f64)> for SearchParameterHint2D` creates Parameter variant
13. `From<((f64,f64),(f64,f64))> for SearchParameterHint2D` creates Range variant
14. `From<Option<(f64,f64)>> for SearchParameterHint2D` creates Parameter from Some
15. PartialEq works for hints (assert_eq between two same-valued hints)
16. Debug is implemented (format!("{:?}", hint) doesn't panic)

```rust
use monstertruck_traits::*;
```

**monstertruck-traits/tests/invertible_transformed.rs:**
Test Invertible and Transformed traits using existing implementations:

1. `Invertible for (usize, usize)`: invert swaps elements
2. `Invertible for (usize, usize)`: inverse returns swapped without mutating original
3. `Invertible for Vec<P>`: invert reverses the vec
4. `Invertible for Vec<P>`: inverse returns reversed without mutating original
5. `Invertible for Box<T>`: delegated invert works
6. `Invertible for Box<T>`: delegated inverse works
7. `Transformed for Point3 with Matrix4`: transform_by applies matrix
8. `Transformed for Point3 with Matrix4`: transformed returns new point
9. `Transformed for Point3 with Matrix3`: works for 3x3 matrix
10. `Transformed for Point2 with Matrix3`: works for 2D transform
11. `Transformed for Box<Point3>`: delegation works
12. `ToSameGeometry` trait existence (compile-time check)

Use imports:
```rust
use monstertruck_core::cgmath64::*;
use monstertruck_traits::*;
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-traits --features polynomial --test search_parameter_tests --test invertible_transformed` and verify all tests pass.</verify>
  <done>SearchParameter hint types and Invertible/Transformed trait tests created and passing.</done>
</task>

<task type="auto">
  <name>Task 4: Verify all traits tests pass and coverage is meaningful</name>
  <files>monstertruck-traits/tests/curve_traits.rs, monstertruck-traits/tests/surface_traits.rs</files>
  <action>
Run the full test suite for monstertruck-traits to ensure everything passes together:

```
cargo nextest run -p monstertruck-traits --features polynomial,derive
```

Verify that:
1. All new tests pass alongside existing tests (curve.rs, surface.rs, derives.rs)
2. No test name conflicts exist
3. The polynomial feature gate works correctly (tests are skipped when feature is off)

If any tests fail, diagnose and fix the issue.

Optionally, if cargo-tarpaulin is available, run:
```
cargo tarpaulin -p monstertruck-traits --features polynomial --out Stdout 2>&1 | tail -5
```
to verify coverage is non-zero and growing.

Also verify the combined command from success criteria works:
```
cargo nextest run -p monstertruck-core -p monstertruck-traits --features polynomial
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core -p monstertruck-traits --features polynomial` and confirm all tests pass green.</verify>
  <done>All monstertruck-traits tests pass, coverage is non-zero, and combined test command succeeds.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-traits --features polynomial` passes with all new and existing tests green
2. Every public trait method in ParametricCurve has at least one test
3. Every public trait method in ParametricSurface has at least one test
4. ParametricSurface3D normal/normal_uder/normal_vder are tested
5. SearchParameterHint1D and SearchParameterHint2D From implementations are tested
6. Invertible and Transformed trait implementations are tested
7. No test modifies production code
8. Combined command `cargo nextest run -p monstertruck-core -p monstertruck-traits --features polynomial` passes
</verification>

<success_criteria>
- monstertruck-traits has at least one test per public trait method for curve and surface trait implementations
- `cargo nextest run -p monstertruck-traits --features polynomial` passes with all new tests green
- `cargo nextest run -p monstertruck-core -p monstertruck-traits --features polynomial` passes
- COV-06 requirement is fully satisfied
</success_criteria>

<output>
After completion, create `.tendrion/phases/26-core-and-traits-coverage/26-2-SUMMARY.md`
</output>
