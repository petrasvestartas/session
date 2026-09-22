// Red for faces seen from behind.
const BACKFACE_COLOR: vec3<f32> = vec3<f32>(0.80, 0.05, 0.05);

// One mesh vertex from the vertex buffers.
struct VsIn {
    @location(0) position: vec3<f32>, // object-space position
    @location(1) normal: vec3<f32>, // normal, or zero
    @location(2) color: vec3<f32>, // rgb
    @location(3) inst_id: u32, // object row
}

// What the vertex shader hands the fragment shader.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) color: vec3<f32>, // rgb, yellow when selected
    @location(1) world_pos: vec3<f32>, // scene-space position
    @location(2) normal: vec3<f32>, // scene-space normal
    @location(3) print: f32, // 1 for a sheet fill
    @location(4) @interpolate(flat) inst_id: u32, // object row
    @location(5) @interpolate(flat) mirrored: u32, // 1 when the object matrix flips handedness
    // --8<-- [start:step-6a]
    @location(6) @interpolate(flat) selected: u32, // nonzero when selected
    // --8<-- [end:step-6a]
}

// A vertex placed off screen, so nothing is drawn.
fn dead_vertex() -> VsOut {
    var dead: VsOut; // all zero; only the position matters
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec3<f32>(0.0);
    dead.world_pos = vec3<f32>(0.0);
    dead.normal = vec3<f32>(0.0);
    dead.print = 0.0;
    dead.inst_id = 0u;
    dead.mirrored = 0u;
    // --8<-- [start:step-6b]
    dead.selected = 0u;
    // --8<-- [end:step-6b]
    return dead;
}

@vertex
// Vertices from the vertex buffers.
fn vs_main(in: VsIn) -> VsOut {
    let inst = instances[in.inst_id];

    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_vertex();
    }

    let world = place(in.inst_id, in.position);
    let clip = mvp * vec4<f32>(world, 1.0);
    var o: VsOut;
    o.pos = clip;
    var color = in.color.rgb * inst.color.rgb;

    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = SELECT_COLOR;
    }

    o.color = color;
    o.world_pos = world;
    o.normal = face_normal(inst.model, in.normal);
    // negative determinant: winding is flipped
    o.mirrored = select(0u, 1u, dot(inst.model[0].xyz, cross(inst.model[1].xyz, inst.model[2].xyz)) < 0.0);
    o.print = select(0.0, 1.0, (inst.flags & FLAG_PRINT) != 0u);
    o.inst_id = in.inst_id;
    // --8<-- [start:step-6c]
    o.selected = inst.flags & FLAG_SELECTED;
    // --8<-- [end:step-6c]
    return o;
}

// Direction toward the camera; constant in ortho.
fn view_dir(world_pos: vec3<f32>) -> vec3<f32> {
    if (line.ortho_h > 0.0) {
        return normalize(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
    }

    return normalize(vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - world_pos);
}

// Face color with lighting, opacity and back-face red.
fn shade(in: VsOut, raster_front: bool) -> vec4<f32> {
    let front = raster_front != (in.mirrored != 0u);
    // flat normal from screen derivatives, when the mesh has none
    let flat_n = cross(dpdy(in.world_pos), dpdx(in.world_pos));
    var n = vec3<f32>(0.0, 0.0, 1.0);

    if (dot(in.normal, in.normal) > 1e-12) {
        n = normalize(in.normal);
    } else if (dot(flat_n, flat_n) > 1e-24) {
        n = normalize(flat_n);
    }

    // headlight: the lamp rides the camera, a little above it
    let v = view_dir(in.world_pos);

    // turn the normal toward the eye
    if (dot(n, v) < 0.0) {
        n = -n;
    }

    let l = normalize(v + vec3<f32>(0.0, 0.0, 0.35));
    let h = normalize(l + v);
    // wrapped diffuse: no hard terminator
    let wrap = clamp((dot(n, l) + 0.5) / 1.5, 0.0, 1.0);
    // soft highlight
    let spec = pow(max(dot(n, h), 0.0), 32.0) * 0.15;
    let lit = min(0.40 + 0.55 * wrap + spec, 1.0);

    // back faces red when asked; sheet fills never
    let backface = !front && in.print <= 0.5 && line.backface > 0.5;
    let base = select(in.color, BACKFACE_COLOR, backface);
    let shaded = select(1.0, lit, line.lit > 0.5 && in.print <= 0.5);
    return vec4<f32>(base * shaded, 1.0);
}

// Pick id: object row + 1 and the tagged face id.
@fragment
// --8<-- [start:step-6d]
fn fs_id(in: VsOut) -> PhysicalId {
    return PhysicalId(vec2<u32>(in.inst_id + 1u, 0u), physical_gradient(in.pos.z));
}

@fragment
// Selected faces into a mask.
fn fs_selection_mask(in: VsOut) -> @location(0) vec4<f32> {
    if (in.selected == 0u) {
        discard;
    }

    return vec4<f32>(1.0);
}

@fragment
// Shaded face color and depth slope.
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> PhysicalColor {
    return PhysicalColor(shade(in, front), physical_gradient(in.pos.z));
    // --8<-- [end:step-6d]
}
