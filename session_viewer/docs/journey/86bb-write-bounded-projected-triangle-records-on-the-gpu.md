# 17.1.6 · Write bounded projected triangle records on the GPU

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 27–54 minutes.** [Estimate](typing-load.md).

Dispatch one invocation per original triangle and write screen bounds, inward edge equations and a conditioned depth plane.

## Type

Continue from [Clip original triangles before dividing by depth](86ba-clip-original-triangles-before-dividing-by-depth.md). [Save or recover your work](recovery.md).

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

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.6
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The finite projection compute recipe is assembled for source validation; its runtime pipeline is connected by the existing following lessons.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for write bounded projected triangle records on the gpu.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Guard the live triangle range before writing. Two dispatch dimensions cover large triangle counts without exceeding the per-dimension workgroup limit. Store one empty record for degenerate polygons; otherwise record inward edge equations, conservative bounds and the depth reference used by finite ink.

write bounded projected triangle records on the gpu.



Why use a nearby depth reference for small distant triangles?

Extrapolating a small distant face to the viewport centre magnifies cancellation. A nearby reference keeps its plane evaluation stable; large clipped faces use the centre to avoid huge corner coordinates.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict the output for a degenerate triangle and for a small distant face. After the later pipeline connection, inspect finite ink around a near-clipped face without changing original geometry.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.6
npm --prefix ../session_tests run course -- save 17.1.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Develop the bounded projection recipe using existing course buffers. The current packed-arena/reversed-depth adapter remains a separate implementation bridge.

Compile the cumulative browser app;86c validates the completed assembled WGSL. Actual GPU output is checked at the existing finite-visibility connection.

The browser contract is authored and requires an actual browser run. Native software-GPU results do not measure the Lenovo.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.1.6
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
