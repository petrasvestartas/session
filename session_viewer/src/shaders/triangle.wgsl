// Mesh faces: lit triangles from the arena, on the scene contract of scene.wgsl.

const BACKFACE_COLOR: vec3<f32> = vec3<f32>(0.80, 0.05, 0.05);

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) inst_id: u32,
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) print: f32,
    @location(4) @interpolate(flat) inst_id: u32,
    @location(5) @interpolate(flat) mirrored: u32,
    @location(6) @interpolate(flat) selected: u32,
    @location(7) @interpolate(flat) source_face: u32,
    @location(8) @interpolate(flat) primitive: u32,
    // A closed solid dims with `line.opacity`; an open sheet (a flat contact polygon, a hole
    // boundary) stays opaque, or looking through it would just show its own back face.
    @location(9) @interpolate(flat) closed: u32,
    // X-ray (`P`, opacity 0): a multi-face solid loses its faces; a single face (a surface, a
    // flat polygon, a sheet or print fill) has no inside to show and keeps its shading.
    @location(10) @interpolate(flat) xray: u32,
}

// A hidden row's triangle, parked outside the clip volume: the ID pass shares this vertex
// stage, so a hidden object stops being pickable as well as drawn.
fn dead_vertex() -> VsOut {
    var dead: VsOut;  // zero-valued; the position parks it, no source face is 0xffffffff
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.source_face = 0xffffffffu;
    return dead;
}

fn transform_vertex(in: VsIn) -> VsOut {
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
    o.mirrored = select(0u, 1u, dot(inst.model[0].xyz, cross(inst.model[1].xyz, inst.model[2].xyz)) < 0.0);
    o.print = select(0.0, 1.0, (inst.flags & FLAG_PRINT) != 0u);
    o.inst_id = in.inst_id;
    o.selected = inst.flags & FLAG_SELECTED;
    o.source_face = 0xffffffffu;
    o.closed = select(1u, 0u, (inst.flags & FLAG_OPEN) != 0u);
    o.xray = select(0u, 1u, line.opacity <= 0.0 && (inst.flags & (FLAG_PRINT | FLAG_SHEET | FLAG_SINGLE)) == 0u);
    return o;
}

@vertex
fn vs_main(in: VsIn) -> VsOut { return transform_vertex(in); }

@group(3) @binding(0) var<storage, read> face_vertices: array<f32>;
@group(3) @binding(1) var<storage, read> face_objects: array<u32>;
@group(3) @binding(2) var<storage, read> face_indices: array<u32>;
@group(3) @binding(3) var<storage, read> source_faces: array<u32>;
@group(3) @binding(4) var<uniform> selected_face: vec4<u32>;

fn pull_triangle(index: u32) -> VsOut {
    let vertex = face_indices[index];
    let start = vertex * 10u; // RenderVertex is ten tightly packed f32 values.
    let position = vec3<f32>(face_vertices[start], face_vertices[start+1u], face_vertices[start+2u]);
    let normal = vec3<f32>(face_vertices[start+3u], face_vertices[start+4u], face_vertices[start+5u]);
    let color = vec3<f32>(face_vertices[start+6u], face_vertices[start+7u], face_vertices[start+8u]);
    var out = transform_vertex(VsIn(position, normal, color, face_objects[vertex]));
    out.primitive = index/3u+1u;
    return out;
}

@vertex
fn vs_triangle(@builtin(vertex_index) index: u32) -> VsOut { return pull_triangle(index); }

@vertex
fn vs_face(@builtin(vertex_index) index: u32) -> VsOut {
    var out=pull_triangle(index);
    out.source_face = source_faces[index/3u];
    out.selected = select(0u, 1u, out.source_face != 0xffffffffu && out.source_face == selected_face.x);
    if (out.selected != 0u) { out.color = SELECT_COLOR; }
    return out;
}

// The direction a fragment sees the camera in: a ray from the point under perspective,
// one fixed direction under orthographic, where every ray is parallel (as `ink_visibility.wgsl`
// reads it out of the same matrix row).
fn view_dir(world_pos: vec3<f32>) -> vec3<f32> {
    if (line.ortho_h > 0.0) {
        return normalize(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
    }
    return normalize(vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - world_pos);
}

fn shade(in: VsOut, raster_front: bool) -> vec4<f32> {
    let front = raster_front != (in.mirrored != 0u);
    // Flat normal from screen-space derivatives when the mesh baked none (y is down).
    let flat_n = cross(dpdy(in.world_pos), dpdx(in.world_pos));
    var n = vec3<f32>(0.0, 0.0, 1.0);
    if (dot(in.normal, in.normal) > 1e-12) {
        n = normalize(in.normal);
    } else if (dot(flat_n, flat_n) > 1e-24) {
        n = normalize(flat_n);
    }

    // A headlight, as every CAD viewport shades: the lamp rides the camera, tilted a little
    // above it so a horizontal face still reads brighter than a vertical one. A world-fixed key
    // left every underside near black and made a colour unreadable from half the orbit.
    //
    // The diffuse is wrapped - the lit hemisphere is stretched over the whole sphere - so the
    // terminator is a gradient across a curved surface rather than a hard edge, and the darkest
    // a visible face can get is its silhouette, not black. Blinn-Phong on top puts a soft bloom
    // where the normal splits the lamp and the eye; the gain leaves it room to show.
    //
    // Measured on the mixed-solids scene (1400x900, Intel iGPU, 2026-09-08), as the ratio of the
    // lit frame to the same frame under `VIEWER_LIT off`, both linearised: a face square to the
    // camera holds 1.00 of its colour, the sphere's silhouette - its normal square to the view -
    // reads 0.59..0.62, and the darkest face pixel in either frame, iso or from below, is 0.47.
    let v = view_dir(in.world_pos);
    // Face the normal toward the eye by its own dot product, not by winding: `front`/`mirrored`
    // answer which side of the WINDING is visible, so a flipped authored normal on an
    // otherwise-front triangle used to light as if seen from behind it. Two coplanar flat
    // polygons that differ only in which way their normal was authored now shade identically.
    if (dot(n, v) < 0.0) { n = -n; }
    let l = normalize(v + vec3<f32>(0.0, 0.0, 0.35));
    let h = normalize(l + v);
    let wrap = clamp((dot(n, l) + 0.5) / 1.5, 0.0, 1.0);
    let spec = pow(max(dot(n, h), 0.0), 32.0) * 0.15;
    let lit = min(0.40 + 0.55 * wrap + spec, 1.0);

    // A back face is a flipped normal or the inside of an open solid: shown red. Print is
    // paper, read from both sides, lit flat.
    let backface = !front && in.print <= 0.5 && line.backface > 0.5;
    let base = select(in.color, BACKFACE_COLOR, backface);
    let shaded = select(1.0, lit, line.lit > 0.5 && in.print <= 0.5);
    let alpha = select(1.0, line.opacity, in.closed != 0u);
    return vec4<f32>(base * shaded, alpha);
}

// The id pass: (object row + 1, 0).
@fragment
fn fs_id(in: VsOut) -> PhysicalId {
    if (in.xray != 0u) { discard; }
    let sub = select((0x20000000u | in.source_face) + 1u, 0u, in.source_face == 0xffffffffu);
    return PhysicalId(vec2<u32>(in.inst_id + 1u, sub), physical_triangle(in.pos.z, in.primitive));
}

@fragment
fn fs_selection_mask(in: VsOut) -> @location(0) vec4<f32> {
    if (in.selected == 0u || in.xray != 0u) { discard; }
    return vec4<f32>(1.0);
}

@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> PhysicalColor {
    if (in.xray != 0u) { discard; }
    return PhysicalColor(shade(in, front), physical_triangle(in.pos.z, in.primitive));
}

@fragment
fn fs_face_highlight(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (in.selected == 0u || in.xray != 0u) { discard; }
    return shade(in, front);
}

// All visible solid faces contribute to one coverage mask. Touching or overlapping
// objects create no artificial seam in the group's outside silhouette.
@fragment
fn fs_solid_mask(in: VsOut) -> @location(0) vec4<f32> {
    if (in.xray != 0u) { discard; }
    return vec4<f32>(1.0);
}

// Both masks from one rasterization: every visible solid face is coverage, the selected
// ones also select. The targets blend with MAX, so a 0 here is the discard above.
struct MaskPair {
    @location(0) solid: vec4<f32>,
    @location(1) selected: vec4<f32>,
};

@fragment
fn fs_masks(in: VsOut) -> MaskPair {
    if (in.xray != 0u) { discard; }
    return MaskPair(vec4<f32>(1.0), vec4<f32>(f32(in.selected != 0u)));
}
