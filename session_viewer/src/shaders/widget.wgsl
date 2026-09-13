struct Widget {
    mvp: mat4x4<f32>,
    rect: vec4<f32>,
    settings: vec4<f32>,
};
@group(0) @binding(0) var<uniform> widget: Widget;
@group(1) @binding(0) var picture: texture_2d<f32>;
@group(1) @binding(1) var picture_sampler: sampler;

struct Varying {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};
@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) color: u32,
           @location(2) handle: u32) -> Varying {
    var out: Varying;
    out.position = widget.mvp * vec4<f32>(position, 1.0);
    out.color = unpack4x8unorm(color).rgb;
    if f32(handle) == widget.settings.x {
        out.color = vec3<f32>(0.95, 0.65, 0.08);
    }
    return out;
}
@fragment
fn fs_main(in: Varying) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

struct Composite {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};
@vertex
fn vs_composite(@builtin(vertex_index) index: u32) -> Composite {
    let corners = array<vec2<f32>, 6>(vec2(0., 0.), vec2(1., 0.), vec2(1., 1.),
                                     vec2(0., 0.), vec2(1., 1.), vec2(0., 1.));
    var out: Composite;
    out.uv = corners[index];
    out.position = vec4<f32>(widget.rect.xy + out.uv * widget.rect.zw, 0., 1.);
    return out;
}
@fragment
fn fs_composite(in: Composite) -> @location(0) vec4<f32> {
    let pixel = textureSample(picture, picture_sampler, in.uv);
    return vec4<f32>(pixel.rgb / max(pixel.a, 0.00001), pixel.a);
}
