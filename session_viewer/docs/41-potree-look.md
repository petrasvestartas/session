# 41 The Potree look — EDL and attenuated splats

> Direct-path chain (36-44); every step below is replay-verified against a clean
> end-of-35 checkout, applied on top of lesson 40's production lane.

## Goal

Close the visual gap to Potree. Two techniques here, in the order of how much they
matter: **Eye-Dome Lighting** (depth-based shading — the "3D pop") and **attenuated point
sizes** (world-sized splats that close into gap-free surfaces near the camera). Both live
inside lesson [40](40-compute-splatting.md)'s splat lane; neither touches a vertex. The
third — per-point **normals** with a lambert term — is lesson [42](42-cloud-normals.md),
together with the two clouds that carry them.

## Step 1 — Eye-Dome Lighting: shading without normals

A scan has colours but no normals, so it renders flat. EDL (Boucheny; CloudCompare;
Potree) fakes shading from the depth buffer alone: darken a pixel by how much CLOSER its
neighbours are. Depth discontinuities become dark rims; creases and silhouettes pop. The
splat lane already OWNS a per-pixel depth buffer, so the resolve triangle gets it for the
price of four taps.

**1a.** In `src/shaders/splat_resolve.wgsl`, **find** (in `CloudUniform`):

```wgsl
    _pad: f32,
```

**Replace with:**

```wgsl
    _pad: f32, // EDL strength; 0 = off
```

**1b.** Same file, **find** (in `fs_main`):

```wgsl
    var o: FsOut;
    o.color = vec4<f32>(unpack4x8unorm(scolor[idx]).rgb, 1.0);
```

**Replace with:**

```wgsl
    var o: FsOut;
    var rgb = unpack4x8unorm(scolor[idx]).rgb;

    // EYE-DOME LIGHTING (CloudCompare/Potree formula): darken a pixel by how much CLOSER
    // its neighbours are - depth discontinuities become dark rims, and a normal-less LiDAR
    // cloud suddenly reads as a 3D surface. All from the splat depth buffer, four taps.
    // Our depth is reverse-Z ndc bits; -log2(z) grows with distance like Potree's log depth.
    let strength = cloud._pad;
    if (strength > 0.0) {
        let w = i32(cloud.vp_w);
        let h = i32(cloud.vp_h);
        let me = -log2(max(bitcast<f32>(d), 1.0e-7));
        var sum = 0.0;
        for (var k = 0; k < 4; k++) {
            var q = vec2<i32>(in.pos.xy);
            if (k == 0) { q.x -= 1; } else if (k == 1) { q.x += 1; }
            else if (k == 2) { q.y -= 1; } else { q.y += 1; }
            if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) { continue; }
            let nd = sdepth[u32(q.y) * u32(w) + u32(q.x)];
            if (nd == 0u) { continue; } // empty neighbour: no opinion
            sum += max(0.0, me - (-log2(max(bitcast<f32>(nd), 1.0e-7))));
        }
        // floor at 0.25: an edge darkens, it never goes pure black - sparse dots
        // otherwise grow cartoon outlines instead of shading.
        let shade = max(exp(-sum * 75.0 * strength), 0.25);
        rgb *= shade;
    }

    o.color = vec4<f32>(rgb, 1.0);
```

The shade FLOOR is ours, not Potree's: at Potree's densities a hard black rim reads as
shading, at a sparse scan it reads as a cartoon outline.

**1c.** The strength rides the cloud uniform's spare word. In `src/engine/gpu/mod.rs`,
**find** (from lesson 39):

```rust
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
```

**Add below it:**

```rust
    pub edl_strength: f32, // Eye-Dome Lighting strength; 0 = off (VIEWER_EDL)
```

**Find** in the struct literal:

```rust
            last_rebase_ms: 0.0,
```

**Add below it:**

```rust
            edl_strength: std::env::var("VIEWER_EDL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.25),
```

**Find** in `write_frame_uniforms` (the cloud-uniform write from lesson 36 — NOT the
`_pad: 0.0,` in `Gpu::new`'s cloud-buffer init, which stays):

```rust
            vp_h: self.config.height as f32,
            _pad: 0.0,
```

**Replace with:**

```rust
            vp_h: self.config.height as f32,
            _pad: self.edl_strength, // EDL strength, read by the splat resolve
```

## Step 2 — attenuated sizes: a splat covers its footprint

A fixed-px dot is a lie at both ends: gappy up close, blobby far away. Potree sizes
points by `spacing × projFactor` — a point covers its own world-space footprint. The
spacing was measured in lesson [36](36-cloud-tables.md) (`cloud_spacing`, the 4th slot of
`cloud_draws`); this step folds the projection into the record so the shader's whole job
is one divide.

**2a.** In `src/shaders/splat.wgsl`, **find**:

```wgsl
    s.r = bitcast<f32>(table[base + 23u]);
```

**Replace with:**

```wgsl
    // ATTENUATED radius: the record's k folds the cloud's world-space point footprint and
    // the projection, so the screen radius is one divide - big near, dust far, gap-free in
    // between (Potree's attenuated mode). The floor (tint.a) keeps the manifest px at range.
    let r_min = rec_f(base, 19u);
    s.r = clamp(bitcast<f32>(table[base + 23u]) * cloud.vp_h / clip.w, r_min, 8.0);
```

**2b.** The ortho half-height must reach the record builder. In `src/engine/gpu/mod.rs`,
**find** (the field added in step 1c):

```rust
    pub edl_strength: f32, // Eye-Dome Lighting strength; 0 = off (VIEWER_EDL)
```

**Add below it:**

```rust
    last_ortho_h: f32, // ortho half-height this frame (0 = perspective), for the splat k
```

**Find** in the struct literal (from step 1c):

```rust
            edl_strength: std::env::var("VIEWER_EDL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.25),
```

**Add below it:**

```rust
            last_ortho_h: 0.0,
```

**Find** in `write_frame_uniforms` (from lesson 36's step 8g):

```rust
        self.mvp_f32 = view_proj.to_f32();
```

**Add below it:**

```rust
        self.last_ortho_h = Self::ortho_half_height(view_proj);
```

**2c.** The record builder. In `encode_frame`'s compute prelude, **find**:

```rust
            for &(first, count, inst, _spacing) in &self.cloud_draws {
```

**Replace with:**

```rust
            // Attenuated (world-sized) dots, Potree-style: the record carries k such that
            // the shader's radius is clamp(k * vp_h / clip.w, ...) px - a point covers its
            // own world-space footprint, so near surfaces close up gap-free and far points
            // shrink. The manifest px is a size FACTOR on the measured spacing.
            let ortho_h = self.last_ortho_h as f64;
            for &(first, count, inst, spacing) in &self.cloud_draws {
```

**Find** (lesson 40's tint + meta pushes):

```rust
                    let tint = [row.color[0], row.color[1], row.color[2], 1.0f32];
                    recs.extend_from_slice(bytemuck::cast_slice(&tint));
                    recs.extend_from_slice(bytemuck::cast_slice(&[first, count, cum, (px * 0.5).to_bits()]));
```

**Replace with:**

```rust
                    // tint.a smuggles the MINIMUM radius (the manifest px, halved): without a
                    // floor, attenuation turns distant clouds to dust - Potree avoids that with
                    // octree LOD (far nodes have bigger spacing); we keep the user's px instead.
                    let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];
                    recs.extend_from_slice(bytemuck::cast_slice(&tint));
                    // world radius = spacing x (px/6): manifest 6 ~ a full spacing of radius,
                    // 3 ~ half. k folds the projection so the shader only divides by clip.w:
                    //   perspective: r_px = world_r * cot(fov/2) * (vp_h/2) / w
                    //   ortho:       r_px = world_r * vp_h / (2*ortho_h), and w = 1
                    // spacing was measured in the cloud's LOCAL units; the model may scale -
                    // col0's length is that scale, so the footprint reaches world units first.
                    let mscale = ((row.model[0] as f64).powi(2) + (row.model[1] as f64).powi(2) + (row.model[2] as f64).powi(2)).sqrt();
                    let world_r = (spacing as f64).max(1.0e-9) * mscale * 0.001 * (px as f64) / 6.0; // metres
                    let k = if ortho_h > 0.0 { world_r / (2.0 * ortho_h) }
                            else { world_r * 1.7320508 * 0.5 }; // cot(30 deg) / 2
                    recs.extend_from_slice(bytemuck::cast_slice(&[first, count, cum, (k as f32).to_bits()]));
```

The floor is the part Potree does differently: it lets far points shrink to dust and
relies on octree LOD to keep the picture full. We have no octree yet, so the manifest px
doubles as the far-size floor — the per-cloud size control from lesson 36 survives, and
`[` `]` still scale everything.

## Expected state

- Both shaders `naga`-clean; wasm check clean.
- The lion reads as a SURFACE, not a fog of dots: EDL puts dark rims on every depth
  discontinuity, and attenuation closes the gaps as you zoom in — a point now covers its
  own world-space footprint instead of a fixed 3 px.
- `VIEWER_EDL=0` turns the shading off for an A/B; the geometry must not move.
- The pixel gate for this pair lands at the end of lesson [42](42-cloud-normals.md)
  (`325369`), once the lambert term is in and the clouds that carry normals exist.
