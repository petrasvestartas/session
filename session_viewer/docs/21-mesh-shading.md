# 21 Mesh shading — give the box a shape

The box is a flat blue silhouette: every face is the same colour, so a cube reads as a hexagon. This
lesson lights it. We give each face a **normal** (which way it points) and a simple **light model**
(hemisphere ambient + two directional lights), so faces facing the light read bright and faces turned
away read dark — the solid finally looks solid.

It's a **one-file** lesson: only the mesh's fragment shader changes. The pipeline, buffers, camera,
and grid all stay exactly as they are.

## Why

Shading needs one thing the flat shader ignored: a **normal** per pixel. Two ways to get it —

```
per-vertex normals (in RenderVertex @1)   → smooth shading   ← lesson 22
per-face normal from screen derivatives   → flat shading     ← THIS lesson
```

The box mesh may not carry good vertex normals, and a cube *wants* hard edges anyway, so we take the
robust route: reconstruct the face normal in the fragment shader from how the world position changes
across the triangle — `dpdx`/`dpdy` are the screen-space derivatives, and their cross product is the
face normal. No vertex normals required, works on any mesh.

```
world_pos (passed from the vertex shader)
   │  n = normalize(cross(dpdy(world_pos), dpdx(world_pos)))   ← per-face normal
   ▼
light model:  hemisphere ambient  +  key·max(dot(n,key),0)  +  fill·max(dot(n,fill),0)
   ▼
out = base_color × lit
```

**Two sign traps** (both from WebGPU's conventions, both easy to get backwards):
- WebGPU's framebuffer **Y points down**, so `cross(dpdx, dpdy)` points *into* the surface — every
  dot product goes negative and nothing lights. Use `cross(dpdy, dpdx)` for an **outward** normal.
- Back faces have the opposite geometric normal, so we flip `n` when `@builtin(front_facing)` is
  false — otherwise the inside of the box would be the lit side.

## Files we touch

```
src/shaders/triangle.wgsl   # pass world_pos through; light the fragment (the only change)
```

That's it — the mesh still draws through the existing pipeline. (The `time` uniform the old pulse used
is now unused by this shader; leaving it bound is harmless — we'll drop it when the mesh gets its own
pipeline in a later lesson.)

## Step 1 — pass the world position through: `src/shaders/triangle.wgsl`

The fragment shader needs the un-projected position to take derivatives of. Add it to `VsOut` and set
it in the vertex shader. The box has no per-object transform yet, so model space *is* world space —
`in.position` is the world position:

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(2) color: vec3<f32>,   // RenderVertex color is RGBA @2 — we read the RGB
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,   // ← NEW: model = world (no per-object matrix yet)
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    var o: VsOut;
    o.pos = mvp * vec4<f32>(in.position, 1.0);
    o.color = in.color;
    o.world_pos = in.position;           // ← NEW
    return o;
}
```

## Step 2 — light the fragment: `src/shaders/triangle.wgsl`

Replace the time-pulse fragment shader with the light model. Compute the normal **first** (derivatives
must run in uniform control flow, before any `if`), flip it for back faces, then sum ambient + key +
fill:

```wgsl
@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Per-face normal from screen-space derivatives — no vertex normals needed.
    // Y is DOWN in WebGPU, so cross(dpdy, dpdx) (not dpdx, dpdy) points OUTWARD.
    var n = normalize(cross(dpdy(in.world_pos), dpdx(in.world_pos)));
    if !front { n = -n; }                // back faces have the opposite normal

    // Two fixed world-space lights (a later lesson makes them follow the camera).
    let key_dir  = normalize(vec3<f32>(-0.3, -0.5, 0.8));
    let fill_dir = normalize(vec3<f32>( 0.6,  0.3, 0.4));
    let key  = max(dot(n, key_dir),  0.0) * 0.65;
    let fill = max(dot(n, fill_dir), 0.0) * 0.30;

    // Hemisphere ambient: darker "ground" → lighter "sky" along world +Z (up in this viewer).
    let hemi = mix(0.20, 0.35, 0.5 + 0.5 * n.z);

    let lit = hemi + key + fill;
    return vec4<f32>(in.color * lit, 1.0);
}
```

The old `time` uniform declaration (`@group(1) @binding(0) var<uniform> time`) is no longer used —
you can delete it from the shader or leave it; either compiles. The pipeline still binds group 1, and
an unused bound group is harmless.

## Step 3 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The cube now reads as a cube: its top face is brightest (normal points at the sky term and the key
light), the sides step down in tone, and the faces turned away sit in ambient. Orbit (right-drag) and
the shading shifts as faces turn toward and away from the fixed lights — the flat blue hexagon is
gone. Because the normal is per-face, the edges stay crisp (that's flat shading; smooth shading in
lesson 22 will round curved meshes without softening a cube's corners).

## Recap

```
Ch 20: the box sat flat on the grid — one colour, no shape.
Ch 21: shade it. Pass world_pos from the vertex shader; in the fragment, build a per-face normal with
       n = normalize(cross(dpdy, dpdx)) (that order for an OUTWARD normal under WebGPU's Y-down
       framebuffer), flip it for back faces, and light it with hemisphere ambient + a key and fill
       directional term. base_color × lit. One file changed; no vertex normals needed.
```

Edited: `shaders/triangle.wgsl` (add `world_pos` to `VsOut`/`vs_main`; replace the pulse `fs_main`
with a flat-normal hemisphere+key+fill light model).

## Next

`22-flat-vs-smooth.md` — use the **per-vertex** normal sitting in `RenderVertex` at location 1. A
smooth term (interpolated normal) rounds curved meshes, while a flag keeps hard edges on a cube — the
flat-vs-smooth choice, made per mesh.
