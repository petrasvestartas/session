@group(1) @binding(0) var<uniform> pick: mat4x4<f32>;

fn plane_vertex(position: vec4<f32>, uv: vec2<f32>, color: vec4<f32>, clip: vec4<f32>,
                object: u32, selected: f32, plate: f32) -> Vertex {
    var out: Vertex;
    out.original = position;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    out.object = object; out.selected = selected; out.plate = plate;
    return out;
}

@vertex
fn vs_main(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
           @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
           @location(4) object: u32, @location(5) selected: f32, @location(6) plate: f32) -> Vertex {
    return plane_vertex(position, uv, color, clip, object, selected, plate);
}

@vertex
fn vs_id(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
         @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
         @location(4) object: u32, @location(5) selected: f32, @location(6) plate: f32) -> Vertex {
    var out = plane_vertex(position, uv, color, clip, object, selected, plate);
    out.position = pick * position;
    return out;
}
