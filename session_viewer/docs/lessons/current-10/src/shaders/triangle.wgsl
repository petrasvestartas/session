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
    @location(6) @interpolate(flat) selected: u32, // nonzero when selected
    @location(7) @interpolate(flat) source_face: u32, // source face id, or none
    @location(8) @interpolate(flat) primitive: u32, // triangle index + 1
    // closed solids take the opacity; open sheets stay opaque
    @location(9) @interpolate(flat) closed: u32,
    // x-ray drops the faces of multi-face solids only
    @location(10) @interpolate(flat) xray: u32,
}

// A vertex placed off screen, so nothing is drawn.
fn dead_vertex() -> VsOut {
    var dead: VsOut; // all zero; only the position matters
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.source_face = 0xffffffffu;
    return dead;
}

// Place and color one vertex.
fn transform_vertex(in: VsIn) -> VsOut {
    let inst = instances[in.inst_id];

    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_vertex();
    }

    let world = place(in.inst_id, in.position);
    let clip = mvp * vec4<f32>(world, 1.0);
    var o: VsOut;
    o.pos = clip;
    var color = object_color(vec4<f32>(in.color.rgb, 1.0), inst).rgb;

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
    o.selected = inst.flags & FLAG_SELECTED;
    o.source_face = 0xffffffffu;
    o.closed = select(1u, 0u, (inst.flags & FLAG_OPEN) != 0u);
    o.xray = select(0u, 1u, line.opacity <= 0.0 && (inst.flags & (FLAG_PRINT | FLAG_SHEET | FLAG_SINGLE)) == 0u);
    return o;
}

@vertex
// Vertices from the vertex buffers.
fn vs_main(in: VsIn) -> VsOut {
    return transform_vertex(in);
}

@group(3) @binding(0) var<storage, read> face_vertices: array<f32>; // mesh vertices, ten floats each

// Bit that marks a pick id as a face; matches FACE_TAG in Rust.
const FACE_TAG: u32 = 0x20000000u;
@group(3) @binding(1) var<storage, read> face_objects: array<u32>; // object row per vertex
@group(3) @binding(2) var<storage, read> face_indices: array<u32>; // triangle indices
@group(3) @binding(3) var<storage, read> source_faces: array<u32>; // source face per triangle
@group(3) @binding(4) var<uniform> selected_face: vec4<u32>; // x: selected source face

// Vertex `index` read from the storage buffers instead of vertex inputs.
fn pull_triangle(index: u32) -> VsOut {
    let vertex = face_indices[index];
    let start = vertex * 10u; // ten floats per vertex: position, normal, color, padding
    let position = vec3<f32>(face_vertices[start], face_vertices[start+1u], face_vertices[start+2u]);
    let normal = vec3<f32>(face_vertices[start+3u], face_vertices[start+4u], face_vertices[start+5u]);
    let color = vec3<f32>(face_vertices[start+6u], face_vertices[start+7u], face_vertices[start+8u]);
    var out = transform_vertex(VsIn(position, normal, color, face_objects[vertex]));
    out.primitive = index/3u+1u;
    return out;
}

@vertex
// Vertices read by index; object ids.
fn vs_triangle(@builtin(vertex_index) index: u32) -> VsOut {
    return pull_triangle(index);
}

@vertex
// Vertices read by index, with the source face and its selection.
fn vs_face(@builtin(vertex_index) index: u32) -> VsOut {
    var out=pull_triangle(index);
    out.source_face = source_faces[index/3u];
    out.selected = select(0u, 1u, out.source_face != 0xffffffffu && out.source_face == selected_face.x);

    if (out.selected != 0u) {
        out.color = SELECT_COLOR;
    }

    return out;
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
    let alpha = select(1.0, line.opacity, in.closed != 0u);
    return vec4<f32>(base * shaded, alpha);
}

// Pick id: object row + 1 and the tagged face id.
@fragment
fn fs_id(in: VsOut) -> PhysicalId {
    if (in.xray != 0u) {
        discard;
    }

    let sub = select((FACE_TAG | in.source_face) + 1u, 0u, in.source_face == 0xffffffffu);
    return PhysicalId(vec2<u32>(in.inst_id + 1u, sub), physical_triangle(in.pos.z, in.primitive));
}

@fragment
// Selected faces into a mask.
fn fs_selection_mask(in: VsOut) -> @location(0) vec4<f32> {
    if (in.selected == 0u || in.xray != 0u) {
        discard;
    }

    return vec4<f32>(1.0);
}

@fragment
// Shaded face color and depth slope.
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> PhysicalColor {
    if (in.xray != 0u) {
        discard;
    }

    return PhysicalColor(shade(in, front), physical_triangle(in.pos.z, in.primitive));
}

@fragment
// The selected source face, shaded.
fn fs_face_highlight(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (in.selected == 0u || in.xray != 0u) {
        discard;
    }

    return shade(in, front);
}

// Every face into the solid mask.
@fragment
fn fs_solid_mask(in: VsOut) -> @location(0) vec4<f32> {
    if (in.xray != 0u) {
        discard;
    }

    return vec4<f32>(1.0);
}

// Output into both outline masks at once.
struct MaskPair {
    @location(0) solid: vec4<f32>, // every solid's mask
    @location(1) selected: vec4<f32>, // the selection's mask
};

@fragment
// Every face into the solid mask; selected ones into both.
fn fs_masks(in: VsOut) -> MaskPair {
    if (in.xray != 0u) {
        discard;
    }

    return MaskPair(vec4<f32>(1.0), vec4<f32>(f32(in.selected != 0u)));
}
