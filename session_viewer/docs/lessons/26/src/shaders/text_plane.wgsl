// --8<-- [start:plane-vertex]
// What the vertex shader hands the fragment shader.
struct Vertex {
    @builtin(position) position: vec4<f32>, // clip position
    @location(0) uv: vec2<f32>, // texture coordinate, 0..1 across the label's texture
    @location(1) color: vec4<f32>, // ink color
    @location(2) @interpolate(flat) clip: vec4<f32>, // screen box the label is cut to
    @location(3) @interpolate(flat) object: u32, // object row + 1, or 0
    @location(4) @interpolate(flat) selected: f32, // 1 when selected
}

// A sampler reads a texture between its pixels; linear blends the four nearest, so a tilted label stays smooth.
@group(0) @binding(0) var coverage_texture: texture_2d<f32>; // rasterized glyphs
@group(0) @binding(1) var coverage_sampler: sampler;

// Canvas clip space to the pick window; id pass only.
@group(1) @binding(0) var<uniform> pick: mat4x4<f32>;

// Pack the vertex attributes.
fn plane_vertex(position: vec4<f32>, uv: vec2<f32>, color: vec4<f32>, clip: vec4<f32>,
                object: u32, selected: f32) -> Vertex {
    var out: Vertex;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    out.object = object; out.selected = selected;
    return out;
}

@vertex
// Vertices arrive already in clip space.
fn vs_main(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
           @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
           @location(4) object: u32, @location(5) selected: f32) -> Vertex {
    return plane_vertex(position, uv, color, clip, object, selected);
}

@vertex
// The same, mapped into the pick window.
fn vs_id(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
         @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
         @location(4) object: u32, @location(5) selected: f32) -> Vertex {
    return plane_vertex(pick * position, uv, color, clip, object, selected);
}
// --8<-- [end:plane-vertex]

// --8<-- [start:plane-fragment]
// Signed distance to the rounded plate edge, in texture pixels.
// The same rounded box as text_plate.wgsl, sized from the texture itself.
fn plate_distance(uv: vec2<f32>) -> f32 {
    let half_size = vec2<f32>(textureDimensions(coverage_texture)) * 0.5;
    let radius = min(half_size.x, half_size.y);
    let q = abs((uv * 2.0 - 1.0) * half_size) - half_size + radius;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

// Glyph color over a black plate, yellow when selected.
@fragment
fn fs_main(in: Vertex) -> @location(0) vec4<f32> {
    // outside the clip box
    // in a fragment shader, position.xy is the pixel's own coordinate in framebuffer pixels
    if (in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) {
        discard;
    }

    // coverage: 0 = no glyph here, 1 = fully inside a letter
    let coverage = textureSample(coverage_texture, coverage_sampler, in.uv).r;
    let distance = plate_distance(in.uv);
    let plate_coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    let background = vec3<f32>(in.selected, in.selected, 0.0);
    return vec4<f32>(mix(background, in.color.rgb, coverage), in.color.a * plate_coverage);
}

@fragment
// Object id inside the plate.
fn fs_id(in: Vertex) -> @location(0) vec2<u32> {
    if (in.object == 0u || in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) {
        discard;
    }

    if (plate_distance(in.uv) > 0.0) {
        discard;
    }

    return vec2<u32>(in.object, 0u);
}
// --8<-- [end:plane-fragment]
