# Phase 1: Core Stabilization -- Planning Output

## Files Created

| File | Wave | Requirement | Description |
|------|------|-------------|-------------|
| `1-1-PLAN.md` | 1 | CORE-01 | Fix 9 IntersectionCurve unimplemented!() arms in geometry.rs |
| `1-2-PLAN.md` | 1 | CORE-03 | Replace deprecated proc-macro-error with proc-macro-error2 |
| `1-3-PLAN.md` | 1 | CORE-02 | Audit/reduce unwrap() density in solid (16) and meshing (32) |
| `1-4-PLAN.md` | 2 | CORE-04 | Add criterion benchmarking for NURBS, tessellation, booleans |

## Wave Structure

- **Wave 1** (3 plans, parallel): Plans 1-1, 1-2, 1-3
- **Wave 2** (1 plan): Plan 1-4 (depends on 1-1 for boolean op benchmarks)

## Requirement Coverage Matrix

| Requirement | Plan(s) | Coverage |
|-------------|---------|----------|
| CORE-01: Fix IntersectionCurve unimplemented arms | 1-1 | Full (all 9 arms) |
| CORE-02: Reduce unwrap() density in solid/meshing | 1-3 | Full (48 calls in 17 files) |
| CORE-03: Replace deprecated proc-macro-error | 1-2 | Full (dep swap + 21 macro attrs) |
| CORE-04: Add benchmarking infrastructure | 1-4 | Full (3 crates with criterion) |

## Validation

- All frontmatter fields present in all 4 plans
- No file conflicts within wave 1 (disjoint file sets)
- Wave numbers consistent with depends_on (plan 4 wave 2, depends on plan 1 wave 1)
- All plans autonomous (no checkpoint tasks)
- All 4 requirements covered

## Totals

- **Plans:** 4
- **Waves:** 2
