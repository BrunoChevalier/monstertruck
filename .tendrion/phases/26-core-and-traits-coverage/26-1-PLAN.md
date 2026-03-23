---
phase: 26-core-and-traits-coverage
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-core/tests/tolerance_traits.rs
  - monstertruck-core/tests/bounding_box.rs
  - monstertruck-core/tests/id_tests.rs
  - monstertruck-core/tests/entry_map.rs
  - monstertruck-core/tests/derivatives_api.rs
  - monstertruck-core/tests/cgmath_extend_traits.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo nextest run -p monstertruck-core and all new tests pass green"
    - "User runs cargo tarpaulin -p monstertruck-core and coverage reaches 55% or above"
    - "Tolerance trait methods (near, near2, so_small, so_small2) have dedicated tests on f64, Vector2, Vector3, Point3"
    - "OperationTolerance pipeline tracking is tested for multi-step error accumulation"
    - "BoundingBox operations (push, is_empty, contains, union, intersection, partial_ord, from_iter, center, diagonal, diameter, size) have dedicated tests"
    - "Id struct creation, equality, hashing, and Debug formatting are tested"
    - "EntryMap key deduplication and HashMap conversion are tested"
  artifacts:
    - path: "monstertruck-core/tests/tolerance_traits.rs"
      provides: "Unit tests for Tolerance and Origin trait methods on multiple types"
      min_lines: 80
      contains: "so_small"
    - path: "monstertruck-core/tests/bounding_box.rs"
      provides: "Comprehensive BoundingBox API tests including edge cases"
      min_lines: 100
      contains: "BoundingBox"
    - path: "monstertruck-core/tests/id_tests.rs"
      provides: "Tests for Id struct Copy, Hash, Eq, Debug"
      min_lines: 30
      contains: "Id"
    - path: "monstertruck-core/tests/entry_map.rs"
      provides: "Tests for EntryMap deduplication and conversion"
      min_lines: 30
      contains: "EntryMap"
    - path: "monstertruck-core/tests/derivatives_api.rs"
      provides: "Tests for CurveDerivatives and SurfaceDerivatives construction and methods"
      min_lines: 60
      contains: "CurveDerivatives"
  key_links:
    - from: "monstertruck-core/tests/tolerance_traits.rs"
      to: "monstertruck-core/src/tolerance.rs"
      via: "imports Tolerance, Origin, TOLERANCE, TOLERANCE2"
      pattern: "use monstertruck_core::tolerance"
    - from: "monstertruck-core/tests/bounding_box.rs"
      to: "monstertruck-core/src/bounding_box.rs"
      via: "imports BoundingBox and Bounded"
      pattern: "use monstertruck_core::bounding_box"
---

<objective>
Increase monstertruck-core test coverage from 40% to 55%+ by adding dedicated unit tests for the tolerance infrastructure (Tolerance/Origin traits, assert_near/assert_near2 macros, OperationTolerance), BoundingBox API, Id struct, EntryMap utility, and CurveDerivatives/SurfaceDerivatives types.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-core/src/tolerance.rs
@monstertruck-core/src/tolerance_constants.rs
@monstertruck-core/src/bounding_box.rs
@monstertruck-core/src/id.rs
@monstertruck-core/src/entry_map.rs
@monstertruck-core/src/derivatives.rs
@monstertruck-core/src/cgmath_extend_traits.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Tolerance trait and Origin trait unit tests</name>
  <files>monstertruck-core/tests/tolerance_traits.rs</files>
  <action>
Create a new test file `monstertruck-core/tests/tolerance_traits.rs` with comprehensive tests for the tolerance infrastructure:

1. **Tolerance trait tests on f64:**
   - `near` returns true for values within TOLERANCE
   - `near` returns false for values outside TOLERANCE
   - `near2` returns true for values within TOLERANCE2
   - `near2` returns false for values outside TOLERANCE2
   - Boundary cases: exactly at TOLERANCE distance

2. **Tolerance trait tests on Vector2, Vector3, Point3:**
   - `near` for Vector2 values within and outside TOLERANCE
   - `near` for Vector3 values within and outside TOLERANCE
   - `near` for Point3 values within and outside TOLERANCE
   - `near2` for the same types

3. **Origin trait tests:**
   - `so_small` returns true for f64 near zero
   - `so_small` returns false for f64 not near zero
   - `so_small2` returns true for f64 near zero within TOLERANCE2
   - `so_small` for Vector2::zero and small vectors
   - `so_small` for Vector3::zero and small vectors

4. **OperationTolerance extended tests:**
   - `new` with custom base tolerance
   - `from_global` uses TOLERANCE
   - Chain of 5+ operations accumulates correctly
   - `effective_tolerance` after multiple operations
   - `within_budget` transitions from true to false as error grows
   - `within_budget` boundary: accumulated == base returns false
   - `last_operation` tracks the most recent tag
   - Clone produces independent copy

5. **Macro tests:**
   - `assert_near!` succeeds for values within tolerance
   - `assert_near2!` succeeds for values within TOLERANCE2

Use imports:
```rust
use monstertruck_core::cgmath64::*;
use monstertruck_core::tolerance::*;
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core --test tolerance_traits` and verify all tests pass.</verify>
  <done>Tolerance trait, Origin trait, OperationTolerance, and macro tests created and passing.</done>
</task>

<task type="auto">
  <name>Task 2: BoundingBox comprehensive tests</name>
  <files>monstertruck-core/tests/bounding_box.rs</files>
  <action>
Create `monstertruck-core/tests/bounding_box.rs` with comprehensive tests for the BoundingBox API. Note that there are already inline tests in bounding_box.rs (test_bbox_non_crossing, test_bbox_touch_edge, test_bbox_touch_corner), so focus on untested paths:

1. **Construction and emptiness:**
   - `new()` creates an empty bounding box
   - `is_empty` returns true for default
   - `is_empty` returns false after a single push
   - `from_iter` with references
   - `from_iter` with owned values

2. **Push and NaN handling:**
   - Push multiple points, verify min/max
   - Push NaN-containing point, verify it is ignored
   - Push all-NaN point followed by valid point

3. **Geometric properties:**
   - `diagonal` for known box
   - `diameter` for 2D and 3D boxes
   - `size` (max edge length) for 3D box
   - `center` for known box
   - `center` of empty box returns NaN components
   - `diagonal` of empty box returns NEG_INFINITY
   - `diameter` of empty box returns NEG_INFINITY
   - `size` of empty box returns NEG_INFINITY

4. **Containment:**
   - `contains` for interior point
   - `contains` for boundary point
   - `contains` for exterior point

5. **Union and intersection operators:**
   - `+=` with BoundingBox (owned and ref)
   - `+` operator (all four ref/owned combinations)
   - `^=` intersection (owned and ref)
   - `^` intersection (all four combinations)
   - Intersection of non-overlapping boxes is empty
   - Union with empty box preserves original

6. **PartialOrd (inclusion):**
   - Box containing another is Greater
   - Box contained by another is Less
   - Equal boxes compare Equal
   - Overlapping but non-containing boxes return None (no ordering)

7. **Type coverage:**
   - Tests using Point2, Point3, Vector2, Vector3 to exercise all impl_bounded macro expansions

Use imports:
```rust
use monstertruck_core::bounding_box::*;
use monstertruck_core::cgmath64::*;
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core --test bounding_box` and verify all tests pass.</verify>
  <done>BoundingBox API tests created and passing, covering construction, geometry, containment, operators, and PartialOrd.</done>
</task>

<task type="auto">
  <name>Task 3: Id, EntryMap, and Derivatives API tests</name>
  <files>monstertruck-core/tests/id_tests.rs, monstertruck-core/tests/entry_map.rs, monstertruck-core/tests/derivatives_api.rs</files>
  <action>
Create three test files for remaining uncovered modules:

**monstertruck-core/tests/id_tests.rs:**
1. `Id::new` creates from pointer and can be compared for equality
2. Two Ids from same pointer are equal
3. Two Ids from different pointers are not equal
4. Id is Copy (assign and compare independently)
5. Id can be used as HashMap key (Hash + Eq)
6. Debug format matches pointer format (0x...)

Use imports:
```rust
use monstertruck_core::id::Id;
use std::collections::HashMap;
```

**monstertruck-core/tests/entry_map.rs:**
1. Basic deduplication: same key returns same value
2. Different keys return different values
3. `into_iter` yields all entries
4. `From<EntryMap> for HashMap` conversion works
5. Key closure and value closure are invoked correctly

Use imports:
```rust
use monstertruck_core::entry_map::EntryMap;
use std::collections::HashMap;
```

**monstertruck-core/tests/derivatives_api.rs:**
1. `CurveDerivatives::new` creates zeroed structure
2. `CurveDerivatives::push` appends element
3. `CurveDerivatives::derivative()` returns derivative of derivative structure
4. `TryFrom<[V; N]>` conversion
5. `TryFrom<&[V]>` conversion
6. `FromIterator` conversion
7. `max_order` reflects construction
8. `Deref` to slice works
9. `Mul` by scalar scales all elements
10. `Div` by scalar divides all elements
11. `SurfaceDerivatives::new` creates zeroed structure
12. `SurfaceDerivatives::derivative_u` and `derivative_v` shift correctly
13. Index/IndexMut work for SurfaceDerivatives
14. `to_array` conversion
15. AbsDiffEq implementation

Use imports:
```rust
use monstertruck_core::cgmath64::*;
use monstertruck_core::derivatives::*;
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core --test id_tests --test entry_map --test derivatives_api` and verify all tests pass.</verify>
  <done>Id, EntryMap, and Derivatives API test files created and passing.</done>
</task>

<task type="auto">
  <name>Task 4: Verify coverage threshold and fix gaps</name>
  <files>monstertruck-core/tests/tolerance_traits.rs, monstertruck-core/tests/bounding_box.rs, monstertruck-core/tests/cgmath_extend_traits.rs</files>
  <action>
Run cargo-tarpaulin to measure coverage:
```
cargo tarpaulin -p monstertruck-core --out Stdout 2>&1 | tail -5
```

If coverage is below 55%, identify the largest uncovered modules and add targeted tests:

1. **cgmath_extend_traits.rs** - If not sufficiently covered, create `monstertruck-core/tests/cgmath_extend_traits.rs` with tests for:
   - `Homogeneous` trait: `truncate`, `weight`, `from_point`, `to_point`, `from_point_weight` for Vector2, Vector3, Vector4
   - `rat_der` function with known polynomial derivatives
   - `rat_ders` function verifying multi-order output
   - `multi_rat_der` and `multi_rat_ders` for surface derivatives
   - `abs_ders` function for magnitude derivative computation
   - `ControlPoint` trait: `origin`, `to_vec`, `from_vec` for Point1/2/3, Vector1/2/3/4

2. **hash.rs** - `take_one_unit` function: verify output is unit length for several seeds

3. **newton.rs** - If `NewtonLog` methods are uncovered, add tests for:
   - `NewtonLog::new(true, ...)` vs `NewtonLog::new(false, ...)`
   - `Display` formatting
   - `degenerate()` and `used_fallback()` accessors

Ensure test files compile and all new tests pass. Re-run tarpaulin to verify 55%+ is achieved.

Note: cargo-tarpaulin may not be installed. If not available, verify coverage by confirming that every public function/method in the crate has at least one test that exercises it (manually check against the source).
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core` to verify all tests pass. If tarpaulin is available, verify coverage >= 55%.</verify>
  <done>monstertruck-core test coverage verified at 55% or above with all tests passing.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-core` passes with all new and existing tests green
2. Coverage reaches 55%+ as measured by cargo-tarpaulin (or verified by function-level coverage analysis)
3. Tolerance infrastructure (Tolerance trait, Origin trait, OperationTolerance, assert_near macros) each have dedicated tests
4. BoundingBox API is tested for construction, geometry, containment, operators, and ordering
5. Id, EntryMap, CurveDerivatives, and SurfaceDerivatives public APIs are tested
6. No test file modifies production code
</verification>

<success_criteria>
- monstertruck-core test coverage reaches 55% or higher
- Tolerance infrastructure functions have dedicated unit tests
- All new tests pass via `cargo nextest run -p monstertruck-core`
- COV-05 requirement is fully satisfied
</success_criteria>

<output>
After completion, create `.tendrion/phases/26-core-and-traits-coverage/26-1-SUMMARY.md`
</output>
