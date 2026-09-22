const STEP: f32 = 1000.0; // mm per cell
const HALF: f32 = 5000.0; // grid reaches 5 m each way
const N: u32 = 5u; // cells from the center to the edge
const PER_DIR: u32 = 22u; // vertices per direction: 11 lines, 2 ends
const FLOOR: u32 = 44u; // floor vertices; axis vertices follow

// Floor line color.
const GREY: vec3<f32> = vec3<f32>(0.55, 0.55, 0.55);
// X axis color.
const RED: vec3<f32> = vec3<f32>(0.85, 0.30, 0.30);
// Y axis color.
const GREEN: vec3<f32> = vec3<f32>(0.30, 0.70, 0.30);
// Z axis color.
const BLUE: vec3<f32> = vec3<f32>(0.30, 0.45, 0.85);

// One grid vertex.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) color: vec3<f32>, // line color
}

@vertex
// Place one endpoint of a floor line or an axis from its index.
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    // odd index = far end of the line
    let far = (vid % 2u) == 1u;
    var wp: vec3<f32>;
    var c: vec3<f32>;

    // floor lines: 11 along x, then 11 along y
    if (vid < FLOOR) {
        let dir = vid / PER_DIR;
        let k = (vid % PER_DIR) / 2u;
        let t = (f32(k) - f32(N)) * STEP;
        let end = select(-HALF, HALF, far);
        wp = select(vec3<f32>(end, t, 0.0), vec3<f32>(t, end, 0.0), dir == 1u);
        c = GREY;
    } else {
        // axes: x, y, then a short z
        let axis = (vid - FLOOR) / 2u;

        if (axis == 0u) {
            wp = vec3<f32>(select(0.0, HALF, far), 0.0, 0.0);
            c = RED;
        } else if (axis == 1u) {
            wp = vec3<f32>(0.0, select(0.0, HALF, far), 0.0);
            c = GREEN;
        } else {
            // z axis is one cell long
            wp = vec3<f32>(0.0, 0.0, select(0.0, STEP, far));
            c = BLUE;
        }
    }

    var o: VsOut;
    // world to clip, relative to the scene origin
    o.pos = mvp * vec4<f32>(wp - line.anchor, 1.0);
    o.color = c;
    return o;
}

@fragment
// Flat line color.
fn fs_main(in: VsOut) -> PhysicalColor {
    // --8<-- [start:step-4]
    return PhysicalColor(vec4<f32>(in.color, 1.0), vec4<f32>(0.0));
    // --8<-- [end:step-4]
}
