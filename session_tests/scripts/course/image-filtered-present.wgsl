@group(0) @binding(0) var image: texture_2d<f32>;
@group(0) @binding(1) var linear_sampler: sampler;

struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex fn vertex(@builtin(vertex_index) index: u32) -> Vertex {
    let p = array(vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0))[index];
    return Vertex(vec4(p, 0.0, 1.0), vec2(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5));
}

@fragment fn fragment(v: Vertex) -> @location(0) vec4<f32> {
    return textureSample(image, linear_sampler, v.uv);
}
