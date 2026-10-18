# 20.2.11 · Test rasterized neighbors at the stroke source

**Chapter 20 · Shared GPU rendering · Section 20.2 · Surface-fitted finite ink** · [Contents](contents.md)

**Typing: 14–27 minutes.** [Estimate](typing-load.md).

A thick stroke may touch a triangle outside its center line.

## Type

Continue from [Connect surface-fitted point visibility](96zw-connect-surface-fitted-point-visibility.md). [Save or recover your work](recovery.md).

### 1. `src/ink_plane.wgsl`

Build a weighted source axis and reuse one exact clipped-triangle predicate for fringe and source neighbors.

<details>
<summary>Locate the existing block</summary>

```wgsl
            return ink_axis_visible(pixel, axis, sample);
        }
        gradient = finite_projected[triangle].gradient.xy;
    }
    if (any(abs(gradient) >= vec2<f32>(SLOPE_INVALID))) {
        return ink_axis_visible(pixel, axis, sample);
    }
    let delta = axis.at - pixel;
    let predicted = z + dot(gradient, delta);
    let tolerance = ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)) + ink_sag_slack(axis.sag, gradient);
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
```

</details>

After that block, type:

```wgsl

fn finite_ink_axis(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>)->InkAxis{
 let delta=ends.zw-ends.xy;let len=length(delta);let along=delta/max(len,1e-6);
 let distance=clamp(dot(pixel-ends.xy,along),0.,len);let at=ends.xy+along*distance;
 let depth=select(depths.x,(depths.x*(len-distance)+depths.y*distance)/max(len,1e-6),len>1e-6);
 return InkAxis(at,depth,along,(depths.y-depths.x)/max(len,1e-3),vec2<f32>(0.));
}
fn ink_exact_floor(axis:InkAxis,triangle:ProjectedTriangle)->f32{
 return axis.depth+abs(axis.depth)*DEPTH_REL_TOL+ink_sag_slack(axis.sag,triangle.gradient.xy);
}
fn finite_triangle_hides(primitive:u32,axis:InkAxis)->bool{
 if(primitive==0u||primitive>arrayLength(&finite_projected)){return false;}
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,1.-hit.x)));
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis)->bool{
 let fringe=ink_primitive(pixel,0u);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),0u);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
 }
 return false;
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zx-test-the-rasterized-neighbors-at-the-stroke-source-1.wgsl"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.2.11
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The view stays unchanged while the exact source helpers compile. Original geometry and integer owners remain unchanged.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for test rasterized neighbors at the stroke source.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

After projection, depth is affine along the screen segment. Weight each endpoint by its distance to the other end, avoiding cancellation when a near-clipped endpoint lies far off screen. Read the triangle at its coverage pixel and the four pixels surrounding the source. Test each projected triangle at the exact center line, using the current depth floor and clipping predicate. Reject empty or out-of-range primitive addresses. Keep this helper separate from tile traversal so the same exact hit rule is used once. This stage prepares the rule without changing active stroke visibility.

Projected stroke axis → primitive image → exact source hit and neighboring triangle hits.



Why test the exact center line after fitting a rasterized surface?

Coverage pixels can lie beside the center line or across a face edge. Exact projected triangles settle whether the source itself is occluded.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one center-line sample across a projected face edge and compare its original triangle address with the rasterized neighbor. The browser picture stays unchanged until the [visibility connection](96zy-connect-fitted-surfaces-and-exact-stroke-occlusion.md).

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.2.11
npm --prefix ../session_tests run course -- save 20.2.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Bridge current surface fitting and exact source-neighbor tests to the existing single-sample forward-depth adapter. Source sag, cap primitive sentinels, the smooth-boundary flag and multisampling remain later bridges.

Chrome acceptance for shared vertex reads, original normal pixels, placed instances and selection is pending. Integer primitive readback has separate GPU fixture evidence.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.2.11
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
