// Screen-space annotation vertices retain local physical coordinates for pixel coverage.
struct PlateVertex {
    @builtin(position) position: vec4<f32>,
    @location(0) local: vec2<f32>,
    @location(1) half_size: vec2<f32>,
    @location(2) radius: f32,
    @location(3) @interpolate(flat) object: u32,
    @location(4) @interpolate(flat) selected: f32,
}

// The pick pass renders a window of the canvas into a window-sized attachment: this is the
// clip-space map from the canvas projection the vertices were placed with to that window.
@group(0) @binding(0) var<uniform> pick: mat4x4<f32>;

fn plate_vertex(position: vec4<f32>, local: vec2<f32>, half_size: vec2<f32>, radius: f32,
                object: u32, selected: f32) -> PlateVertex {
    var out: PlateVertex;
    out.position = position;
    out.local = local;
    out.half_size = half_size;
    out.radius = radius;
    out.object = object;
    out.selected = selected;
    return out;
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) local: vec2<f32>,
           @location(2) half_size: vec2<f32>, @location(3) radius: f32, @location(4) object: u32,
           @location(5) selected: f32) -> PlateVertex {
    return plate_vertex(vec4<f32>(position, 1.0), local, half_size, radius, object, selected);
}

@vertex
fn vs_id(@location(0) position: vec3<f32>, @location(1) local: vec2<f32>,
         @location(2) half_size: vec2<f32>, @location(3) radius: f32, @location(4) object: u32,
         @location(5) selected: f32) -> PlateVertex {
    return plate_vertex(pick * vec4<f32>(position, 1.0), local, half_size, radius, object, selected);
}

// One distance defines visible coverage and picking, independent of selection color.
fn plate_distance(in: PlateVertex) -> f32 {
    let radius = min(in.radius, min(in.half_size.x, in.half_size.y));
    let q = abs(in.local) - in.half_size + vec2<f32>(radius);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: PlateVertex) -> @location(0) vec4<f32> {
    let distance = plate_distance(in);
    let coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    return vec4<f32>(in.selected, in.selected, 0.0, coverage);
}

@fragment
fn fs_id(in: PlateVertex) -> @location(0) vec2<u32> {
    if (in.object == 0u || plate_distance(in) > 0.0) { discard; }
    return vec2<u32>(in.object, 0u);
}
