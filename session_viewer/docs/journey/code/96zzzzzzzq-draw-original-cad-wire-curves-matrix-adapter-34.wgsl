    @location(3) @interpolate(flat) object: u32,
    @location(4) @interpolate(flat) selected: f32,
    @location(5) original: vec4<f32>,
    @location(6) @interpolate(flat) plate: f32,
}

@group(0) @binding(0) var coverage_texture: texture_2d<f32>;
