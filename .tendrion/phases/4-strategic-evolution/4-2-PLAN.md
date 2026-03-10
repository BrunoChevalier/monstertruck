---
phase: 4-strategic-evolution
plan: 2
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-math/Cargo.toml
  - monstertruck-math/src/lib.rs
  - monstertruck-math/src/types.rs
  - monstertruck-math/src/traits.rs
  - monstertruck-math/src/conversions.rs
  - monstertruck-math/tests/compatibility.rs
  - Cargo.toml
  - monstertruck-core/Cargo.toml
  - monstertruck-core/src/lib.rs
  - monstertruck-core/src/nalgebra64.rs
  - monstertruck-core/src/cgmath64.rs
  - monstertruck-core/src/cgmath_extend_traits.rs
  - monstertruck-core/src/bounding_box.rs
  - monstertruck-core/src/tolerance.rs
  - monstertruck-core/src/derivatives.rs
  - monstertruck-core/src/hash.rs
autonomous: true
must_haves:
  truths:
    - "User adds monstertruck-math as a dependency and gets nalgebra types re-exported through a cgmath-compatible API"
    - "monstertruck-core compiles against nalgebra with the adapter layer providing type aliases and trait implementations"
    - "All existing monstertruck-core tests pass without modification to test logic, only import paths change"
    - "cgmath is removed from monstertruck-core's Cargo.toml dependencies"
    - "Downstream crates that depend on monstertruck-core continue to compile"
  artifacts:
    - path: "monstertruck-math/src/lib.rs"
      provides: "Math adapter crate root re-exporting nalgebra types with cgmath-compatible API"
      min_lines: 30
      contains: "nalgebra"
    - path: "monstertruck-math/src/types.rs"
      provides: "Type aliases mapping cgmath names to nalgebra equivalents"
      min_lines: 20
      contains: "Vector3"
    - path: "monstertruck-math/src/traits.rs"
      provides: "Trait bridges for cgmath traits like BaseFloat, EuclideanSpace, VectorSpace, InnerSpace"
      min_lines: 100
      contains: "BaseFloat"
    - path: "monstertruck-core/src/lib.rs"
      provides: "Core crate using monstertruck-math instead of cgmath"
      min_lines: 30
      contains: "monstertruck_math"
  key_links:
    - from: "Cargo.toml"
      to: "monstertruck-math/Cargo.toml"
      via: "workspace.dependencies declaration"
      pattern: "monstertruck-math"
    - from: "monstertruck-core/Cargo.toml"
      to: "monstertruck-math/Cargo.toml"
      via: "dependency declaration using workspace = true"
      pattern: "monstertruck-math"
    - from: "monstertruck-core/src/lib.rs"
      to: "monstertruck-math/src/lib.rs"
      via: "use/re-export of math adapter types"
      pattern: "monstertruck_math"
---

<objective>
Create a monstertruck-math adapter crate that wraps nalgebra with a cgmath-compatible API, then migrate monstertruck-core to use it, removing cgmath as a direct dependency from the foundation layer. Verify downstream crates compile.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@Cargo.toml
@monstertruck-core/Cargo.toml
@monstertruck-core/src/lib.rs
@monstertruck-core/src/cgmath64.rs
@monstertruck-core/src/cgmath_extend_traits.rs
@monstertruck-core/src/bounding_box.rs
@monstertruck-core/src/derivatives.rs
@monstertruck-core/src/tolerance.rs
@monstertruck-core/src/hash.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create monstertruck-math adapter crate with nalgebra backend</name>
  <files>monstertruck-math/Cargo.toml, monstertruck-math/src/lib.rs, monstertruck-math/src/types.rs, monstertruck-math/src/traits.rs, monstertruck-math/src/conversions.rs, monstertruck-math/tests/compatibility.rs, Cargo.toml</files>
  <action>
Create a new crate `monstertruck-math` that provides the same type names and trait surface as cgmath but backed by nalgebra.

1. **Add nalgebra to workspace.dependencies** in root `Cargo.toml`:
   ```toml
   nalgebra = { version = "0.33", features = ["serde-serialize"] }
   ```
   Also add `monstertruck-math` to both the `[workspace]` members list and the `[workspace.dependencies]` section:
   ```toml
   # In members:
   "monstertruck-math",
   # In workspace.dependencies:
   monstertruck-math = { version = "0.1", path = "monstertruck-math" }
   ```
   IMPORTANT: nalgebra MUST be declared at the workspace level (not crate-level) to be consistent with all other dependencies in this repo. The crate's Cargo.toml should reference it via `nalgebra = { workspace = true }`.

2. **Create `monstertruck-math/Cargo.toml`**:
   ```toml
   [package]
   name = "monstertruck-math"
   version = "0.1.0"
   edition = "2024"
   description = "Math adapter crate providing cgmath-compatible API over nalgebra"

   [dependencies]
   nalgebra = { workspace = true }
   num-traits = "0.2"
   serde = { version = "1", features = ["derive"] }
   approx = "0.5"
   ```

3. **Create `monstertruck-math/src/types.rs`** with type aliases:
   - `Vector1 = nalgebra::SVector<f64, 1>` (and generic `Vector1_<S>`)
   - `Vector2 = nalgebra::Vector2<f64>` (and generic)
   - `Vector3 = nalgebra::Vector3<f64>` (and generic)
   - `Vector4 = nalgebra::Vector4<f64>` (and generic)
   - `Point1 = nalgebra::Point1<f64>` (and generic)
   - `Point2 = nalgebra::Point2<f64>` (and generic)
   - `Point3 = nalgebra::Point3<f64>` (and generic)
   - `Matrix2 = nalgebra::Matrix2<f64>` (and generic)
   - `Matrix3 = nalgebra::Matrix3<f64>` (and generic)
   - `Matrix4 = nalgebra::Matrix4<f64>` (and generic)
   - `Rad<S>` and `Deg<S>` wrapper types with conversion
   - Implement `From` conversions and arithmetic for Rad/Deg

4. **Create `monstertruck-math/src/traits.rs`** with trait bridges:
   - `BaseFloat` trait that nalgebra float types satisfy (re-export from num_traits or define shim)
   - `VectorSpace` trait implemented for nalgebra vectors (mul/div by scalar, zero)
   - `InnerSpace` trait with `dot`, `magnitude`, `magnitude2`, `normalize`
   - `EuclideanSpace` trait with `origin`, `to_vec`, `from_vec`, associated `Diff` type
   - `MetricSpace` trait with `distance` and `distance2`
   - `Zero` trait re-export from num_traits
   - `One` trait re-export from num_traits
   - `Array` trait for index access (nalgebra vectors are indexable)
   - Ensure all trait impls cover Vector1..4, Point1..3, Matrix2..4

5. **Create `monstertruck-math/src/conversions.rs`**:
   - `truncate()` methods for vectors (Vector4 -> Vector3, Vector3 -> Vector2)
   - `to_homogeneous()` for Point3 -> Vector4
   - `extend()` for vectors
   - Extension traits to add cgmath-style methods to nalgebra types

6. **Create `monstertruck-math/src/lib.rs`**:
   - Re-export all types, traits, and functions
   - Provide a `prelude` module mimicking `cgmath::prelude::*`
   - Re-export `perspective`, `ortho`, `frustum`, `Deg`, `Rad` (implement projection functions using nalgebra)
   - Provide `look_at_rh` as a function or method extension

7. **Write compatibility tests** in `monstertruck-math/tests/compatibility.rs`:
   - Test that Vector3::new(1.0, 2.0, 3.0) works
   - Test dot product, cross product, magnitude
   - Test Point3 operations (origin, to_vec, from_vec)
   - Test Matrix4 look_at_rh matches expected values
   - Test Rad/Deg conversions
   - Test Index/IndexMut for vectors and points
   - Test serialization round-trips with serde
  </action>
  <verify>Run `cargo test -p monstertruck-math` and confirm all compatibility tests pass. Run `cargo check -p monstertruck-math` for clean compilation.</verify>
  <done>monstertruck-math adapter crate was created with nalgebra backend, workspace-level dependency declaration, type aliases, trait bridges, and passing compatibility tests.</done>
</task>

<task type="auto">
  <name>Task 2: Migrate monstertruck-core from cgmath to monstertruck-math</name>
  <files>monstertruck-core/Cargo.toml, monstertruck-core/src/lib.rs, monstertruck-core/src/nalgebra64.rs, monstertruck-core/src/cgmath64.rs, monstertruck-core/src/cgmath_extend_traits.rs, monstertruck-core/src/bounding_box.rs, monstertruck-core/src/tolerance.rs, monstertruck-core/src/derivatives.rs, monstertruck-core/src/hash.rs</files>
  <action>
Migrate monstertruck-core to use monstertruck-math instead of cgmath:

1. **Update `monstertruck-core/Cargo.toml`**:
   - Replace `cgmath = { workspace = true }` with `monstertruck-math = { workspace = true }`
   - Remove `matext4cgmath = { workspace = true }` (its functionality is absorbed into monstertruck-math)
   - Add `num-traits = "0.2"` if not already present (needed for trait bounds)

2. **Refactor `cgmath64.rs` -> keep as compatibility shim OR rename**:
   - Option A (safer): Keep `cgmath64.rs` as a module name for backward compatibility but change its contents to re-export from `monstertruck_math`
   - The module should re-export: all type aliases (Vector1..4, Point1..3, Matrix2..4), Rad, Deg
   - Re-export the `control_point` module with `ControlPoint` trait
   - Re-export `Homogeneous` trait and `rat_der`, `rat_ders`, `multi_rat_der`, `multi_rat_ders`, `abs_ders` functions
   - Update function implementations to use nalgebra types via monstertruck-math

3. **Refactor `cgmath_extend_traits.rs`**:
   - The `ControlPoint` trait currently depends on cgmath's `BaseFloat`, `Zero`, `EuclideanSpace`
   - Update all trait bounds to use monstertruck-math's re-exported traits
   - The `Homogeneous` trait uses `VectorSpace` and `EuclideanSpace` -- update these bounds
   - Implement `ControlPoint` for nalgebra Point and Vector types
   - Implement `Homogeneous` for nalgebra Vector2, Vector3, Vector4

4. **Update `bounding_box.rs`**:
   - Replace `use cgmath::*` with `use monstertruck_math::prelude::*` (or specific imports)
   - The `Bounded` trait uses `BaseFloat`, `MetricSpace` -- update imports
   - Update all implementations for Point1, Point2, Point3, Vector1..4

5. **Update `derivatives.rs`**:
   - Replace cgmath imports with monstertruck-math imports
   - The derivative utilities heavily use `BaseFloat`, `InnerSpace`, `EuclideanSpace`, `VectorSpace`

6. **Update `tolerance.rs` and `hash.rs`**:
   - Replace cgmath imports with monstertruck-math imports

7. **Update `lib.rs`**:
   - Change `pub use cgmath::prelude::*` to `pub use monstertruck_math::prelude::*`
   - Change `pub use cgmath::{Deg, Rad, frustum, ortho, perspective}` to monstertruck-math re-exports
   - Remove `pub use matext4cgmath::*`
   - Keep the `f64_type!` macro but have it generate aliases from monstertruck-math types
   - Keep module declarations for `cgmath64` and `cgmath_extend_traits` (for API compatibility) but update contents

8. **Preserve API surface**: The public API of monstertruck-core must remain the same. Downstream crates should compile without changes to their source code (only their transitive dependency tree changes).
  </action>
  <verify>
Run `cargo test -p monstertruck-core` and confirm all existing tests pass.
Run `cargo check -p monstertruck-core` for clean compilation.
Verify cgmath is no longer in monstertruck-core/Cargo.toml.
Run `cargo check -p monstertruck-geometry` to verify downstream compilation.
  </verify>
  <done>monstertruck-core was migrated from cgmath to monstertruck-math with all tests passing and cgmath removed from its dependencies.</done>
</task>

<task type="auto">
  <name>Task 3: Verify downstream compilation across all dependent crates</name>
  <files>monstertruck-math/src/traits.rs, monstertruck-math/src/conversions.rs</files>
  <action>
Verify that all crates depending on monstertruck-core still compile, fixing any API gaps in monstertruck-math as needed. This task is verify-only for downstream crates -- all fixes go into monstertruck-math's adapter layer, not downstream source files.

1. **Check each downstream crate**:
   - `cargo check -p monstertruck-traits` (uses cgmath types from core)
   - `cargo check -p monstertruck-geometry` (heavy user of core math types)
   - `cargo check -p monstertruck-topology` (uses tolerance/core types)
   - `cargo check -p monstertruck-meshing` (uses core types for tessellation)
   - `cargo check -p monstertruck-gpu` (uses cgmath64 types for camera/rendering)

2. **Note**: Some downstream crates may still have their OWN direct `cgmath` dependency. The goal for this plan is only to remove cgmath from monstertruck-core. Downstream crates keep their cgmath dependency for now and will be migrated in a future phase.

3. **Fix API compatibility issues in monstertruck-math only**:
   - If nalgebra types have different method signatures than cgmath (e.g., `truncate()` behavior), add extension methods in `monstertruck-math/src/conversions.rs` to bridge the gap
   - If cgmath-specific trait bounds (like `cgmath::BaseFloat`) appear in downstream trait bounds, ensure monstertruck-math's `BaseFloat` in `monstertruck-math/src/traits.rs` is a compatible substitute
   - Pay special attention to `matext4cgmath` -- any functionality from this crate that's used downstream needs to be replicated in monstertruck-math
   - Do NOT modify downstream crate source files -- only monstertruck-math's adapter layer

4. **Run the full test suite**: `cargo test -p monstertruck-core -p monstertruck-traits` to verify the foundation layer is clean.
  </action>
  <verify>
`cargo check -p monstertruck-core` succeeds.
`cargo check -p monstertruck-traits` succeeds.
`cargo check -p monstertruck-geometry` succeeds.
`cargo check -p monstertruck-topology` succeeds.
`cargo check -p monstertruck-meshing` succeeds.
`cargo check -p monstertruck-gpu` succeeds.
`cargo test -p monstertruck-core` passes all tests.
`grep -c "cgmath" monstertruck-core/Cargo.toml` returns 0.
  </verify>
  <done>All downstream crates were verified to compile against the migrated monstertruck-core, with compatibility fixes applied in monstertruck-math as needed.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-math` passes all compatibility tests
2. `cargo test -p monstertruck-core` passes all existing tests
3. `cgmath` does not appear in `monstertruck-core/Cargo.toml` dependencies
4. `monstertruck-math` appears in workspace members and workspace.dependencies in root `Cargo.toml`
5. `nalgebra` is declared in workspace.dependencies (not crate-level)
6. `cargo check` for all downstream crates (traits, geometry, topology, meshing, gpu) succeeds
7. The public API surface of monstertruck-core remains backward-compatible
</verification>

<success_criteria>
- monstertruck-math adapter crate exists with nalgebra backend and cgmath-compatible API
- nalgebra is declared in workspace.dependencies, consistent with repo conventions
- monstertruck-core compiles against nalgebra (via monstertruck-math)
- cgmath is removed from monstertruck-core's Cargo.toml
- All monstertruck-core tests pass
- All downstream crates compile cleanly (verified, not modified)
</success_criteria>

<output>
After completion, create `.tendrion/phases/4-strategic-evolution/4-2-SUMMARY.md`
</output>
