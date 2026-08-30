# 42 Cloud scenes — datasets, bbox packing, and the stress test

> Direct-path chain (36–44); replay-verified.

## Goal

Assemble the scene that exercises everything: real datasets placed by MEASURED bounding
boxes, and the final numbers the chain has earned.

## Placing clouds: pack by bounding box, but the RIGHT box

Hand-guessed translations either overlap scans or strand them 90 m apart. The honest way
is to measure: `examples/pb_bbox.rs` loads each `.pb` and prints every cloud's min/max
bounds — and, crucially, its **2–98 percentile box**:

```
assets/pb/lidar_scan000.pb  min/max ~67 x 69 x 33 m   p2..p98 ~9.8 x 8.4 x 4.2 m
```

A terrestrial scan's min/max box is mostly empty air — a handful of sparse far returns
inflate it 7×. Packing on it leaves the dense cores tens of metres apart; packing on the
percentile core with no margin makes the outskirts overlap. The layout that works:
**cursor-pack the percentile cores along x with a deliberate visible gap (25 m here),
centre each core's y on 0, and ground each floor (p2 z) at 0** — the last part is what
makes unregistered scans sit on one shared ground plane. Rotated placements (the lion's
xform) get their box corners transformed before packing. The translations in
`cloud_mix.json` are those numbers, with the measurement command in the comment.

## The stress scene

`assets/scenes/cloud_mix.toml`: the bunny mesh (104k edge tubes), four architectural
sheets, the pen-test boxes — and three clouds, each showing a different part of the lane:

- **scan000** — 3.65 M points, reflectance colours, no normals, `point_size: 1`
- **Takanawa lion** — 342k points, colours AND normals (lesson [40](40-potree-look.md)),
  `point_size: 3`
- **scan006** — 3.50 M points, `point_size: 6`

Three sizes on screen at once is the per-cloud size feature demonstrating itself; the
lion's lambert against the scans' EDL-only shading is the normals feature doing the same.

## The numbers this chain ends on

Intel RPL-S iGPU (Vulkan under BrowserWebGpu), 1332×927, rAF medians:

```
        full scene, 7.5 M cloud points + 210k objects     presented fps
        ─────────────────────────────────────────────     ─────────────
        fit view, idle                                    60
        fit view, orbiting                                60
        fit view, wheel-zooming                           60
        deep zoom inside a scan, orbiting                 60
```

The remaining known costs are NOT the clouds: the load-phase jank (1–3 fps while sheets
parse) is the main-thread prost decode — lesson [43](43-streaming-cloud.md)'s territory —
and the occasional 20 ms rebase blip is the 210k-object instance table, throttled to
≤5/s by lesson [38](38-big-scenes.md).

## Tooling steps this lesson adds

**Create `examples/pb_bbox.rs`** (the measurement tool behind the packing numbers):

```rust
// Print each point cloud's bounding box from .pb files - feeds the bbox-packing layout.
fn main() {
    for path in std::env::args().skip(1) {
        let bytes = std::fs::read(&path).expect("read");
        let s = session_rust::Session::pb_loads(&bytes).expect("parse");
        for g in s.order() {
            if let Some(session_rust::Geometry::PointCloud(pc)) = s.lookup.get(&g) {
                let c = pc.coords();
                let mut mn = [f64::INFINITY; 3];
                let mut mx = [f64::NEG_INFINITY; 3];
                for i in (0..c.len()).step_by(3) {
                    for k in 0..3 { mn[k] = mn[k].min(c[i + k]); mx[k] = mx[k].max(c[i + k]); }
                }
                // percentile bounds too: a scan's min/max box is mostly empty air
                let n = c.len() / 3;
                let mut pl = [0.0f64; 3];
                let mut ph = [0.0f64; 3];
                for k in 0..3 {
                    let mut v: Vec<f64> = (0..n).step_by((n / 20000).max(1)).map(|i| c[i * 3 + k]).collect();
                    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    pl[k] = v[v.len() * 2 / 100];
                    ph[k] = v[v.len() * 98 / 100];
                }
                println!("{path} {mn:?} {mx:?} p2 {pl:?} p98 {ph:?}");
            }
        }
    }
}
```

**The frame benchmark.** In `src/selftest.rs`, **find** (the final render in
`render_scene`):

```rust
    let rgba = gpu.render_offscreen(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj);
```

**Add above it:**

```rust
    // VIEWER_FRAMES=N times N full offscreen frames (each one submits and reads
    // back, so the wall clock includes the GPU actually finishing) and reports the median.
    if let Some(n) = std::env::var("VIEWER_FRAMES").ok().and_then(|v| v.parse::<usize>().ok()).map(|n| n.max(1)) {
        let mut ms: Vec<f64> = Vec::new();
        for _ in 0..n {
            let t = std::time::Instant::now();
            let _ = gpu.render_offscreen(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj);
            ms.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("frames: n={} median {:.1} ms ({:.0} fps) min {:.1} max {:.1} | cloud scale x{}",
            n, ms[n / 2], 1000.0 / ms[n / 2], ms[0], ms[n - 1], gpu.cloud_size);
    }
```

Same-camera frames hit lesson 39's static skip, so this measures the CACHED path; treat
it comparatively.

**The browser scene.** In `src/lib.rs`, **find**:

```rust
const DEMO_SCENE_URL: &str = "scenes/bunny_drawings.toml";
```

**Replace with:**

```rust
const DEMO_SCENE_URL: &str = "scenes/cloud_mix.toml"; // was bunny_drawings.json
```

The scene manifests themselves — `cloud_mix.json` (the packed stress scene),
`lion.json`, `bunny_cloud.json` — are data, in `assets/scenes/`.

## Keys and knobs

```
        [  ]          global cloud size scale, ×0.25 steps
        F             fit; also re-grows the far-plane floor
        VIEWER_EDL    EDL strength (default 0.25, 0 = off)
        point_size    per cloud, in the manifest; 0 = the pb's own
```

## What would come next

Potree's remaining edge is the **octree**: a multi-res hierarchy selected by screen-space
error. Lesson [44](44-cloud-octree.md) builds it for the walked lane on the kernel's own
`SpatialOctree`; streaming BY OCTREE NODE (unbounded scale) stays future work beside
lesson [43](43-streaming-cloud.md)'s byte-range streaming.


## Expected state

```
VIEWER_W=1600 VIEWER_H=700 VIEWER_ZOOM=3 \
cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- \
    out.ppm assets/scenes/cloud_mix.toml
# => non-background pixels: 12143 (1.1%)
```

![the packed stress scene](img/41-cloud-mix.png)

And in the browser: 60 fps at the fit view — idle, orbiting, wheel-zooming — measured
with an rAF probe, not the frame counter.
