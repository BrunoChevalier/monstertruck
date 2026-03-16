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
