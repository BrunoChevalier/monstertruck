# Review Context: Plan 24-2 (Planning Review, Round 3)

## Round Information
- **Round:** 3 of 3
- **Review Type:** planning
- **Plan ID:** 24-2
- **Phase:** 24 (GPU Test Reliability)

---

## Plan Under Review

Plan ID: 24-2
Plan Path: .tendrion/phases/24-gpu-test-reliability/24-2-PLAN.md

---
phase: 24-gpu-test-reliability
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-gpu/tests/common.rs
  - monstertruck-gpu/tests/bindgroup.rs
  - monstertruck-gpu/tests/msaa.rs
  - monstertruck-gpu/tests/wgsl-utils.rs
autonomous: true
must_haves:
  truths:
    - "User runs `cargo nextest run -p monstertruck-gpu` on a machine without GPU hardware and all render tests skip gracefully with informative messages instead of panicking"
    - "User runs `cargo nextest run -p monstertruck-gpu` on a machine with GPU hardware and all render tests execute normally"
    - "User runs `cargo nextest run -p monstertruck-gpu --success-output final` and sees eprintln skip messages when no GPU adapter is available"
    - "User runs `cargo nextest run -p monstertruck-gpu` and no GPU-dependent test fails due to missing hardware (workspace-wide GPU test reliability for other crates like monstertruck-render is out of scope for this plan)"
  artifacts:
    - path: "monstertruck-gpu/tests/common.rs"
      provides: "A try_init_device function that returns Option<DeviceHandler> using .await.ok()? pattern, plus os_alt_try_exec_test helper"
      min_lines: 150
      contains: "try_init_device"
    - path: "monstertruck-gpu/tests/bindgroup.rs"
      provides: "Render test that skips gracefully when no GPU is available"
      min_lines: 80
      contains: "Skipping"
    - path: "monstertruck-gpu/tests/msaa.rs"
      provides: "Render test that skips gracefully when no GPU is available"
      min_lines: 40
      contains: "Skipping"
    - path: "monstertruck-gpu/tests/wgsl-utils.rs"
      provides: "Render test that skips gracefully when no GPU is available"
      min_lines: 50
      contains: "Skipping"
  key_links:
    - from: "monstertruck-gpu/tests/bindgroup.rs"
      to: "monstertruck-gpu/tests/common.rs"
      via: "try_init_device function call through os_alt_try_exec_test"
      pattern: "try_init_device"
    - from: "monstertruck-gpu/tests/msaa.rs"
      to: "monstertruck-gpu/tests/common.rs"
      via: "try_init_device function call through os_alt_try_exec_test"
      pattern: "try_init_device"
    - from: "monstertruck-gpu/tests/wgsl-utils.rs"
      to: "monstertruck-gpu/tests/common.rs"
      via: "try_init_device function call through os_alt_try_exec_test"
      pattern: "try_init_device"
---

<objective>
Make all GPU/render tests gracefully skip when no GPU hardware is available instead of panicking with an unwrap failure on adapter request. The compute_tessellation tests already handle this pattern correctly using `.await.ok()?`; apply the same proven pattern to the render tests (bindgroup, msaa, wgsl-utils). Also verify the entire workspace has no GPU test failures.
</objective>

<tasks>

<task type="auto">
  <name>Task 1: Add try_init_device and os_alt_try_exec_test to common.rs</name>
  <files>monstertruck-gpu/tests/common.rs</files>
  <action>
Add a new `try_init_device` function to `monstertruck-gpu/tests/common.rs` alongside the existing `init_device`. This function must use the **exact same pattern** as `compute_tessellation.rs:14` -- specifically using `.await.ok()?` for both `request_adapter` and `request_device`, NOT `.await?`.

The wgpu 28 API has `Instance::request_adapter` returning `Result<Adapter, RequestAdapterError>`, so `.await.ok()?` converts the `Result` into an `Option` to match the `Option<DeviceHandler>` return type.

Add this function after the existing `init_device` function (after line 152):

```rust
pub fn try_init_device(backends: Backends) -> Option<DeviceHandler> {
    pollster::block_on(async {
        let instance = Instance::new(&InstanceDescriptor {
            backends,
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok()?;
        writeln!(&mut std::io::stderr(), "{:?}", adapter.get_info()).unwrap();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                required_features: Default::default(),
                required_limits: Default::default(),
                memory_hints: Default::default(),
                experimental_features: ExperimentalFeatures::disabled(),
                trace: Default::default(),
                label: None,
            })
            .await
            .ok()?;
        Some(DeviceHandler::new(adapter, device, queue))
    })
}
```

Also add a new `os_alt_try_exec_test` helper function after `os_alt_exec_test` (after line 177):

```rust
pub fn os_alt_try_exec_test<F: Fn(DeviceHandler, &str)>(test_name: &str, test: F) {
    let _ = env_logger::try_init();
    let try_run = |backends: Backends, out_dir: &str| {
        match try_init_device(backends) {
            Some(handler) => test(handler, out_dir),
            None => {
                eprintln!("Skipping {test_name}: no GPU adapter available for {backends:?}.");
            }
        }
    };
    if cfg!(target_os = "windows") {
        try_run(Backends::VULKAN, "output/vulkan/");
        try_run(Backends::DX12, "output/dx12/");
    } else if cfg!(target_os = "macos") {
        try_run(Backends::METAL, "output/");
    } else {
        try_run(Backends::VULKAN, "output/");
    }
}
```

Keep the existing `init_device` and `os_alt_exec_test` functions unchanged. The `#![allow(dead_code)]` at the top of common.rs already suppresses unused warnings for helper functions.
  </action>
  <verify>Run `cargo clippy -p monstertruck-gpu --tests` and confirm no errors. The new functions should compile without warnings.</verify>
  <done>Added `try_init_device` (using `.await.ok()?` pattern) and `os_alt_try_exec_test` to common.rs.</done>
</task>

<task type="auto">
  <name>Task 2: Update render tests to use graceful skip pattern</name>
  <files>monstertruck-gpu/tests/bindgroup.rs, monstertruck-gpu/tests/msaa.rs, monstertruck-gpu/tests/wgsl-utils.rs</files>
  <action>
Update all three render test files to use `os_alt_try_exec_test` instead of `os_alt_exec_test`, changing their inner functions to accept `DeviceHandler` instead of `Backends`.

**bindgroup.rs:**
Change `exec_bind_group_test` signature from `fn exec_bind_group_test(backend: Backends, out_dir: &str)` to:
```rust
fn exec_bind_group_test(handler: DeviceHandler, out_dir: &str) {
```
Remove the line `let handler = common::init_device(backend);` (line 73) since handler is now a parameter.
Change the test function:
```rust
#[test]
fn bind_group_test() {
    common::os_alt_try_exec_test("bind_group_test", exec_bind_group_test);
}
```

**msaa.rs:**
Change `exec_msaa_test` signature from `fn exec_msaa_test(backend: Backends, out_dir: &str)` to:
```rust
fn exec_msaa_test(handler: DeviceHandler, out_dir: &str) {
```
Remove the line `let handler = common::init_device(backend);` (line 23) since handler is now a parameter.
Change the test function:
```rust
#[test]
fn msaa_test() {
    common::os_alt_try_exec_test("msaa_test", exec_msaa_test);
}
```

**wgsl-utils.rs:**
Change `exec_math_util_test` signature from `fn exec_math_util_test(backend: Backends, out_dir: &str)` to:
```rust
fn exec_math_util_test(handler: DeviceHandler, out_dir: &str) {
```
Remove the `common::init_device(backend)` call on line 30 and use the `handler` parameter instead:
```rust
let mut scene = Scene::new(handler, &desc);
```
Change the test function:
```rust
#[test]
fn math_util_test() {
    common::os_alt_try_exec_test("math_util_test", exec_math_util_test);
}
```

Note: `DeviceHandler` should already be available through `use monstertruck_gpu::*;` in each file. The `Backends` import may become unused in some files -- remove it from the `use wgpu::*` if clippy warns, or leave it since `wgpu::*` brings in many items.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-gpu -E 'test(bind_group_test) | test(msaa_test) | test(math_util_test)'` and confirm they either pass (with GPU) or skip gracefully (without GPU). Run `cargo clippy -p monstertruck-gpu --tests` for warnings.</verify>
  <done>All three render test files updated to use the graceful skip pattern via os_alt_try_exec_test.</done>
</task>

<task type="auto">
  <name>Task 3: Workspace-wide GPU test verification</name>
  <files>monstertruck-gpu/tests/common.rs</files>
  <action>
Run the complete test suite to verify no GPU test failures remain anywhere in the workspace.

**Step 1:** Run the full monstertruck-gpu test suite:
```
cargo nextest run -p monstertruck-gpu
```
Confirm all tests either pass or skip gracefully. No test should panic due to missing GPU hardware.

**Step 2:** Verify skip messages are visible by running with output enabled:
```
cargo nextest run -p monstertruck-gpu --success-output final -E 'test(bind_group_test) | test(msaa_test) | test(math_util_test)'
```
On a machine without GPU, confirm the output contains "Skipping" messages from eprintln. On a machine with GPU, confirm all tests pass normally.

**Step 3:** Run `cargo nextest run -p monstertruck-gpu` to confirm all monstertruck-gpu tests pass or skip gracefully. Note: `monstertruck-render` also has GPU tests with unconditional unwrap patterns, but fixing those is out of scope for this plan (RELY-02 targets `monstertruck-gpu` tests specifically). If a future phase needs workspace-wide GPU resilience, `monstertruck-render` tests would need the same `try_init_device` pattern applied.

**Step 4:** Run clippy across the GPU crate tests to confirm clean code:
```
cargo clippy -p monstertruck-gpu --tests -- -D warnings
```
Fix any clippy warnings that arise from the changes.

**Step 5:** Check that the compute_tessellation tests (which already had graceful skipping) and nurbs_shader_validation tests (which don't need GPU) still pass without issues.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-gpu` and confirm all tests pass or skip. Run `cargo clippy -p monstertruck-gpu --tests` for clean output.</verify>
  <done>monstertruck-gpu test verification complete. All GPU tests either pass or skip gracefully. No clippy warnings.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-gpu` runs all tests -- GPU tests either pass or skip gracefully
2. `cargo nextest run -p monstertruck-gpu --success-output final` shows "Skipping" messages when no GPU is available
3. `cargo clippy -p monstertruck-gpu --tests` produces no warnings
4. compute_tessellation tests continue to work with their existing skip pattern
5. nurbs_shader_validation tests (no GPU needed) still pass
</verification>

<success_criteria>
- All render tests (bindgroup, msaa, wgsl-utils) skip gracefully when no GPU adapter is available
- No test panics due to missing GPU hardware
- All tests pass when GPU hardware IS available
- try_init_device uses `.await.ok()?` matching the compute_tessellation.rs pattern
- RELY-02 fully satisfied for monstertruck-gpu (monstertruck-render GPU tests are noted but out of scope)
</success_criteria>

---

## Sibling Plans

| Plan ID | Wave | Objective |
|---------|------|-----------|
| 24-1 | 1 | Fix GPU camera proptest failures for degenerate point clouds with strictly positive clip intervals and screen sizes, scoped to monstertruck-gpu |

Full sibling plans can be read from .tendrion/phases/24-gpu-test-reliability/{sibling_plan_id}-PLAN.md if cross-plan analysis is needed.

---

## Roadmap: Phase 24

### Phase 24: GPU Test Reliability
**Goal**: GPU camera proptests pass on all inputs and render tests skip gracefully when no GPU hardware is available
**Depends on**: None
**Requirements**: RELY-01, RELY-02
**Success Criteria** (what must be TRUE):
  1. Running `cargo test` in monstertruck-rendimpl produces zero proptest failures for camera perspective and parallel view fitting
  2. Degenerate all-zero-point inputs are handled with explicit early returns or clamped defaults instead of NaN propagation
  3. GPU/render tests on a headless CI machine (no GPU) exit with skip status rather than failure
  4. `cargo test --workspace` shows no GPU-related test failures regardless of hardware availability

### Requirement Details

- **RELY-01**: Fix 2 GPU camera proptest failures caused by degenerate all-zero-point input handling in perspective and parallel view fitting
- **RELY-02**: Make GPU/render tests gracefully skip when no GPU hardware is available instead of failing

---

## Review Info

- Round: 3 of 3
- This is a subsequent round review. Check whether previous blockers were addressed.

## Previous Review (Round 2)

---
target: "24-2"
type: "planning"
round: 2
max_rounds: 3
reviewer: "codex"
stage: "planning"
date: "2026-03-23"
verdict: "FAIL"
confidence_threshold: 80
---

## Verdict

**FAIL**

**Rationale:** Round 1 blockers `B1` and `B2` are addressed, and suggestions `S1` and `S2` are reflected in the revised plan. Structural validation also passed. However, the plan now claims a clean `cargo nextest run --workspace` outcome without owning the other workspace GPU tests that still use the same unconditional adapter/device initialization pattern, so the workspace-level success criterion is not achievable as scoped.

## Findings

### Blockers

#### B3: Workspace-wide success is infeasible with the current file scope [confidence: 97]
- **Confidence:** 97
- **File:** `.tendrion/phases/24-gpu-test-reliability/24-2-PLAN.md:222, .tendrion/phases/24-gpu-test-reliability/24-2-PLAN.md:226, .tendrion/phases/24-gpu-test-reliability/24-2-PLAN.md:236, .tendrion/phases/24-gpu-test-reliability/24-2-PLAN.md:246, .tendrion/phases/24-gpu-test-reliability/24-2-PLAN.md:256`
- **Issue:** Task 3 now requires `cargo nextest run --workspace` and the verification/success criteria say there must be no GPU-related failures anywhere in the workspace, but the plan still only modifies `monstertruck-gpu/tests/*`. That is not enough in this workspace. `monstertruck-render` is a workspace member (`Cargo.toml:1-22`), `nextest` has no package exclusions (`.config/nextest.toml:1-2`), and `monstertruck-render` still has GPU tests that go through `common::init_device()` and `common::os_alt_exec_test()` with unconditional `.unwrap()` on adapter/device acquisition (`monstertruck-render/tests/common.rs:158-180,231-240`, `monstertruck-render/tests/render.rs:18-24,86-105,151-173`, `monstertruck-render/tests/polygon_bind_group.rs:100-107,151-219`, `monstertruck-render/tests/microfacet.rs:8-16,55-57`).
- **Impact:** An executor can complete every edit in `24-2` and still fail the mandated workspace smoke on a machine without GPU hardware. The line saying failures in other crates are out of scope and that the phase criterion is specifically about `monstertruck-gpu` conflicts with the plan's own workspace must-haves and success criteria, so the plan can report success for an outcome it does not actually satisfy.
- **Suggested fix:** Either expand this plan's scope to cover the analogous GPU tests in `monstertruck-render` (and any other workspace GPU tests reached by `cargo nextest run --workspace`), or narrow this plan back to `monstertruck-gpu` only and move the workspace-wide claim and validation to a separate plan that owns those fixes.

### Suggestions

None.

### Nits

None

## Summary

`B1` and `B2` are fixed in this revision: the plan now uses `cargo nextest run`, the `wgpu` 28 helper pattern is corrected to `.await.ok()?`, skip-message visibility is verified with `--success-output final`, and a workspace smoke step is explicitly assigned. The structure validator passed with zero errors or warnings.

The remaining problem is scope. The plan now promises a workspace-wide GPU pass, but the workspace still contains `monstertruck-render` GPU tests on the same failing init path and the plan does not own those files. Because that makes the stated workspace outcome unattainable as written, the review stays at `FAIL` on `B3`.
