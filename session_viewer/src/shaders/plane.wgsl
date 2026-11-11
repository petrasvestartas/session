// Planes: one instance per PlaneRow, a 10 by 10 grey grid hidden by solids and its x, y and z arrows over them.
#include "arrow.wgsl"

// One plane, 48 bytes; matches PlaneRow in Rust.
struct PlaneRow {
    origin: vec3<f32>, // object space
    instance_id: u32, // object row
    x: vec3<f32>, // unit x axis
    radius: f32, // pen: 0 = default; > 0 world mm; < 0 px
    y: vec3<f32>, // unit y axis
    pad: u32, // to 48 bytes
};

@group(3) @binding(0) var<storage, read> planes: array<PlaneRow>; // one row per plane

// Grid lines per plane: 11 across x, 11 across y.
const PLANE_LINES: u32 = 22u;
// The grey of the ground grid's lines, packed red lowest.
const PLANE_GREY: u32 = 0xff8c8c8cu;
// The ground grid's axis colors, x pink, y yellow-green, z blue.
const PLANE_AXES = array<u32, 3>(0xff8b47e8u, 0xff32cd9au, 0xffea9621u);

// Corner `vid` of plane `row`: six per grid line, then eighteen per arrow.
@vertex
fn vs_plane(@builtin(vertex_index) vid: u32, @builtin(instance_index) row: u32) -> VsOut {
    let p = planes[row];
    let size = line.plane_size; // half side, the arrows' length

    // a grid line runs edge to edge, at one of eleven steps across
    if (vid < PLANE_LINES * 6u) {
        let k = vid / 6u;
        let across = k < 11u;
        let runs = select(p.x, p.y, across);
        let steps = select(p.y, p.x, across);
        let t = (f32(k % 11u) / 5.0 - 1.0) * size;
        let tail = p.origin + steps * t - runs * size;
        let head = p.origin + steps * t + runs * size;
        return arrow_corner(tail, head, p.radius, p.instance_id, PLANE_GREY, 0.0, 0u, 0.0, 0.0, vid % 6u, false);
    }

    let i = vid - PLANE_LINES * 6u;
    let axis = i / 18u;
    var axes = array<vec3<f32>, 3>(p.x, p.y, cross(p.x, p.y));
    var colors = PLANE_AXES;
    let tip = p.origin + axes[axis] * size;
    return arrow_corner(p.origin, tip, p.radius, p.instance_id, colors[axis], 0.0, HEAD_END, 0.0, 0.0, i % 18u, true);
}
