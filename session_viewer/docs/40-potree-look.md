# 40 The Potree look — EDL, attenuated splats, and normals

> Direct-path chain (36-41); every step below is replay-verified against a clean
> end-of-35 checkout, applied on top of lesson 39's production lane.

## Goal

Close the visual gap to Potree. Three techniques, in the order of how much they matter:
**Eye-Dome Lighting** (depth-based shading — the "3D pop"), **attenuated point sizes**
(world-sized splats that close into gap-free surfaces near the camera), and **per-point
normals** with a lambert term for clouds that have them. All three live inside lesson
[39](39-compute-splatting.md)'s splat lane; none of them touches a vertex.

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
**find** (from lesson 38):

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

**Find** (lesson 39's tint + meta pushes):

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

## Step 3 — normals: lambert for clouds that have them

The kernel's `PointCloud` has carried a `normals` array since the proto was written, and
lesson 36 already packs it oct16 into `point_nrm_buffer` — unused until now. The record
grows the model's rotation columns so the normal reaches world space even under a
rotated placement.

**3a.** In `src/shaders/splat.wgsl`, **find** (lesson 39's record width):

```wgsl
const REC_WORDS: u32 = 24u;
```

**Replace with:**

```wgsl
// The record table is read as RAW WORDS - 4-word header {n, total, 0, 0}, then 36 words per
// record: 16 matrix (mvp x model, column-major), 4 tint (.a = minimum radius px),
// {first, count, cum, k-bits}, then 12 words of the model's rotation columns for normals.
// Raw indexing sidesteps every struct-layout question between Rust packing and WGSL rules.
const REC_WORDS: u32 = 36u;
```

**3b.** Same file, **find**:

```wgsl
@group(1) @binding(3) var<storage, read_write> scolor: array<u32>;
```

**Add below it:**

```wgsl
@group(1) @binding(4) var<storage, read> normals: array<u32>; // oct16; MAX = point has none
```

**3c.** Same file, **find** (lesson 39's tint pack in `project`):

```wgsl
    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0);
    s.color = pack4x8unorm(unpack4x8unorm(colors[i]) * tint);
```

**Replace with:**

```wgsl
    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0); // .a is r_min
    var rgba = unpack4x8unorm(colors[i]) * tint;
    // LAMBERT, when the point HAS a normal (scans do not; sampled/imported clouds do). The
    // record's trailing words carry the model's rotation columns, so the oct16 normal reaches
    // world space; abs() because a scanned normal's orientation is a coin toss.
    let packed_n = normals[i];
    if (packed_n != 0xffffffffu) {
        let rot = mat3x3<f32>(
            vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
            vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
            vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
        );
        let nw = normalize(rot * oct16_decode(packed_n));
        let light = normalize(vec3<f32>(0.4, 0.4, 0.8)); // fixed key light
        let lambert = 0.25 + 0.75 * abs(dot(nw, light));
        rgba = vec4<f32>(rgba.rgb * lambert, rgba.a);
    }
    s.color = pack4x8unorm(rgba);
```

**3d.** Same file, at the very end (after `cs_color`'s closing brace), **add:**

```wgsl

// Octahedral decode: undo the fold, then normalize - the mirror of scene.rs oct16().
fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(
        f32(i32(p << 24u) >> 24u) / 127.0,
        f32(i32(p << 16u) >> 24u) / 127.0,
    );
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0){
        let sgn = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * sgn.x, (1.0 - abs(n.x)) * sgn.y, n.z);
    }
    return normalize(n);
}
```

**3e.** The fifth binding, CPU side. In `src/engine/gpu/mod.rs`, **find** (the group-1
layout in `Gpu::new`):

```wgsl
                Self::splat_entry(3, wgpu::BufferBindingType::Storage { read_only: false }),
```

**Add below it:**

```rust
                Self::splat_entry(4, wgpu::BufferBindingType::Storage { read_only: true }),
```

**Find** in `mk_splat_group1`'s parameter list:

```rust
        col: &wgpu::Buffer,
```

**Add below it** (the normals ride next to the colours; the BINDING number below is what the
shader reads, so the parameter's position in the list is free):

```rust
        nrm: &wgpu::Buffer,
```

**Find** (inside `mk_splat_group1`):

```rust
                wgpu::BindGroupEntry{binding: 3, resource: scolor.as_entire_binding()},
```

**Add below it:**

```rust
                wgpu::BindGroupEntry{binding: 4, resource: nrm.as_entire_binding()},
```

**Find** (call site in `Gpu::new`):

```rust
            &point_col_buffer,
```

**Add below it:**

```rust
            &point_nrm_buffer,
```

**Find** (call site in `rebuild_splat_groups` — this one IS a single line):

```rust
        self.splat_group1 = Self::mk_splat_group1(&self.device, &self.splat_group1_layout, &self.point_buffer, &self.point_col_buffer, &self.splat_depth_buf, &self.splat_color_buf);
```

**Replace with:**

```rust
        self.splat_group1 = Self::mk_splat_group1(&self.device, &self.splat_group1_layout, &self.point_buffer, &self.point_col_buffer, &self.point_nrm_buffer, &self.splat_depth_buf, &self.splat_color_buf);
```

**3f.** The rotation columns ride the record. In `encode_frame`, **find** (the k push
from step 2c):

```rust
                    recs.extend_from_slice(bytemuck::cast_slice(&[first, count, cum, (k as f32).to_bits()]));
```

**Add below it:**

```rust
                    // the MODEL rotation columns (translation-free), so a cloud with
                    // normals can rotate them into world space for the lambert term
                    let b = &row.model;
                    recs.extend_from_slice(bytemuck::cast_slice(&[
                        b[0], b[1], b[2], 0.0f32,
                        b[4], b[5], b[6], 0.0,
                        b[8], b[9], b[10], 0.0,
                    ]));
```

## Step 4 — demo data: clouds that HAVE normals

Two generator tools; both write `.pb`s that already ship in `assets/pb`, so typing them
is optional if you only want to render.

**The real one: Potree's own Takanawa lion** — 342k points with colours AND
`NORMAL_OCT16` normals, in the Potree 1.x octree format. The format is pleasantly
decodable: every point lives in exactly ONE node file, so the union of all `r*.bin`
files IS the cloud; a node's box comes from the root cube by walking its name's digits
(bit 4 = +x half, 2 = +y, 1 = +z); positions are `u32 × scale` relative to that box's
min; normals are two UNSIGNED bytes, octahedral. 18 bytes a record.

**Create `examples/potree_import.rs`** (cargo auto-discovers it):

```rust
// Import a Potree 1.x octree (POSITION_CARTESIAN + COLOR_PACKED + NORMAL_OCT16) into one
// kernel PointCloud .pb. Every point lives in exactly ONE node file, so the union of all
// r*.bin files IS the cloud; each node's positions are u32 * scale relative to the NODE's
// bounding box min, and a node's box comes from the root cube by walking the digits of its
// name (bit 4 = +x half, bit 2 = +y half, bit 1 = +z half).
//
// cargo run --example potree_import --target x86_64-unknown-linux-gnu --release -- \
//     assets/lion_src assets/pb/lion.pb 1000

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let dir = a.first().cloned().unwrap_or_else(|| "assets/lion_src".into());
    let out = a.get(1).cloned().unwrap_or_else(|| "assets/pb/lion.pb".into());
    let unit: f64 = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(1000.0); // metres -> mm

    let cloud: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(format!("{dir}/cloud.js")).expect("cloud.js")).expect("json");
    let bb = &cloud["boundingBox"];
    let root_min = [bb["lx"].as_f64().unwrap(), bb["ly"].as_f64().unwrap(), bb["lz"].as_f64().unwrap()];
    let root_size = bb["ux"].as_f64().unwrap() - root_min[0]; // potree root is a cube
    let scale = cloud["scale"].as_f64().unwrap();

    let mut coords = Vec::new();
    let mut colors = Vec::new();
    let mut normals = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "bin"))
        .collect();
    files.sort();
    for path in &files {
        let name = path.file_stem().unwrap().to_str().unwrap(); // "r", "r07", ...
        let (mut min, mut size) = (root_min, root_size);
        for d in name[1..].chars() {
            let i = d.to_digit(10).unwrap();
            size *= 0.5;
            if i & 0b100 != 0 { min[0] += size; }
            if i & 0b010 != 0 { min[1] += size; }
            if i & 0b001 != 0 { min[2] += size; }
        }
        let data = std::fs::read(path).unwrap();
        for rec in data.chunks_exact(18) {
            for k in 0..3 {
                let v = u32::from_le_bytes(rec[k * 4..k * 4 + 4].try_into().unwrap()) as f64;
                coords.push((v * scale + min[k]) * unit);
            }
            colors.extend_from_slice(&[rec[12] as i32, rec[13] as i32, rec[14] as i32, 255]);
            // potree NORMAL_OCT16: two UNSIGNED bytes mapped to [-1,1], octahedral unfold
            let u = rec[16] as f64 / 255.0 * 2.0 - 1.0;
            let v = rec[17] as f64 / 255.0 * 2.0 - 1.0;
            let z = 1.0 - u.abs() - v.abs();
            let (x, y) = if z < 0.0 {
                let s = |t: f64| if t < 0.0 { -1.0 } else { 1.0 };
                ((1.0 - v.abs()) * s(u), (1.0 - u.abs()) * s(v))
            } else { (u, v) };
            let l = (x * x + y * y + z * z).sqrt().max(1e-9);
            normals.extend_from_slice(&[x / l, y / l, z / l]);
        }
    }
    let n = coords.len() / 3;
    let mut pc = session_rust::PointCloud::from_coords(coords, colors, normals);
    pc.point_size = 3.0;
    pc.name = "lion_takanawa".to_string();
    let mut s = session_rust::Session::new("lion_takanawa");
    s.add_pointcloud(pc, None);
    s.pb_dump(&out);
    println!("wrote {out}: {n} points from {} nodes", files.len());
}
```

```
curl the r*.bin node files + cloud.js into assets/lion_src/, then
cargo run --example potree_import --target x86_64-unknown-linux-gnu --release -- \
    assets/lion_src assets/pb/lion.pb 1000
```

**The synthetic one** — a mesh sampled into a cloud with EXACT ground-truth normals, to
check shading against (`assets/scenes/bunny_cloud.toml` keeps it one keypress away).

**Create `examples/mk_bunny_cloud.rs`:**

```rust
// Sample a mesh's surface into a point cloud WITH normals - the demo data for the splat
// lane's lambert shading (scans carry no normals; a sampled surface does). Area-weighted
// triangle sampling, barycentric position + interpolated vertex normal per point.
//
// cargo run --example mk_bunny_cloud --target x86_64-unknown-linux-gnu --release -- \
//     assets/pb/mesh_bunny_grey.pb assets/pb/bunny_cloud.pb 1500000

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let src = a.first().cloned().unwrap_or_else(|| "assets/pb/mesh_bunny_grey.pb".into());
    let out = a.get(1).cloned().unwrap_or_else(|| "assets/pb/bunny_cloud.pb".into());
    let count: usize = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(1_500_000);

    let bytes = std::fs::read(&src).expect("read src pb");
    let session = session_rust::Session::pb_loads(&bytes).expect("parse src pb");
    let mesh = session.order().into_iter().find_map(|g| match session.lookup.get(&g) {
        Some(session_rust::Geometry::Mesh(m)) => Some(m.clone()),
        _ => None,
    }).expect("no mesh in source pb");

    let rm = mesh.to_render();
    // cumulative triangle areas for area-weighted sampling
    let tri = |i: usize| {
        let f = [rm.indices[i * 3] as usize, rm.indices[i * 3 + 1] as usize, rm.indices[i * 3 + 2] as usize];
        f.map(|k| rm.vertices[k].clone())
    };
    let ntri = rm.indices.len() / 3;
    let mut cum = Vec::with_capacity(ntri);
    let mut total = 0.0f64;
    for i in 0..ntri {
        let [a, b, c] = tri(i);
        let u = [b.position[0] as f64 - a.position[0] as f64, b.position[1] as f64 - a.position[1] as f64, b.position[2] as f64 - a.position[2] as f64];
        let v = [c.position[0] as f64 - a.position[0] as f64, c.position[1] as f64 - a.position[1] as f64, c.position[2] as f64 - a.position[2] as f64];
        let x = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        total += 0.5 * (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
        cum.push(total);
    }

    // deterministic LCG - no rand dependency, same cloud every run
    let mut state = 0x2545F491_4F6CDD1Du64;
    let mut rnd = move || { state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (state >> 33) as f64 / (1u64 << 31) as f64 };

    let mut coords = Vec::with_capacity(count * 3);
    let mut colors = Vec::with_capacity(count * 4);
    let mut normals = Vec::with_capacity(count * 3);
    for _ in 0..count {
        let r = rnd() * total;
        let t = cum.partition_point(|&c| c < r).min(ntri - 1);
        let [va, vb, vc] = tri(t);
        // uniform barycentric via the sqrt trick
        let (mut b1, mut b2) = (rnd(), rnd());
        if b1 + b2 > 1.0 { b1 = 1.0 - b1; b2 = 1.0 - b2; }
        let b0 = 1.0 - b1 - b2;
        for k in 0..3 {
            coords.push(b0 * va.position[k] as f64 + b1 * vb.position[k] as f64 + b2 * vc.position[k] as f64);
            normals.push(b0 * va.normal[k] as f64 + b1 * vb.normal[k] as f64 + b2 * vc.normal[k] as f64);
        }
        // near-white so the lambert term IS the picture
        colors.extend_from_slice(&[235, 230, 220, 255]);
    }

    let mut pc = session_rust::PointCloud::from_coords(coords, colors, normals);
    pc.point_size = 3.0;
    pc.name = "bunny_cloud".to_string();
    let mut s = session_rust::Session::new("bunny_cloud");
    s.add_pointcloud(pc, None);
    s.pb_dump(&out);
    println!("wrote {out}: {count} points with normals");
}
```

```
cargo run --example mk_bunny_cloud --target x86_64-unknown-linux-gnu --release -- \
    assets/pb/mesh_bunny_grey.pb assets/pb/bunny_cloud.pb 1500000
```

## What is still Potree's, not ours

The octree. Potree preprocesses (PotreeConverter) into a multi-res hierarchy and streams
nodes by screen-space error under a point budget — that is both its unbounded scale AND
its uniform on-screen density. Lesson [44](44-cloud-octree.md) builds exactly that for
the WALKED lane, on the kernel's own `SpatialOctree`.

## Expected state

- Both shaders `naga`-clean; wasm check clean; the examples build.
- The lion, now shaded (same command as lesson 36):
  `non-background pixels: 325369 (33.9%)` — attenuation fills the surface, lambert and
  EDL light it.

![the lion, lambert + EDL + attenuated splats](img/40-lion-shaded.png)

- The sampled bunny (`assets/scenes/bunny_cloud.toml`, ground-truth normals):

![bunny cloud closeup](img/40-bunny.png)
