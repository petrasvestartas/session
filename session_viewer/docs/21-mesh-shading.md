# 21 Mesh shading — give the box a shape

The box is a flat blue silhouette — every face the same colour, so a cube reads as a hexagon. This
lesson gives each face a **normal** and a simple **light model** (hemisphere ambient + two directional
lights): faces toward the light go bright, faces away go dark.

**One file changes**: the mesh's fragment shader. Pipeline, buffers, camera, grid — untouched.

## Why

Shading needs a **normal** per pixel — two ways to get one:

```
per-vertex normals (in RenderVertex @1)   → smooth shading   ← lesson 22
per-face normal from screen derivatives   → flat shading     ← THIS lesson
```

The box may lack vertex normals, and a cube wants hard edges anyway — so reconstruct the face normal
in the fragment shader from how world position changes across the triangle: `dpdx`/`dpdy` are
screen-space derivatives, their cross product is the face normal. Works on any mesh, no vertex
normals needed.

```
world_pos (passed from the vertex shader)
   │  n = normalize(cross(dpdy(world_pos), dpdx(world_pos)))   ← per-face normal
   ▼
light model:  hemisphere ambient  +  key·max(dot(n,key),0)  +  fill·max(dot(n,fill),0)
   ▼
out = base_color × lit
```

<svg viewBox="0 0 680 190" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the three light terms on a box: hemisphere ambient brighter toward up, a key light from the upper left, a fill light from the right; faces sum the terms by their normal direction" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(110,30)">
    <path d="M 0,70 L 70,100 L 150,70 L 80,42 Z" fill="#8fb7e0" stroke="#3a3a3a"/>
    <path d="M 0,70 L 0,120 L 70,150 L 70,100 Z" fill="#3d5878" stroke="#3a3a3a"/>
    <path d="M 70,100 L 70,150 L 150,120 L 150,70 Z" fill="#5d82ab" stroke="#3a3a3a"/>
    <text x="80" y="66" fill="#0d0f12" font-size="10" text-anchor="middle">bright: faces the key + sky</text>
    <text x="34" y="118" fill="#c9d4e0" font-size="9">dark: away from both</text>
  </g>
  <path d="M 60,20 L 130,72" stroke="#e0b040" stroke-width="1.6" marker-end="url(#ah21)"/>
  <text x="46" y="16" fill="#e0b040">key (upper-left)</text>
  <path d="M 340,80 L 272,120" stroke="#888" stroke-width="1.3" marker-end="url(#ah21g)"/>
  <text x="346" y="78" fill="#888">fill (weaker, opposite)</text>
  <g transform="translate(430,24)">
    <path d="M 0,60 A 60,60 0 0 1 120,60" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
    <line x1="0" y1="60" x2="120" y2="60" stroke="#3a3a3a"/>
    <text x="60" y="20" fill="#6fb3ff" text-anchor="middle" font-size="10">sky: bright</text>
    <text x="60" y="76" fill="#666" text-anchor="middle" font-size="10">ground: dim</text>
    <text x="60" y="100" fill="#888" text-anchor="middle" font-size="10">hemisphere ambient:</text>
    <text x="60" y="114" fill="#888" text-anchor="middle" font-size="10">mix by how much n points up</text>
  </g>
  <text x="340" y="172" fill="#666" text-anchor="middle" font-size="10">three cheap terms, summed per pixel — no shadows, no textures, yet the box reads as a solid</text>
  <defs>
    <marker id="ah21" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#e0b040"/></marker>
    <marker id="ah21g" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#888"/></marker>
  </defs>
</svg>

**Two sign traps**, both from WebGPU's conventions:
- Framebuffer **Y points down**, so `cross(dpdx, dpdy)` points *into* the surface and every dot
  product goes negative. Use `cross(dpdy, dpdx)` for an **outward** normal.
- Back faces have the opposite geometric normal — flip `n` when `@builtin(front_facing)` is false, or
  the box's inside becomes the lit side.

## Files we touch

```
src/shaders/triangle.wgsl   # pass world_pos through; light the fragment (the only change)
```

That's it — the mesh still draws through the existing pipeline. (The old `time` uniform is unused now
but harmless to leave bound; dropped once the mesh gets its own pipeline, later.)

## Step 1 — pass the world position through: `src/shaders/triangle.wgsl`

The fragment shader needs the un-projected position for derivatives. Add it to `VsOut` and set it in
the vertex shader — the box has no per-object transform yet, so model space *is* world space:
`in.position` is the world position.

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

Replace the time-pulse fragment shader with the light model. Compute the normal **first**
(derivatives need uniform control flow, before any `if`), flip it for back faces, sum
ambient + key + fill:

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

The old `time` uniform declaration (`@group(1) @binding(0) var<uniform> time`) is unused now — delete
it or leave it, either compiles; an unused bound group is harmless.

## Step 3 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The cube reads as a cube: top face brightest (sky + key light), sides stepping down, faces away
sitting in ambient. Orbit (right-drag) and shading shifts with the lights — the flat blue hexagon is
gone. Edges stay crisp since the normal is per-face: flat shading. Lesson 22 rounds curved meshes
without softening a cube's corners.

## Recap

```
Ch 20: the box sat flat on the grid — one colour, no shape.
Ch 21: shade it. Pass world_pos from the vertex shader; in the fragment build a per-face normal with
       n = normalize(cross(dpdy, dpdx)) (that order for an OUTWARD normal under WebGPU's Y-down
       framebuffer), flip it for back faces, and light it with hemisphere ambient + a key and fill
       directional term. base_color × lit. One file changed; no vertex normals needed.
```

Edited: `shaders/triangle.wgsl` (add `world_pos` to `VsOut`/`vs_main`; replace the pulse `fs_main`
with a flat-normal hemisphere+key+fill light model).

## Next

`22-flat-vs-smooth.md` — use the **per-vertex** normal already in `RenderVertex` at location 1. An
interpolated normal rounds curved meshes while a cube keeps hard edges — the flat-vs-smooth choice,
made per mesh.
