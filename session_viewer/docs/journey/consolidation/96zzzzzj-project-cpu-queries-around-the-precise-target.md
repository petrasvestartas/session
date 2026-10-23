# 21.2 · Precise CPU queries and origin-stamped picks

**Goal:** Refresh local draw inputs without replacing the document or cancelling its gesture. · **Steps:** 7 · **Source:** [camera.rs](../../../src/camera.rs) · [state.rs](../../../src/state.rs) · [engine/gpu/objects.rs](../../../src/engine/gpu/objects.rs) · [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs) · [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs) · [shaders/grid.wgsl](../../../src/shaders/grid.wgsl) · [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl) · [app/cplane.rs](../../../src/app/cplane.rs) (`src/camera.rs,src/app/units.rs,src/engine/gpu/anchor.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 21.2 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/snap_math.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/editor.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/trim_draft.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">7 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/units.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Project CPU queries aroun…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. f64 Camera Anchor Subtract</text>
  <rect x="596" y="52" width="208" height="38" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">p_rel = (p_f64 - anchor_f64)</text>
  <rect x="596" y="100" width="208" height="38" rx="5" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Zero f32 Precision Jitter</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Precise CPU queries and ori…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Unit-Aware Physical Camera</text>
  <rect x="882" y="52" width="188" height="38" rx="5" fill="#dcfce7" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Units: mm / m / inch</text>
  <rect x="882" y="100" width="188" height="38" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Physical Viewport Limits</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Share derived inputs for…</text>
</svg>
</div>

Make rectangles, lasso, handles and asynchronous picks agree with the current anchored frame.

## Ownership and data flow

CPU queries subtract the precise target/origin before projecting source world coordinates. Label polygons, rectangle/lasso selection and handle math share that convention. An asynchronous pick records the origin that produced its pixels; a rebase invalidates that semantic result and retries with the latest cursor rather than decoding an old address in a new frame. Derived caches retain source/view/origin witnesses so refresh updates every dependent input coherently.

## The maintained implementation

camera.rs ray and projection methods operate in actual scene units; state/box_select.rs and app/gesture own current query input. objects.rs Rebase reports moved/pending origin state and engine/gpu/pick.rs owns active readback resources. Course origin-stamped ticket types explicitly stage the invalidation contract; production names and completion guards remain authoritative.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [camera.rs](../../../src/camera.rs), [state.rs](../../../src/state.rs), [engine/gpu/objects.rs](../../../src/engine/gpu/objects.rs), [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs), [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs), [shaders/grid.wgsl](../../../src/shaders/grid.wgsl), [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl), [app/cplane.rs](../../../src/app/cplane.rs), [build.rs](../../../build.rs).

## <a id="checkpoint-96zzzzzj-project-cpu-queries-around-the-precise-target"></a>1. Project CPU queries around the precise target

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/snap_math.rs, src/edito…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">World query − precise tar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local camera matrix</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pixel query</text>
</svg>
</div>

**What this code does:** Keep CPU screen queries precise before converting world points to pixels.

### 1. `src/snap_math.rs`

Retain the coordinate frame in query and cache identity. Build one matching precise origin and local camera matrix for CPU queries. Subtract the precise origin before matrix products; retain existing clipping and finite guards.

**Locate this existing block:**

```rust
    pub size: [f64; 2],
}

impl Screen {
    pub fn point(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        if self
            .size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let clip: [f64; 4] = std::array::from_fn(|row| {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub size: [f64; 2],
    pub origin: [f64; 3],
}

impl Screen {
    pub fn camera(camera: &crate::camera::Camera, size: [f64; 2]) -> Self {
        Self {
            matrix: camera
                .view_projection_anchored(camera.target)
                .m,
            size,
            origin: camera.target,
        }
    }
    pub fn point(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        if self
            .size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let point = std::array::from_fn::<_, 3, _>(|i| point[i] - self.origin[i]);
        let clip: [f64; 4] = std::array::from_fn(|row| {
```

### 2. `src/editor.rs`

Reuse the precise camera query frame at the existing editor call site.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            })
            .and_then(|d| {
                let point = *d.points.first()?;
                let viewport = crate::snap_math::Screen {
                    matrix: self.camera.view_projection().m,
                    size,
                };
                let p = viewport.point(point)?;
```

Replace it with:

```rust
            })
            .and_then(|d| {
                let point = *d.points.first()?;
                let viewport = crate::snap_math::Screen::camera(&self.camera, size);
                let p = viewport.point(point)?;
```

**Locate this block:**

```rust
            let index = self.snap_index.get_or_insert_with(|| {
                crate::snap_index::Index::new(&self.scene).excluding(&exclude)
            });
            let viewport = crate::snap_math::Screen {
                matrix: self.camera.view_projection().m,
                size,
            };
            index.query(
                viewport,
                &ray,
```

Replace it with:

```rust
            let index = self.snap_index.get_or_insert_with(|| {
                crate::snap_index::Index::new(&self.scene).excluding(&exclude)
            });
            let viewport = crate::snap_math::Screen::camera(&self.camera, size);
            index.query(
                viewport,
                &ray,
```

**Locate this block:**

```rust
            .as_ref()
            .and_then(|d| d.extend.clone())
            .ok_or("No extension tool is running")?;
        let view = crate::snap_math::Screen {
            matrix: self.camera.view_projection().m,
            size,
        };
        next.follow(
            &self.scene,
            &view,
```

Replace it with:

```rust
            .as_ref()
            .and_then(|d| d.extend.clone())
            .ok_or("No extension tool is running")?;
        let view = crate::snap_math::Screen::camera(&self.camera, size);
        next.follow(
            &self.scene,
            &view,
```

**Locate this block:**

```rust
        let point = edit.gizmo.handle_point(handle, scale);
        let at = crate::snap_math::Screen {
            matrix: matrix.m,
            size,
        }
        .point([point[0], point[1], point[2]])?;
        let unit = handle.labels().2;
        Some(crate::number_box::NumberPrompt {
            handle,
```

Replace it with:

```rust
        let point = edit.gizmo.handle_point(handle, scale);
        let at = crate::snap_math::Screen::camera(&self.camera, size)
            .point([point[0], point[1], point[2]])?;
        let unit = handle.labels().2;
        Some(crate::number_box::NumberPrompt {
            handle,
```

### 3. `src/trim_draft.rs`

Project trim boundaries through the same CPU frame owner.

**Locate this existing block:**

```rust
    ) -> Option<(usize, usize)> {
        let project = crate::snap_math::Screen {
            matrix: camera.view_projection().m,
            size,
        };
        let at = (
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Option<(usize, usize)> {
        let project = crate::snap_math::Screen::camera(camera, size);
        let at = (
```

### 4. `src/snap_tests.rs`

Keep every original arbitrary-matrix snapping assertion in its explicit zero-origin frame.

**Locate this existing block:**

```rust
        size: [200., 100.],
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        size: [200., 100.],
        origin: [0.; 3],
    }
```

### 5. `src/camera_space_tests.rs`

Supply exact distant/local CPU pixels, equal-matrix origin witnesses and retained clip/finite/size guards without changing source coordinates.

**Locate this existing block:**

```rust
    assert_eq!(camera.target, [1e12, -1e12, 1e12]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(camera.target, [1e12, -1e12, 1e12]);
}

#[test]
fn cpu_screen_queries_keep_distant_pixels_and_original_clip_guards() {
    use crate::snap_math::Screen;
    let anchor = [1e12, -1e12, 1e12];
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        camera.projection = projection;
        camera.distance = 2.;
        let baseline = Screen::camera(&camera, [640., 480.]);
        camera.target = anchor;
        let far = Screen::camera(&camera, [640., 480.]);
        assert_eq!(baseline.matrix, far.matrix);
        assert!(baseline != far);
        for point in [
            [0.25, 0.125, -0.5],
            [-0.125, -0.25, 0.125],
            [0., 0., 0.],
        ] {
            let world = std::array::from_fn(|i| anchor[i] + point[i]);
            assert_eq!(far.point(world), baseline.point(point));
            assert_eq!(far.origin, anchor);
        }
    }
    let mut view = Screen {
        matrix: session_rust::Xform::identity().m,
        size: [200., 100.],
        origin: anchor,
    };
    assert_eq!(
        view.point([
            anchor[0] + 0.25,
            anchor[1] + 0.125,
            anchor[2] + 0.5
        ]),
        Some([125., 43.75])
    );
    for point in [
        [anchor[0], anchor[1], anchor[2] - 0.5],
        [anchor[0], anchor[1], anchor[2] + 1.5],
        [f64::NAN, 0., 0.],
    ] {
        assert_eq!(view.point(point), None);
    }
    view.size[0] = 0.;
    assert_eq!(view.point(anchor), None);
    view.size[0] = 200.;
    view.origin[0] = f64::NAN;
    assert_eq!(view.point(anchor), None);
    view.origin = anchor;
    view.matrix[15] = 0.;
    assert_eq!(view.point(anchor), None);
}
```

## <a id="checkpoint-96zzzzzk-prepare-precise-text-selection-polygons"></a>2. Prepare precise text selection polygons

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/snap_math.rs, src/rect_…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original text placement −…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local polygon</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared pixel projection</text>
</svg>
</div>

**What this code does:** Project original text bounds in a precise local query frame.

### 1. `src/snap_math.rs`

Share coordinate conversion while preserving depth, positive-w and logical-size rules.

**Locate this existing block:**

```rust
    }
    pub fn point(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        if self
            .size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let point = std::array::from_fn::<_, 3, _>(|i| point[i] - self.origin[i]);
        let clip: [f64; 4] = std::array::from_fn(|row| {
            point
                .iter()
                .enumerate()
                .fold(self.matrix[12 + row], |sum, (i, v)| {
                    sum + self.matrix[i * 4 + row] * v
                })
        });
        if clip.iter().any(|v| !v.is_finite())
            || clip[3] <= 1e-9
            || clip[2] < 0.
            || clip[2] > clip[3]
        {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn clip(&self, point: [f64; 3]) -> [f64; 4] {
        let point = std::array::from_fn::<_, 3, _>(|i| point[i] - self.origin[i]);
        let clip: [f64; 4] = std::array::from_fn(|row| {
            point
                .iter()
                .enumerate()
                .fold(self.matrix[12 + row], |sum, (i, v)| {
                    sum + self.matrix[i * 4 + row] * v
                })
        });
        clip
    }
    pub fn point(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        let clip = self.clip(point);
        if clip.iter().any(|v| !v.is_finite())
            || clip[3] <= 1e-9
            || clip[2] < 0.
            || clip[2] > clip[3]
        {
            return None;
        }
        self.pixel(clip)
    }
    pub fn unclipped(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        let clip = self.clip(point);
        if !clip[3].is_finite() || clip[3] <= 0. {
            return None;
        }
        self.pixel(clip)
    }
    fn pixel(&self, clip: [f64; 4]) -> Option<[f64; 2]> {
        if self
            .size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
```

### 2. `src/rect_select.rs`

Reuse the shared projection without adding depth clipping to region selection.

**Locate this existing block:**

```rust
pub fn screen(matrix: &[f64; 16], point: [f64; 3], size: [f64; 2]) -> Option<[f64; 2]> {
    let clip = |row: usize| {
        point
            .iter()
            .enumerate()
            .map(|(i, value)| matrix[i * 4 + row] * value)
            .sum::<f64>()
            + matrix[12 + row]
    };
    let w = clip(3);
    if !w.is_finite()
        || w <= 0.
        || size
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.)
    {
        return None;
    }
    let point = [
        (clip(0) / w + 1.) * size[0] / 2.,
        (1. - clip(1) / w) * size[1] / 2.,
    ];
    point
        .iter()
        .all(|value| value.is_finite())
        .then_some(point)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn screen(matrix: &[f64; 16], point: [f64; 3], size: [f64; 2]) -> Option<[f64; 2]> {
    crate::snap_math::Screen {
        matrix: *matrix,
        size,
        origin: [0.; 3],
    }
    .unclipped(point)
}
```

### 3. `src/scene_label_screen.rs`

Use the explicit query frame instead of a second projection recipe. Derive text coordinates after precise placement subtraction; keep the original source owner. Construct small text-plane corners in the local frame. Compute billboard scale from the same local clip transform. Project the local text anchor through the shared query. Keep the existing window/crossing polygon predicate with an explicit frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    size: [f64; 2],
    rectangle: Rectangle,
) -> bool {
    let Some(points) = polygon(row, matrix, size) else {
        return false;
```

Replace it with:

```rust
    size: [f64; 2],
    rectangle: Rectangle,
) -> bool {
    matches_in(
        row,
        &Screen {
            matrix: *matrix,
            size,
            origin: [0.; 3],
        },
        rectangle,
    )
}
pub fn matches_in(row: &Row, view: &Screen, rectangle: Rectangle) -> bool {
    let Some(points) = polygon_in(row, view) else {
        return false;
```

**Locate this block:**

```rust
        }
        _ => return None,
    };
    let [mut left, mut top] = screen(matrix, world, size)?;
    left += offset[0] as f64;
    top += offset[1] as f64;
    if center {
```

Replace it with:

```rust
        }
        _ => return None,
    };
    let [mut left, mut top] = local.unclipped(world)?;
    left += offset[0] as f64;
    top += offset[1] as f64;
    if center {
```

**Locate this block:**

```rust
            world,
            world_height,
        } => {
            let w = (0..3)
                .map(|i| matrix[i * 4 + 3] * world[i])
                .sum::<f64>()
                + matrix[15];
            let projection = [matrix[1], matrix[5], matrix[9]]
                .iter()
                .map(|n| n * n)
```

Replace it with:

```rust
            world,
            world_height,
        } => {
            let w = local.clip(world)[3];
            let projection = [matrix[1], matrix[5], matrix[9]]
                .iter()
                .map(|n| n * n)
```

**Locate this block:**

```rust
        ]
        .into_iter()
        .map(|[x, y]| {
            screen(
                matrix,
                std::array::from_fn(|i| world[i] + (right[i] * x - up[i] * y) * unit),
                size,
            )
        })
        .collect();
    }
```

Replace it with:

```rust
        ]
        .into_iter()
        .map(|[x, y]| {
            local.unclipped(std::array::from_fn(|i| {
                world[i] + (right[i] * x - up[i] * y) * unit
            }))
        })
        .collect();
    }
```

**Locate this block:**

```rust
use crate::{
    rect_select::{Rectangle, screen},
    scene_label::Row,
    text::TextPlacement,
};

pub fn polygon(row: &Row, matrix: &[f64; 16], size: [f64; 2]) -> Option<Vec<[f64; 2]>> {
    let label = row.label(false, true).ok()?;
    let [width, height] = row.prepared.extent;
    if let TextPlacement::WorldPlane {
```

Replace it with:

```rust
use crate::{
    rect_select::Rectangle, scene_label::Row, snap_math::Screen, text::TextPlacement,
};

pub fn polygon(row: &Row, matrix: &[f64; 16], size: [f64; 2]) -> Option<Vec<[f64; 2]>> {
    polygon_in(
        row,
        &Screen {
            matrix: *matrix,
            size,
            origin: [0.; 3],
        },
    )
}
pub fn polygon_in(row: &Row, view: &Screen) -> Option<Vec<[f64; 2]>> {
    let row = Row {
        model: crate::render_space::placed(&row.model, view.origin),
        ..row.clone()
    };
    let local = Screen {
        matrix: view.matrix,
        size: view.size,
        origin: [0.; 3],
    };
    let matrix = &local.matrix;
    let size = local.size;
    let label = row.label(false, true).ok()?;
    let [width, height] = row.prepared.extent;
    if let TextPlacement::WorldPlane {
```

### 4. `src/camera_space_tests.rs`

Supply exact local/distant plane/billboard/anchor/nameplate polygons and unchanged source owners, retaining distinct unused-depth, tiny-w, finite and size predicates.

**Locate this existing block:**

```rust
    assert_eq!(view.point(anchor), None);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(view.point(anchor), None);
}

#[test]
fn text_query_polygons_keep_precise_placements_sources_and_unclipped_rules() {
    use crate::{snap_math::Screen, text::TextPlacement};
    let anchor = [1e12, -1e12, 1e12];
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        camera.projection = projection;
        camera.distance = 3.;
        let view = Screen::camera(&camera, [800., 600.]);
        camera.target = anchor;
        let far_view = Screen::camera(&camera, [800., 600.]);
        for placement in [
            TextPlacement::WorldPlane {
                world: [0.125, 0.25, -0.5],
                right: [1., 0., 0.],
                up: [0., 1., 0.],
                world_height: 0.25,
            },
            TextPlacement::WorldBillboard {
                world: [0.125, 0.25, -0.5],
                world_height: 0.25,
            },
            TextPlacement::Anchor {
                world: [0.125, 0.25, -0.5],
                offset: [3., 5.],
            },
            TextPlacement::Nameplate {
                world: [0.125, 0.25, -0.5],
                padding: [4., 6.],
                rounded: true,
            },
        ] {
            let mut scene = crate::scene::Scene::empty();
            let source = std::rc::Rc::new(crate::scene_label::example(placement));
            let id = scene
                .insert_text(crate::scene_label::Prepared::new(source.clone()).unwrap())
                .unwrap();
            let baseline =
                crate::scene_label_screen::polygon_in(&scene.texts()[0], &view)
                    .unwrap();
            let original = serde_json::to_vec(source.as_ref()).unwrap();
            scene.translate(id, anchor).unwrap();
            assert_eq!(
                crate::scene_label_screen::polygon_in(&scene.texts()[0], &far_view)
                    .unwrap(),
                baseline
            );
            assert!(std::rc::Rc::ptr_eq(
                &scene.texts()[0].prepared.source,
                &source
            ));
            assert_eq!(
                serde_json::to_vec(source.as_ref()).unwrap(),
                original
            );
            let bounds = [
                baseline
                    .iter()
                    .map(|p| p[0])
                    .fold(f64::INFINITY, f64::min)
                    - 1.,
                baseline
                    .iter()
                    .map(|p| p[1])
                    .fold(f64::INFINITY, f64::min)
                    - 1.,
                baseline
                    .iter()
                    .map(|p| p[0])
                    .fold(f64::NEG_INFINITY, f64::max)
                    + 1.,
                baseline
                    .iter()
                    .map(|p| p[1])
                    .fold(f64::NEG_INFINITY, f64::max)
                    + 1.,
            ];
            let rectangle = crate::rect_select::Rectangle {
                bounds,
                crossing: false,
            };
            assert!(crate::scene_label_screen::matches_in(
                &scene.texts()[0],
                &far_view,
                rectangle
            ));
        }
    }
    let mut view = Screen {
        matrix: session_rust::Xform::identity().m,
        size: [200., 100.],
        origin: [0.; 3],
    };
    assert!(view.unclipped([0., 0., -1.]).is_some());
    assert!(view.point([0., 0., -1.]).is_none());
    view.matrix[2] = f64::NAN;
    assert_eq!(
        view.unclipped([0.25, 0.125, 0.5]),
        Some([125., 43.75])
    );
    assert!(view.point([0.25, 0.125, 0.5]).is_none());
    view.matrix = session_rust::Xform::identity().m;
    view.matrix[15] = 1e-10;
    assert!(view.unclipped([0.25, 0.125, 0.5]).is_some());
    assert!(view.point([0.25, 0.125, 0.5]).is_none());
    view.matrix[15] = 0.;
    assert!(view.unclipped([0.; 3]).is_none());
    view.matrix[15] = 1.;
    view.size[0] = 0.;
    assert!(view.unclipped([0.; 3]).is_none());
}
```

## <a id="checkpoint-96zzzzzl-connect-precise-rectangle-selection-queries"></a>3. Connect precise rectangle selection queries

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/rect_select.rs, src/edi…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original source + query-r…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unclipped pixels</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">window/crossing predicate</text>
</svg>
</div>

**What this code does:** Select original geometry and text through one precise CPU frame.

### 1. `src/rect_select.rs`

Reuse one rectangle predicate while placing each original family in the explicit query frame.

**Locate this existing block:**

```rust
}

fn world(matrix: &session_rust::Xform, point: [f64; 3]) -> [f64; 3] {
    let point = matrix.transform_point(&session_rust::Point::new(
        point[0], point[1], point[2],
    ));
    [point[0], point[1], point[2]]
}

fn matches(
    matrix: &[f64; 16],
    size: [f64; 2],
    rectangle: Rectangle,
    strokes: Vec<Vec<[f64; 3]>>,
) -> bool {
    if strokes.is_empty() {
        return false;
    }
    if !rectangle.crossing {
        return strokes.iter().flatten().all(|point| {
            screen(matrix, *point, size).is_some_and(|point| rectangle.contains(point))
        });
    }
    strokes.iter().any(|stroke| {
        let points = stroke
            .iter()
            .map(|point| screen(matrix, *point, size))
            .collect::<Vec<_>>();
        points
            .iter()
            .flatten()
            .any(|point| rectangle.contains(*point))
            || points.windows(2).any(
                |pair| matches!(pair,[Some(a),Some(b)] if rectangle.crosses(*a,*b)),
            )
    })
}

pub fn found(
    scene: &Scene,
    matrix: &[f64; 16],
    size: [f64; 2],
    rectangle: Rectangle,
) -> Vec<ObjectId> {
    let eligible = |id: ObjectId| scene.pick_id(id.value()).is_some();
    let mut found = Vec::new();
    for row in scene
        .objects()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let mut strokes = row
            .mesh
            .parts
            .endpoints
            .iter()
            .map(|ends| {
                ends.iter()
                    .map(|point| world(&row.model, *point))
                    .collect()
            })
            .collect::<Vec<_>>();
        if let Some(source) = row.geometry() {
            strokes.extend(
                source
                    .vertices()
                    .into_iter()
                    .filter_map(|key| source.vertex_point(key))
                    .map(|point| {
                        vec![world(
                            &row.model,
                            [point[0], point[1], point[2]],
                        )]
                    }),
            );
        }
        if matches(matrix, size, rectangle, strokes) {
            found.push(row.id);
        }
    }
    for row in scene
        .lines()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if matches(
            matrix,
            size,
            rectangle,
            vec![
                row.endpoints()
                    .map(|point| [point[0], point[1], point[2]])
                    .to_vec(),
            ],
        ) {
            found.push(row.id);
        }
    }
    for row in scene
        .paths()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if matches(
            matrix,
            size,
            rectangle,
            vec![
                row.points()
                    .into_iter()
                    .map(|point| [point[0], point[1], point[2]])
                    .collect(),
            ],
        ) {
            found.push(row.id);
        }
    }
    for row in scene
        .points()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let point = row.point();
        if matches(
            matrix,
            size,
            rectangle,
            vec![vec![[point[0], point[1], point[2]]]],
        ) {
            found.push(row.id);
        }
    }
    for row in scene
        .texts()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if crate::scene_label_screen::matches(row, matrix, size, rectangle) {
            found.push(row.id);
        }
    }
    for row in scene
        .clouds()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let mut points = row
            .prepared
            .source
            .coords()
            .chunks_exact(3)
            .map(|p| {
                screen(
                    matrix,
                    world(&row.model, [p[0], p[1], p[2]]),
                    size,
                )
            });
        let hit = if rectangle.crossing {
            points.any(|p| p.is_some_and(|p| rectangle.contains(p)))
        } else {
            points.all(|p| p.is_some_and(|p| rectangle.contains(p)))
        };
        if hit {
            found.push(row.id);
        }
    }
    for row in scene
        .sheets()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let strokes = row
            .prepared
            .pages()
            .iter()
            .flat_map(|page| {
                page.coordinates
                    .chunks_exact(6)
                    .map(|p| {
                        vec![
                            world(&row.model, [p[0], p[1], p[2]]),
                            world(&row.model, [p[3], p[4], p[5]]),
                        ]
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        if matches(matrix, size, rectangle, strokes) {
            found.push(row.id);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

fn matches(
    view: &crate::snap_math::Screen,
    rectangle: Rectangle,
    strokes: Vec<Vec<[f64; 3]>>,
) -> bool {
    if strokes.is_empty() {
        return false;
    }
    if !rectangle.crossing {
        return strokes.iter().flatten().all(|point| {
            view.unclipped(*point)
                .is_some_and(|point| rectangle.contains(point))
        });
    }
    strokes.iter().any(|stroke| {
        let points = stroke
            .iter()
            .map(|point| view.unclipped(*point))
            .collect::<Vec<_>>();
        points
            .iter()
            .flatten()
            .any(|point| rectangle.contains(*point))
            || points.windows(2).any(
                |pair| matches!(pair,[Some(a),Some(b)] if rectangle.crosses(*a,*b)),
            )
    })
}

pub fn found(
    scene: &Scene,
    matrix: &[f64; 16],
    size: [f64; 2],
    rectangle: Rectangle,
) -> Vec<ObjectId> {
    found_in(
        scene,
        &crate::snap_math::Screen {
            matrix: *matrix,
            size,
            origin: [0.; 3],
        },
        rectangle,
    )
}
pub fn found_in(
    scene: &Scene,
    view: &crate::snap_math::Screen,
    rectangle: Rectangle,
) -> Vec<ObjectId> {
    let origin = view.origin;
    let local = crate::snap_math::Screen {
        matrix: view.matrix,
        size: view.size,
        origin: [0.; 3],
    };
    let placed = |model: &session_rust::Xform, p: [f64; 3]| {
        let model = crate::render_space::placed(model, origin);
        let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
        [p[0], p[1], p[2]]
    };
    let eligible = |id: ObjectId| scene.pick_id(id.value()).is_some();
    let mut found = Vec::new();
    for row in scene
        .objects()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let mut strokes = row
            .mesh
            .parts
            .endpoints
            .iter()
            .map(|ends| {
                ends.iter()
                    .map(|point| placed(&row.model, *point))
                    .collect()
            })
            .collect::<Vec<_>>();
        if let Some(source) = row.geometry() {
            strokes.extend(
                source
                    .vertices()
                    .into_iter()
                    .filter_map(|key| source.vertex_point(key))
                    .map(|point| {
                        vec![placed(
                            &row.model,
                            [point[0], point[1], point[2]],
                        )]
                    }),
            );
        }
        if matches(&local, rectangle, strokes) {
            found.push(row.id);
        }
    }
    for row in scene
        .lines()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if matches(
            &local,
            rectangle,
            vec![
                row.endpoints_at(origin)
                    .map(|point| [point[0], point[1], point[2]])
                    .to_vec(),
            ],
        ) {
            found.push(row.id);
        }
    }
    for row in scene
        .paths()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if matches(
            &local,
            rectangle,
            vec![
                row.points_at(origin)
                    .into_iter()
                    .map(|point| [point[0], point[1], point[2]])
                    .collect(),
            ],
        ) {
            found.push(row.id);
        }
    }
    for row in scene
        .points()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let point = row.point_at(origin);
        if matches(
            &local,
            rectangle,
            vec![vec![[point[0], point[1], point[2]]]],
        ) {
            found.push(row.id);
        }
    }
    for row in scene
        .texts()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if crate::scene_label_screen::matches_in(row, view, rectangle) {
            found.push(row.id);
        }
    }
    for row in scene
        .clouds()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let mut points = row
            .prepared
            .source
            .coords()
            .chunks_exact(3)
            .map(|p| local.unclipped(placed(&row.model, [p[0], p[1], p[2]])));
        let hit = if rectangle.crossing {
            points.any(|p| p.is_some_and(|p| rectangle.contains(p)))
        } else {
            points.all(|p| p.is_some_and(|p| rectangle.contains(p)))
        };
        if hit {
            found.push(row.id);
        }
    }
    for row in scene
        .sheets()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let strokes = row
            .prepared
            .pages()
            .iter()
            .flat_map(|page| {
                page.coordinates
                    .chunks_exact(6)
                    .map(|p| {
                        vec![
                            placed(&row.model, [p[0], p[1], p[2]]),
                            placed(&row.model, [p[3], p[4], p[5]]),
                        ]
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        if matches(&local, rectangle, strokes) {
            found.push(row.id);
```

### 2. `src/editor.rs`

Connect the existing window/crossing gesture to the precise camera query owner.

**Locate this existing block:**

```rust
            if let Some(rectangle) = drag.rectangle() {
                let ids = crate::rect_select::found(
                    &self.scene,
                    &self.camera.view_projection().m,
                    drag.size,
                    rectangle,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            if let Some(rectangle) = drag.rectangle() {
                let ids = crate::rect_select::found_in(
                    &self.scene,
                    &crate::snap_math::Screen::camera(&self.camera, drag.size),
                    rectangle,
```

### 3. `src/camera_space_tests.rs`

Supply exact local/distant original window/crossing sets, text/point source identity, hidden eligibility and empty queries with unchanged earlier predicates.

**Locate this existing block:**

```rust
    assert!(view.unclipped([0.; 3]).is_none());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(view.unclipped([0.; 3]).is_none());
}

#[test]
fn rectangle_queries_keep_distant_originals_and_exact_window_crossing_sets() {
    use crate::{
        rect_select::{Rectangle, found_in},
        scene::Scene,
        snap_math::Screen,
    };
    use std::rc::Rc;
    let mut scene = Scene::empty();
    let mesh = scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [-0.25, -0.25, -0.5],
                    [0.25, -0.25, -0.5],
                    [0., 0.25, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let line = scene
        .insert_line(
            crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                -0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let path = scene
        .insert_path(
            crate::chain::PreparedChain::polyline(Rc::new(
                session_rust::Polyline::new(vec![
                    session_rust::Point::new(-0.5, -0.5, -0.5),
                    session_rust::Point::new(0., -0.25, -0.5),
                    session_rust::Point::new(0.5, -0.5, -0.5),
                ]),
            ))
            .unwrap(),
        )
        .unwrap();
    let point = scene
        .insert_point(
            crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                0.75, 0., -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let cloud = scene
        .insert_cloud(
            crate::cloud::Prepared::new(Rc::new(
                session_rust::PointCloud::from_coords(
                    vec![-0.75, 0., -0.5],
                    vec![],
                    vec![],
                ),
            ))
            .unwrap(),
        )
        .unwrap();
    let text_source = Rc::new(crate::scene_label::example(
        crate::text::TextPlacement::Anchor {
            world: [-0.5, 0.75, -0.5],
            offset: [0.; 2],
        },
    ));
    let text = scene
        .insert_text(crate::scene_label::Prepared::new(text_source.clone()).unwrap())
        .unwrap();
    let sheet = Rc::new(session_rust::proto::Sheet {
        guid: crate::sheet_example::source().guid,
        coords: vec![0.25, 0.25, -0.5, 0.75, 0.25, -0.5],
        widths: vec![0.0625],
        colors: vec![0xff00_0000],
        segment_count: 1,
        entity_count: 1,
        source_ids: vec![0],
        ..Default::default()
    });
    let drawing = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(sheet.clone()).unwrap(),
        ))
        .unwrap();
    let point_source = scene.points()[0].prepared.source.clone();
    let mut camera = Camera::default();
    camera.projection = Projection::Orthographic;
    camera.distance = 3.;
    let view = Screen::camera(&camera, [800., 600.]);
    let wide = Rectangle::new([0., 0.], [800., 600.]).unwrap();
    let all = found_in(&scene, &view, wide);
    for id in [mesh, line, path, point, cloud, text, drawing] {
        assert!(all.contains(&id));
    }
    let crossing = Rectangle::new([420., 310.], [380., 290.]).unwrap();
    let small = Rectangle::new([380., 290.], [420., 310.]).unwrap();
    let expected_crossing = found_in(&scene, &view, crossing);
    let expected_small = found_in(&scene, &view, small);
    assert!(expected_crossing.contains(&mesh));
    assert!(!expected_small.contains(&mesh));
    let mut far = scene.clone();
    let anchor = [1e12, -1e12, 1e12];
    for id in [mesh, line, path, point, cloud, text, drawing] {
        far.translate(id, anchor).unwrap();
    }
    camera.target = anchor;
    let far_view = Screen::camera(&camera, [800., 600.]);
    assert_eq!(view.matrix, far_view.matrix);
    for (rectangle, expected) in [
        (wide, all),
        (crossing, expected_crossing),
        (small, expected_small),
    ] {
        assert_eq!(
            found_in(&far, &far_view, rectangle),
            expected
        );
    }
    assert!(Rc::ptr_eq(
        &far.points()[0].prepared.source,
        &point_source
    ));
    assert!(Rc::ptr_eq(
        &far.texts()[0].prepared.source,
        &text_source
    ));
    assert!(Rc::ptr_eq(
        &far.sheets()[0].prepared.original,
        &sheet
    ));
    Rc::make_mut(&mut far.view).hide(&[line], true);
    assert!(!found_in(&far, &far_view, wide).contains(&line));
    Rc::make_mut(&mut far.view).lock(&[point], true);
    assert!(!found_in(&far, &far_view, wide).contains(&point));
    assert!(found_in(&Scene::empty(), &far_view, wide).is_empty());
}
```

## <a id="checkpoint-96zzzzzm-finish-precise-lasso-and-handle-queries"></a>4. Finish precise lasso and handle queries

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/lasso.rs, src/widget_la…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Precise query frame</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">lasso membership and hand…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local GPU widget</text>
</svg>
</div>

**What this code does:** Keep lasso selection and handle sizing in precise local frames.

### 1. `src/lasso.rs`

Retain the precise lasso query origin and physical pixel dimensions. Reuse the existing unclipped query predicate. Place original samples locally before projecting them into the lasso. Convert existing world bounds to the same query frame for fallback families.

**Locate this existing block:**

```rust
        let region = crate::selection_region::Region::new(&self.points);
        let view = camera.view_projection();
        let inside = |p: [f64; 3]| {
            crate::rect_select::screen(
                &view.m,
                p,
                self.size.map(|v| v * self.density),
            )
            .is_some_and(|p| region.contains((p[0], p[1])))
        };
        let mut ids = scene
            .all_ids()
            .into_iter()
            .filter(|id| scene.pick_id(id.value()) == Some(*id))
            .filter(|id| {
                if let Some(source) = crate::source_edit::original(scene, *id) {
                    let model = scene.placement(*id).unwrap();
                    if let Some(yes) =
                        crate::selection_region::enclosed(&source, &|p| {
                            let p = model.transform_point(&session_rust::Point::new(
                                p[0], p[1], p[2],
                            ));
                            inside([p[0], p[1], p[2]])
                        })
                    {
                        return yes;
                    }
                }
                scene
                    .selected_bounds(*id)
                    .is_some_and(|bounds| {
                        (0..8).all(|mask| {
                            inside(std::array::from_fn(|i| {
                                if mask & (1 << i) == 0 {
                                    bounds.min[i]
                                } else {
                                    bounds.max[i]
                                }
                            }))
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let region = crate::selection_region::Region::new(&self.points);
        let view = crate::snap_math::Screen::camera(
            camera,
            self.size.map(|v| v * self.density),
        );
        let local = crate::snap_math::Screen {
            matrix: view.matrix,
            size: view.size,
            origin: [0.; 3],
        };
        let inside = |p: [f64; 3]| {
            local
                .unclipped(p)
                .is_some_and(|p| region.contains((p[0], p[1])))
        };
        let mut ids = scene
            .all_ids()
            .into_iter()
            .filter(|id| scene.pick_id(id.value()) == Some(*id))
            .filter(|id| {
                if let Some(source) = crate::source_edit::original(scene, *id) {
                    let model = crate::render_space::placed(
                        &scene.placement(*id).unwrap(),
                        view.origin,
                    );
                    if let Some(yes) =
                        crate::selection_region::enclosed(&source, &|p| {
                            let p = model.transform_point(&session_rust::Point::new(
                                p[0], p[1], p[2],
                            ));
                            inside([p[0], p[1], p[2]])
                        })
                    {
                        return yes;
                    }
                }
                scene
                    .selected_bounds(*id)
                    .is_some_and(|bounds| {
                        (0..8).all(|mask| {
                            inside(std::array::from_fn(|i| {
                                (if mask & (1 << i) == 0 {
                                    bounds.min[i]
                                } else {
                                    bounds.max[i]
                                }) - view.origin[i]
                            }))
```

### 2. `src/widget_layout.rs`

Share precise camera-relative handle sizing without repeating its projection recipe.

**Locate this existing block:**

```rust
}

// Measuring the projected pivot keeps a 96-pixel handle the same size while the camera zooms or changes projection.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn camera_scale(
    camera: &crate::camera::Camera,
    origin: [f64; 3],
    height: f64,
) -> Option<f64> {
    let matrix = camera.view_projection_anchored(camera.target);
    world_per_pixel(
        &matrix,
        std::array::from_fn(|i| origin[i] - camera.target[i]),
        height,
    )
}

// Measuring the projected pivot keeps a 96-pixel handle the same size while the camera zooms or changes projection.
```

### 3. `src/editor.rs`

Use the shared precise scale for handle hit testing. Use the same scale for numeric-prompt handle positions.

**Locate this existing block:**

```rust
        let origin = self.gizmo_origin()?;
        let scale = crate::widget_layout::world_per_pixel(
            &self.camera.view_projection(),
            origin,
            size[1],
        )?;
        let ray = self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ))
        .hit_with_radius(&ray.origin, &ray.direction, scale, radius)
    }

    pub fn number_prompt(
        &self,
        size: [f64; 2],
    ) -> Option<crate::number_box::NumberPrompt> {
        let edit = self.gizmo_edit.as_ref()?;
        let handle = edit.gizmo.typing?;
        let origin = edit.origin();
        let matrix = self.camera.view_projection();
        let scale = crate::widget_layout::world_per_pixel(&matrix, origin, size[1])?;
        let point = edit.gizmo.handle_point(handle, scale);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let origin = self.gizmo_origin()?;
        let scale = crate::widget_layout::camera_scale(&self.camera, origin, size[1])?;
        let ray = self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ))
        .hit_with_radius(&ray.origin, &ray.direction, scale, radius)
    }

    pub fn number_prompt(
        &self,
        size: [f64; 2],
    ) -> Option<crate::number_box::NumberPrompt> {
        let edit = self.gizmo_edit.as_ref()?;
        let handle = edit.gizmo.typing?;
        let origin = edit.origin();
        let scale = crate::widget_layout::camera_scale(&self.camera, origin, size[1])?;
        let point = edit.gizmo.handle_point(handle, scale);
```

### 4. `src/renderer.rs`

Prepare the widget pivot and GPU layout in the actual renderer frame.

**Locate this existing block:**

```rust
    ) {
        let matrix = camera.view_projection();
        let layout = origin.and_then(|p| {
            crate::widget_layout::world_per_pixel(
                &matrix,
                p,
                self.size[1] as f64 / self.density as f64,
            )
            .and_then(|scale| {
                crate::widget_layout::layout(&matrix, p, scale, self.size, active)
            })
        });
        self.widget
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) {
        let anchor = self.anchor();
        let matrix = camera.view_projection_anchored(anchor);
        let layout = origin
            .map(|p| std::array::from_fn(|i| p[i] - anchor[i]))
            .and_then(|p| {
                crate::widget_layout::world_per_pixel(
                    &matrix,
                    p,
                    self.size[1] as f64 / self.density as f64,
                )
                .and_then(|scale| {
                    crate::widget_layout::layout(&matrix, p, scale, self.size, active)
                })
            });
        self.widget
```

### 5. `src/camera_space_tests.rs`

Supply local/distant original lasso sets at two densities, source identity and locked/empty eligibility.

**Locate this existing block:**

```rust
    assert!(found_in(&Scene::empty(), &far_view, wide).is_empty());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(found_in(&Scene::empty(), &far_view, wide).is_empty());
}

#[test]
fn lasso_queries_keep_distant_original_sets_and_density() {
    use std::rc::Rc;
    let mut scene = crate::scene::Scene::empty();
    let line = scene
        .insert_line(
            crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                -0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let point = scene
        .insert_point(
            crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                0.75, 0., -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let mesh = scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [-0.25, -0.25, -0.5],
                    [0.25, -0.25, -0.5],
                    [0., 0.25, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let source = scene.points()[0].prepared.source.clone();
    let anchor = [1e12, -1e12, 1e12];
    let mut far = scene.clone();
    for id in [line, point, mesh] {
        far.translate(id, anchor).unwrap();
    }
    for density in [1., 2.] {
        let size = [800., 600.];
        let mut lasso = crate::lasso::Lasso::default();
        lasso.size = size;
        lasso.density = density;
        lasso.points = vec![
            (0., 0.),
            (800. * density, 0.),
            (800. * density, 600. * density),
            (0., 600. * density),
            (0., 0.),
        ];
        let mut camera = Camera::default();
        camera.projection = Projection::Orthographic;
        camera.distance = 3.;
        let baseline = lasso.found(&scene, &camera).unwrap();
        assert_eq!(baseline, vec![line, point, mesh]);
        camera.target = anchor;
        assert_eq!(lasso.found(&far, &camera).unwrap(), baseline);
        assert!(Rc::ptr_eq(
            &far.points()[0].prepared.source,
            &source
        ));
        let mut locked = far.clone();
        Rc::make_mut(&mut locked.view).lock(&[point], true);
        assert_eq!(
            lasso.found(&locked, &camera).unwrap(),
            vec![line, mesh]
        );
        assert!(
            lasso
                .found(&crate::scene::Scene::empty(), &camera)
                .is_err()
        );
    }
}
```

### 6. `src/widget_tests.rs`

Reuse the existing supplied widget readback helper from the renderer fixture module.

**Locate this existing block:**

```rust
}

fn pixels(renderer: &mut Renderer, transform: &[f32; 16], size: [u32; 2]) -> Vec<u8> {
    let texture = renderer
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub(crate) fn pixels(
    renderer: &mut Renderer,
    transform: &[f32; 16],
    size: [u32; 2],
) -> Vec<u8> {
    let texture = renderer
```

### 7. `src/global_outline_tests.rs`

Supply actual local/distant widget pixels and camera scale at two densities/projections, retained tiles, no invented original picks and tile release.

**Locate this existing block:**

```rust
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}

#[test]
fn actual_distant_widgets_keep_local_pixels_scale_picks_and_tile_release() {
    let scene = Scene::empty();
    let anchor = [1e12, -1e12, 1e12];
    let pivot = [0.125, 0.25, -0.5];
    let mut renderer = renderer(&scene);
    for projection in [
        crate::camera::Projection::Perspective,
        crate::camera::Projection::Orthographic,
    ] {
        for density in [1u32, 2] {
            let mut camera = crate::camera::Camera::default();
            camera.isometric();
            camera.projection = projection;
            camera.aspect = 4. / 3.;
            let size = [320 * density, 240 * density];
            renderer.resize(crate::viewport::Viewport {
                width: size[0],
                height: size[1],
            });
            renderer.set_density(density as f32);
            renderer.space.forget();
            let scale =
                crate::widget_layout::camera_scale(&camera, pivot, 240.).unwrap();
            renderer.set_gizmo(&camera, Some(pivot), -1.);
            let usage = renderer.gizmo_usage();
            let extent = renderer.gizmo_extent();
            let transform = camera.uniform();
            let baseline = crate::widget_tests::pixels(&mut renderer, &transform, size);
            assert!(
                baseline
                    .chunks_exact(4)
                    .any(|p| p[..3] != [255; 3])
            );
            renderer
                .space
                .update(anchor, 3., 0.)
                .unwrap();
            camera.target = anchor;
            let distant = std::array::from_fn(|i| anchor[i] + pivot[i]);
            assert_eq!(
                crate::widget_layout::camera_scale(&camera, distant, 240.).unwrap(),
                scale
            );
            renderer.set_gizmo(&camera, Some(distant), -1.);
            let local = camera
                .view_projection_anchored(anchor)
                .to_f32();
            assert_eq!(local, transform);
            let actual = crate::widget_tests::pixels(&mut renderer, &local, size);
            assert!(
                actual == baseline,
                "Precise distant widget pixels differ at density {density}"
            );
            assert_eq!(renderer.gizmo_extent(), extent);
            assert_eq!(renderer.gizmo_usage()[2], usage[2]);
            renderer
                .request_pick([0., 0.], local)
                .unwrap();
            crate::cap_gpu_tests::wait(&renderer);
            assert!(
                renderer
                    .take_pick(local)
                    .unwrap()
                    .unwrap()
                    .is_none()
            );
            renderer.set_gizmo(&camera, None, -1.);
            assert_eq!(renderer.gizmo_usage()[1], 0);
            assert!(renderer.gizmo_extent().is_none());
        }
    }
}
```

## <a id="checkpoint-96zzzzzn-stamp-asynchronous-picks-with-the-origin"></a>5. Stamp asynchronous picks with the scene origin

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/pick_view.rs, src/pick_…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Matrix + viewport + densi…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pick ticket</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">matching-frame acceptance</text>
</svg>
</div>

**What this code does:** Reject pick readbacks from a different local coordinate frame.

### 1. `src/pick_view.rs`

Retain the precise local frame in readback identity. State the complete existing snapshot invariant. Preserve zero-origin callers while exposing one complete snapshot constructor. Reject invalid origins before issuing a pick ticket. Compare all origin bits without narrowing to GPU floats.

**Locate this existing block:**

```rust
    density: u32,
}

impl View {
    // Readback belongs to the camera matrix, physical viewport and CSS density that produced it, even when the document has not changed.
    pub fn new(
        transform: [f32; 16],
        size: [u32; 2],
        density: f32,
    ) -> Result<Self, &'static str> {
        if transform.iter().any(|n| !n.is_finite())
            || size.contains(&0)
            || !density.is_finite()
            || density <= 0.0
        {
            return Err("Invalid pick view");
        }
        Ok(Self {
            transform: transform.map(f32::to_bits),
            size,
            density: density.to_bits(),
        })
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    density: u32,
    origin: [u64; 3],
}

impl View {
    // Readback belongs to its matrix, physical viewport, CSS density and precise origin, even when the document has not changed.
    pub fn new(
        transform: [f32; 16],
        size: [u32; 2],
        density: f32,
    ) -> Result<Self, &'static str> {
        Self::anchored(transform, size, density, [0.; 3])
    }
    pub fn anchored(
        transform: [f32; 16],
        size: [u32; 2],
        density: f32,
        origin: [f64; 3],
    ) -> Result<Self, &'static str> {
        if origin.iter().any(|v| !v.is_finite())
            || transform.iter().any(|n| !n.is_finite())
            || size.contains(&0)
            || !density.is_finite()
            || density <= 0.0
        {
            return Err("Invalid pick view");
        }
        Ok(Self {
            transform: transform.map(f32::to_bits),
            size,
            density: density.to_bits(),
            origin: origin.map(f64::to_bits),
        })
```

### 2. `src/pick_gpu.rs`

Stamp the same explicit frame at queue time without changing existing request data.

**Locate this existing block:**

```rust
    pub fn queue(&mut self, request: Request) -> Result<(), &'static str> {
        let view = crate::pick_view::View::new(
            request.transform,
            request.window.full,
            request.density,
        )?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn queue(&mut self, request: Request) -> Result<(), &'static str> {
        self.queue_anchored(request, [0.; 3])
    }
    pub fn queue_anchored(
        &mut self,
        request: Request,
        origin: [f64; 3],
    ) -> Result<(), &'static str> {
        let view = crate::pick_view::View::anchored(
            request.transform,
            request.window.full,
            request.density,
            origin,
        )?;
```

### 3. `src/renderer.rs`

Queue the current renderer frame with the original screen request. Dispatch only requests whose complete frame is still current. Match the same origin when consuming a completed readback.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &mut self,
        transform: [f32; 16],
    ) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density)
        {
            Ok(view) => view,
```

Replace it with:

```rust
        &mut self,
        transform: [f32; 16],
    ) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(
            transform,
            self.size,
            self.density,
            self.anchor(),
        ) {
            Ok(view) => view,
```

**Locate this block:**

```rust
            .ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request {
            window,
            transform,
            density: self.density,
            mode: self.pick_mode,
        })?;
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density)
        else {
            self.picker.cancel();
            return;
        };
```

Replace it with:

```rust
            .ok_or("Invalid pick cursor")?;
        let origin = self.anchor();
        self.picker.queue_anchored(
            crate::pick_gpu::Request {
                window,
                transform,
                density: self.density,
                mode: self.pick_mode,
            },
            origin,
        )?;
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(
            transform,
            self.size,
            self.density,
            self.anchor(),
        ) else {
            self.picker.cancel();
            return;
        };
```

### 4. `src/pick_view.rs`

Supply exact-origin-bit identity and retained invalid matrix/size/density guards.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn origin_bits_complete_the_pick_snapshot_and_reject_invalid_inputs() {
        let matrix = session_rust::Xform::identity().to_f32();
        let size = [320, 240];
        let zero = View::new(matrix, size, 1.).unwrap();
        assert_eq!(
            zero,
            View::anchored(matrix, size, 1., [0.; 3]).unwrap()
        );
        assert_ne!(
            zero,
            View::anchored(matrix, size, 1., [2000., 0., 0.]).unwrap()
        );
        assert_ne!(
            zero,
            View::anchored(matrix, size, 1., [-0., 0., 0.]).unwrap()
        );
        assert!(View::anchored(matrix, size, 1., [f64::NAN, 0., 0.]).is_err());
        assert!(View::anchored(matrix, [0, 240], 1., [0.; 3]).is_err());
        assert!(View::anchored(matrix, size, 0., [0.; 3]).is_err());
        let mut invalid = matrix;
        invalid[0] = f32::NAN;
        assert!(View::anchored(invalid, size, 1., [0.; 3]).is_err());
    }
}
```

### 5. `src/global_outline_tests.rs`

Supply actual same-matrix old-origin rejection, fresh empty/current results, restored original owner and unchanged source uploads.

**Locate this existing block:**

```rust
            assert!(renderer.gizmo_extent().is_none());
        }
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            assert!(renderer.gizmo_extent().is_none());
        }
    }
}

#[test]
fn actual_origin_only_pick_views_discard_old_hits_and_restore_owners() {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.4, -0.4, -0.5],
                    [0.4, -0.4, -0.5],
                    [0., 0.4, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut gpu = renderer(&scene);
    let transform = matrix();
    let uploads = gpu.stats()[0];
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    wait(&gpu);
    gpu.space
        .update([2000., 0., 0.], 3., 0.)
        .unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, gpu.space.anchor());
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    assert!(
        gpu.take_pick(transform).is_none(),
        "Identical matrix in a new origin must not accept the old owner"
    );
    assert_eq!(&gpu.pick_usage()[..2], &[0, 0]);
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    wait(&gpu);
    assert!(
        gpu.take_pick(transform)
            .unwrap()
            .unwrap()
            .is_none()
    );
    gpu.space.update([0.; 3], 3., 300.).unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, [0.; 3]);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    wait(&gpu);
    assert_eq!(
        gpu.take_pick(transform)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

## <a id="checkpoint-96zzzzzo-retry-rebased-picks-with-the-latest-cursor"></a>6. Retry rebased picks with the latest cursor

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs, src/global…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Latest successful cursor…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stale rebase</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one fresh request</text>
</svg>
</div>

**What this code does:** Finish a released gesture when its asynchronous pick frame is rebased.

### 1. `src/renderer.rs`

Retain only the latest successfully queued cursor and its exact frame origin. Begin without retained input intent. Save intent after the existing queue validation and ticket issuance succeed. Share cursor and flight cancellation at the existing semantic boundary. Make explicit cancellation final for both retained intent and GPU work. Consume retained intent only with an accepted current reply. Retry only a rebased latest cursor; preserve matrix-only stale cancellation and newest-pending behavior. Discard retained input and pending work after a failed readback.

**Locate this existing block:**

```rust
    pick_mode: crate::pick_sources::Mode,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(
            &device,
            crate::clipping_gpu::source(include_str!("cloud_points.wgsl"), 2, 0),
            "Cloud points",
        );
        crate::shader_check::check(
            &device,
            include_str!("cloud_resolve.wgsl").into(),
            "Cloud resolve",
        );
        crate::shader_check::check(
            &device,
            crate::finite_project_source::source(),
            "Finite projection",
        );
        crate::shader_check::check(
            &device,
            crate::finite_project_source::tiles(),
            "Finite tiles",
        );
        crate::shader_check::check(
            &device,
            crate::finite_project_source::scan(),
            "Finite scan",
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0).into(),
            ),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout =
            crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"),
            bind_group_layouts: &[
                Some(&view_layout),
                Some(&model_layout),
                Some(&crate::clipping_gpu::layout(&device)),
                Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device)),
            ],
            immediate_size: 0,
        });
        let mesh_pipeline = |depth, primitive_write| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("triangle"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[crate::draw_slots::layout()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(crate::primitive_image::target(
                            primitive_write,
                        )),
                    ],
                }),
                primitive: Default::default(),
                depth_stencil: Some(depth),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = mesh_pipeline(crate::depth::surface(), true);
        let tiles = std::cell::RefCell::new(crate::tile_gpu::Tiles::new(
            &device,
            &view_layout,
        ));
        let instances =
            crate::instance_gpu::Lane::new(&device, &queue, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(), false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let primitive = crate::primitive_image::Image::new(&device, [640, 480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets = crate::sheet_gpu::Lane::new(&device, format);
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            space: Default::default(),
            opacity: std::cell::Cell::new(1.),
            ambient_enabled: Default::default(),
            ambient: Default::default(),
            ambient_rows: Default::default(),
            pyramid: Default::default(),
            pyramid_revision: 0,
            caps: std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),
            clipping: std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(
                &device,
            )),
            tiles,
            guides,
            measurement,
            sheets,
            instances,
            instance_rows: Default::default(),
            clouds: Default::default(),
            cloud_view: Default::default(),
            cloud_error: Default::default(),
            outline: Default::default(),
            global_outline: Default::default(),
            outlines_enabled: Default::default(),
            outline_composite: Default::default(),
            full_text_fonts: Default::default(),
            presentation_text: Default::default(),
            scene_text: Default::default(),
            text: std::cell::RefCell::new(None),
            text_error: std::cell::RefCell::new(None),
            format,
            device,
            queue,
            pipeline,
            highlight_pipeline,
            highlight,
            preview,
            widget,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
            settings_writes: 0,
            settings_bytes: 0,
            uniform,
            view_group,
            depth,
            primitive,
            strokes,
            paths,
            markers,
            grid,
            grid_settings: None,
            grid_plane: crate::cplane::CPlane::Xy,
            grid_anchor: [0.; 3],
            size: [640, 480],
            density: 1.0,
            picker,
            faces,
            mesh_ink,
            sources: Default::default(),
            pick_mode: Default::default(),
        };
        for lane in [
            &mut renderer.mesh_ink,
            &mut renderer.strokes,
            &mut renderer.paths,
            &mut renderer.markers,
            &mut renderer.grid,
        ] {
            lane.enable_finite(&renderer.device, format);
        }
        renderer
            .highlight
            .enable_finite(&renderer.device, format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self) -> [f64; 3] {
        self.space.anchor()
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] {
            self.picker.cancel();
        }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive = crate::primitive_image::Image::new(&self.device, self.size);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density {
            self.picker.cancel();
            self.density = density;
        }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [
            count,
            bytes,
            self.strokes.uploads,
            self.strokes.uploaded_bytes,
        ]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [
            count,
            bytes,
            self.paths.uploads,
            self.paths.uploaded_bytes,
        ]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [
            count,
            bytes,
            self.markers.uploads,
            self.markers.uploaded_bytes,
        ]
    }

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {
        self.instances.ordinary.clear();
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor = self.anchor();
        self.ambient_rows.set(scene);
        if let Some(ambient) = self.ambient.borrow_mut().as_mut() {
            if self.ambient_enabled.get() {
                ambient.sources.set(
                    &self.device,
                    &self.queue,
                    &self.ambient_rows.specs,
                );
            }
        }
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        self.picker.cancel();
        self.faces.clear();
        self.preview
            .set(&self.device, None)
            .expect("Empty tool preview");
        self.highlight
            .set(
                &self.device,
                &self.pipeline.get_bind_group_layout(1),
                scene,
                None,
            )
            .expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene)
            .expect("Prepared source topology is bounded");
        self.mesh_ink
            .set(
                &self.device,
                self.sources.mesh_ink.iter().copied(),
            )
            .expect("Prepared source edges");
        self.mesh_ink
            .set_ids(
                &self.device,
                self.sources.mesh_ids.iter().copied(),
            )
            .expect("Source edge IDs match");
        let planes = crate::clipping_rows::owners(scene);
        let flags = self
            .sources
            .mesh_ids
            .iter()
            .map(|id| {
                crate::sub_id::decode(id[1])
                    .and_then(|(_, address)| self.sources.edges.get(address))
                    .map_or(0, |target| {
                        if planes.contains(&target.owner) {
                            4096
                        } else {
                            0
                        }
                    })
            })
            .collect();
        self.mesh_ink
            .set_flags(&self.device, flags)
            .expect("Original plane ink flags match source addresses");
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut marker = row.marker();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            marker.colour = colour;
                        }
                        marker
                    })
                    .chain(
                        scene
                            .control_markers()
                            .into_iter()
                            .map(|c| c.display),
                    ),
            )
            .expect("Prepared point placement fits the display range");
        self.paths
            .set_chains(
                &self.device,
                scene
                    .paths()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .flat_map(|row| {
                        let mut segments = row.segments();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            for segment in &mut segments {
                                segment.stroke.colour = colour;
                            }
                        }
                        segments
                    }),
            )
            .expect("Prepared path placement fits the display range");
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut stroke = row.stroke();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            stroke.colour = colour;
                        }
                        stroke
                    }),
            )
            .expect("Prepared line placement fits the display range");
        self.markers
            .set_ids(
                &self.device,
                self.sources.marker_ids.iter().copied(),
            )
            .expect("Marker owners match");
        self.paths
            .set_ids(
                &self.device,
                self.sources.path_ids.iter().copied(),
            )
            .expect("Path owners match");
        self.strokes
            .set_ids(
                &self.device,
                self.sources.line_ids.iter().copied(),
            )
            .expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
            .iter()
            .filter(|row| scene.visible(row.id))
        {
            let matching = old.iter().position(|row| {
                row.id == object.id
                    && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh)
            });
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(
                    &self.queue,
                    object,
                    style(scene, object.id, selected == Some(object.id)),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(
                    &crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue),
                    &object.mesh,
                );
                self.settings_allocations += 1;
                GpuMesh::with_geometry(
                    &self.device,
                    &layout,
                    object,
                    style(scene, object.id, selected == Some(object.id)),
                    geometry,
                )
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(
            &self.queue,
            scene,
            &mut self.meshes,
            &self.clipping.borrow().rows,
        );
        for mesh in &mut self.meshes {
            mesh.set_xray(&self.queue, self.opacity.get() == 0.);
        }
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
            self.meshes.iter().map(|row| {
                (
                    row.id,
                    std::rc::Rc::as_ptr(&row.geometry) as usize,
                    scene.instances.contains_key(&row.id),
                )
            }),
            crate::object_rows::capacity(&self.device),
        );
        self.instances
            .set(
                &self.device,
                &self.queue,
                &mut self.meshes,
                &self.instance_rows,
                true,
            )
            .expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        self.sheets
            .set_anchored(
                &self.device,
                &self.queue,
                scene,
                selected.as_slice(),
                &self.sources.sheet_bases,
                anchor,
            )
            .expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        if let Err(error) = self.set_clouds(scene, selected.as_slice()) {
            *self.cloud_error.borrow_mut() = Some(error.to_string());
        }
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
    }

    pub fn request_pick(
        &mut self,
        screen: [f32; 2],
        transform: [f32; 16],
    ) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density)
            .ok_or("Invalid pick cursor")?;
        let origin = self.anchor();
        self.picker.queue_anchored(
            crate::pick_gpu::Request {
                window,
                transform,
                density: self.density,
                mode: self.pick_mode,
            },
            origin,
        )?;
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(
            transform,
            self.size,
            self.density,
            self.anchor(),
        ) else {
            self.picker.cancel();
            return;
        };
        if self.pick_mode == crate::pick_sources::Mode::Face
            && !self.faces.ready(self.meshes.len())
        {
            self.faces.set(
                &self.device,
                &self.meshes,
                &self.sources.mesh_bases,
                &self.clipping.borrow().buffer,
            );
        }
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            if let Err(error) = lane.prepare(
                &self.text_context(),
                &self.text_frame(transform),
            ) {
                *self.text_error.borrow_mut() = Some(error.to_string());
                return;
            }
        }
        self.clipping.borrow_mut().frame_anchored(
            &self.queue,
            transform,
            self.anchor(),
            self.size[1],
            self.density,
        );
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping = self.clipping.borrow_mut();
            clipping.frame_anchored(
                &self.queue,
                transform,
                self.anchor(),
                self.size[1],
                self.density,
            );
            let mut bytes = transform
                .iter()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>();
            bytes.resize(80, 0);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
                let mut tiles = self.tiles.borrow_mut();
                tiles.prepare(&self.device, &self.queue, self.size);
                let mut encoder = self
                    .device
                    .create_command_encoder(&Default::default());
                tiles.encode(
                    &self.device,
                    &mut encoder,
                    &self.view_group,
                    transform,
                );
                self.queue.submit([encoder.finish()]);
                tiles.map_report();
                crate::finite_ink::Tables {
                    projected: tiles.projected.clone(),
                    tiles: tiles.buffer.clone(),
                    primitive: self.primitive.view.clone(),
                }
            };
            self.picker.submit_finite(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[
                    &self.mesh_ink,
                    &self.strokes,
                    &self.paths,
                    &self.markers,
                ],
                &self.faces,
                self.text.borrow().as_ref(),
                self.clouds.borrow().as_ref(),
                Some(&self.sheets),
                Some(&tables),
                &clipping.group,
                &clipping.buffer,
                Some(&mut self.caps.borrow_mut()),
                Some(&self.instances),
            );
        }
    }

    pub fn take_pick(
        &mut self,
        transform: [f32; 16],
    ) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(
            transform,
            self.size,
            self.density,
            self.anchor(),
        ) {
            Ok(view) => view,
            Err(error) => {
                self.picker.cancel();
                return Some(Err(error));
            }
        };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => {
                self.pump_pick(transform);
                None
            }
            Some(Err(error)) => Some(Err(error)),
            None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode {
            self.picker.cancel();
            self.pick_mode = mode;
        }
    }

    pub fn resolve_pick(
        &self,
        scene: &Scene,
        pixel: [u32; 2],
    ) -> Option<crate::pick_sources::Target> {
        self.sources
            .resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.mesh_ink.usage();
        [
            count,
            bytes,
            self.faces.bytes(),
            self.sources.bytes() as u64,
        ]
    }

    pub fn cancel_pick(&mut self) {
        self.picker.cancel();
    }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [
            active,
            pending,
            bytes
                + self.strokes.pick_bytes()
                + self.paths.pick_bytes()
                + self.markers.pick_bytes()
                + self.mesh_ink.pick_bytes()
                + self.mesh_ink.usage()[1]
                + self.faces.bytes(),
            self.picker.submitted,
            self.picker.coalesced,
        ]
    }

    pub fn geometry_owners(
        &self,
    ) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] {
        let mut usage = crate::memory::gpu(&self.meshes);
        usage[1] += self.geometry.allocated_bytes();
        usage
    }
    pub fn arena_cpu_bytes(&self) -> usize {
        self.geometry.cpu_bytes()
    }

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes
            + self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * crate::object_record::BYTES as u64
            + self.settings_bytes as u64
            + self.strokes.uploaded_bytes
            + self.paths.uploaded_bytes
            + self.markers.uploaded_bytes
            + self.grid.uploaded_bytes
    }

    pub fn set_grid(
        &mut self,
        settings: Option<crate::grid::Settings>,
    ) -> Result<(), &'static str> {
        self.set_grid_on(settings, crate::cplane::CPlane::Xy)
    }
    pub fn set_grid_on(
        &mut self,
        settings: Option<crate::grid::Settings>,
        plane: crate::cplane::CPlane,
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        if settings == self.grid_settings
            && plane == self.grid_plane
            && anchor == self.grid_anchor
        {
            return Ok(());
        }
        let strokes = settings
            .map(|s| s.strokes_on_at(plane, anchor))
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        self.grid_plane = plane;
        self.grid_anchor = anchor;
        Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage();
        [
            count,
            bytes,
            self.grid.uploads,
            self.grid.uploaded_bytes,
        ]
    }

    pub fn set_highlight(
        &mut self,
        scene: &Scene,
        target: Option<crate::pick_sources::Target>,
    ) -> Result<(), &'static str> {
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        let anchor = self.anchor();
        self.highlight.set_at(
            &self.device,
            &self.pipeline.get_bind_group_layout(1),
            scene,
            target,
            anchor,
        )
    }

    pub fn select_objects(
        &mut self,
        scene: &Scene,
        ids: &[ObjectId],
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        for mesh in &mut self.meshes {
            if let Some(row) = scene
                .objects()
                .iter()
                .find(|r| r.id == mesh.id)
            {
                let [calls, bytes] = mesh.update(
                    &self.queue,
                    row,
                    style(scene, row.id, ids.contains(&row.id)),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
            }
        }
        self.instances.set(
            &self.device,
            &self.queue,
            &mut self.meshes,
            &self.instance_rows,
            false,
        )?;
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        self.set_clouds(scene, ids)?;
        self.sheets.set_anchored(
            &self.device,
            &self.queue,
            scene,
            ids,
            &self.sources.sheet_bases,
            anchor,
        )?;
        if !ids.is_empty() {
            self.highlight
                .objects_at(&self.device, scene, ids, anchor)?;
        }
        self.set_scene_text(scene, ids)
            .map_err(|_| "Cannot prepare scene labels")?;
        Ok(())
    }

    pub fn set_guides(
        &mut self,
        overlay: &crate::guide_gpu::Overlay,
    ) -> Result<(), &'static str> {
        if self.guides.set(&self.device, overlay)? {
            self.set_scene_measure_text()
                .map_err(|_| "Cannot prepare editing guide label")?
        }
        Ok(())
    }
    pub fn guide_bytes(&self) -> u64 {
        self.guides.bytes()
    }
    pub fn set_measurement(
        &mut self,
        mark: Option<&crate::measure_mark::Mark>,
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        if self
            .measurement
            .set_at(&self.device, mark, anchor)?
        {
            self.set_scene_measure_text()
                .map_err(|_| "Cannot prepare measurement annotation")?;
        }
        Ok(())
    }
    pub fn measurement_bytes(&self) -> u64 {
        self.measurement.bytes()
    }
    fn merge_measurement(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> Vec<crate::text::TextLabel> {
        if let Some(mark) = &self.measurement.mark {
            labels.extend(mark.labels(&labels));
        }
        labels.extend(self.guides.overlay.labels(&labels));
        labels
    }
    fn set_scene_measure_text(&self) -> anyhow::Result<()> {
        let mut labels = self.scene_text.borrow().clone();
        labels.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(labels))
    }
    pub fn set_preview(
        &mut self,
        draft: Option<&crate::tool::Draft>,
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        self.preview
            .set_at(&self.device, draft, anchor)
    }
    pub fn preview_bytes(&self) -> u64 {
        self.preview.bytes()
    }

    pub fn set_gizmo(
        &mut self,
        camera: &crate::camera::Camera,
        origin: Option<[f64; 3]>,
        active: f32,
    ) {
        let anchor = self.anchor();
        let matrix = camera.view_projection_anchored(anchor);
        let layout = origin
            .map(|p| std::array::from_fn(|i| p[i] - anchor[i]))
            .and_then(|p| {
                crate::widget_layout::world_per_pixel(
                    &matrix,
                    p,
                    self.size[1] as f64 / self.density as f64,
                )
                .and_then(|scale| {
                    crate::widget_layout::layout(&matrix, p, scale, self.size, active)
                })
            });
        self.widget
            .prepare(&self.device, &self.queue, layout);
    }
    pub fn gizmo_usage(&self) -> [u64; 4] {
        self.widget.usage()
    }
    pub fn gizmo_extent(&self) -> Option<[u32; 2]> {
        self.widget.extent()
    }

    fn text_context(&self) -> crate::text_gpu_support::GpuCtx {
        crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue)
    }
    fn text_frame(&self, transform: [f32; 16]) -> crate::scene_text_gpu::TextFrame {
        crate::scene_text_gpu::TextFrame {
            mvp: transform,
            origin: self.anchor(),
            framebuffer: self.size,
            logical: [
                self.size[0] as f64 / self.density as f64,
                self.size[1] as f64 / self.density as f64,
            ],
            ortho_half_height: 0.,
            clip: {
                let clip = self.clipping.borrow();
                let mut planes = [[0.; 4]; 6];
                if clip.enabled {
                    for (p, row) in planes.iter_mut().zip(&clip.rows) {
                        *p = [
                            row.plane.normal[0],
                            row.plane.normal[1],
                            row.plane.normal[2],
                            row.plane.offset,
                        ];
                    }
                }
                planes
            },
        }
    }
    fn adopt_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
            self.text_error.borrow_mut().take();
            return Ok(());
        }
        let mut lane = self.text.borrow_mut();
        if lane.is_none() {
            let mut new = crate::scene_text_gpu::Lane::new(
                &self.text_context(),
                crate::text_gpu_support::Target {
                    format: self.format,
                    samples: 1,
                },
            );
            if let Some(fonts) = self.full_text_fonts.borrow().as_ref() {
                new.document.replace_fonts(fonts.clone())?;
            }
            *lane = Some(new);
        }
        lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> anyhow::Result<()> {
        for (i, label) in labels.iter_mut().enumerate() {
            label.id = u32::MAX - i as u32;
            label.object = None;
        }
        let mut merged = self.scene_text.borrow().clone();
        merged.extend(labels.clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.presentation_text.borrow_mut() = labels;
        Ok(())
    }
    fn set_scene_text(
        &self,
        scene: &Scene,
        selected: &[ObjectId],
    ) -> anyhow::Result<()> {
        let labels = scene
            .text_labels(selected)
            .map_err(anyhow::Error::msg)?;
        let mut merged = labels.clone();
        merged.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
    pub fn refresh_text_bounds(&self, scene: &mut Scene) {
        if let Some(lane) = self.text.borrow().as_ref() {
            for row in scene.texts_mut() {
                if let Some(run) = lane
                    .document
                    .runs
                    .iter()
                    .find(|run| run.label.id == row.id.value())
                {
                    row.prepared.extent =
                        run.buffer
                            .layout_runs()
                            .fold([0f64; 2], |[w, h], line| {
                                [
                                    w.max(line.line_w as f64),
                                    h.max((line.line_top + line.line_height) as f64),
                                ]
                            });
                }
            }
        }
    }
    pub fn text_stats(&self) -> Option<crate::scene_text_gpu::TextStats> {
        self.text
            .borrow()
            .as_ref()
            .map(|lane| lane.stats.clone())
    }
    pub fn text_scene_usage(&self) -> [u64; 4] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 4], |lane| {
                [
                    lane.allocated_bytes(),
                    lane.texture_bytes(),
                    lane.stats.world_plane_rasterizations,
                    lane.document.runs.len() as u64,
                ]
            })
    }
    pub fn replace_text_fonts(&self, fonts: Vec<Vec<u8>>) -> anyhow::Result<()> {
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            lane.document.replace_fonts(fonts.clone())?;
        } else {
            let mut document = crate::text::TextDocument::new();
            document.replace_fonts(fonts.clone())?;
        }
        *self.full_text_fonts.borrow_mut() = Some(fonts);
        Ok(())
    }
    pub fn clear_text_fonts(&self) {
        self.full_text_fonts.borrow_mut().take();
    }

    pub fn text_error(&self) -> Option<String> {
        self.text_error.borrow().clone()
    }
    pub fn text_usage(&self) -> [u64; 6] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 6], |lane| {
                [
                    lane.document.shape_count,
                    lane.stats.preparations,
                    lane.stats.skipped_preparations,
                    lane.stats.raster_images as u64,
                    lane.stats.raster_image_capacity_bytes as u64,
                    lane.stats.atlas_resets,
                ]
            })
    }
    pub fn text_demo(&self) -> anyhow::Result<()> {
        self.set_text(vec![crate::text_example::label()])
    }

    pub fn cloud_camera(&self, camera: &crate::camera::Camera) {
        self.cloud_view
            .set(crate::cloud_gpu::View::anchored(
                camera,
                self.anchor(),
            ));
        self.sheets.camera(camera);
    }
    pub fn cloud_usage(&self) -> [u64; 5] {
        self.clouds
            .borrow()
            .as_ref()
            .map_or([0; 5], |lane| lane.usage())
    }
    pub fn cloud_error(&self) -> Option<String> {
        self.cloud_error.borrow().clone()
    }
    fn set_clouds(
        &self,
        scene: &Scene,
        selected: &[ObjectId],
    ) -> Result<(), &'static str> {
        if scene.clouds().is_empty() && scene.streams().is_empty() {
            self.clouds.borrow_mut().take();
            self.cloud_error.borrow_mut().take();
            return Ok(());
        }
        let mut owner = self.clouds.borrow_mut();
        let lane = owner.get_or_insert_with(|| {
            crate::cloud_gpu::Lane::new(
                &self.text_context(),
                self.format,
                &self.clipping.borrow().group,
            )
        });
        lane.set_scene(&self.text_context(), scene, selected)
    }
    fn prepare_clouds(&self, transform: [f32; 16]) {
        if let Some(lane) = self.clouds.borrow_mut().as_mut() {
            lane.set_clipping(self.clipping.borrow().writes);
            if let Err(error) = lane.prepare(
                &self.text_context(),
                transform,
                self.size,
                self.density,
                self.cloud_view.get(),
            ) {
                lane.failed();
                *self.cloud_error.borrow_mut() = Some(error.to_string());
            } else {
                self.cloud_error.borrow_mut().take();
            }
        }
    }
    pub fn set_opacity(&mut self, value: f32) {
        if !value.is_finite() {
            return;
        }
        let value = value.clamp(0., 1.);
        if self.opacity.get() == value {
            return;
        }
        self.opacity.set(value);
        self.picker.cancel();
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pick_mode: crate::pick_sources::Mode,
    pick_cursor: Option<([f32; 2], [u64; 3])>,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(
            &device,
            crate::clipping_gpu::source(include_str!("cloud_points.wgsl"), 2, 0),
            "Cloud points",
        );
        crate::shader_check::check(
            &device,
            include_str!("cloud_resolve.wgsl").into(),
            "Cloud resolve",
        );
        crate::shader_check::check(
            &device,
            crate::finite_project_source::source(),
            "Finite projection",
        );
        crate::shader_check::check(
            &device,
            crate::finite_project_source::tiles(),
            "Finite tiles",
        );
        crate::shader_check::check(
            &device,
            crate::finite_project_source::scan(),
            "Finite scan",
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0).into(),
            ),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout =
            crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"),
            bind_group_layouts: &[
                Some(&view_layout),
                Some(&model_layout),
                Some(&crate::clipping_gpu::layout(&device)),
                Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device)),
            ],
            immediate_size: 0,
        });
        let mesh_pipeline = |depth, primitive_write| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("triangle"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[crate::draw_slots::layout()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(crate::primitive_image::target(
                            primitive_write,
                        )),
                    ],
                }),
                primitive: Default::default(),
                depth_stencil: Some(depth),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = mesh_pipeline(crate::depth::surface(), true);
        let tiles = std::cell::RefCell::new(crate::tile_gpu::Tiles::new(
            &device,
            &view_layout,
        ));
        let instances =
            crate::instance_gpu::Lane::new(&device, &queue, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(), false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let primitive = crate::primitive_image::Image::new(&device, [640, 480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets = crate::sheet_gpu::Lane::new(&device, format);
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            space: Default::default(),
            opacity: std::cell::Cell::new(1.),
            ambient_enabled: Default::default(),
            ambient: Default::default(),
            ambient_rows: Default::default(),
            pyramid: Default::default(),
            pyramid_revision: 0,
            caps: std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),
            clipping: std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(
                &device,
            )),
            tiles,
            guides,
            measurement,
            sheets,
            instances,
            instance_rows: Default::default(),
            clouds: Default::default(),
            cloud_view: Default::default(),
            cloud_error: Default::default(),
            outline: Default::default(),
            global_outline: Default::default(),
            outlines_enabled: Default::default(),
            outline_composite: Default::default(),
            full_text_fonts: Default::default(),
            presentation_text: Default::default(),
            scene_text: Default::default(),
            text: std::cell::RefCell::new(None),
            text_error: std::cell::RefCell::new(None),
            format,
            device,
            queue,
            pipeline,
            highlight_pipeline,
            highlight,
            preview,
            widget,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
            settings_writes: 0,
            settings_bytes: 0,
            uniform,
            view_group,
            depth,
            primitive,
            strokes,
            paths,
            markers,
            grid,
            grid_settings: None,
            grid_plane: crate::cplane::CPlane::Xy,
            grid_anchor: [0.; 3],
            size: [640, 480],
            density: 1.0,
            picker,
            faces,
            mesh_ink,
            sources: Default::default(),
            pick_mode: Default::default(),
            pick_cursor: None,
        };
        for lane in [
            &mut renderer.mesh_ink,
            &mut renderer.strokes,
            &mut renderer.paths,
            &mut renderer.markers,
            &mut renderer.grid,
        ] {
            lane.enable_finite(&renderer.device, format);
        }
        renderer
            .highlight
            .enable_finite(&renderer.device, format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self) -> [f64; 3] {
        self.space.anchor()
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] {
            self.cancel_pick();
        }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive = crate::primitive_image::Image::new(&self.device, self.size);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density {
            self.cancel_pick();
            self.density = density;
        }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [
            count,
            bytes,
            self.strokes.uploads,
            self.strokes.uploaded_bytes,
        ]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [
            count,
            bytes,
            self.paths.uploads,
            self.paths.uploaded_bytes,
        ]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [
            count,
            bytes,
            self.markers.uploads,
            self.markers.uploaded_bytes,
        ]
    }

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {
        self.instances.ordinary.clear();
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor = self.anchor();
        self.ambient_rows.set(scene);
        if let Some(ambient) = self.ambient.borrow_mut().as_mut() {
            if self.ambient_enabled.get() {
                ambient.sources.set(
                    &self.device,
                    &self.queue,
                    &self.ambient_rows.specs,
                );
            }
        }
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        self.cancel_pick();
        self.faces.clear();
        self.preview
            .set(&self.device, None)
            .expect("Empty tool preview");
        self.highlight
            .set(
                &self.device,
                &self.pipeline.get_bind_group_layout(1),
                scene,
                None,
            )
            .expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene)
            .expect("Prepared source topology is bounded");
        self.mesh_ink
            .set(
                &self.device,
                self.sources.mesh_ink.iter().copied(),
            )
            .expect("Prepared source edges");
        self.mesh_ink
            .set_ids(
                &self.device,
                self.sources.mesh_ids.iter().copied(),
            )
            .expect("Source edge IDs match");
        let planes = crate::clipping_rows::owners(scene);
        let flags = self
            .sources
            .mesh_ids
            .iter()
            .map(|id| {
                crate::sub_id::decode(id[1])
                    .and_then(|(_, address)| self.sources.edges.get(address))
                    .map_or(0, |target| {
                        if planes.contains(&target.owner) {
                            4096
                        } else {
                            0
                        }
                    })
            })
            .collect();
        self.mesh_ink
            .set_flags(&self.device, flags)
            .expect("Original plane ink flags match source addresses");
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut marker = row.marker();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            marker.colour = colour;
                        }
                        marker
                    })
                    .chain(
                        scene
                            .control_markers()
                            .into_iter()
                            .map(|c| c.display),
                    ),
            )
            .expect("Prepared point placement fits the display range");
        self.paths
            .set_chains(
                &self.device,
                scene
                    .paths()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .flat_map(|row| {
                        let mut segments = row.segments();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            for segment in &mut segments {
                                segment.stroke.colour = colour;
                            }
                        }
                        segments
                    }),
            )
            .expect("Prepared path placement fits the display range");
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut stroke = row.stroke();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            stroke.colour = colour;
                        }
                        stroke
                    }),
            )
            .expect("Prepared line placement fits the display range");
        self.markers
            .set_ids(
                &self.device,
                self.sources.marker_ids.iter().copied(),
            )
            .expect("Marker owners match");
        self.paths
            .set_ids(
                &self.device,
                self.sources.path_ids.iter().copied(),
            )
            .expect("Path owners match");
        self.strokes
            .set_ids(
                &self.device,
                self.sources.line_ids.iter().copied(),
            )
            .expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
            .iter()
            .filter(|row| scene.visible(row.id))
        {
            let matching = old.iter().position(|row| {
                row.id == object.id
                    && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh)
            });
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(
                    &self.queue,
                    object,
                    style(scene, object.id, selected == Some(object.id)),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(
                    &crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue),
                    &object.mesh,
                );
                self.settings_allocations += 1;
                GpuMesh::with_geometry(
                    &self.device,
                    &layout,
                    object,
                    style(scene, object.id, selected == Some(object.id)),
                    geometry,
                )
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(
            &self.queue,
            scene,
            &mut self.meshes,
            &self.clipping.borrow().rows,
        );
        for mesh in &mut self.meshes {
            mesh.set_xray(&self.queue, self.opacity.get() == 0.);
        }
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
            self.meshes.iter().map(|row| {
                (
                    row.id,
                    std::rc::Rc::as_ptr(&row.geometry) as usize,
                    scene.instances.contains_key(&row.id),
                )
            }),
            crate::object_rows::capacity(&self.device),
        );
        self.instances
            .set(
                &self.device,
                &self.queue,
                &mut self.meshes,
                &self.instance_rows,
                true,
            )
            .expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        self.sheets
            .set_anchored(
                &self.device,
                &self.queue,
                scene,
                selected.as_slice(),
                &self.sources.sheet_bases,
                anchor,
            )
            .expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        if let Err(error) = self.set_clouds(scene, selected.as_slice()) {
            *self.cloud_error.borrow_mut() = Some(error.to_string());
        }
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
    }

    pub fn request_pick(
        &mut self,
        screen: [f32; 2],
        transform: [f32; 16],
    ) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density)
            .ok_or("Invalid pick cursor")?;
        let origin = self.anchor();
        self.picker.queue_anchored(
            crate::pick_gpu::Request {
                window,
                transform,
                density: self.density,
                mode: self.pick_mode,
            },
            origin,
        )?;
        self.pick_cursor = Some((screen, origin.map(f64::to_bits)));
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(
            transform,
            self.size,
            self.density,
            self.anchor(),
        ) else {
            self.cancel_pick();
            return;
        };
        if self.pick_mode == crate::pick_sources::Mode::Face
            && !self.faces.ready(self.meshes.len())
        {
            self.faces.set(
                &self.device,
                &self.meshes,
                &self.sources.mesh_bases,
                &self.clipping.borrow().buffer,
            );
        }
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            if let Err(error) = lane.prepare(
                &self.text_context(),
                &self.text_frame(transform),
            ) {
                *self.text_error.borrow_mut() = Some(error.to_string());
                return;
            }
        }
        self.clipping.borrow_mut().frame_anchored(
            &self.queue,
            transform,
            self.anchor(),
            self.size[1],
            self.density,
        );
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping = self.clipping.borrow_mut();
            clipping.frame_anchored(
                &self.queue,
                transform,
                self.anchor(),
                self.size[1],
                self.density,
            );
            let mut bytes = transform
                .iter()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>();
            bytes.resize(80, 0);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
                let mut tiles = self.tiles.borrow_mut();
                tiles.prepare(&self.device, &self.queue, self.size);
                let mut encoder = self
                    .device
                    .create_command_encoder(&Default::default());
                tiles.encode(
                    &self.device,
                    &mut encoder,
                    &self.view_group,
                    transform,
                );
                self.queue.submit([encoder.finish()]);
                tiles.map_report();
                crate::finite_ink::Tables {
                    projected: tiles.projected.clone(),
                    tiles: tiles.buffer.clone(),
                    primitive: self.primitive.view.clone(),
                }
            };
            self.picker.submit_finite(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[
                    &self.mesh_ink,
                    &self.strokes,
                    &self.paths,
                    &self.markers,
                ],
                &self.faces,
                self.text.borrow().as_ref(),
                self.clouds.borrow().as_ref(),
                Some(&self.sheets),
                Some(&tables),
                &clipping.group,
                &clipping.buffer,
                Some(&mut self.caps.borrow_mut()),
                Some(&self.instances),
            );
        }
    }

    pub fn take_pick(
        &mut self,
        transform: [f32; 16],
    ) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(
            transform,
            self.size,
            self.density,
            self.anchor(),
        ) {
            Ok(view) => view,
            Err(error) => {
                self.cancel_pick();
                return Some(Err(error));
            }
        };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => {
                self.pick_cursor = None;
                Some(Ok(hit))
            }
            Some(Ok(crate::pick_gpu::Reply::Stale)) => {
                if let Some((screen, _)) = self
                    .pick_cursor
                    .filter(|(_, origin)| *origin != self.anchor().map(f64::to_bits))
                {
                    if let Err(error) = self.request_pick(screen, transform) {
                        self.cancel_pick();
                        return Some(Err(error));
                    }
                } else {
                    self.pump_pick(transform);
                    if self.picker.usage()[0] == 0 && !self.picker.queued() {
                        self.pick_cursor = None;
                    }
                }
                None
            }
            Some(Err(error)) => {
                self.cancel_pick();
                Some(Err(error))
            }
            None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode {
            self.cancel_pick();
            self.pick_mode = mode;
        }
    }

    pub fn resolve_pick(
        &self,
        scene: &Scene,
        pixel: [u32; 2],
    ) -> Option<crate::pick_sources::Target> {
        self.sources
            .resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.mesh_ink.usage();
        [
            count,
            bytes,
            self.faces.bytes(),
            self.sources.bytes() as u64,
        ]
    }

    pub fn cancel_pick(&mut self) {
        self.pick_cursor = None;
        self.picker.cancel();
    }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [
            active,
            pending,
            bytes
                + self.strokes.pick_bytes()
                + self.paths.pick_bytes()
                + self.markers.pick_bytes()
                + self.mesh_ink.pick_bytes()
                + self.mesh_ink.usage()[1]
                + self.faces.bytes(),
            self.picker.submitted,
            self.picker.coalesced,
        ]
    }

    pub fn geometry_owners(
        &self,
    ) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] {
        let mut usage = crate::memory::gpu(&self.meshes);
        usage[1] += self.geometry.allocated_bytes();
        usage
    }
    pub fn arena_cpu_bytes(&self) -> usize {
        self.geometry.cpu_bytes()
    }

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes
            + self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * crate::object_record::BYTES as u64
            + self.settings_bytes as u64
            + self.strokes.uploaded_bytes
            + self.paths.uploaded_bytes
            + self.markers.uploaded_bytes
            + self.grid.uploaded_bytes
    }

    pub fn set_grid(
        &mut self,
        settings: Option<crate::grid::Settings>,
    ) -> Result<(), &'static str> {
        self.set_grid_on(settings, crate::cplane::CPlane::Xy)
    }
    pub fn set_grid_on(
        &mut self,
        settings: Option<crate::grid::Settings>,
        plane: crate::cplane::CPlane,
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        if settings == self.grid_settings
            && plane == self.grid_plane
            && anchor == self.grid_anchor
        {
            return Ok(());
        }
        let strokes = settings
            .map(|s| s.strokes_on_at(plane, anchor))
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        self.grid_plane = plane;
        self.grid_anchor = anchor;
        Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage();
        [
            count,
            bytes,
            self.grid.uploads,
            self.grid.uploaded_bytes,
        ]
    }

    pub fn set_highlight(
        &mut self,
        scene: &Scene,
        target: Option<crate::pick_sources::Target>,
    ) -> Result<(), &'static str> {
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        let anchor = self.anchor();
        self.highlight.set_at(
            &self.device,
            &self.pipeline.get_bind_group_layout(1),
            scene,
            target,
            anchor,
        )
    }

    pub fn select_objects(
        &mut self,
        scene: &Scene,
        ids: &[ObjectId],
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        for mesh in &mut self.meshes {
            if let Some(row) = scene
                .objects()
                .iter()
                .find(|r| r.id == mesh.id)
            {
                let [calls, bytes] = mesh.update(
                    &self.queue,
                    row,
                    style(scene, row.id, ids.contains(&row.id)),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
            }
        }
        self.instances.set(
            &self.device,
            &self.queue,
            &mut self.meshes,
            &self.instance_rows,
            false,
        )?;
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        self.set_clouds(scene, ids)?;
        self.sheets.set_anchored(
            &self.device,
            &self.queue,
            scene,
            ids,
            &self.sources.sheet_bases,
            anchor,
        )?;
        if !ids.is_empty() {
            self.highlight
                .objects_at(&self.device, scene, ids, anchor)?;
        }
        self.set_scene_text(scene, ids)
            .map_err(|_| "Cannot prepare scene labels")?;
        Ok(())
    }

    pub fn set_guides(
        &mut self,
        overlay: &crate::guide_gpu::Overlay,
    ) -> Result<(), &'static str> {
        if self.guides.set(&self.device, overlay)? {
            self.set_scene_measure_text()
                .map_err(|_| "Cannot prepare editing guide label")?
        }
        Ok(())
    }
    pub fn guide_bytes(&self) -> u64 {
        self.guides.bytes()
    }
    pub fn set_measurement(
        &mut self,
        mark: Option<&crate::measure_mark::Mark>,
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        if self
            .measurement
            .set_at(&self.device, mark, anchor)?
        {
            self.set_scene_measure_text()
                .map_err(|_| "Cannot prepare measurement annotation")?;
        }
        Ok(())
    }
    pub fn measurement_bytes(&self) -> u64 {
        self.measurement.bytes()
    }
    fn merge_measurement(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> Vec<crate::text::TextLabel> {
        if let Some(mark) = &self.measurement.mark {
            labels.extend(mark.labels(&labels));
        }
        labels.extend(self.guides.overlay.labels(&labels));
        labels
    }
    fn set_scene_measure_text(&self) -> anyhow::Result<()> {
        let mut labels = self.scene_text.borrow().clone();
        labels.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(labels))
    }
    pub fn set_preview(
        &mut self,
        draft: Option<&crate::tool::Draft>,
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        self.preview
            .set_at(&self.device, draft, anchor)
    }
    pub fn preview_bytes(&self) -> u64 {
        self.preview.bytes()
    }

    pub fn set_gizmo(
        &mut self,
        camera: &crate::camera::Camera,
        origin: Option<[f64; 3]>,
        active: f32,
    ) {
        let anchor = self.anchor();
        let matrix = camera.view_projection_anchored(anchor);
        let layout = origin
            .map(|p| std::array::from_fn(|i| p[i] - anchor[i]))
            .and_then(|p| {
                crate::widget_layout::world_per_pixel(
                    &matrix,
                    p,
                    self.size[1] as f64 / self.density as f64,
                )
                .and_then(|scale| {
                    crate::widget_layout::layout(&matrix, p, scale, self.size, active)
                })
            });
        self.widget
            .prepare(&self.device, &self.queue, layout);
    }
    pub fn gizmo_usage(&self) -> [u64; 4] {
        self.widget.usage()
    }
    pub fn gizmo_extent(&self) -> Option<[u32; 2]> {
        self.widget.extent()
    }

    fn text_context(&self) -> crate::text_gpu_support::GpuCtx {
        crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue)
    }
    fn text_frame(&self, transform: [f32; 16]) -> crate::scene_text_gpu::TextFrame {
        crate::scene_text_gpu::TextFrame {
            mvp: transform,
            origin: self.anchor(),
            framebuffer: self.size,
            logical: [
                self.size[0] as f64 / self.density as f64,
                self.size[1] as f64 / self.density as f64,
            ],
            ortho_half_height: 0.,
            clip: {
                let clip = self.clipping.borrow();
                let mut planes = [[0.; 4]; 6];
                if clip.enabled {
                    for (p, row) in planes.iter_mut().zip(&clip.rows) {
                        *p = [
                            row.plane.normal[0],
                            row.plane.normal[1],
                            row.plane.normal[2],
                            row.plane.offset,
                        ];
                    }
                }
                planes
            },
        }
    }
    fn adopt_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
            self.text_error.borrow_mut().take();
            return Ok(());
        }
        let mut lane = self.text.borrow_mut();
        if lane.is_none() {
            let mut new = crate::scene_text_gpu::Lane::new(
                &self.text_context(),
                crate::text_gpu_support::Target {
                    format: self.format,
                    samples: 1,
                },
            );
            if let Some(fonts) = self.full_text_fonts.borrow().as_ref() {
                new.document.replace_fonts(fonts.clone())?;
            }
            *lane = Some(new);
        }
        lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> anyhow::Result<()> {
        for (i, label) in labels.iter_mut().enumerate() {
            label.id = u32::MAX - i as u32;
            label.object = None;
        }
        let mut merged = self.scene_text.borrow().clone();
        merged.extend(labels.clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.presentation_text.borrow_mut() = labels;
        Ok(())
    }
    fn set_scene_text(
        &self,
        scene: &Scene,
        selected: &[ObjectId],
    ) -> anyhow::Result<()> {
        let labels = scene
            .text_labels(selected)
            .map_err(anyhow::Error::msg)?;
        let mut merged = labels.clone();
        merged.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
    pub fn refresh_text_bounds(&self, scene: &mut Scene) {
        if let Some(lane) = self.text.borrow().as_ref() {
            for row in scene.texts_mut() {
                if let Some(run) = lane
                    .document
                    .runs
                    .iter()
                    .find(|run| run.label.id == row.id.value())
                {
                    row.prepared.extent =
                        run.buffer
                            .layout_runs()
                            .fold([0f64; 2], |[w, h], line| {
                                [
                                    w.max(line.line_w as f64),
                                    h.max((line.line_top + line.line_height) as f64),
                                ]
                            });
                }
            }
        }
    }
    pub fn text_stats(&self) -> Option<crate::scene_text_gpu::TextStats> {
        self.text
            .borrow()
            .as_ref()
            .map(|lane| lane.stats.clone())
    }
    pub fn text_scene_usage(&self) -> [u64; 4] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 4], |lane| {
                [
                    lane.allocated_bytes(),
                    lane.texture_bytes(),
                    lane.stats.world_plane_rasterizations,
                    lane.document.runs.len() as u64,
                ]
            })
    }
    pub fn replace_text_fonts(&self, fonts: Vec<Vec<u8>>) -> anyhow::Result<()> {
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            lane.document.replace_fonts(fonts.clone())?;
        } else {
            let mut document = crate::text::TextDocument::new();
            document.replace_fonts(fonts.clone())?;
        }
        *self.full_text_fonts.borrow_mut() = Some(fonts);
        Ok(())
    }
    pub fn clear_text_fonts(&self) {
        self.full_text_fonts.borrow_mut().take();
    }

    pub fn text_error(&self) -> Option<String> {
        self.text_error.borrow().clone()
    }
    pub fn text_usage(&self) -> [u64; 6] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 6], |lane| {
                [
                    lane.document.shape_count,
                    lane.stats.preparations,
                    lane.stats.skipped_preparations,
                    lane.stats.raster_images as u64,
                    lane.stats.raster_image_capacity_bytes as u64,
                    lane.stats.atlas_resets,
                ]
            })
    }
    pub fn text_demo(&self) -> anyhow::Result<()> {
        self.set_text(vec![crate::text_example::label()])
    }

    pub fn cloud_camera(&self, camera: &crate::camera::Camera) {
        self.cloud_view
            .set(crate::cloud_gpu::View::anchored(
                camera,
                self.anchor(),
            ));
        self.sheets.camera(camera);
    }
    pub fn cloud_usage(&self) -> [u64; 5] {
        self.clouds
            .borrow()
            .as_ref()
            .map_or([0; 5], |lane| lane.usage())
    }
    pub fn cloud_error(&self) -> Option<String> {
        self.cloud_error.borrow().clone()
    }
    fn set_clouds(
        &self,
        scene: &Scene,
        selected: &[ObjectId],
    ) -> Result<(), &'static str> {
        if scene.clouds().is_empty() && scene.streams().is_empty() {
            self.clouds.borrow_mut().take();
            self.cloud_error.borrow_mut().take();
            return Ok(());
        }
        let mut owner = self.clouds.borrow_mut();
        let lane = owner.get_or_insert_with(|| {
            crate::cloud_gpu::Lane::new(
                &self.text_context(),
                self.format,
                &self.clipping.borrow().group,
            )
        });
        lane.set_scene(&self.text_context(), scene, selected)
    }
    fn prepare_clouds(&self, transform: [f32; 16]) {
        if let Some(lane) = self.clouds.borrow_mut().as_mut() {
            lane.set_clipping(self.clipping.borrow().writes);
            if let Err(error) = lane.prepare(
                &self.text_context(),
                transform,
                self.size,
                self.density,
                self.cloud_view.get(),
            ) {
                lane.failed();
                *self.cloud_error.borrow_mut() = Some(error.to_string());
            } else {
                self.cloud_error.borrow_mut().take();
            }
        }
    }
    pub fn set_opacity(&mut self, value: f32) {
        if !value.is_finite() {
            return;
        }
        let value = value.clamp(0., 1.);
        if self.opacity.get() == value {
            return;
        }
        self.opacity.set(value);
        self.cancel_pick();
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
```

### 2. `src/global_outline_tests.rs`

Retain old-origin rejection, fresh empty picks and restored-owner assertions while verifying the newly connected automatic retry. Supply real released-gesture completion/Undo/source reuse, newest pending cursor replay, one-flight bounds and final explicit/source cancellation across rebases.

**Locate this existing block:**

```rust
    );
    assert_eq!(&gpu.pick_usage()[..2], &[0, 0]);
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    wait(&gpu);
    assert!(
        gpu.take_pick(transform)
            .unwrap()
            .unwrap()
            .is_none()
    );
    gpu.space.update([0.; 3], 3., 300.).unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, [0.; 3]);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    wait(&gpu);
    assert_eq!(
        gpu.take_pick(transform)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    assert_eq!(&gpu.pick_usage()[..2], &[1, 0]);
    wait(&gpu);
    assert!(
        gpu.take_pick(transform)
            .unwrap()
            .unwrap()
            .is_none(),
        "The automatic current-frame retry is empty"
    );
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    wait(&gpu);
    assert!(
        gpu.take_pick(transform)
            .unwrap()
            .unwrap()
            .is_none()
    );
    gpu.space.update([0.; 3], 3., 300.).unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, [0.; 3]);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    wait(&gpu);
    assert_eq!(
        gpu.take_pick(transform)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}

#[test]
fn actual_rebased_pick_finishes_a_released_drag_without_reuploading_sources() {
    use crate::editor::{Action, Change, Editor};
    let mut editor = Editor::default();
    editor.scene.clear();
    let owner = editor
        .scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.4, -0.4, -0.5],
                    [0.4, -0.4, -0.5],
                    [0., 0.4, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    editor
        .scene
        .translate(owner, [2000., 0., 0.])
        .unwrap();
    editor.camera.target = [2000., 0., 0.];
    let original = editor.scene.placement(owner).unwrap().m;
    let source = editor.scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    let mut gpu = renderer(&editor.scene);
    let uploads = gpu.stats()[0];
    let transform = editor.camera.uniform();
    let size = [320., 240.];
    editor
        .apply(Action::CanvasStart {
            from: [0., 0.],
            to: [0.2, 0.],
            size,
            modified: false,
            pointer: 1,
        })
        .unwrap();
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    editor
        .apply(Action::CanvasRelease {
            screen: [0.4, 0.],
            size,
            add: false,
            remove: false,
        })
        .unwrap();
    assert_eq!(
        editor.scene.placement(owner).unwrap().m,
        original
    );
    gpu.space
        .update(
            editor.camera.target,
            editor.camera.distance,
            0.,
        )
        .unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, editor.camera.target);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    let local = editor
        .camera
        .view_projection_anchored(gpu.anchor())
        .to_f32();
    assert_ne!(local, transform);
    wait(&gpu);
    assert!(gpu.take_pick(local).is_none());
    assert_eq!(&gpu.pick_usage()[..2], &[1, 0]);
    assert_eq!(
        editor.scene.placement(owner).unwrap().m,
        original
    );
    wait(&gpu);
    let hit = gpu
        .take_pick(local)
        .unwrap()
        .unwrap()
        .and_then(|pixel| gpu.resolve_pick(&editor.scene, pixel));
    assert_eq!(hit.unwrap().owner, owner);
    assert_eq!(
        editor
            .apply(Action::CanvasHit(
                hit.map(|target| target.owner)
            ))
            .unwrap(),
        Change::Scene
    );
    assert_ne!(
        editor.scene.placement(owner).unwrap().m,
        original
    );
    assert!(Rc::ptr_eq(
        &source,
        editor.scene.objects()[0].geometry().unwrap()
    ));
    gpu.set_scene(&editor.scene, editor.selected);
    assert_eq!(gpu.stats()[0], uploads);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(
        editor.scene.placement(owner).unwrap().m,
        original
    );
}
#[test]
fn actual_rebased_pending_cursor_stays_newest_and_cancellation_never_revives_it() {
    let mut scene = Scene::empty();
    let make = || {
        PreparedMesh::triangle(
            [
                [-0.18, -0.18, -0.5],
                [0.18, -0.18, -0.5],
                [0., 0.18, -0.5],
            ],
            [0.2, 0.4, 0.8],
        )
        .unwrap()
    };
    let a = scene.insert(make()).unwrap();
    let b = scene.insert(make()).unwrap();
    scene.translate(a, [1999.5, 0., 0.]).unwrap();
    scene.translate(b, [2000.5, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    let mut old = matrix();
    old[12] = -2000.;
    let local = matrix();
    gpu.request_pick([-0.5, 0.], old).unwrap();
    gpu.request_pick([0.5, 0.], old).unwrap();
    assert_eq!(&gpu.pick_usage()[..2], &[1, 1]);
    gpu.space
        .update([2000., 0., 0.], 3., 0.)
        .unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, [2000., 0., 0.]);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    wait(&gpu);
    assert!(gpu.take_pick(local).is_none());
    assert_eq!(&gpu.pick_usage()[..2], &[1, 0]);
    wait(&gpu);
    assert_eq!(
        gpu.take_pick(local)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        b.value()
    );
    assert_eq!(gpu.pick_usage()[3], 2);
    assert!(gpu.take_pick(local).is_none());
    gpu.request_pick([-0.5, 0.], local).unwrap();
    gpu.cancel_pick();
    gpu.space.update([0.; 3], 3., 300.).unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, [0.; 3]);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    wait(&gpu);
    assert!(gpu.take_pick(old).is_none());
    assert_eq!(&gpu.pick_usage()[..2], &[0, 0]);
    gpu.request_pick([-0.5, 0.], old).unwrap();
    gpu.set_scene(&Scene::empty(), None);
    gpu.space
        .update([2000., 0., 0.], 3., 600.)
        .unwrap();
    wait(&gpu);
    assert!(gpu.take_pick(local).is_none());
    assert_eq!(&gpu.pick_usage()[..2], &[0, 0]);
    assert_eq!(gpu.usage(), [0; 4]);
```

## <a id="checkpoint-96zzzzzp-share-derived-inputs-for-origin-refresh"></a>7. Share derived inputs for origin refresh

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs, src/highli…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current origin</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing source packers</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">synchronized ink/IDs and…</text>
</svg>
</div>

**What this code does:** Refresh local draw inputs without replacing the document or cancelling its gesture.

### 1. `src/renderer.rs`

Keep the existing source-preparation block for the helper below, then replace this call site with the shared owner. Introduce one owner for the existing source preparation. Move the complete source preparation already typed; its algorithms and original identity rules are reused. Use one current origin throughout the reused preparation. Reuse the precise local source accessor without changing styles or original pick addresses. Initialize newly adopted records directly in the renderer frame.

**Locate this existing block:**

```rust
            .expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene)
            .expect("Prepared source topology is bounded");
        self.mesh_ink
            .set(
                &self.device,
                self.sources.mesh_ink.iter().copied(),
            )
            .expect("Prepared source edges");
        self.mesh_ink
            .set_ids(
                &self.device,
                self.sources.mesh_ids.iter().copied(),
            )
            .expect("Source edge IDs match");
        let planes = crate::clipping_rows::owners(scene);
        let flags = self
            .sources
            .mesh_ids
            .iter()
            .map(|id| {
                crate::sub_id::decode(id[1])
                    .and_then(|(_, address)| self.sources.edges.get(address))
                    .map_or(0, |target| {
                        if planes.contains(&target.owner) {
                            4096
                        } else {
                            0
                        }
                    })
            })
            .collect();
        self.mesh_ink
            .set_flags(&self.device, flags)
            .expect("Original plane ink flags match source addresses");
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut marker = row.marker();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            marker.colour = colour;
                        }
                        marker
                    })
                    .chain(
                        scene
                            .control_markers()
                            .into_iter()
                            .map(|c| c.display),
                    ),
            )
            .expect("Prepared point placement fits the display range");
        self.paths
            .set_chains(
                &self.device,
                scene
                    .paths()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .flat_map(|row| {
                        let mut segments = row.segments();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            for segment in &mut segments {
                                segment.stroke.colour = colour;
                            }
                        }
                        segments
                    }),
            )
            .expect("Prepared path placement fits the display range");
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut stroke = row.stroke();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            stroke.colour = colour;
                        }
                        stroke
                    }),
            )
            .expect("Prepared line placement fits the display range");
        self.markers
            .set_ids(
                &self.device,
                self.sources.marker_ids.iter().copied(),
            )
            .expect("Marker owners match");
        self.paths
            .set_ids(
                &self.device,
                self.sources.path_ids.iter().copied(),
            )
            .expect("Path owners match");
        self.strokes
            .set_ids(
                &self.device,
                self.sources.line_ids.iter().copied(),
            )
            .expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
            .iter()
            .filter(|row| scene.visible(row.id))
        {
            let matching = old.iter().position(|row| {
                row.id == object.id
                    && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh)
            });
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(
                    &self.queue,
                    object,
                    style(scene, object.id, selected == Some(object.id)),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(
                    &crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue),
                    &object.mesh,
                );
                self.settings_allocations += 1;
                GpuMesh::with_geometry(
                    &self.device,
                    &layout,
                    object,
                    style(scene, object.id, selected == Some(object.id)),
                    geometry,
                )
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(
            &self.queue,
            scene,
            &mut self.meshes,
            &self.clipping.borrow().rows,
        );
        for mesh in &mut self.meshes {
            mesh.set_xray(&self.queue, self.opacity.get() == 0.);
        }
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
            self.meshes.iter().map(|row| {
                (
                    row.id,
                    std::rc::Rc::as_ptr(&row.geometry) as usize,
                    scene.instances.contains_key(&row.id),
                )
            }),
            crate::object_rows::capacity(&self.device),
        );
        self.instances
            .set(
                &self.device,
                &self.queue,
                &mut self.meshes,
                &self.instance_rows,
                true,
            )
            .expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        self.sheets
            .set_anchored(
                &self.device,
                &self.queue,
                scene,
                selected.as_slice(),
                &self.sources.sheet_bases,
                anchor,
            )
            .expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        if let Err(error) = self.set_clouds(scene, selected.as_slice()) {
            *self.cloud_error.borrow_mut() = Some(error.to_string());
        }
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
            .iter()
            .filter(|row| scene.visible(row.id))
        {
            let matching = old.iter().position(|row| {
                row.id == object.id
                    && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh)
            });
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(
                    &self.queue,
                    object,
                    style(scene, object.id, selected == Some(object.id)),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(
                    &crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue),
                    &object.mesh,
                );
                self.settings_allocations += 1;
                GpuMesh::with_geometry_at(
                    &self.device,
                    &layout,
                    object,
                    style(scene, object.id, selected == Some(object.id)),
                    geometry,
                    anchor,
                )
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(
            &self.queue,
            scene,
            &mut self.meshes,
            &self.clipping.borrow().rows,
        );
        for mesh in &mut self.meshes {
            mesh.set_xray(&self.queue, self.opacity.get() == 0.);
        }
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
            self.meshes.iter().map(|row| {
                (
                    row.id,
                    std::rc::Rc::as_ptr(&row.geometry) as usize,
                    scene.instances.contains_key(&row.id),
                )
            }),
            crate::object_rows::capacity(&self.device),
        );
        self.instances
            .set(
                &self.device,
                &self.queue,
                &mut self.meshes,
                &self.instance_rows,
                true,
            )
            .expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        self.sheets
            .set_anchored(
                &self.device,
                &self.queue,
                scene,
                selected.as_slice(),
                &self.sources.sheet_bases,
                anchor,
            )
            .expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        if let Err(error) = self.set_clouds(scene, selected.as_slice()) {
            *self.cloud_error.borrow_mut() = Some(error.to_string());
        }
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
    }

    fn set_source_ink(&mut self, scene: &Scene) {
        let anchor = self.anchor();
        self.sources = crate::pick_sources::Sources::anchored(scene, anchor)
            .expect("Prepared source topology is bounded");
        self.mesh_ink
            .set(
                &self.device,
                self.sources.mesh_ink.iter().copied(),
            )
            .expect("Prepared source edges");
        self.mesh_ink
            .set_ids(
                &self.device,
                self.sources.mesh_ids.iter().copied(),
            )
            .expect("Source edge IDs match");
        let planes = crate::clipping_rows::owners(scene);
        let flags = self
            .sources
            .mesh_ids
            .iter()
            .map(|id| {
                crate::sub_id::decode(id[1])
                    .and_then(|(_, address)| self.sources.edges.get(address))
                    .map_or(0, |target| {
                        if planes.contains(&target.owner) {
                            4096
                        } else {
                            0
                        }
                    })
            })
            .collect();
        self.mesh_ink
            .set_flags(&self.device, flags)
            .expect("Original plane ink flags match source addresses");
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut marker = row.marker_at(anchor);
                        if let Some(colour) = scene.face_rgba(row.id) {
                            marker.colour = colour;
                        }
                        marker
                    })
                    .chain(
                        scene
                            .control_markers_at(anchor)
                            .into_iter()
                            .map(|c| c.display),
                    ),
            )
            .expect("Prepared point placement fits the display range");
        self.paths
            .set_chains(
                &self.device,
                scene
                    .paths()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .flat_map(|row| {
                        let mut segments = row.segments_at(anchor);
                        if let Some(colour) = scene.face_rgba(row.id) {
                            for segment in &mut segments {
                                segment.stroke.colour = colour;
                            }
                        }
                        segments
                    }),
            )
            .expect("Prepared path placement fits the display range");
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut stroke = row.stroke_at(anchor);
                        if let Some(colour) = scene.face_rgba(row.id) {
                            stroke.colour = colour;
                        }
                        stroke
                    }),
            )
            .expect("Prepared line placement fits the display range");
        self.markers
            .set_ids(
                &self.device,
                self.sources.marker_ids.iter().copied(),
            )
            .expect("Marker owners match");
        self.paths
            .set_ids(
                &self.device,
                self.sources.path_ids.iter().copied(),
            )
            .expect("Path owners match");
        self.strokes
            .set_ids(
                &self.device,
                self.sources.line_ids.iter().copied(),
            )
            .expect("Line owners match");
    }
```

### 2. `src/highlight.rs`

Retain only the identity of the temporary selected part. Begin with no cached selected face. Record the same validated target used for highlight preparation. Reuse the validated target instead of checking its identity twice. Keep plane flags attached to that same validated target. Whole-object ink replaces the temporary part cache. Reuse an unchanged selected face by updating its existing precise record. Supply a read-only geometry-identity witness for the optional GPU fixture.

**Locate this existing block:**

```rust
pub struct Highlight {
    mesh: Option<GpuMesh>,
    strokes: Lane,
    paths: Lane,
    markers: Lane,
}

impl Highlight {
    pub fn set_opacity(&self, value: f32) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.set_opacity(value);
        }
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            mesh: None,
            strokes: Lane::new(device, format),
            paths: Lane::joined(device, format),
            markers: Lane::markers(device, format),
        }
    }

    pub fn enable_finite(
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) {
        for lane in [
            &mut self.strokes,
            &mut self.paths,
            &mut self.markers,
        ] {
            lane.enable_finite(device, format)
        }
    }
    pub fn finite_frame(
        &self,
        device: &wgpu::Device,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.finite_frame(device, tables, depth)
        }
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        scene: &Scene,
        target: Option<Target>,
    ) -> Result<(), &'static str> {
        self.set_at(device, layout, scene, target, [0.; 3])
    }
    pub fn set_at(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        scene: &Scene,
        target: Option<Target>,
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let mut strokes = Vec::new();
        let mut paths = Vec::new();
        let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target) = target.filter(|t| t.valid(scene)) {
            match target.part {
                Part::Face(key) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let face = row
                        .mesh
                        .parts
                        .faces
                        .iter()
                        .position(|&f| f == key)
                        .unwrap() as u32;
                    // Highlights own derived display data; changing selection changes presentation without mutating source geometry or adding a document history transaction.
                    let mut vertices = Vec::new();
                    let mut normals = Vec::new();
                    let mut indices = Vec::new();
                    for (tri, &source) in row
                        .mesh
                        .indices()
                        .chunks_exact(3)
                        .zip(&row.mesh.parts.triangles)
                    {
                        if source == face {
                            for &index in tri {
                                indices.push(vertices.len() as u32);
                                vertices.push(row.mesh.vertices()[index as usize]);
                                normals.push(row.mesh.normals()[index as usize]);
                            }
                        }
                    }
                    let display = crate::mesh::Mesh::new(vertices, indices)?
                        .with_normals(normals)?;
                    let mut object = row.clone();
                    object.mesh = Rc::new(display);
                    self.mesh = Some(GpuMesh::upload_at(
                        device, layout, &object, true, anchor,
                    ));
                }
                Part::BRepEdge(edge) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let points = &row
                        .cad
                        .as_ref()
                        .unwrap()
                        .edges
                        .iter()
                        .find(|border| border.edge == edge)
                        .unwrap()
                        .points;
                    for pair in points.windows(2) {
                        let ends = [pair[0], pair[1]].map(|p| {
                            crate::render_space::placed(&row.model, anchor)
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32()
                        });
                        strokes.push(crate::stroke::Stroke {
                            start: ends[0],
                            end: ends[1],
                            colour: yellow,
                            width: 3.,
                        });
                    }
                }
                Part::SurfaceEdge(edge) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let surface = row.surface.as_ref().unwrap();
                    let boundary = surface
                        .grid
                        .boundaries
                        .iter()
                        .find(|b| b.edge == edge)
                        .unwrap();
                    for ends in &boundary.segments {
                        let ends = ends.map(|index| {
                            let p = surface.grid.samples[index as usize].point;
                            crate::render_space::placed(&row.model, anchor)
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32()
                        });
                        strokes.push(crate::stroke::Stroke {
                            start: ends[0],
                            end: ends[1],
                            colour: yellow,
                            width: 3.,
                        });
                    }
                }
                Part::MeshEdge(a, b) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let index = row
                        .mesh
                        .parts
                        .edges
                        .iter()
                        .position(|&edge| edge == (a, b))
                        .unwrap();
                    let ends = row.mesh.parts.endpoints[index].map(|p| {
                        crate::render_space::placed(&row.model, anchor)
                            .transform_point(&session_rust::Point::new(
                                p[0], p[1], p[2],
                            ))
                            .to_f32()
                    });
                    strokes.push(crate::stroke::Stroke {
                        start: ends[0],
                        end: ends[1],
                        colour: yellow,
                        width: 3.,
                    });
                }
                Part::PathEdge(index) => {
                    let row = scene
                        .paths()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let points = row.points_at(anchor);
                    strokes.push(crate::stroke::Stroke {
                        start: points[index].to_f32(),
                        end: points[index + 1].to_f32(),
                        colour: yellow,
                        width: row.prepared.width + 2.,
                    });
                }
                Part::Curve => {
                    if let Some(row) = scene
                        .lines()
                        .iter()
                        .find(|r| r.id == target.owner)
                    {
                        strokes.push(crate::stroke::Stroke {
                            colour: yellow,
                            width: row.prepared.display.width + 2.,
                            ..row.stroke_at(anchor)
                        });
                    } else if let Some(row) = scene
                        .paths()
                        .iter()
                        .find(|r| r.id == target.owner)
                    {
                        paths.extend(row.segments_at(anchor).into_iter().map(
                            |mut segment| {
                                segment.stroke.colour = yellow;
                                segment.stroke.width += 2.;
                                segment
                            },
                        ));
                    }
                }
                Part::Control(_) | Part::SourceControl(_) => {
                    let marker = scene
                        .control_markers_at(anchor)
                        .into_iter()
                        .find(|m| m.parent == target.owner && m.part == target.part)
                        .unwrap();
                    markers.push(crate::marker::Marker {
                        colour: [1., 1., 0., 1.],
                        ..marker.display
                    });
                }
                Part::CloudPoint(id) => {
                    let p =
                        crate::cloud_pick::position_at(scene, target.owner, id, anchor)
                            .ok_or("Cloud point is missing")?;
                    markers.push(crate::marker::Marker {
                        center: p.to_f32(),
                        diameter: 11.,
                        colour: yellow,
                    });
                }
                Part::SheetEntity(_) | Part::Object => {}
            }
        }
        let flag = target
            .filter(|t| t.valid(scene))
            .map_or(0, |t| {
                if crate::clipping_rows::owners(scene).contains(&t.owner) {
                    4096
                } else {
                    0
                }
            });
        let count = strokes.len();
        self.strokes.set(device, strokes)?;
        self.strokes
            .set_flags(device, vec![flag; count])?;
        self.paths.set_chains(device, paths)?;
        self.markers.set_points(device, markers)?;
        Ok(())
    }

    pub fn objects(
        &mut self,
        device: &wgpu::Device,
        scene: &Scene,
        ids: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        self.objects_at(device, scene, ids, [0.; 3])
    }
    pub fn objects_at(
        &mut self,
        device: &wgpu::Device,
        scene: &Scene,
        ids: &[crate::scene::ObjectId],
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        self.mesh = None;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Highlight {
    target: Option<Target>,
    mesh: Option<GpuMesh>,
    strokes: Lane,
    paths: Lane,
    markers: Lane,
}

impl Highlight {
    #[cfg(test)]
    pub(crate) fn geometry_for_test(
        &self,
    ) -> Option<&Rc<crate::gpu_geometry::GpuGeometry>> {
        self.mesh.as_ref().map(|m| &m.geometry)
    }
    pub fn rebase_face(
        &mut self,
        queue: &wgpu::Queue,
        anchor: [f64; 3],
        target: Option<Target>,
    ) -> bool {
        if self.target != target
            || !target.is_some_and(|t| matches!(t.part, Part::Face(_)))
        {
            return false;
        }
        let Some(mesh) = self.mesh.as_mut() else {
            return false;
        };
        mesh.rebase(queue, anchor);
        true
    }

    pub fn set_opacity(&self, value: f32) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.set_opacity(value);
        }
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            target: None,
            mesh: None,
            strokes: Lane::new(device, format),
            paths: Lane::joined(device, format),
            markers: Lane::markers(device, format),
        }
    }

    pub fn enable_finite(
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) {
        for lane in [
            &mut self.strokes,
            &mut self.paths,
            &mut self.markers,
        ] {
            lane.enable_finite(device, format)
        }
    }
    pub fn finite_frame(
        &self,
        device: &wgpu::Device,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.finite_frame(device, tables, depth)
        }
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        scene: &Scene,
        target: Option<Target>,
    ) -> Result<(), &'static str> {
        self.set_at(device, layout, scene, target, [0.; 3])
    }
    pub fn set_at(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        scene: &Scene,
        target: Option<Target>,
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        self.target = target.filter(|t| t.valid(scene));
        self.mesh = None;
        let mut strokes = Vec::new();
        let mut paths = Vec::new();
        let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target) = self.target {
            match target.part {
                Part::Face(key) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let face = row
                        .mesh
                        .parts
                        .faces
                        .iter()
                        .position(|&f| f == key)
                        .unwrap() as u32;
                    // Highlights own derived display data; changing selection changes presentation without mutating source geometry or adding a document history transaction.
                    let mut vertices = Vec::new();
                    let mut normals = Vec::new();
                    let mut indices = Vec::new();
                    for (tri, &source) in row
                        .mesh
                        .indices()
                        .chunks_exact(3)
                        .zip(&row.mesh.parts.triangles)
                    {
                        if source == face {
                            for &index in tri {
                                indices.push(vertices.len() as u32);
                                vertices.push(row.mesh.vertices()[index as usize]);
                                normals.push(row.mesh.normals()[index as usize]);
                            }
                        }
                    }
                    let display = crate::mesh::Mesh::new(vertices, indices)?
                        .with_normals(normals)?;
                    let mut object = row.clone();
                    object.mesh = Rc::new(display);
                    self.mesh = Some(GpuMesh::upload_at(
                        device, layout, &object, true, anchor,
                    ));
                }
                Part::BRepEdge(edge) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let points = &row
                        .cad
                        .as_ref()
                        .unwrap()
                        .edges
                        .iter()
                        .find(|border| border.edge == edge)
                        .unwrap()
                        .points;
                    for pair in points.windows(2) {
                        let ends = [pair[0], pair[1]].map(|p| {
                            crate::render_space::placed(&row.model, anchor)
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32()
                        });
                        strokes.push(crate::stroke::Stroke {
                            start: ends[0],
                            end: ends[1],
                            colour: yellow,
                            width: 3.,
                        });
                    }
                }
                Part::SurfaceEdge(edge) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let surface = row.surface.as_ref().unwrap();
                    let boundary = surface
                        .grid
                        .boundaries
                        .iter()
                        .find(|b| b.edge == edge)
                        .unwrap();
                    for ends in &boundary.segments {
                        let ends = ends.map(|index| {
                            let p = surface.grid.samples[index as usize].point;
                            crate::render_space::placed(&row.model, anchor)
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32()
                        });
                        strokes.push(crate::stroke::Stroke {
                            start: ends[0],
                            end: ends[1],
                            colour: yellow,
                            width: 3.,
                        });
                    }
                }
                Part::MeshEdge(a, b) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let index = row
                        .mesh
                        .parts
                        .edges
                        .iter()
                        .position(|&edge| edge == (a, b))
                        .unwrap();
                    let ends = row.mesh.parts.endpoints[index].map(|p| {
                        crate::render_space::placed(&row.model, anchor)
                            .transform_point(&session_rust::Point::new(
                                p[0], p[1], p[2],
                            ))
                            .to_f32()
                    });
                    strokes.push(crate::stroke::Stroke {
                        start: ends[0],
                        end: ends[1],
                        colour: yellow,
                        width: 3.,
                    });
                }
                Part::PathEdge(index) => {
                    let row = scene
                        .paths()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let points = row.points_at(anchor);
                    strokes.push(crate::stroke::Stroke {
                        start: points[index].to_f32(),
                        end: points[index + 1].to_f32(),
                        colour: yellow,
                        width: row.prepared.width + 2.,
                    });
                }
                Part::Curve => {
                    if let Some(row) = scene
                        .lines()
                        .iter()
                        .find(|r| r.id == target.owner)
                    {
                        strokes.push(crate::stroke::Stroke {
                            colour: yellow,
                            width: row.prepared.display.width + 2.,
                            ..row.stroke_at(anchor)
                        });
                    } else if let Some(row) = scene
                        .paths()
                        .iter()
                        .find(|r| r.id == target.owner)
                    {
                        paths.extend(row.segments_at(anchor).into_iter().map(
                            |mut segment| {
                                segment.stroke.colour = yellow;
                                segment.stroke.width += 2.;
                                segment
                            },
                        ));
                    }
                }
                Part::Control(_) | Part::SourceControl(_) => {
                    let marker = scene
                        .control_markers_at(anchor)
                        .into_iter()
                        .find(|m| m.parent == target.owner && m.part == target.part)
                        .unwrap();
                    markers.push(crate::marker::Marker {
                        colour: [1., 1., 0., 1.],
                        ..marker.display
                    });
                }
                Part::CloudPoint(id) => {
                    let p =
                        crate::cloud_pick::position_at(scene, target.owner, id, anchor)
                            .ok_or("Cloud point is missing")?;
                    markers.push(crate::marker::Marker {
                        center: p.to_f32(),
                        diameter: 11.,
                        colour: yellow,
                    });
                }
                Part::SheetEntity(_) | Part::Object => {}
            }
        }
        let flag = self.target.map_or(0, |t| {
            if crate::clipping_rows::owners(scene).contains(&t.owner) {
                4096
            } else {
                0
            }
        });
        let count = strokes.len();
        self.strokes.set(device, strokes)?;
        self.strokes
            .set_flags(device, vec![flag; count])?;
        self.paths.set_chains(device, paths)?;
        self.markers.set_points(device, markers)?;
        Ok(())
    }

    pub fn objects(
        &mut self,
        device: &wgpu::Device,
        scene: &Scene,
        ids: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        self.objects_at(device, scene, ids, [0.; 3])
    }
    pub fn objects_at(
        &mut self,
        device: &wgpu::Device,
        scene: &Scene,
        ids: &[crate::scene::ObjectId],
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        self.target = None;
        self.mesh = None;
```

### 3. `src/global_outline_tests.rs`

Reuse the shared source owner in the existing exact original-highlight fixture; retain its pixel, address and source assertions. Supply exact original face/ink pixels, primitive addresses, retained derived GPU geometry and original ownership through a representation-only rebase.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    assert!(gpu.take_pick(local).is_none());
    assert_eq!(&gpu.pick_usage()[..2], &[0, 0]);
    assert_eq!(gpu.usage(), [0; 4]);
```

Replace it with:

```rust
    assert!(gpu.take_pick(local).is_none());
    assert_eq!(&gpu.pick_usage()[..2], &[0, 0]);
    assert_eq!(gpu.usage(), [0; 4]);
}

#[test]
fn actual_origin_refresh_reuses_selected_face_geometry_and_original_ink() {
    use crate::pick_sources::{Part, Target};
    let mut scene = Scene::empty();
    let owner = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.25, -0.25, -0.5],
                    [0.25, -0.25, -0.5],
                    [0., 0.25, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    scene
        .translate(owner, [2000., 0., 0.])
        .unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let key = *scene.objects()[0]
        .geometry()
        .unwrap()
        .face
        .keys()
        .next()
        .unwrap();
    let target = Some(Target {
        owner,
        part: Part::Face(key),
    });
    let mut gpu = renderer(&scene);
    let mut old = matrix();
    old[12] = -2000.;
    gpu.set_highlight(&scene, target).unwrap();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, old);
    let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
    let geometry = gpu
        .highlight
        .geometry_for_test()
        .unwrap()
        .clone();
    let uploads = gpu.stats()[0];
    gpu.space
        .update([2000., 0., 0.], 3., 0.)
        .unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, [2000., 0., 0.]);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    gpu.set_source_ink(&scene);
    assert!(
        gpu.highlight
            .rebase_face(&gpu.queue, gpu.anchor(), target)
    );
    assert!(Rc::ptr_eq(
        gpu.highlight.geometry_for_test().unwrap(),
        &geometry
    ));
    let actual = crate::cap_gpu_tests::pixels_at(&gpu, matrix());
    assert!(
        actual == baseline,
        "Rebased original face and ink pixels must match exactly"
    );
    assert_eq!(
        primitive_pixels(&gpu, &gpu.primitive.texture),
        addresses
    );
    assert!(
        gpu.highlight
            .rebase_face(&gpu.queue, gpu.anchor(), target)
    );
    assert!(Rc::ptr_eq(
        gpu.highlight.geometry_for_test().unwrap(),
        &geometry
    ));
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    drop(geometry);
    gpu.set_highlight(&scene, None).unwrap();
    assert!(gpu.highlight.geometry_for_test().is_none());
    assert!(
        !gpu.highlight
            .rebase_face(&gpu.queue, gpu.anchor(), target)
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
```

**Locate this block:**

```rust
    );
    gpu.sources = crate::pick_sources::Sources::anchored(&far, anchor).unwrap();
    gpu.mesh_ink
        .set(
            &gpu.device,
            gpu.sources.mesh_ink.iter().copied(),
        )
        .unwrap();
    gpu.strokes
        .set(
            &gpu.device,
            scene.lines().iter().map(|r| r.stroke()),
        )
        .unwrap();
    gpu.paths
        .set_chains(
            &gpu.device,
            scene
                .paths()
                .iter()
                .flat_map(|r| r.segments()),
        )
        .unwrap();
    gpu.markers
        .set_points(
            &gpu.device,
            scene
                .points()
                .iter()
                .map(|r| r.marker())
                .chain(
                    scene
                        .control_markers()
                        .into_iter()
                        .map(|r| r.display),
                ),
        )
        .unwrap();
    gpu.strokes
        .set_ids(
            &gpu.device,
            gpu.sources.line_ids.iter().copied(),
        )
        .unwrap();
    gpu.paths
        .set_ids(
            &gpu.device,
            gpu.sources.path_ids.iter().copied(),
        )
        .unwrap();
    gpu.markers
        .set_ids(
            &gpu.device,
            gpu.sources.marker_ids.iter().copied(),
        )
        .unwrap();
    gpu.set_clouds(&far, &[]).unwrap();
    camera.target = anchor;
    gpu.cloud_camera(&camera);
```

Replace it with:

```rust
    );
    gpu.set_source_ink(&far);
    gpu.set_clouds(&far, &[]).unwrap();
    camera.target = anchor;
    gpu.cloud_camera(&camera);
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 21.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 21.2
npm --prefix ../session_tests run course -- save 21.2
```

Continue with [21.3 · Browser ownership of frame matrices and wakeups](96zzzzzq-prepare-one-shared-render-origin.md).
