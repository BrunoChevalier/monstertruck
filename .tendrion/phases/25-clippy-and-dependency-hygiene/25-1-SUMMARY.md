---
phase: 25-clippy-and-dependency-hygiene
plan: 1
tags: [dependency-update, vtkio, nom, quick-xml]
key-files:
  - Cargo.toml
  - monstertruck-step/examples/step-to-mesh.rs
decisions:
  - "Used Version::Auto instead of Version::new_xml(1, 0) -- the Auto variant handles versioning automatically for both legacy and XML formats, which is the recommended approach in vtkio 0.7"
metrics:
  tests_passed: 834
  tests_skipped: 4
  tests_failed: 0
  deviations: 1
---

## What was built

Updated the `vtkio` dependency from v0.6.3 to v0.7.0-rc2 to eliminate deprecated transitive dependencies `nom v3.2.1` and `quick-xml v0.22.0`.

### Files modified

- **Cargo.toml**: Updated `vtkio` from `"0.6"` to `"0.7.0-rc2"` in workspace dependencies.
- **monstertruck-step/examples/step-to-mesh.rs**: Replaced `version: (1, 0).into()` with `version: Version::Auto` (2 occurrences) to adapt to the `Version` type changing from a struct to an enum in vtkio 0.7.

### Files unchanged (no API breakage)

- **monstertruck-meshing/src/vtk.rs**: No changes needed. All vtkio types used (`IOBuffer`, `DataSet`, `Piece`, `VertexNumbers`, `CellType`, `Attributes`, `DataArray`, `ElementType`, `Cells`, `PolyDataPiece`, `UnstructuredGridPiece`) retained their APIs.
- **monstertruck-meshing/tests/vtk.rs**: No changes needed. All 7 VTK tests pass without modification.

## Task commits

| SHA | Message |
|-----|---------|
| `94dc4e09` | feat(deps): update vtkio 0.6 to 0.7.0-rc2, eliminating deprecated nom v3 and quick-xml v0.22 |
| `962c4222` | chore(td): log TDD deviation for vtkio dependency update |

## Verification results

- `cargo tree --workspace -i nom@3.2.1` -- "did not match any packages" (eliminated)
- `cargo tree --workspace -i quick-xml@0.22.0` -- "did not match any packages" (eliminated)
- `cargo clippy --workspace` -- exit code 0 (pre-existing warnings in monstertruck-mesh and monstertruck-solid only)
- `cargo nextest run -p monstertruck-meshing` -- 52 tests passed, 1 skipped
- `cargo nextest run --workspace` -- 834 tests passed, 4 skipped, 0 failed
- No `future-incompat` warnings in build output

## Deviations

1. **TDD exemption (auto-fix, dependency)**: Dependency update with existing test coverage. 7 VTK tests already validate the API contract and pass without modification. The only code change (Version enum migration) is in an example binary, not testable unit code.

## Self-check

- [x] `nom v3.2.1` eliminated from dependency tree
- [x] `quick-xml v0.22.0` eliminated from dependency tree
- [x] Workspace compiles with `cargo clippy`
- [x] All 834 workspace tests pass
- [x] No future-incompat warnings
- [x] RELY-04 satisfied
