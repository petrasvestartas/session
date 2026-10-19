# 20.2.10 · Connect surface-fitted point visibility

**Chapter 20 · Shared GPU rendering · Section 20.2 · Surface-fitted finite ink** · [Contents](contents.md)

**Typing: 7–14 minutes.** [Estimate](typing-load.md).

A point is a disc, so a stroke axis cannot describe its visibility.

## Type

Continue from [Carry the rasterized triangle gradient to ink](96zv-carry-the-rasterized-triangle-gradient-to-ink.md). [Save or recover your work](recovery.md).

### 1. `src/finite_visibility.wgsl`

Connect the point-specific surface and center tests after clipping and section checks.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);let glass=view.opacity>0.&&view.opacity<1.;return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

</details>

After that block, type:

```wgsl

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zw-connect-surface-fitted-point-visibility-1.wgsl"
```

</details>

### 2. `src/finite_marker.wgsl`

Use the same original point predicate for color and cropped integer ownership.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zw-connect-surface-fitted-point-visibility-2.wgsl"
```

</details>

### 3. `src/finite_marker_pick.wgsl`

Use the same original point predicate for color and cropped integer ownership.

<details>
<summary>Locate the existing block</summary>

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)<=0.){discard;}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags)<=0.){discard;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zw-connect-surface-fitted-point-visibility-3.wgsl"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/finite_visibility_tests.rs`

Supply an actual point disc pixel and first-pick regression at both physical display scales.

<details>
<summary>Locate the existing block</summary>

```text
1.,0.]};renderer.set_grid_on(Some(crate::grid::Settings{spacing:0.1,half_cells:1,..Default::default()}),plane).unwrap();assert_eq!(red_pixels(&renderer,size),baseline,"coplanar red construction grid cannot paint over the solid");let first=renderer.tile_usage();renderer.resize(crate::viewport::Viewport{width:319,height:239});let _=red_pixels(&renderer,[319,239]);assert_eq!(renderer.tile_usage()[5],first[5]+1,"physical viewport change invalidates projection even with the same tile dimensions");
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zw-connect-surface-fitted-point-visibility-4.rs" skip=500
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.2.10
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

A visible point retains its full CSS disc on a sloping face. A hidden point paints no opaque ink and cannot win the first pick.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for connect surface-fitted point visibility.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Connect the two-axis surface fit and the center occlusion check prepared in the preceding steps. Use the same function in color and cropped integer picking. Keep the section and clipping checks, and preserve hidden-ink alpha through translucent faces. The camera still uses the declared forward-depth conversion; ray-facing discs and multisampling remain later steps.

Original point → projected disc → surface fit and center check → color and cropped owner pick..



Why must the center test run in integer picking too?

A visible fringe of a point whose center is behind a face must not win a pick. The same disc predicate keeps color and ownership consistent.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a point on a sloping face with a hidden point. Pick before drawing any color frame, repeat at both display scales, and verify that the original point owner wins only when visible.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.2.10
npm --prefix ../session_tests run course -- save 20.2.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Connect current disc-plane and center predicates to original point color and first picking. Retain the explicit camera conversion and separate section image until their dedicated bridges.

Chrome acceptance for shared vertex reads, original normal pixels, placed instances and selection is pending. Integer primitive readback has separate GPU fixture evidence.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.2.10
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
