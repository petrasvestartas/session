// --8<-- [start:triangle-vertex]
const BACKFACE_COLOR: vec3<f32> = vec3<f32>(0.80, 0.05, 0.05);

// Each @location(n) is filled by the vertex attribute with shader_location n: 0-2 come from GpuVertex,
// 3 from the object row buffer.
struct VsIn {
    @location(0) position: vec3<f32>, // object space
    @location(1) normal: u32, // octahedral, see oct32_decode
    @location(2) color: vec4<f32>, // Unorm8x4: the GPU turns 4 bytes into 4 floats 0..1
    @location(3) inst_id: u32, // object row
}

// @interpolate(flat) = every fragment gets the value of the triangle's first vertex, not a blend;
// integers must be flat, since 2.5 is no object row.
struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>, // scene space: world minus the scene origin
    @location(2) normal: vec3<f32>, // scene space
    @location(3) print: f32, // 1 for a sheet fill
    @location(4) @interpolate(flat) inst_id: u32,
    @location(5) @interpolate(flat) mirrored: u32, // 1 when the object matrix flips handedness
    @location(6) @interpolate(flat) selected: u32,
    @location(7) @interpolate(flat) source_face: u32, // face of the source object, or 0xffffffff
    @location(8) @interpolate(flat) primitive: u32, // triangle index + 1; 0 = no triangle id
    @location(9) @interpolate(flat) closed: u32, // closed solids take the opacity; open sheets stay opaque
    @location(10) @interpolate(flat) xray: u32, // x-ray drops the faces of multi-face solids only
}

// Clip space runs -1..1, so x = 3 lies outside it and the GPU drops every triangle this vertex is in.
fn dead_vertex() -> VsOut {
    var dead: VsOut; // a WGSL var starts zeroed
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.source_face = 0xffffffffu;
    return dead;
}

// Place and colour one vertex; every vertex entry point below ends here.
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
    o.normal = face_normal(inst.model, oct32_decode(in.normal));
    // a negative determinant means a mirror, which turns counter-clockwise triangles clockwise
    o.mirrored = select(0u, 1u, dot(inst.model[0].xyz, cross(inst.model[1].xyz, inst.model[2].xyz)) < 0.0);
    o.print = select(0.0, 1.0, (inst.flags & FLAG_PRINT) != 0u);
    o.inst_id = in.inst_id;
    o.selected = inst.flags & FLAG_SELECTED;
    o.source_face = 0xffffffffu;
    o.closed = select(1u, 0u, (inst.flags & FLAG_OPEN) != 0u);
    o.xray = select(0u, 1u, line.opacity <= 0.0 && (inst.flags & (FLAG_PRINT | FLAG_SHEET | FLAG_SINGLE)) == 0u);
    return o;
}

// Slot 0 holds OWN_ROW, so a plain draw keeps each vertex's own row; lesson 18a adds real slots,
// one per placed copy of a shared mesh.
fn slot_row(own: u32, slot: u32) -> u32 {
    return select(slot, own, slot == OWN_ROW);
}

// The entry point of drawing with vertex buffers; `slot` comes from the slot buffer, one per instance.
@vertex
fn vs_main(in: VsIn, @location(4) slot: u32) -> VsOut {
    var placed = in;
    placed.inst_id = slot_row(in.inst_id, slot);
    return transform_vertex(placed);
}
// --8<-- [end:triangle-vertex]

// --8<-- [start:triangle-pulled]
// Group 3 = the arena buffers again, bound as storage so a shader can read any vertex by number.
@group(3) @binding(0) var<storage, read> face_vertices: array<u32>; // 5 words per vertex

// Pick ids with this bit set name a face, not an object part; the same value as FACE_TAG in Rust.
const FACE_TAG: u32 = 0x20000000u;
@group(3) @binding(1) var<storage, read> face_objects: array<u32>; // object row per vertex
@group(3) @binding(2) var<storage, read> face_indices: array<u32>;
@group(3) @binding(3) var<storage, read> source_faces: array<u32>; // source face per triangle
@group(3) @binding(4) var<uniform> selected_face: vec4<u32>; // x = the selected source face

// Slot value that keeps the vertex's own row; the same value as OWN_ROW in Rust.
const OWN_ROW: u32 = 0xffffffffu;

// Vertex pulling: draw with no vertex buffers and read vertex `index` from storage yourself.
// The shader then knows its triangle, index / 3, which a vertex buffer never tells it.
fn pull_triangle(index: u32, slot: u32, base: u32) -> VsOut {
    let vertex = face_indices[index];
    let start = vertex * 5u; // words 0-2 position, 3 octahedral normal, 4 rgba bytes
    // bitcast reads the same 32 bits as another type: here u32 words back into the f32 they were
    let position = bitcast<vec3<f32>>(vec3<u32>(face_vertices[start], face_vertices[start+1u], face_vertices[start+2u]));
    let color = unpack4x8unorm(face_vertices[start+4u]);
    let own = face_objects[vertex];
    var out = transform_vertex(VsIn(position, face_vertices[start+3u], color, slot_row(own, slot)));
    out.primitive = base + index/3u + 1u; // `base` turns the arena triangle into this instance's own id
    return out;
}

@vertex
fn vs_triangle(@builtin(vertex_index) index: u32, @location(0) slot: u32, @location(1) base: u32) -> VsOut {
    return pull_triangle(index, slot, base);
}

// The same, plus the face of the source object, so a click can pick one face of a solid.
@vertex
fn vs_face(@builtin(vertex_index) index: u32, @location(0) slot: u32, @location(1) base: u32) -> VsOut {
    var out=pull_triangle(index, slot, base);
    out.source_face = source_faces[index/3u];
    out.selected = select(0u, 1u, out.source_face != 0xffffffffu && out.source_face == selected_face.x);

    if (out.selected != 0u) {
        out.color = SELECT_COLOR;
    }

    return out;
}
// --8<-- [end:triangle-pulled]

// --8<-- [start:triangle-shade]
// In ortho every ray is parallel, so the direction is the camera's axis, read from the matrix.
fn view_dir(world_pos: vec3<f32>) -> vec3<f32> {
    if (line.ortho_h > 0.0) {
        return normalize(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
    }

    return normalize(vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - world_pos);
}

// Face colour with lighting, opacity and red back faces.
fn shade(in: VsOut, raster_front: bool) -> vec4<f32> {
    let front = raster_front != (in.mirrored != 0u);

    // A translucent solid shows only its near side; `discard` drops the fragment, nothing is written.
    if (!front && in.closed != 0u && line.opacity > 0.0 && line.opacity < 1.0) {
        discard;
    }
    // no normal in the mesh: the cross product of two screen-space slopes is the flat face normal
    let flat_n = cross(dpdy(in.world_pos), dpdx(in.world_pos));
    var n = vec3<f32>(0.0, 0.0, 1.0);

    if (dot(in.normal, in.normal) > 1e-12) {
        n = normalize(in.normal);
    } else if (dot(flat_n, flat_n) > 1e-24) {
        n = normalize(flat_n);
    }

    // a headlight: the lamp rides with the camera, a little above it
    let v = view_dir(in.world_pos);

    if (dot(n, v) < 0.0) {
        n = -n;
    }

    let l = normalize(v + vec3<f32>(0.0, 0.0, 0.35));
    let h = normalize(l + v);
    // wrapped diffuse: light fades to 0.40, never to black, so there is no hard shadow line
    let wrap = clamp((dot(n, l) + 0.5) / 1.5, 0.0, 1.0);
    let spec = pow(max(dot(n, h), 0.0), 32.0) * 0.15;
    let lit = min(0.40 + 0.55 * wrap + spec, 1.0);

    let backface = !front && in.print <= 0.5 && line.backface > 0.5;
    let base = select(in.color, BACKFACE_COLOR, backface);
    // with SSAO on, a soft sky light from above; the occlusion pass adds the shadows
    let ambient = mix(0.72, 1.0, 0.5 + 0.5 * n.z);
    let lighting = select(lit, ambient, line.lit > 1.5);
    let shaded = select(1.0, lighting, line.lit > 0.5 && in.print <= 0.5);
    let alpha = select(1.0, line.opacity, in.closed != 0u);
    return vec4<f32>(base * shaded, alpha);
}
// --8<-- [end:triangle-shade]

// --8<-- [start:triangle-fragments]
// Flags 0: a face is never part of a clipping plane object.
fn cut(in: VsOut) -> bool {
    return clip_active() && clip_cut(0u, in.world_pos);
}

// The pick id: object row + 1 (0 = background), then the tagged face + 1 (0 = no face).
@fragment
fn fs_id(in: VsOut) -> PhysicalId {
    if (in.xray != 0u || cut(in)) {
        discard;
    }

    let sub = select((FACE_TAG | in.source_face) + 1u, 0u, in.source_face == 0xffffffffu);
    return PhysicalId(vec2<u32>(in.inst_id + 1u, sub), physical_primitive(in.primitive));
}

// A one-channel mask the outline pass traces around the selection.
@fragment
fn fs_selection_mask(in: VsOut) -> @location(0) vec4<f32> {
    if (in.selected == 0u || in.xray != 0u || cut(in)) {
        discard;
    }

    return vec4<f32>(1.0);
}

// `front_facing` is true when the triangle's corners run counter-clockwise on screen.
@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> PhysicalColor {
    if (in.xray != 0u) {
        discard;
    }

    return PhysicalColor(shade(in, front), physical_primitive(in.primitive));
}

struct PhysicalClipped {
    @location(0) color: vec4<f32>,
    @location(1) primitive: vec2<u32>, // triangle index + 1 in 16-bit halves
    @builtin(sample_mask) mask: u32, // written by the shader: only these samples keep the fragment
}

// Cut sample by sample, so the edge a plane leaves is as smooth as any other MSAA edge.
@fragment
fn fs_clipped(in: VsOut, @builtin(front_facing) front: bool) -> PhysicalClipped {
    let mask = clip_mask(0u, in.world_pos);

    if (in.xray != 0u || mask == 0u) {
        discard;
    }

    return PhysicalClipped(shade(in, front), physical_primitive(in.primitive), mask);
}

@fragment
fn fs_face_highlight(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (in.selected == 0u || in.xray != 0u || cut(in)) {
        discard;
    }

    return shade(in, front);
}

// Two render targets written by one fragment: location 0 and location 1.
struct MaskPair {
    @location(0) solid: vec4<f32>, // every solid
    @location(1) selected: vec4<f32>, // the selection only
};

@fragment
fn fs_masks(in: VsOut) -> MaskPair {
    if (in.xray != 0u || cut(in)) {
        discard;
    }

    return MaskPair(vec4<f32>(1.0), vec4<f32>(f32(in.selected != 0u)));
}
// --8<-- [end:triangle-fragments]

// --8<-- [start:triangle-count]
// Section caps count crossings: along the view ray behind a plane, +1 where it leaves a solid and
// -1 where it enters; a sum above 0 means the pixel lies inside a cut solid, and 18b paints a cap.
struct CountOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) @interpolate(flat) inst_id: u32,
    @location(2) @interpolate(flat) plane: u32,
    @location(3) @interpolate(flat) flip: u32, // 1 when the solid is mirrored or wound inward
    @location(4) @interpolate(flat) selected: u32,
}

// One draw instance per plane: instance_index says which plane this copy counts for.
@vertex
fn vs_count(in: VsIn, @builtin(instance_index) plane: u32) -> CountOut {
    return count_corner(in, plane);
}

// Instanced solids take the plane from the per-instance buffer instead.
@vertex
fn vs_count_placed(in: VsIn, @location(4) plane: u32) -> CountOut {
    return count_corner(in, plane);
}

// Only visible, closed solids that are not plane objects count; the rest land off screen.
fn count_corner(in: VsIn, plane: u32) -> CountOut {
    let inst = instances[in.inst_id];
    var out: CountOut;
    out.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);

    if ((inst.flags & (FLAG_HIDDEN | FLAG_CLOSED | FLAG_CLIPPING_PLANE)) != FLAG_CLOSED) {
        return out;
    }

    let world = place(in.inst_id, in.position);
    let mirrored = dot(inst.model[0].xyz, cross(inst.model[1].xyz, inst.model[2].xyz)) < 0.0;
    out.pos = mvp * vec4<f32>(world, 1.0);
    out.world_pos = world;
    out.inst_id = in.inst_id;
    out.plane = plane;
    out.flip = select(0u, 1u, mirrored != ((inst.flags & FLAG_INWARD) != 0u));
    out.selected = inst.flags & FLAG_SELECTED;
    return out;
}

struct CountColor {
    @location(0) count: vec4<f32>, // x: +-1, or +-SELECTED_CROSSING for a selected solid
    @builtin(sample_mask) mask: u32,
}

// Positive behind the plane, as the eye sees it.
fn behind(plane: u32, p: vec3<f32>) -> f32 {
    return -clip_eye_side(plane) * clip_distance(plane, p);
}

// The ray leaves through a back face, unless the solid is mirrored or wound inward.
fn leaving(in: CountOut, front: bool) -> bool {
    return front == (in.flip != 0u);
}

// The target blends by adding, so every crossing of the pixel sums into one number.
@fragment
fn fs_count(in: CountOut, @builtin(front_facing) front: bool) -> CountColor {
    let s = behind(in.plane, in.world_pos);
    let mask = clip_samples(s, vec2<f32>(dpdx(s), dpdy(s)));

    if (clip_eye_side(in.plane) == 0.0 || mask == 0u) {
        discard;
    }

    let sign = select(-1.0, 1.0, leaving(in, front));
    let weight = select(1.0, SELECTED_CROSSING, in.selected != 0u);
    return CountColor(vec4<f32>(sign * weight, 0.0, 0.0, 0.0), mask);
}
// --8<-- [end:triangle-count]

// --8<-- [start:triangle-pick-caps]
// An atomic is a word many fragments may change at once without losing a write.
@group(3) @binding(5) var<storage, read_write> pick_caps: array<atomic<u32>>;

fn pick_cap_word(plane: u32, px: vec2<f32>) -> u32 {
    return clip_pick_word(plane, px, vec2<u32>(u32(line.vp_w), u32(line.vp_h)));
}

// Clicking a cap: the count and the signed id sum name the solid the pixel is inside.
@fragment
fn fs_pick_count(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0) {
        discard;
    }

    let leaves = leaving(in, front);
    let sign = select(-1, 1, leaves);
    let word = pick_cap_word(in.plane, in.pos.xy);
    atomicAdd(&pick_caps[word], bitcast<u32>(sign));
    atomicAdd(&pick_caps[word + 1u], bitcast<u32>(sign * i32(in.inst_id + 1u)));

    // a positive float's bits sort like the float, and here the largest depth is the nearest exit
    if (leaves) {
        atomicMax(&pick_caps[word + 2u], bitcast<u32>(in.pos.z));
    }

    return vec4<f32>(0.0);
}

// Where solids nest, the nearest exit names the innermost one: the owner of the cap.
@fragment
fn fs_pick_owner(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0 || !leaving(in, front)) {
        discard;
    }

    let word = pick_cap_word(in.plane, in.pos.xy);

    if (bitcast<u32>(in.pos.z) == atomicLoad(&pick_caps[word + 2u])) {
        atomicStore(&pick_caps[word + 3u], in.inst_id + 1u);
    }

    return vec4<f32>(0.0);
}
// --8<-- [end:triangle-pick-caps]
