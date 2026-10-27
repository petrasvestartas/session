# 22.5.5 · Generate ink shaders for physical samples

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 29–57 minutes.** [Estimate](typing-load.md).

Make finite-ink visibility read the same physical sample that produces its colour. Carry an explicit sample argument through the existing functions, generate the four-sample inputs when requested, and keep current colour and query owners at one sample until their layouts are connected.

## Type

Continue from [Reuse surface pipelines for each sample count](96zzzzzzx-reuse-count-specific-surface-pipelines.md). [Save or recover your work](recovery.md).

### 1. `src/ink_plane.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,hit.x)));
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis)->bool{
 let fringe=ink_primitive(pixel,0u);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),0u);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
 }
 return false;
```

</details>

Replace it with:

```wgsl
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,hit.x)));
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis,sample:u32)->bool{
 let fringe=ink_primitive(pixel,sample);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),sample);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
 }
 return false;
```

<details>
<summary>Complete updated block reference</summary>

```wgsl
const DEPTH_REL_TOL:f32=1.9073486e-6;
const SLOPE_PX:f32=0.00390625;
const KINK:f32=0.03125;
const SLOPE_INVALID:f32=0.99951171875;
struct InkAxis{at:vec2<f32>,depth:f32,along:vec2<f32>,slope:f32,sag:vec2<f32>}
fn ink_depth(pixel:vec2<f32>,sample:u32)->f32{
    let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));
    if(any(at<vec2<i32>(0))||any(at>=size)){return 0.;}
    return textureLoad(finite_depth,at,0);
}
fn ink_sag_slack(sag: vec2<f32>, gradient: vec2<f32>) -> f32 {
    return sag.x + sag.y * (abs(gradient.x) + abs(gradient.y));
}
fn ink_tolerance(depth: f32, slope: f32, lever: f32) -> f32 {
    return abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (1.0 + lever);
}
fn ink_carry_visible(predicted: f32, z: f32, depth: f32, tolerance: f32) -> bool {
    if (z > depth + abs(depth) * DEPTH_REL_TOL) {
        return abs(predicted - depth) <= tolerance;
    }
    return predicted <= depth + tolerance;
}
fn ink_pair_planar(pixel: vec2<f32>, dir: vec2<f32>, z: f32, sample: u32) -> bool {
    let z_side = ink_depth(pixel + dir, sample);
    if (z_side == 0.0) {
        return false;
    }
    let z_far = ink_depth(pixel + dir * 2.0, sample);
    if (z_far == 0.0) {
        return true;
    }
    let g = z_side - z;
    let g_far = z_far - z_side;
    return abs(g_far - g) <= abs(z) * DEPTH_REL_TOL + KINK * (abs(g) + abs(g_far));
}
fn ink_step(pixel: vec2<f32>, axis: InkAxis) -> vec2<f32> {
    let perp = vec2<f32>(-axis.along.y, axis.along.x);
    var step = vec2<f32>(sign(perp.x), 0.0);
    if (abs(perp.y) > abs(perp.x)) {
        step = vec2<f32>(0.0, sign(perp.y));
    }
    return select(step, -step, dot(pixel - axis.at, step) < 0.0);
}
fn ink_axis_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let step = ink_step(pixel, axis);
    var side = step;
    if (!ink_pair_planar(pixel, side, z, sample)) {
        side = -step;
        if (!ink_pair_planar(pixel, side, z, sample)) {
            return z <= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
        }
    }
    let z_side = ink_depth(pixel + side, sample);
    let e = axis.at - pixel;
    let det = axis.along.x * side.y - axis.along.y * side.x;
    let a = (e.x * side.y - e.y * side.x) / det;
    let b = (axis.along.x * e.y - axis.along.y * e.x) / det;
    let g = z_side - z;
    let predicted = z + a * axis.slope + b * g;
    let tolerance = ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b)) + ink_sag_slack(axis.sag, vec2<f32>(g, axis.slope));
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
fn ink_disc_fragment_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let d = pixel - centre;
    var along_x = vec2<f32>(select(1.0, -1.0, d.x < 0.0), 0.0);
    var along_y = vec2<f32>(0.0, select(1.0, -1.0, d.y < 0.0));
    if (!ink_pair_planar(pixel, along_x, z, sample)) {
        along_x = -along_x;
        if (!ink_pair_planar(pixel, along_x, z, sample)) {
            return z <= depth + abs(depth) * DEPTH_REL_TOL;
        }
    }
    if (!ink_pair_planar(pixel, along_y, z, sample)) {
        along_y = -along_y;
        if (!ink_pair_planar(pixel, along_y, z, sample)) {
            return z <= depth + abs(depth) * DEPTH_REL_TOL;
        }
    }
    let gx = (ink_depth(pixel + along_x, sample) - z) * along_x.x;
    let gy = (ink_depth(pixel + along_y, sample) - z) * along_y.y;
    let predicted = z - d.x * gx - d.y * gy;
    return ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}
fn ink_disc_source_hidden(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return false;
    }
    let d = pixel - centre;
    for (var x = 0u; x < 2u; x++) {
        let dx = vec2<f32>(select(1.0, -1.0, x == 1u), 0.0);
        if (!ink_pair_planar(pixel, dx, z, sample)) {
            continue;
        }
        let gx = (ink_depth(pixel + dx, sample) - z) * dx.x;
        for (var y = 0u; y < 2u; y++) {
            let dy = vec2<f32>(0.0, select(1.0, -1.0, y == 1u));
            if (!ink_pair_planar(pixel, dy, z, sample)) {
                continue;
            }
            let gy = (ink_depth(pixel + dy, sample) - z) * dy.y;
            let diagonal = ink_depth(pixel + dx + dy, sample);
            let expected = z + gx * dx.x + gy * dy.y;
            if (diagonal == 0.0 || abs(diagonal - expected) > ink_tolerance(z, abs(gx) + abs(gy), 2.0)) {
                continue;
            }
            let predicted = z - d.x * gx - d.y * gy;
            if (predicted > depth + ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y))) {
                return true;
            }
        }
    }
    return false;
}
fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    if (!ink_disc_fragment_visible(pixel, centre, depth, sample)) {
        return false;
    }
    let source_pixel = floor(centre) + fract(pixel);
    return !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}
fn ink_primitive(pixel:vec2<f32>,sample:u32)->u32{
    let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_primitive));
    if(any(at<vec2<i32>(0))||any(at>=size)){return 0u;}
    let halves=textureLoad(finite_primitive,at,0).xy;
    return physical_address(halves);
}
fn ink_visible_plane(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let primitive = ink_primitive(pixel, sample);
    var gradient = vec2<f32>(0.0);
    if (clip_is_cap(primitive)) {
        let plane = clip_cap_plane(primitive);
        if (plane >= clipping.count) {
            return ink_axis_visible(pixel, axis, sample);
        }
        gradient = -clip_plane_slope(plane, view.size);
    } else if (primitive != 0u) {
        if (primitive > arrayLength(&finite_projected)) {
            return ink_axis_visible(pixel, axis, sample);
        }
        let triangle = primitive - 1u;
        if (finite_projected[triangle].edge3.w < 3.0) {
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

fn finite_ink_axis(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>)->InkAxis{
 let delta=ends.zw-ends.xy;let len=length(delta);let along=delta/max(len,1e-6);
 let distance=clamp(dot(pixel-ends.xy,along),0.,len);let at=ends.xy+along*distance;
 let depth=select(depths.x,(depths.x*(len-distance)+depths.y*distance)/max(len,1e-6),len>1e-6);
 return InkAxis(at,depth,along,(depths.y-depths.x)/max(len,1e-3),vec2<f32>(0.));
}
fn ink_exact_floor(axis:InkAxis,triangle:ProjectedTriangle)->f32{
 return axis.depth+abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7+ink_sag_slack(axis.sag,triangle.gradient.xy);
}
fn finite_triangle_hides(primitive:u32,axis:InkAxis)->bool{
 if(primitive==0u||primitive>arrayLength(&finite_projected)){return false;}
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,hit.x)));
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis,sample:u32)->bool{
 let fringe=ink_primitive(pixel,sample);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),sample);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
 }
 return false;
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-03.rs"
```

</details>

### 2. `src/finite_visibility.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

<details>
<summary>Locate the existing block</summary>

```wgsl
const FINITE_GRID:bool=false;
@group(1) @binding(0) var<storage,read> finite_projected:array<ProjectedTriangle>;
@group(1) @binding(1) var<storage,read> finite_tiles:array<vec4<u32>>;
@group(1) @binding(2) var finite_depth:texture_depth_2d;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
 let primitive=ink_primitive(pixel,0u);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool,sag:vec2<f32>)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags,sag);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32,sag:vec2<f32>)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 var axis=finite_ink_axis(pixel,ends,depths);axis.sag=sag;let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,0u))){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,0u);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis)){return false;}
 if(plane_visible){return true;}
 if(any(axis.at<vec2<f32>(0.))||any(axis.at>=view.size)){return false;}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size_px=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(axis.at/span);let head=finite_tiles[1u+cell.y*size_px.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return false;}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);
  if(nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let bounds=finite_projected[primitive-1u].bounds;
  if(any(axis.at<bounds.xy-vec2<f32>(SLOPE_PX))||any(axis.at>bounds.zw+vec2<f32>(SLOPE_PX))){continue;}
  if(finite_triangle_hides(primitive,axis)){return false;}
 }
 return true;
}

fn finite_hidden_alpha(hidden:bool,picking:bool)->f32{
 if(picking){return select(1.,0.,hidden);}
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
const FINITE_GRID:bool=false;
@group(1) @binding(0) var<storage,read> finite_projected:array<ProjectedTriangle>;
@group(1) @binding(1) var<storage,read> finite_tiles:array<vec4<u32>>;
@group(1) @binding(2) var finite_depth:texture_depth_2d;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32,sample:u32)->bool{
 let primitive=ink_primitive(pixel,sample);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32,sample:u32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,sample:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative,sample)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative,sample);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative,sample);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative,sample);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool,sag:vec2<f32>,sample:u32)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags,sag,sample);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool,sample:u32)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance,sample)||!ink_disc_visible(pixel,centre,depth,sample);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32,sag:vec2<f32>,sample:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags,sample);}
 var axis=finite_ink_axis(pixel,ends,depths);axis.sag=sag;let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance,sample)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,sample);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis,sample)){return false;}
 if(plane_visible){return true;}
 if(any(axis.at<vec2<f32>(0.))||any(axis.at>=view.size)){return false;}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size_px=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(axis.at/span);let head=finite_tiles[1u+cell.y*size_px.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return false;}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);
  if(nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let bounds=finite_projected[primitive-1u].bounds;
  if(any(axis.at<bounds.xy-vec2<f32>(SLOPE_PX))||any(axis.at>bounds.zw+vec2<f32>(SLOPE_PX))){continue;}
  if(finite_triangle_hides(primitive,axis)){return false;}
 }
 return true;
}

fn finite_hidden_alpha(hidden:bool,picking:bool)->f32{
 if(picking){return select(1.,0.,hidden);}
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-21.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-22.rs"
```

</details>

### 3. `src/finite_stroke.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

<details>
<summary>Locate the existing block</summary>

```wgsl
#include "near_clip.wgsl"
#include "contact_depth.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32, display:vec4<f32>, eye:vec4<f32> }
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    let s=stroke_quad(a,b,width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,flags,s.axis,s.depths,s.pixel,rise);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) metadata:InkRecord) -> Output { return geometry(start, end, colour, width, index,metadata); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
#include "contact_depth.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32, display:vec4<f32>, eye:vec4<f32> }
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    let s=stroke_quad(a,b,width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,flags,s.axis,s.depths,s.pixel,rise);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) metadata:InkRecord) -> Output { return geometry(start, end, colour, width, index,metadata); }
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.),sample);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-23.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-24.rs"
```

</details>

### 4. `src/finite_chain.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(start, end, colour, width, previous, next, heads, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

</details>

Replace it with:

```wgsl
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(start, end, colour, width, previous, next, heads, index,flags); }
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.),sample);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

<details>
<summary>Complete updated block reference</summary>

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
    @location(5) @interpolate(flat) part: u32,
    @location(6) @interpolate(flat) head_length: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
fn screen(p: vec4<f32>) -> vec2<f32> { return p.xy / p.w * view.size * 0.5; }
fn perpendicular(d: vec2<f32>) -> vec2<f32> { return vec2<f32>(-d.y, d.x); }
fn joint(neighbour: vec4<f32>, endpoint: vec4<f32>, d: vec2<f32>, before: bool) -> vec2<f32> {
    let normal = perpendicular(d);
    if (near_distance(neighbour) < 0.0 || neighbour.w <= 0.0) { return normal; }
    let span = select(screen(neighbour) - screen(endpoint), screen(endpoint) - screen(neighbour), before);
    if (dot(span, span) < 0.000001) { return normal; }
    let other = normalize(span); let sum = normal + perpendicular(other);
    if (dot(sum, sum) < 0.000001) { return normal; }
    let bisector = normalize(sum);
    return bisector / max(dot(bisector, normal), 0.25);
}
fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, previous: vec3<f32>, next: vec3<f32>, heads: u32, index: u32,flags:u32) -> Output {
    var hidden: Output; hidden.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let clipped_a = near_distance(a) < 0.0; let clipped_b = near_distance(b) < 0.0;
    if (clipped_a && clipped_b) { return hidden; }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden; }
    let delta = screen(b) - screen(a);
    if (dot(delta, delta) < 0.000001) { return hidden; }
    let direction = normalize(delta); let normal = perpendicular(direction);
    let back = view.transform * vec4<f32>(previous, 1.0);
    let front = view.transform * vec4<f32>(next, 1.0);
    let left = select(joint(back, a, direction, true), normal, clipped_a);
    let right = select(joint(front, b, direction, false), normal, clipped_b);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index % 6u]; let half_width = width * view.density * 0.5;
    let at_start = (heads & 2u) != 0u && !clipped_a;
    let at_end = (heads & 1u) != 0u && !clipped_b;
    let share = select(1.0, 2.0, at_start && at_end);
    let head_length = min(max(10.0 * view.density, 6.0 * half_width), length(delta) * 0.6 / share);
    let part = index / 6u;
    if (part > 0u) {
        let is_start = part == 2u;
        if ((is_start && !at_start) || (!is_start && !at_end)) { return hidden; }
        let tip = select(b, a, is_start); let aim = select(direction, -direction, is_start);
        let offset = -aim * mix(-0.70711, head_length + 0.70711, corner.x)
            + perpendicular(aim) * (head_length * 0.4 + 0.70711) * corner.y;
        let position = vec4<f32>(tip.xy + offset * 2.0 / view.size * tip.w, tip.zw);
        return Output(position, colour, screen(position), screen(tip), aim, half_width, 1u, head_length,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
    }
    let start_offset = direction * select(0.0, head_length, at_start);
    let end_offset = -direction * select(0.0, head_length, at_end);
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + select(start_offset, end_offset, corner.x > 0.5)
        * 2.0 / view.size * position.w, position.zw);
    let offset = select(left, right, corner.x > 0.5) * (half_width + 0.70711) * corner.y;
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, screen(position), screen(a), direction, half_width, 0u, 0.0,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
}
#include "chain_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(start, end, colour, width, previous, next, heads, index,flags); }
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.),sample);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-25.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-26.rs"
```

</details>

### 5. `src/finite_marker.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, radius,flags,axis,depths);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.position.xy,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input.colour,input.radius,input.position.xy-input.finite_axis_value.xy);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, radius,flags,axis,depths);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.position.xy,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false,sample);if(alpha<=0.){discard;}let c=colour(input.colour,input.radius,input.position.xy-input.finite_axis_value.xy);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-27.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-28.rs"
```

</details>

### 6. `src/finite_stroke_pick.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

<details>
<summary>Locate the existing block</summary>

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.))<=0.){discard;}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.),0u)<=0.){discard;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-29.rs"
```

</details>

### 7. `src/finite_chain_pick.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

<details>
<summary>Locate the existing block</summary>

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(0.))<=0.){discard;}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(0.),0u)<=0.){discard;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-30.rs"
```

</details>

### 8. `src/finite_marker_pick.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

<details>
<summary>Locate the existing block</summary>

```wgsl
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true,0u)<=0.){discard;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-31.rs"
```

</details>

### 9. `src/finite_sources.rs`

Generate only four-sample actual depth/address access and sample-qualified coverage; retain the original one-sample marker position.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source(stride:u64,pick:bool)->String{source_style(stride,pick,false)}
pub fn source_style(stride:u64,pick:bool,grid:bool)->String{source_metadata(stride,pick,grid,false)}
pub fn source_metadata(stride:u64,pick:bool,grid:bool,varying:bool)->String{
 let(base,ids)=match stride{44=>(include_str!("finite_stroke.wgsl"),include_str!("finite_stroke_pick.wgsl")),72=>(include_str!("finite_chain.wgsl"),include_str!("finite_chain_pick.wgsl")),32=>(include_str!("finite_marker.wgsl"),include_str!("finite_marker_pick.wgsl")),_=>panic!("Unsupported finite ink layout")};
 let base=crate::finite_project_source::expand(base).replace("#include \"finite_visibility.wgsl\"",include_str!("finite_visibility.wgsl")).replace("#include \"ink_plane.wgsl\"",include_str!("ink_plane.wgsl"));
 let base=if grid{base.replace("const FINITE_GRID:bool=false;","const FINITE_GRID:bool=true;")}else{base};
 let(ty,words)=if varying{("vec2<u32>","m")}else{("u32","vec2<u32>(m,0x80008000u)")};
 let base=format!("alias InkRecord={ty};\nfn ink_words(m:InkRecord)->vec2<u32>{{return {words};}}\n{base}");
 let source=if pick{format!("{}\n{}\n{ids}",base.replace("@group(1)","@group(2)").replace("return vec2<i32>(pixel);","return vec2<i32>(pixel)-vec2<i32>(i32(crop._a),i32(crop._b));"),include_str!("pick_crop.wgsl"))}else{base};crate::clipping_gpu::source(&source,if pick{3}else{2},0)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source(stride:u64,pick:bool)->String{source_style(stride,pick,false)}
pub fn source_style(stride:u64,pick:bool,grid:bool)->String{source_metadata(stride,pick,grid,false)}
pub fn source_metadata(stride:u64,pick:bool,grid:bool,varying:bool)->String{source_with_samples(stride,pick,grid,varying,1)}
pub fn source_with_samples(stride:u64,pick:bool,grid:bool,varying:bool,samples:u32)->String{
 assert!(matches!(samples,1|4)&&(!pick||samples==1));
 let(base,ids)=match stride{44=>(include_str!("finite_stroke.wgsl"),include_str!("finite_stroke_pick.wgsl")),72=>(include_str!("finite_chain.wgsl"),include_str!("finite_chain_pick.wgsl")),32=>(include_str!("finite_marker.wgsl"),include_str!("finite_marker_pick.wgsl")),_=>panic!("Unsupported finite ink layout")};
 let base=crate::finite_project_source::expand(base).replace("#include \"finite_visibility.wgsl\"",include_str!("finite_visibility.wgsl")).replace("#include \"ink_plane.wgsl\"",include_str!("ink_plane.wgsl"));
 let base=if samples==4{multisampled(base,stride)}else{base};
 let base=if grid{base.replace("const FINITE_GRID:bool=false;","const FINITE_GRID:bool=true;")}else{base};
 let(ty,words)=if varying{("vec2<u32>","m")}else{("u32","vec2<u32>(m,0x80008000u)")};
 let base=format!("alias InkRecord={ty};\nfn ink_words(m:InkRecord)->vec2<u32>{{return {words};}}\n{base}");
 let source=if pick{format!("{}\n{}\n{ids}",base.replace("@group(1)","@group(2)").replace("return vec2<i32>(pixel);","return vec2<i32>(pixel)-vec2<i32>(i32(crop._a),i32(crop._b));"),include_str!("pick_crop.wgsl"))}else{base};crate::clipping_gpu::source(&source,if pick{3}else{2},0)
}

fn multisampled(mut source:String,stride:u64)->String {
    source=source.replace("var finite_depth:texture_depth_2d;","var finite_depth:texture_depth_multisampled_2d;")
        .replace("var finite_primitive:texture_2d<u32>;","var finite_primitive:texture_multisampled_2d<u32>;")
        .replace("textureLoad(finite_depth,at,0)","textureLoad(finite_depth,at,i32(sample))")
        .replace("textureLoad(finite_primitive,at,0)","textureLoad(finite_primitive,at,i32(sample))")
        .replace("@interpolate(linear) finite_pixel:","@interpolate(linear,sample) finite_pixel:")
        .replace("@interpolate(linear) pixel:","@interpolate(linear,sample) pixel:");
    if stride==32 {
        source=source.replace("@location(10) @interpolate(flat) finite_depths:vec2<f32>,",
            "@location(10) @interpolate(flat) finite_depths:vec2<f32>,\n    @location(11) @interpolate(linear,sample) finite_pixel:vec2<f32>,")
            .replace("colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0))", "colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0))")
            .replace("Output(position, colour, radius,flags,axis,depths)","Output(position, colour, radius,flags,axis,depths,finite_screen(position))")
            .replace("input.position.xy","input.finite_pixel");
    }
    source
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-34.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/tile_shader_tests.rs`

Retain every original compute/colour/query shader recipe and validate both counts across all six finite-ink styles with the same parser and full Naga assertions.

<details>
<summary>Locate the existing block</summary>

```text
for stride in [44,72,32]{sources.push(("finite color",crate::finite_sources::source(stride,false)));sources.push(("finite pick",crate::finite_sources::source(stride,true)));}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-35.rs"
```

</details>

### 11. `src/finite_ink.rs`

Carry the requested count from the pipeline descriptor into its shader recipe. Assemble the shader for the same count as the pipeline. Use the explicit count in the finite-ink pipeline descriptor. Keep current colour construction single-sample until the owner is connected. Keep the independent integer query factory single-sample.

<details>
<summary>Locate the existing block</summary>

```rust
fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,picking:bool,varying:bool,view:&wgpu::BindGroupLayout,crop:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,attributes:&[wgpu::VertexAttribute])->wgpu::RenderPipeline{
   let source=crate::finite_sources::source_metadata(stride,picking,grid,varying);let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite original ink"),source:wgpu::ShaderSource::Wgsl(source.into())});
   let clipping=crate::clipping_gpu::layout(device);let groups=if picking{vec![Some(view),Some(crop),Some(layout),Some(&clipping)]}else{vec![Some(view),Some(layout),Some(&clipping)]};let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite original ink"),bind_group_layouts:&groups,immediate_size:0});
   let mut buffers=vec![wgpu::VertexBufferLayout{array_stride:stride,step_mode:wgpu::VertexStepMode::Instance,attributes}];let ids=wgpu::vertex_attr_array![7=>Uint32x2];if picking{buffers.push(wgpu::VertexBufferLayout{array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ids});}
   let flags=[wgpu::VertexAttribute{format:if varying{wgpu::VertexFormat::Uint32x2}else{wgpu::VertexFormat::Uint32},offset:0,shader_location:8}];buffers.push(wgpu::VertexBufferLayout{array_stride:if varying{8}else{0},step_mode:wgpu::VertexStepMode::Instance,attributes:&flags});
   let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
   device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("finite original ink"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(if picking{"pick_vertex"}else{"vertex"}),compilation_options:Default::default(),buffers:&buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(if picking{"pick_fragment"}else{"fragment"}),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:if picking{wgpu::TextureFormat::Rg32Uint}else{format},blend:if picking{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(depth),multisample:Default::default(),multiview_mask:None,cache:None})
  }
use std::cell::RefCell;
pub struct Ink{color:wgpu::RenderPipeline,pick:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,bindings:RefCell<Option<(wgpu::Buffer,wgpu::Buffer,wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)>>}
#[derive(Clone)]
pub struct Tables{pub projected:wgpu::Buffer,pub tiles:wgpu::Buffer,pub primitive:wgpu::TextureView}
impl Ink{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool)->Self{
  let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
  let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("finite ink coverage"),entries:&[crate::tile_pipelines::entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),wgpu::BindGroupLayoutEntry{binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture{sample_type:wgpu::TextureSampleType::Depth,view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},crate::text_gpu_support::texture_entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Uint,false)]});
let color=pipeline(device,format,stride,grid,false,varying,&view,&crop,&layout,&attributes);let pick=pipeline(device,format,stride,grid,true,varying,&view,&crop,&layout,&attributes);Self{color,pick,layout,bindings:RefCell::new(None)}
 }
 fn group(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite original ink coverage"),layout:&self.layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:tables.projected.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:tables.tiles.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&tables.primitive)}]})}
 pub fn set(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView){let mut bindings=self.bindings.borrow_mut();if bindings.as_ref().is_some_and(|(p,t,d,i,_)|p==&tables.projected&&t==&tables.tiles&&d==depth&&i==&tables.primitive){return}let group=self.group(device,tables,depth);*bindings=Some((tables.projected.clone(),tables.tiles.clone(),depth.clone(),tables.primitive.clone(),group));}
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32)->bool{let bindings=self.bindings.borrow();let Some((_,_,_,_,group))=bindings.as_ref()else{return false};pass.set_pipeline(&self.color);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,flags.slice(..));pass.draw(0..verts,0..count);true}
 pub fn pick(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,vertices:&wgpu::Buffer,ids:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,tables:&Tables,depth:&wgpu::TextureView){let group=self.group(device,tables,depth);pass.set_pipeline(&self.pick);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,crop,&[]);pass.set_bind_group(2,&group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
fn pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    stride: u64,
    grid: bool,
    picking: bool,
    varying: bool,
    samples: u32,
    view: &wgpu::BindGroupLayout,
    crop: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
    attributes: &[wgpu::VertexAttribute],
) -> wgpu::RenderPipeline {
    let source =
        crate::finite_sources::source_with_samples(stride, picking, grid, varying, samples);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("finite original ink"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let clipping = crate::clipping_gpu::layout(device);
    let groups = if picking {
        vec![Some(view), Some(crop), Some(layout), Some(&clipping)]
    } else {
        vec![Some(view), Some(layout), Some(&clipping)]
    };
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("finite original ink"),
        bind_group_layouts: &groups,
        immediate_size: 0,
    });
    let mut buffers = vec![wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes,
    }];
    let ids = wgpu::vertex_attr_array![7=>Uint32x2];
    if picking {
        buffers.push(wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ids,
        });
    }
    let flags = [wgpu::VertexAttribute {
        format: if varying {
            wgpu::VertexFormat::Uint32x2
        } else {
            wgpu::VertexFormat::Uint32
        },
        offset: 0,
        shader_location: 8,
    }];
    buffers.push(wgpu::VertexBufferLayout {
        array_stride: if varying { 8 } else { 0 },
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &flags,
    });
    let mut depth = crate::depth::ink();
    depth.depth_compare = Some(wgpu::CompareFunction::Always);
    depth.bias = Default::default();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("finite original ink"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(if picking { "pick_vertex" } else { "vertex" }),
            compilation_options: Default::default(),
            buffers: &buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(if picking { "pick_fragment" } else { "fragment" }),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: if picking {
                    wgpu::TextureFormat::Rg32Uint
                } else {
                    format
                },
                blend: if picking {
                    None
                } else {
                    Some(wgpu::BlendState::ALPHA_BLENDING)
                },
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: Some(depth),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}
use std::cell::RefCell;
pub struct Ink{color:wgpu::RenderPipeline,pick:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,bindings:RefCell<Option<(wgpu::Buffer,wgpu::Buffer,wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)>>}
#[derive(Clone)]
pub struct Tables{pub projected:wgpu::Buffer,pub tiles:wgpu::Buffer,pub primitive:wgpu::TextureView}
impl Ink{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool)->Self{
  let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
  let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("finite ink coverage"),entries:&[crate::tile_pipelines::entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),wgpu::BindGroupLayoutEntry{binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture{sample_type:wgpu::TextureSampleType::Depth,view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},crate::text_gpu_support::texture_entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Uint,false)]});
let color=pipeline(device,format,stride,grid,false,varying,1,&view,&crop,&layout,&attributes);let pick=pipeline(device,format,stride,grid,true,varying,1,&view,&crop,&layout,&attributes);Self{color,pick,layout,bindings:RefCell::new(None)}
 }
 fn group(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite original ink coverage"),layout:&self.layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:tables.projected.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:tables.tiles.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&tables.primitive)}]})}
 pub fn set(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView){let mut bindings=self.bindings.borrow_mut();if bindings.as_ref().is_some_and(|(p,t,d,i,_)|p==&tables.projected&&t==&tables.tiles&&d==depth&&i==&tables.primitive){return}let group=self.group(device,tables,depth);*bindings=Some((tables.projected.clone(),tables.tiles.clone(),depth.clone(),tables.primitive.clone(),group));}
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32)->bool{let bindings=self.bindings.borrow();let Some((_,_,_,_,group))=bindings.as_ref()else{return false};pass.set_pipeline(&self.color);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,flags.slice(..));pass.draw(0..verts,0..count);true}
 pub fn pick(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,vertices:&wgpu::Buffer,ids:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,tables:&Tables,depth:&wgpu::TextureView){let group=self.group(device,tables,depth);pass.set_pipeline(&self.pick);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,crop,&[]);pass.set_bind_group(2,&group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-36.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-37.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-38.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-39.rs"
```

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-40.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Finite-ink functions carry an explicit sample, and generated four-sample shaders use matching actual coordinates, depth and primitive addresses. The pipeline descriptor receives the same count. Current colour owners and independent query owners remain single-sample until the next wiring step.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample browser contact fixture after connecting explicit finite-ink sample arguments and shader/pipeline counts. Both densities preserve original face picks, complete sources and archives, shared geometry, contact shading and exact resize restoration. Generated four-sample shaders are validated separately; matching ink-owner resources are connected next. Whole-course visual acceptance remains unfinished.

![Actual single-sample browser contact fixture after connecting explicit finite-ink sample arguments and shader/pipeline counts. Both densities preserve original face picks, complete sources and archives, shared geometry, contact shading and exact resize restoration. Generated four-sample shaders are validated separately; matching ink-owner resources are connected next. Whole-course visual acceptance remains unfinished.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The visibility functions already use original primitive addresses and analytic source planes. Pass the sample through section, physical-depth, grid, line and disc visibility, including nearby original-address checks. Independent integer queries pass sample zero. No depth or primitive values are averaged.

Keep one original shader source. At four samples, assemble multisampled depth and primitive declarations and read the requested sample. Sample-qualified pixel varyings provide the actual rasterization coordinates for both visibility and coverage. The marker keeps its existing builtin position at one sample and receives the extra pixel varying only in its four-sample form. No fixed sample offsets are inserted.

The pipeline factory receives the same count as its assembled shader. Current owners still pass one because their binding layouts and cached resources are connected in the following work. The unchanged parser and validator assertions now cover21 assembled recipes: three compute modules and six finite-ink styles with one- and four-sample colour plus independent single-sample queries.

Explicit sample argument → matching depth and original primitive reads → sample-qualified pixel coordinates → count-specific shader and pipeline descriptor.



Why does a sample index alone fail to establish correct per-sample ink?

The interpolated coverage coordinates, physical depth and original primitive address must refer to that same sample. A fragment-centre coordinate can disagree with a sample-qualified varying, so use the actual sample interpolation rather than fixed offsets.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the supplied assembled-shader validator across all21 recipes and the original line, chain, point, sloped-face and grid regressions. Compare generated one- and four-sample marker declarations: only the four-sample form adds the sample-qualified pixel varying. Follow the explicit sample from the fragment through every original-address and physical-depth read.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.5
npm --prefix ../session_tests run course -- save 22.5.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

An explicit sample argument now connects original primitive addresses, physical depth, analytic source-plane visibility and actual coverage coordinates. The four-sample source uses matching multisampled reads and sample-qualified pixel varyings; the single-sample marker keeps its existing builtin position. No fixed sample offsets or averaged integer/depth values are introduced. The finite-ink pipeline descriptor receives the same count as its assembled shader. Current colour and query owners still construct one-sample resources until their matching layouts are connected. The existing parser and full Naga assertions retain every original input family and validate 21 recipes across six styles. All 993 Native tests, WASM/optimized builds, native frame, canonical Chrome original-contact/resize checks and stored evidence pass. Ten fresh canonical Native/browser pictures were individually inspected. Nine shader/assembly files and the complete pipeline factory exactly match the separately witnessed private Native/Chrome components; that retained component evidence does not claim complete four-sample rendering in this smaller application. Complete four-sample ink-owner and viewer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

Retain the existing WGSL parser and full Naga validator assertions and all original shader input families. Validate21 assembled recipes across six finite-ink styles, keep integer queries at one sample, and retain actual original colour/ownership/sloped-face/grid regressions.

Actual single-sample browser contact fixture after connecting explicit finite-ink sample arguments and shader/pipeline counts. Both densities preserve original face picks, complete sources and archives, shared geometry, contact shading and exact resize restoration. Generated four-sample shaders are validated separately; matching ink-owner resources are connected next. Whole-course visual acceptance remains unfinished.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
