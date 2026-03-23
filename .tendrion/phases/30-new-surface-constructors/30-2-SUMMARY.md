---
phase: 30-new-surface-constructors
plan: 2
tags: [loft, surface-constructor, skin, tdd]
key-files:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-modeling/src/builder.rs
  - monstertruck-modeling/src/lib.rs
  - monstertruck-modeling/tests/surface_constructors.rs
decisions: []
metrics:
  tests_added: 6
  tests_total: 27
  tdd_cycles: 1
  deviations: 0
---

## What was built

- **`SkinOptions.v_degree`** (`surface_options.rs`): Added `v_degree: usize` field to `SkinOptions` with default value `1` (linear). Replaced `#[derive(Default)]` with manual `Default` impl. Higher values produce smoother loft surfaces via clamped-uniform v-direction knot vectors.

- **`try_skin` v_degree support** (`bspline_surface.rs`): Updated the >= 3 curves path to compute effective v_degree clamped to `1..=n-1` and build a clamped uniform knot vector of the appropriate degree. Added private helper `clamped_uniform_knot_vector(degree, n)`.

- **`builder::try_loft`** (`builder.rs`): New public function that validates >= 2 curves (returning `Error::InsufficientSections` otherwise), delegates to `BsplineSurface::try_skin`, and wraps the result in a `Face` via `face_from_bspline_surface`.

- **`SkinOptions` re-export** (`lib.rs`): Added `SkinOptions` to the re-export line alongside other surface option types.

- **Tests** (`surface_constructors.rs`): 6 new tests covering 3-curve loft, 4-curve v_degree=3 loft, 2-curve loft, 1-curve error, empty-vec error, and mixed-degree sections.

## Task commits

| Step | SHA | Message |
|------|-----|---------|
| RED | `bafcc21e` | `test(loft): add failing tests for loft surface constructor` |
| GREEN | `3208c4c4` | `feat(loft): implement try_loft builder with v_degree-aware try_skin and input validation` |

## Decisions made

None. All implementation followed the plan specification exactly.

## Deviations from plan

None.

## Self-check

- [x] `cargo nextest run -p monstertruck-modeling --test surface_constructors` -- 27/27 pass
- [x] `cargo nextest run -p monstertruck-geometry --test try_gordon_skin_test` -- 9/9 pass
- [x] `cargo clippy -p monstertruck-geometry -p monstertruck-modeling --all-targets -- -W warnings` -- no errors in our code
- [x] `SkinOptions` accessible from `monstertruck_modeling::SkinOptions`
- [x] `try_loft` with < 2 curves returns `InsufficientSections`
- [x] `v_degree=3` with 4 curves succeeds
