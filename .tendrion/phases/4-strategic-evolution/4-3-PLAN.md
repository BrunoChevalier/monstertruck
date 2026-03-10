---
phase: 4-strategic-evolution
plan: 3
type: tdd
wave: 2
depends_on: ["4-2"]
files_modified:
  - monstertruck-topology/Cargo.toml
  - monstertruck-topology/src/lib.rs
  - monstertruck-topology/src/vertex.rs
  - monstertruck-topology/src/edge.rs
  - monstertruck-topology/src/face.rs
  - monstertruck-topology/src/shell.rs
  - monstertruck-topology/src/wire.rs
  - monstertruck-topology/src/compress.rs
  - monstertruck-topology/benches/rwlock_contention.rs
autonomous: true
must_haves:
  truths:
    - "User runs topology traversal under concurrent read workloads and observes reduced contention vs Mutex baseline"
    - "All existing monstertruck-topology tests pass with RwLock replacing Mutex for geometry data"
    - "Tessellation of a shell with rayon parallelism works correctly with the RwLock topology"
    - "A benchmark demonstrates measurable contention reduction for concurrent read access patterns"
    - "No Mutex::new calls remain in any topology source file"
  artifacts:
    - path: "monstertruck-topology/src/lib.rs"
      provides: "Topology types using RwLock instead of Mutex for geometry data"
      min_lines: 300
      contains: "RwLock"
    - path: "monstertruck-topology/src/edge.rs"
      provides: "Edge implementation using RwLock for curve storage"
      min_lines: 300
      contains: "RwLock"
    - path: "monstertruck-topology/src/face.rs"
      provides: "Face implementation using RwLock for surface storage with all 5 construction sites migrated"
      min_lines: 800
      contains: "RwLock"
    - path: "monstertruck-topology/benches/rwlock_contention.rs"
      provides: "Benchmark comparing RwLock vs Mutex contention under concurrent reads"
      min_lines: 60
      contains: "criterion"
  key_links:
    - from: "monstertruck-topology/src/edge.rs"
      to: "monstertruck-topology/src/lib.rs"
      via: "Edge struct uses Arc<RwLock<C>> for curve field"
      pattern: "RwLock"
    - from: "monstertruck-topology/src/vertex.rs"
      to: "monstertruck-topology/src/lib.rs"
      via: "Vertex struct uses Arc<RwLock<P>> for point field"
      pattern: "RwLock"
    - from: "monstertruck-topology/src/face.rs"
      to: "monstertruck-topology/src/lib.rs"
      via: "Face struct uses Arc<RwLock<S>> for surface field"
      pattern: "RwLock"
    - from: "monstertruck-topology/benches/rwlock_contention.rs"
      to: "monstertruck-topology/src/lib.rs"
      via: "benchmark exercises concurrent read access on topology types"
      pattern: "read"
---

<objective>
Replace parking_lot::Mutex with parking_lot::RwLock for geometry data in topology types (Vertex, Edge, Face), reducing lock contention for read-heavy workloads like tessellation, and demonstrate the improvement with a benchmark.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-topology/src/lib.rs
@monstertruck-topology/src/vertex.rs
@monstertruck-topology/src/edge.rs
@monstertruck-topology/src/face.rs
@monstertruck-topology/src/shell.rs
@monstertruck-topology/src/wire.rs
@monstertruck-topology/Cargo.toml
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write contention benchmark and failing tests for RwLock behavior</name>
  <files>monstertruck-topology/benches/rwlock_contention.rs, monstertruck-topology/Cargo.toml</files>
  <action>
Create a benchmark that measures concurrent read contention on topology types:

1. **Add criterion to bench dependencies** in `monstertruck-topology/Cargo.toml`:
   ```toml
   [dev-dependencies]
   criterion = { workspace = true }

   [[bench]]
   name = "rwlock_contention"
   harness = false
   ```

2. **Create `monstertruck-topology/benches/rwlock_contention.rs`**:
   - Build a moderately complex topology (a shell with ~20 faces, ~40 edges, ~20 vertices).
   - Benchmark "concurrent_read_points": spawn N threads (e.g., via rayon) that concurrently read vertex points and edge curves. Measure throughput.
   - Benchmark "concurrent_read_surfaces": spawn N threads reading face surfaces concurrently.
   - Benchmark "mixed_read_write": one writer thread updating a vertex point while N readers read. Measure throughput.
   - Add rayon as a dev-dependency if not already present.
   - This benchmark establishes the baseline (currently Mutex) and will show improvement after migration.

3. **Record baseline**: Document expected behavior -- with Mutex, concurrent reads block each other; with RwLock, they should proceed in parallel.
  </action>
  <verify>Run `cargo bench -p monstertruck-topology --bench rwlock_contention` and confirm the benchmark compiles and produces baseline results.</verify>
  <done>Contention benchmark was created with baseline measurements using the current Mutex implementation.</done>
</task>

<task type="auto">
  <name>Task 2: Migrate Vertex, Edge, and Face from Mutex to RwLock</name>
  <files>monstertruck-topology/src/lib.rs, monstertruck-topology/src/vertex.rs, monstertruck-topology/src/edge.rs, monstertruck-topology/src/face.rs, monstertruck-topology/src/shell.rs, monstertruck-topology/src/wire.rs, monstertruck-topology/src/compress.rs</files>
  <action>
Systematically replace `Mutex` with `RwLock` in the topology crate. Use a comprehensive grep across ALL source files to find every Mutex site -- do not rely on a single file.

1. **Update `lib.rs`** (Mutex sites: lines 86, 110, 129, 160, 223, 235, 255, 427, 429, 431):
   - Change `use parking_lot::Mutex` to `use parking_lot::RwLock`
   - Update struct definitions:
     - `Vertex<P>`: change `point: Arc<Mutex<P>>` to `point: Arc<RwLock<P>>`
     - `Edge<P, C>`: change `curve: Arc<Mutex<C>>` to `curve: Arc<RwLock<C>>`
     - `Face<P, C, S>`: change `surface: Arc<Mutex<S>>` to `surface: Arc<RwLock<S>>`
   - Update type aliases:
     - `VertexId<P> = Id<RwLock<P>>`
     - `EdgeId<C> = Id<RwLock<C>>`
     - `FaceId<S> = Id<RwLock<S>>`
   - Update `MutexFmt` in format module to use `RwLock` (rename to `RwLockFmt` or keep name):
     - Change `.lock()` to `.read()` in Debug impl (line 431)

2. **Update `vertex.rs`** (Mutex site: line 17; .lock() sites: lines 43, 66, 79, 96):
   - Line 17: `Arc::new(Mutex::new(point))` -> `Arc::new(RwLock::new(point))`
   - `Vertex::point()` (line 43): `self.point.lock()` -> `self.point.read()`
   - `Vertex::set_point()` (line 66): `*self.point.lock()` -> `*self.point.write()`
   - `try_mapped()` (line 79): `&*self.point.lock()` -> `&*self.point.read()`
   - `mapped()` (line 96): `&*self.point.lock()` -> `&*self.point.read()`
   - MutexFmt refs at lines 195, 202, 205: update if renamed

3. **Update `edge.rs`** (Mutex::new sites: lines 45, 433, 438; .lock() sites: lines 265, 289, 344, 345, 363, 400, 416, 419, 420):
   - Line 45: `Arc::new(Mutex::new(curve))` -> `Arc::new(RwLock::new(curve))`
   - Lines 433, 438 (pre_cut): `Arc::new(Mutex::new(curve0/curve1))` -> `Arc::new(RwLock::new(...))`
   - All `.lock()` for reads (lines 265, 344, 345, 363, 400, 416, 419, 420) -> `.read()`
   - `.lock()` for writes (line 289) -> `.write()`
   - MutexFmt refs at lines 616, 638, 651: update if renamed

4. **Update `face.rs`** -- ALL 5 Mutex::new sites and ALL .lock() sites:
   - **Mutex::new sites** (5 total, all must be migrated):
     - Line 64: `new_unchecked()` constructor -> `Arc::new(RwLock::new(surface))`
     - Line 222: `set_surface()` rebuilder -> `Arc::new(RwLock::new(surface))`
     - Line 910: `cut()` method clone -> `Arc::new(RwLock::new(self.surface()))`
     - Line 933: `cut()` method second clone -> `Arc::new(RwLock::new(self.surface()))`
     - Line 1035: another construction site -> `Arc::new(RwLock::new(surface))`
   - **.lock() sites** (all must be classified as read or write):
     - Lines 435, 507: `surface_mapping(&*self.surface.lock()?)` -> `.read()`
     - Line 530: `self.surface.lock().clone()` -> `.read().clone()`
     - Line 559: `*self.surface.lock() = surface` -> `.write()`
     - Lines 1110, 1111: `self.surface.lock().clone()` / `.inverse()` -> `.read()`
     - Line 1126: `&*self.surface.lock()` -> `.read()`
     - Line 1129: `&*edge.curve.lock()` -> `.read()`
   - MutexFmt refs at lines 1247, 1273, 1296: update if renamed

5. **Update `shell.rs`** (.lock() sites: lines 586, 680):
   - Line 586: `surface_mapping(&*face.surface.lock()?)` -> `.read()`
   - Line 680: `surface_mapping(&*face.surface.lock())` -> `.read()`

6. **Update `wire.rs`** (.lock() sites: lines 589, 607):
   - Line 589: `curve_mapping(&*edge.curve.lock()?)` -> `.read()`
   - Line 607: `curve_mapping(&*edge.curve.lock())` -> `.read()`

7. **Final verification grep**: After all changes, run `grep -rn "Mutex" monstertruck-topology/src/` to confirm zero remaining Mutex references across ALL source files (not just lib.rs).

8. **Key consideration**: `parking_lot::RwLock` API:
   - `.read()` returns `RwLockReadGuard` (shared access)
   - `.write()` returns `RwLockWriteGuard` (exclusive access)
   - parking_lot RwLock is already in the dependencies, no new dependency needed
   - Both Mutex and RwLock from parking_lot have the same non-poisoning semantics
  </action>
  <verify>
`cargo test -p monstertruck-topology` passes all existing tests.
`cargo check -p monstertruck-topology` compiles cleanly.
`grep -rn "Mutex" monstertruck-topology/src/` returns 0 matches (no remaining Mutex in ANY source file -- vertex.rs, edge.rs, face.rs, lib.rs, shell.rs, wire.rs, compress.rs all checked).
  </verify>
  <done>Vertex, Edge, and Face were migrated from Mutex to RwLock across all source files with all existing tests passing.</done>
</task>

<task type="auto">
  <name>Task 3: Verify downstream compilation, deadlock safety, and run contention benchmark</name>
  <files>monstertruck-topology/benches/rwlock_contention.rs</files>
  <action>
1. **Verify ALL downstream crates compile**:
   - `cargo check -p monstertruck-geometry` -- uses topology types
   - `cargo check -p monstertruck-meshing` -- tessellation uses topology
   - `cargo check -p monstertruck-modeling` -- modeling builds on topology
   - `cargo check -p monstertruck-solid` -- solid operations use topology
   - Fix any compilation errors caused by the Mutex -> RwLock change in downstream code (likely `.lock()` calls that need to become `.read()` or `.write()`). Note: current codebase grep shows downstream crates do NOT call `.lock()` directly -- they use the public API methods (`.curve()`, `.surface()`, `.point()`), so compilation should succeed without changes. But verify explicitly.

2. **Run downstream tests**:
   - `cargo test -p monstertruck-geometry` -- ensure T-spline and geometry tests still pass
   - `cargo test -p monstertruck-meshing` -- ensure tessellation works with RwLock
   - `cargo test -p monstertruck-modeling` -- ensure modeling operations work
   - `cargo test -p monstertruck-solid` -- ensure solid operations work

3. **Run the contention benchmark** after migration:
   - `cargo bench -p monstertruck-topology --bench rwlock_contention`
   - Compare results with the baseline captured in Task 1
   - The concurrent_read benchmarks should show improved throughput
   - Document the before/after numbers in comments in the benchmark file

4. **Lock ordering analysis** for deadlock safety:
   - The topology code has patterns where multiple locks are held simultaneously (e.g., edge.rs line 416-420 acquires curve lock, then front/back point locks).
   - With RwLock, concurrent reads are safe (read-read does not deadlock). The risk is write-write or write-read ordering conflicts when multiple locks are acquired.
   - Document the lock acquisition order: curve -> point (in `is_geometric_consistent`). This order must be consistent across all code paths that acquire multiple locks.
   - Verify no code path acquires point lock first, then curve lock (which would risk deadlock with the above order under write contention).
   - Note: parking_lot RwLock does NOT support re-entrance. A thread holding a read lock that attempts to acquire a write lock on the SAME RwLock will deadlock. Verify no code path does this.
  </action>
  <verify>
`cargo test -p monstertruck-topology -p monstertruck-geometry -p monstertruck-meshing -p monstertruck-modeling -p monstertruck-solid` all pass.
`cargo bench -p monstertruck-topology --bench rwlock_contention` shows improved concurrent read performance.
  </verify>
  <done>All downstream crates compile and pass tests with RwLock topology, lock ordering is documented, and benchmark demonstrates reduced read contention.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-topology` passes all tests
2. `cargo test -p monstertruck-meshing` passes tessellation tests
3. `cargo test -p monstertruck-modeling -p monstertruck-solid` passes
4. `grep -rn "Mutex" monstertruck-topology/src/` returns no matches (checked all files, not just lib.rs)
5. `cargo bench -p monstertruck-topology --bench rwlock_contention` produces benchmark results
6. Downstream crates (geometry, meshing, modeling, solid) compile without errors
7. Lock ordering is documented for multi-lock acquisition patterns
</verification>

<success_criteria>
- Topology traversal uses RwLock instead of Mutex for geometry data
- All existing tests pass across topology and all downstream crates (including modeling and solid)
- A benchmark demonstrates reduced contention under concurrent read workloads
- Lock ordering is analyzed and documented (multi-lock, not re-entrance)
- No Mutex references remain in any topology source file
</success_criteria>

<output>
After completion, create `.tendrion/phases/4-strategic-evolution/4-3-SUMMARY.md`
</output>
