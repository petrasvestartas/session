// The ground grid and axes: 50 vertices built from the vertex index, no buffer. Authored in
// world millimetres about the origin, minus the camera anchor the instance rows are rebased on.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,
};

const STEP: f32 = 1000.0;   // mm per cell
const HALF: f32 = 5000.0;   // +-5 m floor
const N: u32 = 5u;          // cells per side of the centre
const PER_DIR: u32 = 22u;   // (2N + 1) lines x 2 endpoints
const FLOOR: u32 = 44u;     // both directions; axes follow

const GREY: vec3<f32> = vec3<f32>(0.55, 0.55, 0.55);
const RED: vec3<f32> = vec3<f32>(0.85, 0.30, 0.30);
const GREEN: vec3<f32> = vec3<f32>(0.30, 0.70, 0.30);
const BLUE: vec3<f32> = vec3<f32>(0.30, 0.45, 0.85);

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let far = (vid % 2u) == 1u;
    var wp: vec3<f32>;
    var c: vec3<f32>;
    if (vid < FLOOR) {
        let dir = vid / PER_DIR;
        let k = (vid % PER_DIR) / 2u;
        let t = (f32(k) - f32(N)) * STEP;
        let end = select(-HALF, HALF, far);
        wp = select(vec3<f32>(end, t, 0.0), vec3<f32>(t, end, 0.0), dir == 1u);
        c = GREY;
    } else {
        let axis = (vid - FLOOR) / 2u;
        if (axis == 0u) {
            wp = vec3<f32>(select(0.0, HALF, far), 0.0, 0.0);
            c = RED;
        } else if (axis == 1u) {
            wp = vec3<f32>(0.0, select(0.0, HALF, far), 0.0);
            c = GREEN;
        } else {
            wp = vec3<f32>(0.0, 0.0, select(0.0, 1000.0, far));
            c = BLUE;
        }
    }
    var o: VsOut;
    o.pos = mvp * vec4<f32>(wp - line.anchor, 1.0);
    o.color = c;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
