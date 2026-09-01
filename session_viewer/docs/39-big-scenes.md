# 39 Big scenes — the far plane, the rebase, and the vertex dead end

> Direct-path chain (36–41); every step below is replay-verified against a clean
> end-of-35 checkout.

## Goal

Two camera-side fixes that a 200-metre, 210k-object scene forces on any viewer: the far
plane must follow streamed files, and re-anchoring must stop costing a frame. (The
measurements that justified the compute lane itself live at the top of lesson
[36](36-cloud-tables.md).)

## Step 1 — the far plane must grow as files stream in

`Camera::fit` sets `scene_extent`, the far-plane floor — and the initial fit runs at
`Msg::Ready`, when only the FIRST manifest file is in. Every later scan then sits past
the far plane and gets **sliced by it as the view turns** ("a box crops the clouds when I
rotate" — the far plane is that box's face). The fix never moves the view, it only widens
the floor.

**Find** in `src/camera.rs` (the end of `fit`):

```rust
        self.scene_extent = extent; // far-plane floor, see view_proj_anchored
        self.update_position();
    }
```

**Add below it:**

```rust
    /// Grow the far-plane floor to cover a scene that streamed in AFTER the last fit,
    /// without touching the view. Same definition as fit's: the farthest scene corner
    /// from the target, in metres.
    pub fn grow_extent(&mut self, min: [f32; 3], max: [f32; 3]) {
        let s = self.unit.to_meters();
        let mut extent: f64 = 0.0;
        for c in 0..8u32 {
            let p = [
                (if c & 1 == 0 { min[0] } else { max[0] }) as f64 * s - self.target[0],
                (if c & 2 == 0 { min[1] } else { max[1] }) as f64 * s - self.target[1],
                (if c & 4 == 0 { min[2] } else { max[2] }) as f64 * s - self.target[2],
            ];
            extent = extent.max((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt());
        }
        if extent.is_finite() && extent > self.scene_extent {
            self.scene_extent = extent;
        }
    }
```

**Find** in `src/lib.rs` (inside the `Msg::File` arm):

```rust
                state.scene.upload_to(&mut state.gpu);
```

**Add below it:**

```rust
                state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);
```

## Step 2 — cache the f64→f32 cast

Wheel-zoom moves the camera target, and every re-anchor rebuilt ALL instance rows with a
full f64→f32 cast — at 210,892 objects that is 30+ ms per wheel tick, exactly the motion
jank the constant-quality rule forbids. Rebase only re-patches 3 translation slots, so
the other 13 floats can be cast ONCE at `set_scene`. All in `src/engine/gpu/mod.rs`.

**Find** in the `Gpu` struct fields:

```rust
    objects_base: Vec<(Mat4, [f32; 4], u32)>, // TRUE world model+color; isntance[] is rebased from this
```

**Add below it:**

```rust
    base_f32: Vec<[f32; 16]>, // model.to_f32() cached once - rebase only re-patches 3 slots
    bounded_rows: Vec<u32>,   // rows with Some(world AABB) - the only ones the inside test walks
```

**Find** in `Gpu::new`:

```rust
        let objects_base: Vec<(Mat4, [f32; 4], u32)> = Vec::new();
```

**Add below it:**

```rust
        let base_f32: Vec<[f32; 16]> = Vec::new();
        let bounded_rows: Vec<u32> = Vec::new();
```

**Find** in the struct literal at the end of `Gpu::new`:

```rust
            objects_base,
```

**Add below it:**

```rust
            base_f32,
            bounded_rows,
```

**Find** in `set_scene` (`base` is the row count BEFORE this file's rows land — the object
table grows by appending, so the cache has to append with it, not be rebuilt):

```rust
        self.objects_base.extend_from_slice(&up.objects[base..]);
```

**Add below it:**

```rust
        // Rebase re-patches only translations, so the 13 other floats can be cast ONCE here
        // instead of per re-anchor: at 210k objects that turns a 20+ ms CPU loop into a copy.
        self.base_f32.extend(up.objects[base..].iter().map(|(m, _, _)| mat_to_f32(m)));
```

**Find** in `set_scene`:

```rust
        self.inside.resize(self.objects_base.len(), false);
```

**Add below it** (rebuilt whole rather than appended: it is one u32 per BOUNDED row, a
fraction of the table, and rebuilding it keeps it honest when a `rebuild` rewinds everything):

```rust
        self.bounded_rows = self.object_bounds_world.iter().enumerate()
            .filter_map(|(i, b)| b.map(|_| i as u32)).collect();
```

**Find** in `rebuild_instances` (the per-row `color` write goes too — colours never
change under a rebase):

```rust
        for (i, (model, color, _)) in self.objects_base.iter().enumerate() {
            let mut m = mat_to_f32(model);
            m[12] = (model[12] - origin[0]) as f32;
            m[13] = (model[13] - origin[1]) as f32;
            m[14] = (model[14] - origin[2]) as f32;
            self.instances[i].model = m;
            self.instances[i].color = *color;
        }
```

**Replace with:**

```rust
        for (i, (model, _, _)) in self.objects_base.iter().enumerate() {
            let mut m = self.base_f32[i]; // rotation/scale cast once at set_scene
            m[12] = (model[12] - origin[0]) as f32;
            m[13] = (model[13] - origin[1]) as f32;
            m[14] = (model[14] - origin[2]) as f32;
            self.instances[i].model = m;
        }
```

## Step 3 — throttle the rebase

**Find** in the `Gpu` struct fields (from lesson 36's step 3g):

```rust
    pub cloud_size: f32, // global SCALE on per-cloud sizes, [ and ] keys
```

**Add below it:**

```rust
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
```

**Find** in the struct literal:

```rust
            cloud_size: std::env::var("VIEWER_CLOUD_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
```

**Add below it:**

```rust
            last_rebase_ms: 0.0,
```

**Find** in `rebase_anchor`:

```rust
        if need {
            self.rebuild_instances(origin);
        }
```

**Replace with:**

```rust
        // Throttled: during a wheel-zoom gesture the target moves every tick, and an
        // every-frame rebuild (~25 ms at 210k rows) is exactly the motion jank the rule
        // forbids. Between rebuilds the old anchor stays valid - it is just farther from
        // the eye than the threshold likes, which costs f32 precision only PAST the
        // threshold distance, never a wrong image.
        let now = crate::engine::performance::now_ms();
        if need && (now - self.last_rebase_ms > 200.0 || self.last_origin.is_none()) {
            self.rebuild_instances(origin);
            self.last_rebase_ms = now;
        }
```

## Step 4 — walk only the bounded rows

A 210k-object scene has a handful of rows with world AABBs; the per-frame inside test
walked them all. **Find** in `update_inside_flags`:

```rust
        if self.object_bounds_world.iter().all(Option::is_none) {
            return;
        }
```

**Replace with:**

```rust
        if self.bounded_rows.is_empty() {
            return;
        }
```

**Find** (the loop head, two lines):

```rust
        for (i, b) in self.object_bounds_world.iter().enumerate() {
            let inside = in_scene && b.is_some_and(|(lo, hi)| (0..3).all(|k| ew[k] >= lo[k] && ew[k] <= hi[k]));
```

**Replace with:**

```rust
        for &row in &self.bounded_rows {
            let i = row as usize;
            let b = &self.object_bounds_world[i];
            let inside = in_scene && b.is_some_and(|(lo, hi)| (0..3).all(|k| ew[k] >= lo[k] && ew[k] <= hi[k]));
```

(The rest of the loop body is unchanged.)

## Expected state

- `cargo check --target wasm32-unknown-unknown --lib`: clean.
- The lion render is still byte-identical: `non-background pixels: 189148 (19.7%)`.
- In the browser with the full mix scene: scans streamed in AFTER the first fit are no
  longer sliced by the far plane as the view turns, and wheel-zoom logs at most ~5
  rebases a second instead of one per tick.

## Next

Lesson [40](40-compute-splatting.md) — **The production splatter: tint, the static skip, and the trap.** The dots stop being a demo - and one of the optimisations is a trap worth meeting.
