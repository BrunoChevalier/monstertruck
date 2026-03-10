---
phase: 3-feature-completeness
plan: 5
tags: [integration-test, re-export, solid-ops, step-export]
key-files:
  - monstertruck-solid/tests/feature_integration.rs
  - monstertruck-modeling/src/lib.rs
  - monstertruck-modeling/Cargo.toml
  - monstertruck-modeling/tests/solid_ops_reexport.rs
  - monstertruck-solid/src/lib.rs
  - monstertruck-solid/Cargo.toml
decisions:
  - "shell_then_step_export constructs hollow solid manually (OffsetCurve/OffsetSurface impls unavailable to integration tests due to circular dep)"
  - "Added OffsetCurve and OffsetSurface to solid-ops re-exports"
  - "Added ruststep as dev-dependency to monstertruck-solid for STEP parsing in integration tests"
metrics:
  tests_added: 6
  tests_passing: true
  clippy_clean: true
---

## What was built

### Files created
- `monstertruck-solid/tests/feature_integration.rs` (208 lines): Cross-feature integration tests validating boolean+chamfer, shell, draft, and chamfer STEP export workflows.
- `monstertruck-modeling/tests/solid_ops_reexport.rs`: Tests verifying solid-ops re-exports are accessible from the modeling crate.

### Files modified
- `monstertruck-modeling/Cargo.toml`: Added `solid-ops` feature flag; `fillet` now implies `solid-ops`.
- `monstertruck-modeling/src/lib.rs` (141 lines): Added `#[cfg(feature = "solid-ops")]` re-exports for `shell_solid`, `offset_shell`, `draft_faces`, `DraftOptions`, `DraftError`, `OffsetCurve`, `OffsetSurface`.
- `monstertruck-solid/src/lib.rs`: Updated crate-level documentation to reflect all Phase 3 operations.
- `monstertruck-solid/Cargo.toml`: Added `ruststep` dev-dependency.

## Task commits

| SHA | Message |
|-----|---------|
| `1b88fd31` | test(integration): add cross-feature integration tests for Phase 3 |
| `642d2052` | test(modeling): add failing test for solid-ops re-exports |
| `70dd6b71` | feat(modeling): add solid-ops feature flag with re-exports |
| `c5acf2ab` | refactor(modeling): also re-export OffsetCurve and OffsetSurface traits |
| `9500b3b4` | docs(solid): update crate-level documentation |

## Decisions

1. **Manual hollow solid construction**: `shell_solid` requires `OffsetCurve`/`OffsetSurface` trait impls that are only available in the solid crate's `#[cfg(test)]` module. Cannot make monstertruck-modeling a regular dependency due to circular dependency. The `shell_then_step_export` test constructs a multi-boundary solid manually instead.

2. **Trait re-exports**: Added `OffsetCurve` and `OffsetSurface` to the `solid-ops` re-exports so downstream users can implement them for custom types.

## Deviations

- Auto-fix (design): OffsetCurve/OffsetSurface trait impls unavailable to integration tests. Mitigated by manual construction of hollow solid.
- Auto-fix (dependency): Integration tests for existing features pass immediately (no RED phase failure expected).

## Self-check

- [x] `monstertruck-solid/tests/feature_integration.rs` exists (208 lines, contains `boolean_then_chamfer_step_export`)
- [x] `monstertruck-modeling/src/lib.rs` contains `shell_solid` (141 lines)
- [x] `cargo test -p monstertruck-solid --test feature_integration` -- 4 tests pass
- [x] `cargo test -p monstertruck-modeling --features solid-ops` -- all tests pass
- [x] `cargo test -p monstertruck-step --test output` -- 10 tests pass
- [x] `cargo clippy -p monstertruck-solid -p monstertruck-step -p monstertruck-modeling` -- clean
