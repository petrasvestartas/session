# Verified at the pin — wgpu 29.0.4

Every claim in `SKILL.md` with the file and line it was read from, so a doubt
can be settled in one `sed -n`. Vendored root, written once here:

```bash
W=$(ls -d ~/.cargo/registry/src/*/wgpu-29.0.4)
T=$(ls -d ~/.cargo/registry/src/*/wgpu-types-29.0.4)
N=$(ls -d ~/.cargo/registry/src/*/naga-29.0.4)
```

Verified 2026-09-04 against `session_viewer/Cargo.lock` = wgpu 29.0.4
(`session_rust` resolves 29.0.3; the API facts below are identical in both).

## Pipeline layout

`$W/src/api/pipeline_layout.rs:38`

```rust
pub bind_group_layouts: &'a [Option<&'a BindGroupLayout>],
```

Every entry is wrapped. `session_viewer/src/engine/pipelines/mod.rs:152-157`
builds the vector and shows the `immediate_size` field in the same call:

```rust
let mut slots: Vec<Option<&wgpu::BindGroupLayout>> = Vec::with_capacity(groups.len());
for g in groups {
    slots.push(Some(*g));
}
device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some(label), bind_group_layouts: &slots, immediate_size: 0 })
```

## Instance

`$W/src/api/instance.rs:63`

```rust
pub fn new(desc: InstanceDescriptor) -> Self
```

By value. `$T/src/instance.rs:27` lists the fields: `backends`, `flags`,
`memory_budget_thresholds`, `backend_options`, `display`. The doc comment on
`backends` is worth reading before anyone proposes a WebGL fallback — it
explains that `Backends::BROWSER_WEBGPU` plus a present `navigator.gpu` makes
the instance WebGPU-only. `session_viewer/src/engine/gpu/device.rs:20-28`
selects `BROWSER_WEBGPU` on wasm and `PRIMARY` natively, on purpose.

## Adapter and device

`$W/src/api/instance.rs:167-170`

```rust
pub fn request_adapter(
    &self,
    options: &RequestAdapterOptions<'_, '_>,
) -> impl Future<Output = Result<Adapter, RequestAdapterError>> + WasmNotSend
```

`$W/src/api/adapter.rs:58-61`

```rust
pub fn request_device(
    &self,
    desc: &DeviceDescriptor<'_>,
) -> impl Future<Output = Result<(Device, Queue), RequestDeviceError>> + WasmNotSend
```

Both are `Result`, both are futures — `.await?`. Pre-29 tutorial code that does
`.await.unwrap()` on an `Option`, or wraps the whole thing in
`pollster::block_on`, is from a different API.

`session_viewer/src/engine/gpu/device.rs:57-66` is the working call, including
`..Default::default()` for the tail of `DeviceDescriptor` and
`device.on_uncaptured_error(...)` immediately after — that handler is how a
validation error becomes visible instead of a black frame.

## Limits

`$T/src/limits.rs:195`

```rust
pub max_inter_stage_shader_variables: u32,
```

`max_inter_stage_shader_components` no longer exists. The new one counts
locations, not scalars, so a numeric value carried over from old code is wrong
by roughly 4x.

Defaults live at `$T/src/limits.rs:397` (`Limits::default`) and `:502` /
`:590` for the downlevel sets. The viewer raises only two of them
(`max_storage_buffer_binding_size`, `max_buffer_size`) to the adapter's own
figure — `device.rs:50-56`.

## Buffer writes

`$W/src/api/buffer.rs:991`

```rust
pub fn slice<'a, S: RangeBounds<usize>>(&'a mut self, bounds: S) -> WriteOnly<'a, [u8]>
```

`$W/src/api/buffer.rs:905` states the rule directly: a `MapMode::Write` mapping
does **not** deref to `&mut [u8]`; `.slice()` returns the `WriteOnly` pointer
type. `copy_from_slice` and `into_chunks::<N>()` are the usual entry points.

## primitive_index

- `$T/src/features.rs:1787` — `const PRIMITIVE_INDEX = WEBGPU_FEATURE_PRIMITIVE_INDEX;`,
  inside `pub struct FeaturesWebGPU` which opens at `:1464`. It is *not* in
  `FeaturesWGPU` (`:611`), and the old `SHADER_PRIMITIVE_INDEX` bit is retired
  (`:1074`).
- `$N/src/front/wgsl/parse/directive/enable_extension.rs:128,215` — the WGSL
  side needs `enable primitive_index;` before `@builtin(primitive_index)` is
  accepted.

## Texture format

`$T/src/texture/format.rs:1617`

```rust
pub fn is_srgb(&self) -> bool
```

`describe().srgb()` is gone. `device.rs:71` picks the surface format with
`caps.formats.iter().find(|f| f.is_srgb())`.

## Re-verifying after a version bump

Read the changelog from the old pin forward first — it is written as
before/after diffs — then re-run these and rewrite this file:

```bash
W=$(ls -d ~/.cargo/registry/src/*/wgpu-<NEW>) ; T=$(ls -d ~/.cargo/registry/src/*/wgpu-types-<NEW>)
rg -n 'pub bind_group_layouts|pub struct PipelineLayoutDescriptor' $W/src/api/pipeline_layout.rs
rg -n 'pub fn new|pub fn request_adapter' $W/src/api/instance.rs
rg -n 'pub fn request_device' $W/src/api/adapter.rs
rg -n 'max_inter_stage' $T/src/limits.rs
rg -n 'pub fn slice|WriteOnly' $W/src/api/buffer.rs
rg -n 'PRIMITIVE_INDEX|pub struct Features' $T/src/features.rs
rg -n 'pub struct InstanceDescriptor' -A20 $T/src/instance.rs
```

Then `cd session_viewer && cargo check && cargo xtest`, and render one
`selftest` frame and compare the ink count with the pre-bump number. A bump
that compiles but changes the ink count changed the picture.
