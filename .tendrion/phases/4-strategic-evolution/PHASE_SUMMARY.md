---
phase: 4-strategic-evolution
status: complete
plans_total: 4
plans_executed: 4
verification: pass
---

## What Was Built

1. **T-spline validation complete (Plan 4-1):** Resolved all TODOs in `t_nurcc.rs` and `t_mesh.rs`. Connection parity (L/R) verified correct per Sederberg et al. 1998; zero knot interval handling tolerates boundary traversal failures. 8 new integration tests in `t_spline_validation.rs`.

2. **nalgebra adapter crate (Plan 4-2):** Created `monstertruck-math` wrapping nalgebra with cgmath-compatible API (Vector/Point aliases, Matrix newtypes, trait bridges). Migrated `monstertruck-core` from cgmath to monstertruck-math. cgmath removed from runtime deps, retained as dev-dependency for unmodifiable test files. 74 tests pass across math + core.

3. **RwLock topology migration (Plan 4-3):** Migrated `Vertex`, `Edge`, `Face` geometry fields from `parking_lot::Mutex` to `parking_lot::RwLock`. Zero Mutex references remain in topology source. Criterion benchmark with concurrent_read_points, concurrent_read_curves, and mixed_read_write scenarios. 14 tests pass.

4. **GPU NURBS tessellation prototype (Plan 4-4):** WGSL compute shader evaluates NURBS surfaces via B-spline basis functions. Rust host (`GpuTessellator`) manages pipeline/buffers with `tessellate_adaptive()` multi-pass refinement. GPU output matches CPU within 1e-4 tolerance. 12 tests, criterion GPU-vs-CPU benchmark.

## Requirement Coverage

| Requirement | Plan | Status |
|-------------|------|--------|
| FEAT-04 (T-spline validation) | 4-1 | Covered |
| EVOLVE-01 (nalgebra migration) | 4-2 | Covered |
| EVOLVE-02 (RwLock contention) | 4-3 | Covered |
| EVOLVE-03 (GPU tessellation) | 4-4 | Covered |

## Test Results

- Plan 4-1: 160 geometry tests pass (8 new)
- Plan 4-2: 74 tests pass (monstertruck-math + monstertruck-core)
- Plan 4-3: 14 topology tests pass, 3 benchmarks compile
- Plan 4-4: 12 GPU tests pass (graceful skip without adapter)

## TDD Compliance

- Level: strict
- Compliant cycles: 1/3 (33%)
- Violations: Plans 4-3 and 4-4 missing REFACTOR commits

## Deviations

- Auto-fixes: 9 total (3 in phase 4: euler_operators cross() fix, downstream solver errors, GPU multiview/naga fixes)
- Approval-needed: 0
- Notable: cgmath retained as dev-dependency in monstertruck-core; downstream crate compilation deferred

## Decisions Made

1. Connection parity (L/R) mapping confirmed correct per Equation 14 (Plan 4-1)
2. Matrix types as newtype wrappers to preserve cgmath column-major convention (Plan 4-2)
3. cgmath kept as dev-dependency due to AGENTS.md test-file restriction (Plan 4-2)
4. Downstream full compilation deferred to follow-up (Plan 4-2)
5. Lock ordering: surface -> curve -> point, all read guards (Plan 4-3)
6. Host-side adaptive refinement with fixed-grid shader for prototype (Plan 4-4)
