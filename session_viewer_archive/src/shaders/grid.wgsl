struct Camera { view_proj: mat4x4<f32>, }
@group(0) @binding(0) var<uniform> camera: Camera;

const COLOR_GREY:  vec3<f32> = vec3<f32>(0.5,  0.5,  0.5);
const COLOR_RED:   vec3<f32> = vec3<f32>(0.78, 0.16, 0.16);
const COLOR_GREEN: vec3<f32> = vec3<f32>(0.16, 0.78, 0.16);
const COLOR_BLUE:  vec3<f32> = vec3<f32>(0.16, 0.31, 0.86);

// Grid is in millimetres to match session geometry.
// view_proj includes MM_TO_UNIT (×0.001) so positions here must be mm.
// STEP = 1000 mm = 1 m per grid cell.
// GRID_HALF_F = N_HALF × STEP = 36 m extent on each side.
const STEP:        f32 = 1000.0; // mm per grid cell
const N_HALF:      u32 = 36u;    // grid lines per side per direction
const GRID_HALF_F: f32 = 36000.0; // = N_HALF * STEP
const N_GREY:      u32 = 144u;   // N_HALF * 2 lines/direction * 2 verts/line

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0)       color:    vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var wp: vec3<f32>;
    var c:  vec3<f32>;

    if vid < N_GREY {
        // Block A: X-parallel grey lines (Y varies in mm, skip Y=0)
        let line_idx = vid / 2u;
        let ep       = vid % 2u;
        let y_raw    = (f32(line_idx) - f32(N_HALF)) * STEP;            // −36000..35000
        let y        = y_raw + select(0.0, STEP, y_raw >= 0.0);         // skip 0
        let x        = select(-GRID_HALF_F, GRID_HALF_F, ep == 1u);
        wp = vec3<f32>(x, y, 0.0);
        c  = COLOR_GREY;

    } else if vid < N_GREY * 2u {
        // Block B: Y-parallel grey lines (X varies in mm, skip X=0)
        let local    = vid - N_GREY;
        let line_idx = local / 2u;
        let ep       = local % 2u;
        let x_raw    = (f32(line_idx) - f32(N_HALF)) * STEP;
        let x        = x_raw + select(0.0, STEP, x_raw >= 0.0);
        let y        = select(-GRID_HALF_F, GRID_HALF_F, ep == 1u);
        wp = vec3<f32>(x, y, 0.0);
        c  = COLOR_GREY;

    } else {
        // Block C: axes (mm positions, same scale)
        let axis_vid = vid - N_GREY * 2u; // 0..9

        if axis_vid < 8u {
            let axis  = axis_vid / 4u;
            let inner = axis_vid % 4u;
            let seg   = inner / 2u;
            let ep    = inner % 2u;
            let coord = select(
                select(-GRID_HALF_F, 0.0, ep == 1u),
                select(0.0, GRID_HALF_F, ep == 1u),
                seg == 1u
            );
            let on_x = axis == 0u;
            wp = vec3<f32>(
                select(0.0, coord, on_x),
                select(0.0, coord, !on_x),
                0.0,
            );
            let axis_color = select(COLOR_GREEN, COLOR_RED, on_x);
            c = select(COLOR_GREY, axis_color, seg == 1u);

        } else {
            // Z axis positive only
            let ep = (axis_vid - 8u) % 2u;
            wp = vec3<f32>(0.0, 0.0, select(0.0, GRID_HALF_F, ep == 1u));
            c  = COLOR_BLUE;
        }
    }

    var out: VsOut;
    out.color    = vec4<f32>(c, 1.0);
    out.clip_pos = camera.view_proj * vec4<f32>(wp, 1.0);
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return in.color;
}
