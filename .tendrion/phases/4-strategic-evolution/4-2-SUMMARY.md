---
phase: 4-strategic-evolution
plan: 2
tags: [cgmath, nalgebra, math-adapter, migration]
key-files:
  - monstertruck-math/Cargo.toml
  - monstertruck-math/src/lib.rs
  - monstertruck-math/src/types.rs
  - monstertruck-math/src/traits.rs
  - monstertruck-math/src/conversions.rs
  - monstertruck-math/tests/compatibility.rs
  - monstertruck-core/Cargo.toml
  - monstertruck-core/src/lib.rs
  - monstertruck-core/src/cgmath64.rs
  - monstertruck-core/src/cgmath_extend_traits.rs
  - monstertruck-core/src/bounding_box.rs
  - monstertruck-core/src/tolerance.rs
  - monstertruck-core/src/derivatives.rs
  - monstertruck-core/src/hash.rs
decisions:
  - "Matrix types implemented as newtype wrappers (#[repr(transparent)]) around nalgebra matrices to preserve cgmath's column-major constructor convention"
  - "Vector/Point types are direct nalgebra type aliases (constructors match between cgmath and nalgebra)"
  - "cgmath retained as dev-dependency in monstertruck-core for test files that directly import cgmath (test files cannot be modified per AGENTS.md)"
  - "Doc-test tuple .into() calls replaced with explicit Vector::new() constructors (orphan rules prevent From<tuple> for nalgebra types)"
  - "Downstream crate compilation deferred: nalgebra API differences (struct field destructuring, tuple conversions, Matrix type mismatches) require either modifying downstream source or creating vector newtype wrappers"
metrics:
  tests-pass: 74
  test-crates: [monstertruck-math, monstertruck-core]
  doc-tests-pass: 52
  downstream-compile: partial
---

## What Was Built

### Task 1: monstertruck-math adapter crate
Created a new `monstertruck-math` crate that wraps nalgebra with a cgmath-compatible API:

- **types.rs**: Vector1-4, Point1-3 as nalgebra type aliases; Matrix2-4 as newtype wrappers with cgmath column-major `new()` convention; Rad/Deg angle wrapper types with arithmetic and conversions
- **traits.rs**: BaseFloat, VectorSpace, InnerSpace, EuclideanSpace (with midpoint), MetricSpace, SquareMatrix, Transform trait bridges with implementations for all vector/point/matrix types
- **conversions.rs**: Truncate, ToHomogeneous, Extend, MulElementWise, Cast extension traits
- **lib.rs**: Re-exports all types/traits, provides `prelude` module, implements perspective/ortho/frustum projection functions
- **compatibility.rs**: 31 compatibility tests covering construction, arithmetic, traits, serde, and matrix operations

### Task 2: monstertruck-core migration
Migrated monstertruck-core from cgmath to monstertruck-math:

- Replaced cgmath dependency with monstertruck-math in Cargo.toml
- Removed matext4cgmath dependency (no downstream usage found)
- Updated all source files to import from monstertruck-math
- Preserved cgmath64 and cgmath_extend_traits module names for backward compatibility
- Updated doc-tests to use explicit constructors instead of tuple .into()
- All 43 unit tests + 52 doc-tests pass

### Task 3: Downstream verification (partial)
- monstertruck-core and monstertruck-math compile and pass all tests
- Downstream crates (monstertruck-traits, etc.) have compilation errors due to fundamental API differences between cgmath and nalgebra that cannot be resolved in the adapter layer alone

## Task Commits

| SHA | Message |
|-----|---------|
| 66ebb5f9 | feat(math): create monstertruck-math adapter crate wrapping nalgebra with cgmath-compatible API |
| e5628510 | feat(core): migrate monstertruck-core from cgmath to monstertruck-math |
| ee74e456 | feat(math): add Transform<Point2> for Matrix3 and midpoint to EuclideanSpace |

## Deviations from Plan

### 1. cgmath retained as dev-dependency
**Category:** design
**Reason:** Test files (tests/hash.rs) directly `use cgmath::*;` and AGENTS.md prohibits modifying test files. Added cgmath as dev-dependency to keep tests compilable.
**Impact:** cgmath remains in dev-dependencies but is removed from runtime dependencies.

### 2. Downstream compilation deferred
**Category:** scope
**Reason:** Fundamental API incompatibilities between nalgebra and cgmath cannot be resolved purely in the adapter layer:
- Struct field destructuring (`Vector2 { x, y }`) requires actual struct fields that nalgebra types don't have
- Tuple `.into()` conversions blocked by orphan rules
- Matrix wrapper types create type mismatches when downstream code expects raw nalgebra types
**Impact:** Downstream crates will need source modifications in a follow-up plan. Options include:
  (a) Making vector types newtype wrappers too (like matrices)
  (b) Modifying downstream source files to use index-based access instead of field destructuring
  (c) Adding cgmath as a direct dependency to downstream crates temporarily

## Self-Check
- [x] monstertruck-math crate exists with nalgebra backend
- [x] nalgebra declared in workspace.dependencies
- [x] monstertruck-core compiles against monstertruck-math
- [x] cgmath removed from monstertruck-core runtime dependencies
- [x] All monstertruck-core tests pass (43 unit + 52 doc)
- [x] All monstertruck-math tests pass (31)
- [ ] Downstream crates compile (deferred - requires follow-up)
