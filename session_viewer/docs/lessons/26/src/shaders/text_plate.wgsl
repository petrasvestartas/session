// --8<-- [start:plate-vertex]
// What the vertex shader hands the fragment shader.
struct PlateVertex {
    @builtin(position) position: vec4<f32>, // clip position
    @location(0) local: vec2<f32>, // pixel offset from the plate center
    @location(1) half_size: vec2<f32>, // half width and height, px
    @location(2) radius: f32, // corner radius, px
    // flat = every pixel gets the first vertex's value, not a blend of three; integers must be flat
    @location(3) @interpolate(flat) object: u32, // object row + 1, or 0
    @location(4) @interpolate(flat) selected: f32, // 1 when selected
}

// Canvas clip space to the pick window; id pass only.
@group(0) @binding(0) var<uniform> pick: mat4x4<f32>;

// Pack the vertex attributes.
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
// Vertices arrive already in clip space.
fn vs_main(@location(0) position: vec3<f32>, @location(1) local: vec2<f32>,
           @location(2) half_size: vec2<f32>, @location(3) radius: f32, @location(4) object: u32,
           @location(5) selected: f32) -> PlateVertex {
    return plate_vertex(vec4<f32>(position, 1.0), local, half_size, radius, object, selected);
}

@vertex
// The same, mapped into the pick window.
fn vs_id(@location(0) position: vec3<f32>, @location(1) local: vec2<f32>,
         @location(2) half_size: vec2<f32>, @location(3) radius: f32, @location(4) object: u32,
         @location(5) selected: f32) -> PlateVertex {
    return plate_vertex(pick * vec4<f32>(position, 1.0), local, half_size, radius, object, selected);
}
// --8<-- [end:plate-vertex]

// --8<-- [start:plate-fragment]
// Signed distance to the rounded plate edge, px.
// A signed distance is negative inside the shape, zero on its edge and positive outside.
fn plate_distance(in: PlateVertex) -> f32 {
    let radius = min(in.radius, min(in.half_size.x, in.half_size.y));
    // fold into one quarter with abs, then measure to a box shrunk by the radius
    let q = abs(in.local) - in.half_size + vec2<f32>(radius);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
// Black plate, yellow when selected, soft edge.
fn fs_main(in: PlateVertex) -> @location(0) vec4<f32> {
    let distance = plate_distance(in);
    // fwidth = how much distance changes to the next pixel; the edge fades over exactly one pixel
    let coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    return vec4<f32>(in.selected, in.selected, 0.0, coverage);
}

@fragment
// Object id inside the plate.
fn fs_id(in: PlateVertex) -> @location(0) vec2<u32> {
    // object is already row + 1; 0 = no object
    // discard drops this pixel: nothing is written
    if (in.object == 0u || plate_distance(in) > 0.0) {
        discard;
    }

    return vec2<u32>(in.object, 0u);
}
// --8<-- [end:plate-fragment]
