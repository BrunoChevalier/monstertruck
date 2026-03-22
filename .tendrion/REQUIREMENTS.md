# Requirements: monstertruck v0.5.3

## Milestone Goal

Improve project quality across all dimensions: fix remaining test failures, dramatically increase code coverage for under-tested crates, complete deferred Ayam port items, add new CAD surface constructors, validate I/O pipelines, and update deprecated dependencies.

### Test Reliability

- [ ] **RELY-01**: Fix 2 GPU camera proptest failures caused by degenerate all-zero-point input handling in perspective and parallel view fitting
- [ ] **RELY-02**: Make GPU/render tests gracefully skip when no GPU hardware is available instead of failing
- [ ] **RELY-03**: Fix all clippy warnings in monstertruck-step (deprecated patterns, unused imports)
- [ ] **RELY-04**: Update deprecated dependencies (nom v3.2.1, quick-xml v0.22.0) flagged for rejection by future Rust versions

### Code Coverage

- [ ] **COV-01**: Increase monstertruck-solid test coverage from 0% to meaningful level with unit tests for boolean operations, fillet pipeline, and healing modules
- [ ] **COV-02**: Increase monstertruck-step test coverage from 0% to meaningful level with STEP import/export round-trip tests
- [ ] **COV-03**: Increase monstertruck-topology test coverage from 31% to 50%+ with edge, wire, face, and shell operation tests
- [ ] **COV-04**: Increase monstertruck-modeling test coverage from 27% to 45%+ with builder, profile, and text module tests
- [ ] **COV-05**: Increase monstertruck-core test coverage from 40% to 55%+ with tolerance infrastructure and error type tests
- [ ] **COV-06**: Add tests for monstertruck-traits (currently at 0% coverage) covering curve and surface trait implementations

### Deferred Ayam Port

- [ ] **PORT-01**: Implement intersection-grid driven Gordon surface variants with automatic grid point computation from curve families
- [ ] **PORT-02**: Improve trim tessellation robustness with fallback heuristics for trimmed surface boundary cases

### New CAD Features

- [ ] **CAD-01**: Implement ruled surface constructor that creates a surface between two boundary curves
- [ ] **CAD-02**: Implement loft surface through multiple cross-section profiles with interpolation control
- [ ] **CAD-03**: Expand geometry healing capabilities with gap repair, edge-curve consistency checking, and surface sewing

### I/O Quality

- [ ] **IO-01**: Add STEP export validation and round-trip fidelity tests verifying geometry preservation through write-read cycles
- [ ] **IO-02**: Add OBJ and STL mesh export validation tests verifying correct output format and geometry integrity

### Documentation

- [ ] **DOC-01**: Write migration guidance for manual workflow users covering new API patterns and deprecated function replacements

## Out of Scope

- Rail/section fixture corpus expansion (low priority, adequate fixtures exist)
- Near-degenerate NURBS fixture corpus (covered by existing stress corpus)
- Pipe sweep along path with varying profile (large scope, deferred to future milestone)

## Traceability

**Milestone:** v0.5.3
**Sources:** AYAM_PORT_PLAN.md deferred items, DEVIATIONS.md pre-existing issues, coverage analysis, CAD kernel feature research
