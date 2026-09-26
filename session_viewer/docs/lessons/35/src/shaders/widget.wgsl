// --8<-- [start:widget-uniform]
// Gumball uniform, 96 bytes; matches the Rust array.
struct Widget {
    mvp: mat4x4<f32>, // gumball to tile clip space
    rect: vec4<f32>, // where the tile lands on the canvas: x, y, w, h
    settings: vec4<f32>, // x: active handle, -1 = none
};

@group(0) @binding(0) var<uniform> widget: Widget; // placement
@group(1) @binding(0) var picture: texture_2d<f32>; // the drawn gumball tile
@group(1) @binding(1) var picture_sampler: sampler; // linear sampler
// --8<-- [end:widget-uniform]

// --8<-- [start:widget-mesh-shader]
// What the mesh vertex shader hands the fragment shader.
struct Varying {
    @builtin(position) position: vec4<f32>, // clip position
    @location(0) color: vec3<f32>, // rgb
};

@vertex
// Place a mesh vertex; the active handle turns orange.
fn vs_main(@location(0) position: vec3<f32>, @location(1) color: u32,
           @location(2) handle: u32) -> Varying {
    var out: Varying;
    out.position = widget.mvp * vec4<f32>(position, 1.0);
    out.color = unpack4x8unorm(color).rgb;

    if f32(handle) == widget.settings.x { // e.g. 3 while the X arc is hovered
        out.color = vec3<f32>(0.95, 0.65, 0.08);
    }

    return out;
}

@fragment
// Flat color.
fn fs_main(in: Varying) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
// --8<-- [end:widget-mesh-shader]

// --8<-- [start:widget-composite]
// What the composite vertex shader hands the fragment shader.
struct Composite {
    @builtin(position) position: vec4<f32>, // clip position
    @location(0) uv: vec2<f32>, // texture coordinate
};

@vertex
// One corner of the tile's quad on the canvas.
fn vs_composite(@builtin(vertex_index) index: u32) -> Composite {
    // six corners: two triangles covering the tile's rectangle
    let corners = array<vec2<f32>, 6>(vec2(0., 0.), vec2(1., 0.), vec2(1., 1.),
                                     vec2(0., 0.), vec2(1., 1.), vec2(0., 1.));
    var out: Composite;
    out.uv = corners[index];
    out.position = vec4<f32>(widget.rect.xy + out.uv * widget.rect.zw, 0., 1.);
    return out;
}

@fragment
// Blend the tile over the frame.
fn fs_composite(in: Composite) -> @location(0) vec4<f32> {
    let pixel = textureSample(picture, picture_sampler, in.uv);
    // resolved edge pixels hold colour already multiplied by coverage; divide it out before blending
    return vec4<f32>(pixel.rgb / max(pixel.a, 0.00001), pixel.a);
}
// --8<-- [end:widget-composite]
