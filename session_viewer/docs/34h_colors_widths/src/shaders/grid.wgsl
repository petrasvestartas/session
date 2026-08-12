// ── Grid size & spacing ────────────────────────────────────────────────
// Authored in mm (the camera's mvp applies the mm→m scale).
//   STEP = distance between adjacent lines   → currently 1000 mm = 1 m per cell
//   N    = number of cells on each side of centre → currently 5  (→ 2*N+1 = 11 lines per direction)
//   HALF = how far the grid reaches from centre   → currently 5000 mm = ±5 m
//
// To change the SPACING: edit STEP (mm between lines).
// To change the SUBDIVISION / SIZE: edit N (cells per side).
// After changing N or STEP, keep these three in sync:
//   HALF    = N * STEP
//   PER_DIR = (2*N + 1) * 2      // verts per direction (2 per line)
//   FLOOR   = 2 * PER_DIR        // floor verts (X-parallel + Y-parallel)
// …and update the draw range in gpu.rs to  FLOOR + 6  (the 6 = 3 axes × 2 verts).


@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

// Shared with the linework shaders; only `anchor` matters here.
struct LineUniform{
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    anchor: vec3<f32>,   // camera-relative anchor, world units
};

const STEP: f32 = 1000.0; // mm per cell (1 m)
const HALF: f32 = 5000.0; // +-5 m floor
const N: u32 = 5u; // lines per side of center (2*n + 1 = 11 lines per direction)
const PER_DIR: u32 = 22u; // (2*N + 1) lines * 2 endpoints
const FLOOR: u32 = 44u;  // 2 * PER_DIR (X-parallel + Y-parallel); Zaxis is vid 44,49

const GREY:  vec3<f32> = vec3<f32>(0.55, 0.55, 0.55);
const RED:   vec3<f32> = vec3<f32>(0.85, 0.30, 0.30);
const GREEN: vec3<f32> = vec3<f32>(0.30, 0.70, 0.30);
const BLUE:  vec3<f32> = vec3<f32>(0.30, 0.45, 0.85);

struct VsOut{
    @builtin(position) pos: vec4<f32>, // Position on the screen
    @location(0) color: vec3<f32>,     // Color passed to the fragment shader
}

// The loop is hidden: the GPU runs vs_main once per vertex, in parallel. 
// draw(0..49) means "call the vertex shader 46 times, with vid = 0, 1, 2, … 45." 
// Each call gets one vid and computes that single vertex's position/color by arithmetic.
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {

    // Create a grid of lines
    let far = (vid % 2u) == 1u; // true or false
    var wp: vec3<f32>; // empty world-position before matrix multiplication
    var c: vec3<f32>; // color

    if vid < FLOOR {
        // Grid
        let dir = vid / PER_DIR; // 0 = X-parallel, 1 = Y-parallel
        let line = (vid % PER_DIR) / 2u;  // which line, 0 .. 2*N
        let t = (f32(line) - f32(N)) * STEP; // line offset, -HALF..HALF
        let end = select(-HALF, HALF, far);
        wp = select(vec3<f32>(end, t, 0.0), vec3<f32>(t, end, 0.0), dir == 1u);
        c = GREY;
    } else {
        let axis = (vid-FLOOR) / 2u; // 0 == X, 1 = Y, 2 = Z
        if axis == 0u {
            wp = vec3<f32>(select (0.0, HALF, far), 0.0, 0.0);
            c = RED;
        }else if axis == 1u {
            wp = vec3<f32>(0.0, select (0.0, HALF, far), 0.0);
            c = GREEN;
        }else{
            wp = vec3<f32>(0.0, 0.0, select (0.0, 1000, far));
            c = BLUE;
        }
    }

    var o: VsOut;
    // The grid is authored in ABSOLUTE world coordinates, but mvp is camera-relative: instance
    // rows have had the anchor subtracted already. Without the same subtraction the floor slides
    // out from under the model every time re-anchoring fires - the grid appears to jump around.
    o.pos = mvp * vec4<f32>(wp - line.anchor, 1.0); // Set the position
    o.color = c; // Set the color
    return o;
    
}

@fragment
fn fs_main(in : VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}