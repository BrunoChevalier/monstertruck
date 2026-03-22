# Requirements: monstertruck v0.5.2

### Fillet Conversion Fidelity

- [ ] **FCONV-01**: Replace degree-1 piecewise-linear sampling in `sample_curve_to_nurbs` and `sample_surface_to_nurbs` with degree-3 cubic interpolation to preserve geometric fidelity across the fillet conversion pipeline
- [ ] **FCONV-02**: Add endpoint snapping in `convert_shell_in`/`convert_shell_out` to ensure converted curve endpoints exactly match vertex positions, preventing gap introduction that breaks shell closure
- [ ] **FCONV-03**: Implement exact `RevolutedCurve` to `NurbsSurface` conversion via rational circle arc tensor product, eliminating the sampling fallback path for this common surface type

### Edge Identity & Topology

- [ ] **ETOPO-01**: Fix `ensure_cuttable_edge` in the fillet pipeline to preserve `Edge` arc identity when re-wrapping IntersectionCurve edges as NURBS, preventing `is_same()` failures in boundary replacement
- [ ] **ETOPO-02**: Widen endpoint matching tolerance in `convert_shell_in` from `TOLERANCE` (1e-6) to `SNAP_TOLERANCE` (1e-5) for edges originating from boolean operations with inherent positional noise

### Error Reporting & Testing

- [ ] **EREP-01**: Replace silent rollback in `fillet_edges_generic` (lines 733-738) with explicit error propagation, returning `Err(FilletError)` instead of silently restoring the original shell
- [ ] **EREP-02**: Fix `test_unit_circle` proptest assertion to use relative tolerance (magnitude-aware) instead of absolute `prop_assert_near!` for rolling-ball fillet circle radius validation

## Out of Scope

- Full NURBS fitting with least-squares optimization (cubic interpolation is sufficient for this milestone)
- Adaptive sample count based on curvature analysis (fixed higher count is acceptable)
- Exact conversion for arbitrary `Processor<T, Matrix4>` surface types beyond `RevolutedCurve`

## Traceability

This requirements document covers milestone **v0.5.2**, fixing the generic fillet conversion pipeline to resolve 7 pre-existing test failures.
