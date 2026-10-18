# 17.1 · Bounded projected triangles and tile lists

Build a demand-sized GPU index of original projected triangles without unbounded allocation or blocking readback.

## Ownership and data flow

A physical tile grid grows its span until the tile count fits. Clip source triangles before perspective division, emit bounded projected records, count conservative tile coverage, scan counts/blocks and fill bounded original-triangle references. Resource owners retain explicit buffers, layouts and pipelines; asynchronous demand reports wake only on completion and justify bounded growth. A source/model/camera witness reuses valid projection and retires obsolete bindings when resources change.

## The maintained implementation

engine/gpu/triangle_tiles.rs owns maintained 96-byte projected records, bounded TileLayout, reference pools and PoolReport readback. src/shaders/project_triangles.wgsl, triangle_tiles.wgsl and scan_triangle_tiles.wgsl supply actual projection/count/scan/fill. build.rs expands each shader include once per output, strips line comments/whitespace and writes OUT_DIR/shaders; pipeline composition must consume that generated source instead of compiling raw #include directives.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/triangle_tiles.rs](../../../src/engine/gpu/triangle_tiles.rs), [shaders/project_triangles.wgsl](../../../src/shaders/project_triangles.wgsl), [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl), [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl), [shaders/vector.wgsl](../../../src/shaders/vector.wgsl), [engine/gpu/clip.rs](../../../src/engine/gpu/clip.rs), [shaders/cap.wgsl](../../../src/shaders/cap.wgsl), [engine/gpu/render.rs](../../../src/engine/gpu/render.rs), [engine/gpu/pass.rs](../../../src/engine/gpu/pass.rs), [build.rs](../../../build.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-86-copy-finite-tile-allocation-rules"></a>

<details>
<summary>Implementation 86-copy-finite-tile-allocation-rules: Bound the physical tile grid and reference storage</summary>

Bound the physical tile grid and reference storage.

Data flow: physical canvas → bounded tile layout → shared word pool.

### 1. `src/tile_math.rs`

Bound the physical tile grid and reference storage.

Create the file and type:

```rust
--8<-- "journey/code/86-copy-finite-tile-allocation-rules-typed-1.rs"
```

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod lasso_geometry_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86-copy-finite-tile-allocation-rules-02.rs" skip=39
```

</details>

<a id="checkpoint-86ma-grow-projection-and-reference-pools-from-demand"></a>

<details>
<summary>Implementation 86ma-grow-projection-and-reference-pools-from-demand: Grow projection and reference pools from demand</summary>

Grow projection and reference pools from demand.

Data flow: physical canvas → bounded tile layout → shared word pool.

### 1. `src/tile_math.rs`

Grow projection and reference pools from demand.

<details>
<summary>Locate the existing block</summary>

```rust
        self.header_records() * 16 + pool_words * 4
    }

    /// Largest pool, in words.
    pub fn max_pool_words(self) -> u64 {
        self.count() as u64 * REFERENCES_PER_TILE * REFERENCE_WORDS
    }

    /// First pool size for `triangles`, in words.
    pub fn initial_pool_words(self, triangles: u32) -> u64 {
        let references = self.count() as u64 * 2 + u64::from(triangles) * 8;
        (references * REFERENCE_WORDS)
            .max(MIN_POOL_WORDS)
            .min(self.max_pool_words())
    }
}
```

</details>

After that block, type:

```rust
/// Projection workgroups across and down for `triangles`, 64 each, rows of PROJECT_ROW_GROUPS.
pub fn project_groups(triangles: u32) -> (u32, u32) {
    let groups = triangles.div_ceil(64);
    (
        groups.min(PROJECT_ROW_GROUPS),
        groups.div_ceil(PROJECT_ROW_GROUPS),
    )
}

/// Next pool size: at least `floor`; doubled when the report overflowed.
/// Records for a table of `need` triangles, None while `capacity` fits: exact after a load jump, an eighth spare after an edit.
pub fn projected_records(need: u64, capacity: u64, most: u64) -> Option<u64> {
    if need <= capacity && need * 4 >= capacity {
        return None;
    }

    let spare = if need > capacity * 3 / 2 { 0 } else { need / 8 };
    Some((need + spare).min(most).max(need))
}

pub fn next_pool_words(
    current: u64,
    floor: u64,
    capacity: u64,
    report: Option<u64>,
    ceiling: u64,
) -> u64 {
    let mut want = current.max(floor);

    // a report at or past capacity means the lists did not fit
    if let Some(needed) = report
        && needed >= capacity
    {
        want = want.max(current.saturating_mul(2));
    }

    want.min(ceiling)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86ma-grow-projection-and-reference-pools-from-demand-typed-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/tile_math.rs`

Grow projection and reference pools from demand.

<details>
<summary>Locate the existing block</summary>

```text
et spare = if need > capacity * 3 / 2 { 0 } else { need / 8 };
    Some((need + spare).min(most).max(need))
}

pub fn next_pool_words(
    current: u64,
    floor: u64,
    capacity: u64,
    report: Option<u64>,
    ceiling: u64,
) -> u64 {
    let mut want = current.max(floor);

    // a report at or past capacity means the lists did not fit
    if let Some(needed) = report
        && needed >= capacity
    {
        want = want.max(current.saturating_mul(2));
    }

    want.min(ceiling)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/86ma-grow-projection-and-reference-pools-from-demand-typed-2.rs" skip=500
```

</details>

</details>

<a id="checkpoint-86a-copy-maintained-projection-and-scan-recipes"></a>

<details>
<summary>Implementation 86a-copy-maintained-projection-and-scan-recipes: Define one shared projected-triangle coverage contract</summary>

Store stable screen-space triangle records and reuse one coverage and tile-span owner.

Data flow: finite polygons → tile counts → prefix offsets → references.

### 1. `src/projected_triangle.wgsl`

Copy the maintained shader recipe.

Create the file and type:

```wgsl
--8<-- "journey/code/86a-copy-maintained-projection-and-scan-recipes-01.rs"
```

</details>

<a id="checkpoint-86b-copy-the-project-recipe"></a>

<details>
<summary>Implementation 86b-copy-the-project-recipe: Declare finite projection buffers and original vertex access</summary>

Bind existing original mesh buffers and transform each indexed corner into the projection convention.

Data flow: declare finite projection buffers and original vertex access.

### 1. `src/finite_project.wgsl`

Create the file and type:

```wgsl
--8<-- "journey/code/86b-copy-the-project-recipe-01.wgsl"
```

</details>

<a id="checkpoint-86ba-clip-original-triangles-before-dividing-by-depth"></a>

<details>
<summary>Implementation 86ba-clip-original-triangles-before-dividing-by-depth: Clip original triangles before dividing by depth</summary>

Clip against the homogeneous near plane, retain up to four corners and measure signed screen area.

Data flow: clip original triangles before dividing by depth.

### 1. `src/finite_project.wgsl`

<details>
<summary>Locate the existing block</summary>

```wgsl
 let base=physical_indices[index]*6u;
 let point=vec3<f32>(physical_vertices[base],physical_vertices[base+1u],physical_vertices[base+2u]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}
```

</details>

After that block, type:

```wgsl

fn project_physical_triangle(primitive: u32, row: u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
        return polygon;
    }

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base), physical_clip_corner(base+1u), physical_clip_corner(base+2u));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
    let determinant = dot(edge0, input[0].xyw);

    if (determinant == 0.0) {
        return polygon;
    }

    let plane = (input[0].z * edge0 + input[1].z * cross(input[2].xyw, input[0].xyw)
        + input[2].z * cross(input[0].xyw, input[1].xyw)) / determinant;
    polygon.depth = vec3<f32>(2.0 * plane.x / line.vp_w, -2.0 * plane.y / line.vp_h, plane.z);
    var clipped: array<vec4<f32>, 4>;
    var previous = input[2];
    var previous_distance = previous.w - previous.z;

    // clip each edge against the near plane
    for (var i = 0u; i<3u; i++) {
        let current = input[i];
        let distance = current.w - current.z;

        if ((distance > 0.0 && previous_distance < 0.0) || (distance < 0.0 && previous_distance > 0.0)) {
            let t = previous_distance / (previous_distance-distance);
            clipped[polygon.count] = mix(previous, current, t);
            polygon.count++;
        }

        if (distance >= 0.0) {
            clipped[polygon.count] = current;
            polygon.count++;
        }

        previous = current;
        previous_distance = distance;
    }

    // clip space to screen pixels
    for (var i = 0u; i<polygon.count; i++) {
        let clip = clipped[i];

        if (clip.w <= 0.0) {
            polygon.count = 0u;
            return polygon;
        }

        let ndc = clip.xyz/clip.w;
        polygon.points[i] = vec3<f32>((ndc.x*0.5+0.5)*line.vp_w, (0.5-ndc.y*0.5)*line.vp_h, ndc.z);
    }

    return polygon;
}
fn physical_cross(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x*b.y-a.y*b.x;
}
fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // zero only for a degenerate polygon
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86ba-clip-original-triangles-before-dividing-by-depth-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86bb-write-bounded-projected-triangle-records-on-the-gpu"></a>

<details>
<summary>Implementation 86bb-write-bounded-projected-triangle-records-on-the-gpu: Write bounded projected triangle records on the GPU</summary>

Dispatch one invocation per original triangle and write screen bounds, inward edge equations and a conditioned depth plane.

Data flow: write bounded projected triangle records on the gpu.

### 1. `src/finite_project.wgsl`

<details>
<summary>Locate the existing block</summary>

```wgsl
    return a.x*b.y-a.y*b.x;
}
fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // zero only for a degenerate polygon
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}
```

</details>

After that block, type:

```wgsl

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // rows of 32768 workgroups (PROJECT_ROW_GROUPS): a dispatch dimension holds at most 65535
    let local = id.x + id.y * 2097152u;
    let index = local + live_count.y;

    if (local>=live_count.x) {
        return;
    }

    let polygon = project_physical_triangle(local+1u, 0u);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
    if (polygon.count>=3u) {
        // screen box
        var lo = polygon.points[0].xy;
        var hi = lo;

        for (var i = 1u;i<polygon.count;i++) {
            lo = min(lo, polygon.points[i].xy);
            hi = max(hi, polygon.points[i].xy);
        }

        let area = physical_polygon_area(polygon);

        // degenerate: empty record
        if (abs(area)<1e-12) {
            projected[index] = out;
            return;
        }

        // one inward line equation per edge
        var equations: array<vec4<f32>, 4>;

        for (var i = 0u;i<polygon.count;i++) {
            let a = polygon.points[i].xy;
            let edge = polygon.points[(i+1u)%polygon.count].xy-a;
            let normal = sign(area)*vec2<f32>(-edge.y, edge.x)/length(edge);
            equations[i] = vec4<f32>(normal, -dot(normal, a), 0.0);
        }

        // Small far triangles need a nearby reference; extrapolating their homogeneous
        // plane to the viewport centre magnifies cancellation. Huge near-clipped faces
        // still use that centre, keeping their million-pixel corners out of the depth fit.
        var reference = vec3<f32>(line.vp_w * 0.5, line.vp_h * 0.5, polygon.depth.z);
        var gradient = polygon.depth.xy;
        let viewport = vec2<f32>(line.vp_w, line.vp_h);
        if (polygon.count == 3u && all(lo >= -viewport) && all(hi <= 2.0 * viewport)) {
            let ab = polygon.points[1] - polygon.points[0];
            let ac = polygon.points[2] - polygon.points[0];
            gradient = vec2<f32>(ab.z * ac.y - ac.z * ab.y, ab.x * ac.z - ac.x * ab.z) / area;
            reference = polygon.points[0];
        }
        out.edge0 = vec4<f32>(equations[0].xyz, reference.x);
        out.edge1 = vec4<f32>(equations[1].xyz, reference.y);
        out.edge2 = vec4<f32>(equations[2].xyz, reference.z);
        out.edge3 = vec4<f32>(equations[3].xyz, f32(polygon.count));
        // nearest corner depth
        var nearest = polygon.points[0].z;

        for (var i = 1u;i<polygon.count;i++) {
            nearest = max(nearest, polygon.points[i].z);
        }

        out.gradient = vec4<f32>(gradient, nearest, 0.0);
        out.bounds = vec4<f32>(lo, hi);
    }

    projected[index] = out;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86bb-write-bounded-projected-triangle-records-on-the-gpu-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86c-project-existing-original-mesh-buffers"></a>

<details>
<summary>Implementation 86c-project-existing-original-mesh-buffers: Assemble and validate the shared finite projection shader</summary>

Reuse shared WGSL declarations and validate the completed compute entry point.

Data flow: original buffers → finite projected records → tile references.

### 1. `src/finite_project_source.rs`

Assemble one shared shader interface and validate its compute entry point.

Create the file and type:

```rust
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-source-browser-info-reuse-check.rs"
```

### 2. `src/lib.rs`

Expose the existing-source assembly boundary for the later pipeline.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod tile_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-register.rs" skip=19
```

### 3. `src/renderer.rs`

Compile the completed recipe on the actual browser device and report asynchronous compilation info.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-04-source-reuse-check.rs"
```

</details>

</details>

<a id="checkpoint-86d-copy-the-triangle-tiles-recipe"></a>

<details>
<summary>Implementation 86d-copy-the-triangle-tiles-recipe: Declare atomic tile records and flat triangle inputs</summary>

Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words.

Data flow: declare atomic tile records and flat triangle inputs.

### 1. `src/finite_triangle_tiles.wgsl`

Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words.

Create the file and type:

```wgsl
--8<-- "journey/code/86d-copy-the-triangle-tiles-recipe-01.wgsl"
```

### 2. `src/finite_project_source.rs`

Expand shared source sections once for projection, tile passes and finite ink.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String {
    let shared = include_str!("projected_triangle.wgsl")
        .split("// Pixels per tile side for this canvas.").next().unwrap();
    include_str!("finite_project.wgsl").replace("#include \"projected_triangle.wgsl\"", shared)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86d-copy-the-triangle-tiles-recipe-02-source.rs"
```

</details>

<a id="checkpoint-86da-count-conservative-triangle-coverage-per-tile"></a>

<details>
<summary>Implementation 86da-count-conservative-triangle-coverage-per-tile: Count conservative triangle coverage per tile</summary>

Rasterize one tile-grid quad per triangle and reject tiles outside any inward edge before counting.

Data flow: count conservative triangle coverage per tile.

### 1. `src/finite_triangle_tiles.wgsl`

Rasterize one tile-grid quad per triangle and reject tiles outside any inward edge before counting.

<details>
<summary>Locate the existing block</summary>

```wgsl
    @location(6) @interpolate(flat) gradient: vec3<f32>, // depth slope and nearest depth
    @location(7) @interpolate(flat) reference: vec3<f32>, // reference point and its depth
};
#include "projected_triangle.wgsl"
#include "projected_triangle.wgsl:viewport-span"
```

</details>

After that block, type:

```wgsl

@vertex
fn vs_main(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> TileVertex {
    let triangle = projected[instance];
    var out: TileVertex;
    out.clip = vec4<f32>(2.0, 2.0, 0.0, 1.0);

    // empty triangle: off screen
    if (triangle.edge3.w<3.0) {
        return out;
    }

    // tile grid size
    let size = ceil(vec2<f32>(line.vp_w, line.vp_h)/f32(visibility_tile_span()));
    let lo = clamp(floor((triangle.bounds.xy-0.00390625)/f32(visibility_tile_span())), vec2<f32>(0.0), size);
    let hi = clamp(floor((triangle.bounds.zw+0.00390625)/f32(visibility_tile_span()))+1.0, vec2<f32>(0.0), size);
    // triangle strip: two triangles from four corners
    let corners = array<vec2<f32>, 4>(vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0));
    let position = mix(lo, hi, corners[vertex])/size;
    out.clip = vec4<f32>(position.x*2.0-1.0, 1.0-position.y*2.0, 0.0, 1.0);
    out.edge0 = triangle.edge0.xyz;
    out.edge1 = triangle.edge1.xyz;
    out.edge2 = triangle.edge2.xyz;
    out.edge3 = triangle.edge3.xyz;
    out.bounds = triangle.bounds;
    out.primitive = instance+1u;
    out.gradient = triangle.gradient.xyz;
    out.reference = vec3<f32>(triangle.edge0.w, triangle.edge1.w, triangle.edge2.w);
    return out;
}
fn tile_outside(edge: vec3<f32>, centre: vec2<f32>) -> bool {
    return dot(edge.xy, centre)+edge.z+0.5*f32(visibility_tile_span())*(abs(edge.x)+abs(edge.y)) < -0.00390625;
}
fn covered_tile(v: TileVertex) -> u32 {
    let centre = (floor(v.clip.xy)+0.5)*f32(visibility_tile_span());

    if (tile_outside(v.edge0, centre) || tile_outside(v.edge1, centre) || tile_outside(v.edge2, centre) || tile_outside(v.edge3, centre)) {
        discard;
    }

    let size = vec2<u32>(ceil(vec2<f32>(line.vp_w, line.vp_h)/f32(visibility_tile_span())));
    let tile = 1u+u32(v.clip.y)*size.x+u32(v.clip.x);
    return tile;
}
@fragment
fn fs_count(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);
    atomicAdd(&tile_records[tile].values[0], 1u);
    return 0.0;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86da-count-conservative-triangle-coverage-per-tile-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86db-fill-bounded-original-triangle-reference-lists"></a>

<details>
<summary>Implementation 86db-fill-bounded-original-triangle-reference-lists: Fill bounded original triangle reference lists</summary>

Atomically reserve a tile slot, store the original projected triangle address and depth, and flag overflow instead of writing past the pool.

Data flow: fill bounded original triangle reference lists.

### 1. `src/finite_triangle_tiles.wgsl`

Atomically reserve a tile slot, store the original projected triangle address and depth, and flag overflow instead of writing past the pool.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let tile = 1u+u32(v.clip.y)*size.x+u32(v.clip.x);
    return tile;
}
@fragment
fn fs_count(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);
    atomicAdd(&tile_records[tile].values[0], 1u);
    return 0.0;
}
```

</details>

After that block, type:

```wgsl

@fragment
fn fs_fill(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);

    // list overflowed: stop
    if (atomicLoad(&tile_records[tile].values[3])!=0u) {
        return 0.0;
    }

    let cursor = atomicAdd(&tile_records[tile].values[2], 1u);

    // more writes than counted: mark overflow
    if (cursor>=atomicLoad(&tile_records[tile].values[0])) {
        atomicStore(&tile_records[tile].values[3], 1u);
        return 0.0;
    }

    let offset = atomicLoad(&tile_records[tile].values[1])+cursor*2u;
    atomicStore(&tile_records[offset/4u].values[offset%4u], v.primitive);
    let centre = (floor(v.clip.xy)+0.5)*f32(visibility_tile_span());
    let slope = abs(v.gradient.x)+abs(v.gradient.y);
    // nearest depth the triangle reaches in this tile
    let nearest = min(v.gradient.z, v.reference.z+dot(v.gradient.xy, centre-v.reference.xy)+0.5*f32(visibility_tile_span())*slope);
    // plus float and vertex-snap tolerance
    let bound = nearest+abs(nearest)*1.9073486e-6+slope*0.00390625;
    atomicStore(&tile_records[(offset+1u)/4u].values[(offset+1u)%4u], bitcast<u32>(bound));
    return 0.0;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86db-fill-bounded-original-triangle-reference-lists-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86e-count-and-fill-original-triangle-references"></a>

<details>
<summary>Implementation 86e-count-and-fill-original-triangle-references: Assemble and validate the bounded tiles shader</summary>

Reuse shared WGSL source and validate the complete tiles pass interfaces.

Data flow: original buffers → finite projected records → tile references.

### 1. `src/renderer.rs`

Check the complete recipe on the actual browser GPU before the later pass connection.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86e-count-and-fill-original-triangle-references-01-source-reuse-check.rs"
```

</details>

### 2. `src/finite_project_source.rs`

Reuse one source expansion owner for the completed recipe.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn tiles() -> String { expand(include_str!("finite_triangle_tiles.wgsl")) }

pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86e-count-and-fill-original-triangle-references-02-source.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/finite_tiles_contract.rs`

Supply independent assembled WGSL entry-point witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/86e-count-and-fill-original-triangle-references-03-source.rs"
```

</details>

### 4. `src/lib.rs`

Register the supplied shader interface fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod finite_project_source;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod finite_tiles_contract;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86e-count-and-fill-original-triangle-references-04-source.rs"
```

</details>

</details>

<a id="checkpoint-86f-copy-the-scan-triangle-tiles-recipe"></a>

<details>
<summary>Implementation 86f-copy-the-scan-triangle-tiles-recipe: Scan bounded tile counts in workgroup memory</summary>

Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage.

Data flow: scan bounded tile counts in workgroup memory.

### 1. `src/finite_scan_triangle_tiles.wgsl`

Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage.

Create the file and type:

```wgsl
--8<-- "journey/code/86f-copy-the-scan-triangle-tiles-recipe-01.wgsl"
```

</details>

<a id="checkpoint-86fa-scan-tile-blocks-and-their-block-totals"></a>

<details>
<summary>Implementation 86fa-scan-tile-blocks-and-their-block-totals: Scan tile blocks and their block totals</summary>

Run the same prefix owner first over tile counts and then over their block totals.

Data flow: scan tile blocks and their block totals.

### 1. `src/finite_scan_triangle_tiles.wgsl`

Run the same prefix owner first over tile counts and then over their block totals.

<details>
<summary>Locate the existing block</summary>

```wgsl
            previous = scan[lane-stride];
        }

        workgroupBarrier();
        scan[lane] = min(scan[lane]+previous, capacity);
        workgroupBarrier();
    }

    return select(scan[lane]-bounded, capacity, scan[lane]==capacity);
}
```

</details>

After that block, type:

```wgsl

@compute @workgroup_size(256)
fn scan_tiles(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lane: u32, @builtin(workgroup_id) group: vec3<u32>) {
    let count = tile_count();
    var value = 0u;

    if (id.x<count) {
        // two words per reference
        value = records[1u+id.x].values[0]*2u;
    }

    let offset = prefix(lane, value);

    if (id.x<count) {
        records[1u+id.x].values[1] = offset;
    }

    // block total, for pass 2
    if (lane==255u) {
        records[1u+count+group.x].values[0] = scan[255];
    }
}
@compute @workgroup_size(256)
fn scan_blocks(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lane: u32, @builtin(workgroup_id) group: vec3<u32>) {
    let count = tile_count();
    let block_count = blocks(count);
    var value = 0u;

    if (id.x<block_count) {
        value = records[1u+count+id.x].values[0];
    }

    let offset = prefix(lane, value);

    if (id.x<block_count) {
        records[1u+count+id.x].values[1] = offset;
    }

    if (lane==255u) {
        records[1u+count+block_count+group.x].values[0] = scan[255];
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fa-scan-tile-blocks-and-their-block-totals-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86fb-finalize-bounded-tile-list-offsets-and-demand"></a>

<details>
<summary>Implementation 86fb-finalize-bounded-tile-list-offsets-and-demand: Finalize bounded tile list offsets and demand</summary>

Combine block and local offsets, reset fill cursors, and record actual pool demand in the shared header.

Data flow: finalize bounded tile list offsets and demand.

### 1. `src/finite_scan_triangle_tiles.wgsl`

Combine block and local offsets, reset fill cursors, and record actual pool demand in the shared header.

<details>
<summary>Locate the existing block</summary>

```wgsl
    let offset = prefix(lane, value);

    if (id.x<block_count) {
        records[1u+count+id.x].values[1] = offset;
    }

    if (lane==255u) {
        records[1u+count+block_count+group.x].values[0] = scan[255];
    }
}
```

</details>

After that block, type:

```wgsl

@compute @workgroup_size(256)
fn finish_offsets(@builtin(global_invocation_id) id: vec3<u32>) {
    let count = tile_count();

    if (id.x==0u) {
        // record 0: valid flag
        records[0].values[0] = 1u;
    }

    if (id.x>=count) {
        return;
    }

    let block_count = blocks(count);
    let pool = 4u*(1u+count+block_count+blocks(block_count));
    let tile = 1u+id.x;
    let block = id.x/256u;
    var offset = records[tile].values[1]+records[1u+count+block].values[1];

    for (var i = 0u;i<block/256u;i++) {
        offset+=records[1u+count+block_count+i].values[0];
    }

    let start = pool+offset;
    records[tile].values[1] = start;
    // overflow flag when the list runs past the buffer
    records[tile].values[3] = select(0u, 1u, start+records[tile].values[0]*2u>arrayLength(&records)*4u);

    // record 0 word 1: words needed; the CPU reads it back
    if (id.x==count-1u) {
        records[0].values[1] = start+records[tile].values[0]*2u;
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fb-finalize-bounded-tile-list-offsets-and-demand-01.wgsl"
```

</details>

</details>

<a id="checkpoint-86fc-assemble-and-validate-the-bounded-scan-shader"></a>

<details>
<summary>Implementation 86fc-assemble-and-validate-the-bounded-scan-shader: Assemble and validate the bounded scan shader</summary>

Reuse shared WGSL source and validate the complete scan pass interfaces.

Data flow: original buffers → finite projected records → tile references.

### 1. `src/renderer.rs`

Check the complete recipe on the actual browser GPU before the later pass connection.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-01-source-reuse-check.rs"
```

</details>

### 2. `src/finite_project_source.rs`

Reuse one source expansion owner for the completed recipe.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn scan() -> String { expand(include_str!("finite_scan_triangle_tiles.wgsl")) }

pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-02-source.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/finite_scan_contract.rs`

Supply independent assembled WGSL entry-point witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-03-source.rs"
```

</details>

### 4. `src/lib.rs`

Register the supplied shader interface fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod finite_project_source;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod finite_scan_contract;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-04-source.rs"
```

</details>

</details>

<a id="checkpoint-86g-wake-only-after-a-tile-report-arrives"></a>

<details>
<summary>Implementation 86g-wake-only-after-a-tile-report-arrives: Wake only after a tile report arrives</summary>

Wake only after a tile report arrives.

Data flow: completed map → browser event → one redraw.

### 1. `src/browser_tiles.rs`

Create the file and type:

```rust
--8<-- "journey/code/86g-wake-only-after-a-tile-report-arrives-01.rs"
```

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod tile_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86g-wake-only-after-a-tile-report-arrives-02.rs" skip=20
```

</details>

<a id="checkpoint-86h-read-actual-pool-demand-asynchronously"></a>

<details>
<summary>Implementation 86h-read-actual-pool-demand-asynchronously: Read actual pool demand asynchronously</summary>

Read GPU tile-pool demand asynchronously, with one map in flight and one reusable readback buffer.

Data flow: scan header → sixteen-byte copy → map result.

### 1. `src/tile_report.rs`

Type the complete existing WebGPU helper and reuse its current consumer. Return actual copy admission so a newer bin generation can retry after an old report.

Create the file and type:

```rust
use std::sync::{Arc,atomic::{AtomicU8,Ordering}};
/// Reads back how many words the last scan needed.
pub struct PoolReport {
    buffer: wgpu::Buffer, // 16-byte CPU-readable copy
    ready: Arc<AtomicU8>, // 0 waiting, 1 mapped, 2 failed
    copied: bool,         // a copy was encoded this frame
    inflight: bool,       // the copy is being mapped
}

impl PoolReport {
    /// Create the readback buffer.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("triangle.tiles.report"),
                size: 16,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            ready: Arc::new(AtomicU8::new(0)),
            copied: false,
            inflight: false,
        }
    }

    /// Copy the first tile record into the readback buffer.
    pub fn copy(&mut self, encoder: &mut wgpu::CommandEncoder, tiles: &wgpu::Buffer) ->bool {
        if self.inflight {
            return false;
        }

        encoder.copy_buffer_to_buffer(tiles, 0, &self.buffer, 0, 16);
        self.copied = true;
        true
    }

    /// Start mapping the copy; call once after submit.
    pub fn map(&mut self) {
        if !self.copied || self.inflight {
            return;
        }

        self.copied = false;
        self.inflight = true;
        let flag = self.ready.clone();
        self.buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                flag.store(if result.is_ok() { 1 } else { 2 }, Ordering::Release);
                #[cfg(target_arch="wasm32")] crate::browser_tiles::wake();
            });
    }

    /// Words the last scan needed, once the copy is readable.
    pub fn poll(&mut self) -> Option<u64> {
        if !self.inflight {
            return None;
        }

        let status = self.ready.load(Ordering::Acquire);

        if status == 0 {
            return None;
        }

        self.ready.store(0, Ordering::Release);
        self.inflight = false;

        if status != 1 {
            return None;
        }

        let words = {
            let bytes = self.buffer.slice(..).get_mapped_range();
            u32::from_le_bytes(bytes[4..8].try_into().expect("report words"))
        };
        self.buffer.unmap();
        Some(u64::from(words))
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86h-read-actual-pool-demand-asynchronously-typed-tile_report.rs"
```

```text
--8<-- "journey/code/86h-read-actual-pool-demand-asynchronously-02.rs"
```

</details>

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod browser_tiles;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86h-read-actual-pool-demand-asynchronously-03.rs" skip=24
```

</details>

<a id="checkpoint-86i-describe-explicit-finite-gpu-layouts"></a>

<details>
<summary>Implementation 86i-describe-explicit-finite-gpu-layouts: Describe explicit finite GPU layouts</summary>

Compile the finite pipeline resource types before their helper functions.

Data flow: uniform and storage visibility → explicit binding layouts.

### 1. `src/tile_pipelines.rs`

Describe explicit finite GPU layouts.

Create the file and type:

```rust
--8<-- "journey/code/86i-describe-explicit-finite-gpu-layouts-01.rs"
```

### 2. `src/lib.rs`

Register the resource types immediately so every following method is checked by the browser compiler.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod tile_report;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86k-connect-projection-count-scan-and-fill-pipelines-02.rs" skip=22
```

</details>

<a id="checkpoint-86j-build-compute-and-count-pipeline-helpers"></a>

<details>
<summary>Implementation 86j-build-compute-and-count-pipeline-helpers: Build compute and count pipeline helpers</summary>

Build compute and count pipeline helpers.

Data flow: validated shader → explicit pipeline layout → GPU pipeline.

### 1. `src/tile_pipelines.rs`

<details>
<summary>Locate the existing block</summary>

```rust


use wgpu::{BufferBindingType as Type,ShaderStages as Stage};


pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86j-build-compute-and-count-pipeline-helpers-01.rs"
```

</details>

<a id="checkpoint-86k-connect-projection-count-scan-and-fill-pipelines"></a>

<details>
<summary>Implementation 86k-connect-projection-count-scan-and-fill-pipelines: Connect projection count scan and fill pipelines</summary>

Connect projection count scan and fill pipelines.

Data flow: project → count → scan → fill.

### 1. `src/tile_pipelines.rs`

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{

}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86k-connect-projection-count-scan-and-fill-pipelines-01-shared-source-shared-bins.rs"
```

</details>

<a id="checkpoint-86l-own-bounded-finite-tile-resources"></a>

<details>
<summary>Implementation 86l-own-bounded-finite-tile-resources: Own bounded finite tile resources</summary>

Compile bounded tile resource types before allocation and population.

Data flow: original buffers → derived GPU owner → explicit cleanup.

### 1. `src/tile_gpu.rs`

Own bounded finite tile resources.

Create the file and type:

```rust
--8<-- "journey/code/86l-own-bounded-finite-tile-resources-01.rs"
```

### 2. `src/gpu_mesh.rs`

Expose existing buffers without changing original data.

<details>
<summary>Locate the existing block</summary>

```rust
    }

    pub(crate) fn packed(&self)->[u8;160] {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())!=0}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86ak-expose-original-gpu-models-to-projection-02.rs"
```

### 3. `src/lib.rs`

Register the resource types immediately so every following method is checked by the browser compiler.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod tile_pipelines;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86r-cache-projection-and-finish-current-reports-02.rs" skip=25
```

</details>

<a id="checkpoint-86m-allocate-empty-tile-placeholders"></a>

<details>
<summary>Implementation 86m-allocate-empty-tile-placeholders: Allocate empty tile placeholders</summary>

Allocate empty tile placeholders.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86m-allocate-empty-tile-placeholders-01.rs"
```

</details>

<a id="checkpoint-86n-witness-original-buffers-and-world-placement"></a>

<details>
<summary>Implementation 86n-witness-original-buffers-and-world-placement: Witness original buffers and world placement</summary>

Witness original buffers and world placement.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
  Self{projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86n-witness-original-buffers-and-world-placement-01.rs"
```

</details>

<a id="checkpoint-86o-release-obsolete-tile-lists-and-bindings"></a>

<details>
<summary>Implementation 86o-release-obsolete-tile-lists-and-bindings: Release obsolete tile lists and bindings</summary>

Release obsolete tile lists and bindings.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86o-release-obsolete-tile-lists-and-bindings-01.rs"
```

</details>

<a id="checkpoint-86p-grow-lists-only-from-actual-demand"></a>

<details>
<summary>Implementation 86p-grow-lists-only-from-actual-demand: Grow lists only from actual demand</summary>

Grow lists only from actual demand.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
 fn rebind(&mut self,device:&wgpu::Device){self.raster=group(device,&self.pipes.raster_layout,&[&self.projected,&self.buffer]);self.scan=group(device,&self.pipes.scan_layout,&[&self.buffer]);for row in &mut self.rows{row.group=None;}}
 fn release(&mut self,device:&wgpu::Device){if self.layout.take().is_none()&&self.projected.size()==PROJECTED_BYTES&&self.buffer.size()==16{return}self.projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);self.buffer=buffer(device,"finite tile placeholder",16);self.pool=0;self.target=None;self.key=None;self.rebind(device);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86p-grow-lists-only-from-actual-demand-01.rs"
```

</details>

<a id="checkpoint-86q-rasterize-bounded-tile-counts-and-references"></a>

<details>
<summary>Implementation 86q-rasterize-bounded-tile-counts-and-references: Rasterize bounded tile counts and references</summary>

Rasterize bounded tile counts and references.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
 }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86q-rasterize-bounded-tile-counts-and-references-01.rs"
```

</details>

<a id="checkpoint-86r-cache-projection-and-finish-current-reports"></a>

<details>
<summary>Implementation 86r-cache-projection-and-finish-current-reports: Cache projection and finish current reports</summary>

Cache projection and finish current reports.

Data flow: current originals and view → bounded buffers → cached finite lists.

### 1. `src/tile_gpu.rs`

<details>
<summary>Locate the existing block</summary>

```rust
  self.layout=Some(layout);if moved{self.key=None;self.rebind(device);}
 }

 fn bin(&self,encoder:&mut wgpu::CommandEncoder,pipeline:&wgpu::RenderPipeline){let Some(target)=&self.target else{return};let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite tile lists"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.raster,&[]);pass.draw(0..4,0..self.triangles);}


}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/86r-cache-projection-and-finish-current-reports-01.rs"
```

</details>

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 86r-cache-projection-and-finish-current-reports
npm --prefix ../session_tests run course -- check 86r-cache-projection-and-finish-current-reports
npm --prefix ../session_tests run course -- save 86r-cache-projection-and-finish-current-reports
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The following is the expected runnable-checkpoint behavior. This prose batch did not rerun its GPU fixtures; current finite-ink, section and comparison acceptance remains pending after algorithm reconciliation.

The existing source drawing remains visible while actual projection/count/scan/fill resources and cache witnesses are connected. Inspect bounded allocation and asynchronous pool-demand diagnostics during resting frames, rotation and dense overlap; unchanged witnesses reuse projection and released geometry retires obsolete bindings. These derived tables alone do not add a new drawn geometry family. The next lesson connects them to visible ink.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Finite ink visibility and original colour/pick ownership](86s-test-visibility-at-the-original-center-line.md).
