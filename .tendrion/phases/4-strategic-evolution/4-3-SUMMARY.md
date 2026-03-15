---
phase: 4-strategic-evolution
plan: 3
tags: [topology, rwlock, concurrency, performance]
key-files:
  - monstertruck-topology/src/lib.rs
  - monstertruck-topology/src/vertex.rs
  - monstertruck-topology/src/edge.rs
  - monstertruck-topology/src/face.rs
  - monstertruck-topology/src/shell.rs
  - monstertruck-topology/src/wire.rs
  - monstertruck-topology/benches/rwlock_contention.rs
  - monstertruck-topology/tests/rwlock_migration.rs
decisions:
  - "Lock ordering documented as surface -> curve -> point (all read guards, no deadlock risk)"
  - "euler_operators.rs cross() API fixed as auto-fix deviation from nalgebra migration"
metrics:
  tests_passed: 14
  tests_skipped: 1
  deviations: 2
---

## What was built

Migrated topology types (`Vertex`, `Edge`, `Face`) from `parking_lot::Mutex` to `parking_lot::RwLock` for geometry data (`point`, `curve`, `surface` fields). This enables concurrent reads without blocking in read-heavy workloads like tessellation.

### Files created
- `monstertruck-topology/benches/rwlock_contention.rs` -- criterion benchmark with 3 scenarios (concurrent_read_points, concurrent_read_curves, mixed_read_write) and lock ordering documentation.
- `monstertruck-topology/tests/rwlock_migration.rs` -- 7 integration tests verifying RwLock type usage, concurrent reads, mixed read-write, and lock ordering safety.

### Files modified
- `monstertruck-topology/src/lib.rs` -- `Mutex` -> `RwLock` in struct definitions, type aliases, and `RwLockFmt`.
- `monstertruck-topology/src/vertex.rs` -- `.lock()` -> `.read()`/`.write()`.
- `monstertruck-topology/src/edge.rs` -- `.lock()` -> `.read()`/`.write()`, `Mutex::new` -> `RwLock::new`.
- `monstertruck-topology/src/face.rs` -- all 5 `Mutex::new` sites and all `.lock()` sites migrated.
- `monstertruck-topology/src/shell.rs` -- `.lock()` -> `.read()`.
- `monstertruck-topology/src/wire.rs` -- `.lock()` -> `.read()`.
- `monstertruck-topology/Cargo.toml` -- added criterion bench dependency.
- `monstertruck-topology/tests/euler_operators.rs` -- fixed `cross()` borrow (pre-existing nalgebra API issue).

## Task commits

| SHA | Message |
|-----|---------|
| `9fa9378b` | test(topology): add failing tests for RwLock migration and contention benchmark |
| `130245a4` | feat(topology): migrate Vertex, Edge, Face from Mutex to RwLock for concurrent reads |
| `de1110d9` | test(topology): add concurrent lock-ordering and mixed read-write tests |
| `8e5d1c7e` | docs(topology): document lock ordering and fix euler_operators cross() API |

## Deviations

1. **auto-fix/bug**: Pre-existing `euler_operators.rs` compilation error from nalgebra migration (`cross()` now requires borrow) -- fixed.
2. **auto-fix/dependency**: Downstream crates (geometry, meshing, modeling, solid) have pre-existing `solver` module compilation errors unrelated to RwLock migration -- cannot verify downstream tests.

## Lock ordering analysis

Multi-lock acquisition order: **surface -> curve -> point** (all read guards).
- `Edge::is_geometric_consistent`: curve.read() -> point.read().
- `Face::is_geometric_consistent`: surface.read() -> curve.read() -> point.read().
- Write access (`set_point`, `set_curve`, `set_surface`) acquires exactly one lock.
- No re-entrance risk: `mapped`/`try_mapped` methods document deadlock warning.

## Verification

- `cargo nextest run -p monstertruck-topology`: 14 passed, 1 skipped.
- `cargo test --benches -p monstertruck-topology`: 3 benchmarks compile and run.
- `grep -rn "Mutex" monstertruck-topology/src/`: 0 matches.
- Downstream verification blocked by pre-existing `solver` module errors in monstertruck-geometry.
