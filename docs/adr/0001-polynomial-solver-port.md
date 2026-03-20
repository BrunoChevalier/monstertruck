# ADR-0001: Port polynomial solvers from matext4cgmath into monstertruck-math

- **Status:** Accepted
- **Date:** 2026-03-16

## Context

The `monstertruck-geometry` crate requires quadratic, cubic, and quartic polynomial solvers for computing curve intersections (hyperbola and parabola specifieds). These solvers were previously provided by the external `matext4cgmath` crate, which depended on the legacy `cgmath` linear algebra types.

With the Phase 4 migration to nalgebra as the math backend, continuing to depend on `matext4cgmath` would reintroduce a `cgmath` transitive dependency and negate the migration benefits.

Additionally, the `monstertruck-traits` crate already has a `polynomial` module (trait definitions for polynomial evaluation). Introducing a solver module with the same name created a namespace collision in downstream crates like `monstertruck-geometry`.

## Decision

1. **Port the polynomial solvers** (`solve_quadratic`, `pre_solve_cubic`, `solve_cubic`, `pre_solve_quartic`, `solve_quartic`) from `matext4cgmath::solver` into a new `monstertruck-math::polynomial` module, rewriting them against `num_traits::Float` and `num_complex::Complex` to avoid any `cgmath` dependency.

2. **Use a `num_traits::Float` alias** (`use num_traits::Float as Fl`) to disambiguate from nalgebra's `ComplexField` methods that shadow identically-named functions like `sqrt` and `abs`.

3. **Resolve the namespace collision** between `monstertruck-math::polynomial` (solvers) and `monstertruck-traits::polynomial` (evaluation traits) by establishing explicit re-export precedence in the geometry crate's base module, so downstream code can access both without ambiguity.

## Alternatives Considered

- **Keep `matext4cgmath` as a dependency:** Rejected because it would pull `cgmath` back into the dependency tree, conflicting with the nalgebra migration goal.
- **Inline solvers directly in `monstertruck-geometry`:** Rejected because the solvers are general-purpose math utilities that belong in the math layer, not in a geometry-specific crate.
- **Rename one of the `polynomial` modules:** Rejected because both names are semantically correct for their contents. Re-export precedence is a lower-disruption solution.

## Consequences

- `matext4cgmath` can be removed from the workspace dependency tree once all call sites are migrated.
- The `monstertruck-math` crate gains a broader role as the canonical location for numerical algorithms, not just linear algebra types.
- Downstream crates importing both `polynomial` modules must use the established re-export pattern to avoid ambiguity.

## Status Update (Phase 6)

The polynomial solvers ported in this ADR continue to function correctly after the Phase 6 topology surgery hardening changes. No modifications to `monstertruck-math::polynomial` were required. See [ADR-0002](0002-fillet-topology-surgery-hardening.md) for the Phase 6 architectural decisions.

## Status Update (Phase 7)

No changes to the polynomial solver module. Phase 7 focused on fillet integration modes and continuity annotations within `monstertruck-solid`; the math layer was unaffected. See [ADR-0003](0003-fillet-integration-mode.md) for Phase 7 architectural decisions.

## Status Update (Phase 8)

No changes to the polynomial solver module. Phase 8 added topology validation (Euler-Poincare assertions and orientation checks) to the fillet pipeline in `monstertruck-solid`; the math layer was unaffected. See [ADR-0002](0002-fillet-topology-surgery-hardening.md) for related updates.

## Status Update (Phase 9)

No changes to the polynomial solver module. Phase 9 focused on boolean operation hardening (face classification, shell healing, tolerance documentation) and establishing a tolerance policy in `monstertruck-core`. A `Matrix4::from_translation` column-placement bug in `monstertruck-math` was fixed during gap-fix verification, but the polynomial solver module itself was unaffected.

## Status Update (Phase 10)

No changes to the polynomial solver module. Phase 10 introduced a NURBS fixture corpus and surface healing hooks in `monstertruck-solid`; the math layer was unaffected.

## Status Update (Phase 11)

No changes to the polynomial solver module. Phase 11 added multi-rail sweep and periodic sweep surface constructors in `monstertruck-geometry`, plus typed builder wrappers in `monstertruck-modeling`. The SVD-based affine fitting used by `sweep_multi_rail` is implemented independently of the polynomial solvers; the math layer was unaffected.

## Status Update (Phase 12 -- v0.4.0 Final)

No changes to the polynomial solver module. Phase 12 added end-to-end font pipeline integration tests and finalized the Ayam port plan. The math layer was unaffected. This ADR remains Accepted with no anticipated changes for v0.4.0.

## Status Update (Phase 13 -- v0.5.0 API Polish)

No changes to the polynomial solver module. Phase 13 added typed option structs, fallible `try_*` surface constructors, and patch split/extract methods in `monstertruck-geometry`, plus option-struct builder functions in `monstertruck-modeling`. The math layer was unaffected.

## Status Update (Phase 14 -- Profile Solid Pipeline)

No changes to the polynomial solver module. Phase 14 added profile revolve/sweep functions, mixed profile face construction, and solid validation in `monstertruck-modeling`. The math layer was unaffected.

## Status Update (Phase 15 -- Font Stress Testing & Performance)

No changes to the polynomial solver module. Phase 15 added a font stress corpus with pathological geometry fixtures and Criterion performance benchmarks for the profile pipeline. The math layer was unaffected.

## Status Update (Phase 16 -- Tolerance Centralization & API Hardening)

No changes to the polynomial solver module. Phase 16 centralized tolerance constants in `monstertruck-core::tolerance_constants`, added `#[non_exhaustive]` to surface option structs, and deduplicated deprecated surface constructors. The math layer was unaffected.

## Status Update (Phase 17 -- Curve-Curve Intersection)

No changes to the polynomial solver module. Phase 17 added a curve-curve intersection module (`monstertruck-geometry::nurbs::curve_intersect`) using subdivision + Newton-Raphson refinement. The intersection algorithm does not use the polynomial solvers; the math layer was unaffected.

## Status Update (Phase 18 -- Gordon Surface from Network)

No changes to the polynomial solver module. Phase 18 added `try_gordon_from_network` and `try_gordon_verified` Gordon surface constructors that leverage the curve intersection engine from Phase 17. The math layer was unaffected.
