// Red for faces seen from behind.
const BACKFACE_COLOR: vec3<f32> = vec3<f32>(0.80, 0.05, 0.05);

// One mesh vertex from the vertex buffers.
struct VsIn {
    @location(0) position: vec3<f32>, // object-space position
    @location(1) normal: u32, // octahedral normal, see oct32_decode
    @location(2) color: vec4<f32>, // rgba
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
    o.normal = face_normal(inst.model, oct32_decode(in.normal));
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

// The row a vertex draws with: its instance's in an instanced draw, else its own.
fn slot_row(own: u32, slot: u32) -> u32 {
    return select(slot, own, slot == OWN_ROW);
}

@vertex
// Vertices from the vertex buffers; `slot` is the instance's row in an instanced draw.
fn vs_main(in: VsIn, @location(4) slot: u32) -> VsOut {
    var placed = in;
    placed.inst_id = slot_row(in.inst_id, slot);
    return transform_vertex(placed);
}

@group(3) @binding(0) var<storage, read> face_vertices: array<u32>; // mesh vertices, five words each

// Bit that marks a pick id as a face; matches FACE_TAG in Rust.
const FACE_TAG: u32 = 0x20000000u;
@group(3) @binding(1) var<storage, read> face_objects: array<u32>; // object row per vertex
@group(3) @binding(2) var<storage, read> face_indices: array<u32>; // triangle indices
@group(3) @binding(3) var<storage, read> source_faces: array<u32>; // source face per triangle
@group(3) @binding(4) var<uniform> selected_face: vec4<u32>; // x: selected source face

// Slot value that keeps the vertex's own row; matches OWN_ROW in Rust.
const OWN_ROW: u32 = 0xffffffffu;

// Vertex `index` read from the storage buffers instead of vertex inputs, drawn with `slot`'s row;
// `base` turns the arena triangle into the instance's own triangle id.
fn pull_triangle(index: u32, slot: u32, base: u32) -> VsOut {
    let vertex = face_indices[index];
    let start = vertex * 5u; // five words per vertex: position, octahedral normal, rgba bytes
    let position = bitcast<vec3<f32>>(vec3<u32>(face_vertices[start], face_vertices[start+1u], face_vertices[start+2u]));
    let color = unpack4x8unorm(face_vertices[start+4u]);
    let own = face_objects[vertex];
    var out = transform_vertex(VsIn(position, face_vertices[start+3u], color, slot_row(own, slot)));
    out.primitive = base + index/3u + 1u;
    return out;
}

@vertex
// Vertices read by index; object ids.
fn vs_triangle(@builtin(vertex_index) index: u32, @location(0) slot: u32, @location(1) base: u32) -> VsOut {
    return pull_triangle(index, slot, base);
}

@vertex
// Vertices read by index, with the source face and its selection.
fn vs_face(@builtin(vertex_index) index: u32, @location(0) slot: u32, @location(1) base: u32) -> VsOut {
    var out=pull_triangle(index, slot, base);
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

    // translucent solids drop their back faces
    if (!front && in.closed != 0u && line.opacity > 0.0 && line.opacity < 1.0) {
        discard;
    }
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
    // SSAO mode: soft sky light from above
    let ambient = mix(0.72, 1.0, 0.5 + 0.5 * n.z);
    let lighting = select(lit, ambient, line.lit > 1.5);
    let shaded = select(1.0, lighting, line.lit > 0.5 && in.print <= 0.5);
    let alpha = select(1.0, line.opacity, in.closed != 0u);
    return vec4<f32>(base * shaded, alpha);
}

// True when a clipping plane cuts this fragment away; faces never belong to a clipping plane.
fn cut(in: VsOut) -> bool {
    return clip_active() && clip_cut(0u, in.world_pos);
}

// Pick id: object row + 1 and the tagged face id.
@fragment
fn fs_id(in: VsOut) -> PhysicalId {
    if (in.xray != 0u || cut(in)) {
        discard;
    }

    let sub = select((FACE_TAG | in.source_face) + 1u, 0u, in.source_face == 0xffffffffu);
    return PhysicalId(vec2<u32>(in.inst_id + 1u, sub), physical_primitive(in.primitive));
}

@fragment
// Selected faces into a mask.
fn fs_selection_mask(in: VsOut) -> @location(0) vec4<f32> {
    if (in.selected == 0u || in.xray != 0u || cut(in)) {
        discard;
    }

    return vec4<f32>(1.0);
}

@fragment
// Shaded face color and triangle id.
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> PhysicalColor {
    if (in.xray != 0u) {
        discard;
    }

    return PhysicalColor(shade(in, front), physical_primitive(in.primitive));
}

// Face color, triangle id and the samples no clipping plane cuts.
struct PhysicalClipped {
    @location(0) color: vec4<f32>, // rgba
    @location(1) primitive: vec2<u32>, // triangle index + 1 in 16-bit halves
    @builtin(sample_mask) mask: u32, // samples on the kept side of every plane
}

@fragment
// Shaded face color, cut by the clipping planes sample by sample.
fn fs_clipped(in: VsOut, @builtin(front_facing) front: bool) -> PhysicalClipped {
    let mask = clip_mask(0u, in.world_pos);

    if (in.xray != 0u || mask == 0u) {
        discard;
    }

    return PhysicalClipped(shade(in, front), physical_primitive(in.primitive), mask);
}

@fragment
// The selected source face, shaded.
fn fs_face_highlight(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (in.selected == 0u || in.xray != 0u || cut(in)) {
        discard;
    }

    return shade(in, front);
}

// Output into both outline masks at once.
struct MaskPair {
    @location(0) solid: vec4<f32>, // every solid's mask
    @location(1) selected: vec4<f32>, // the selection's mask
};

@fragment
// Every face into the solid mask; selected ones into both.
fn fs_masks(in: VsOut) -> MaskPair {
    if (in.xray != 0u || cut(in)) {
        discard;
    }

    return MaskPair(vec4<f32>(1.0), vec4<f32>(f32(in.selected != 0u)));
}

// One corner of a closed solid, for counting crossings behind a clipping plane.
struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) world_pos: vec3<f32>, // scene-space position
    @location(1) @interpolate(flat) inst_id: u32, // object row
    @location(2) @interpolate(flat) plane: u32, // clipping plane
    @location(3) @interpolate(flat) flip: u32, // 1 when the solid is mirrored or wound inward
    @location(4) @interpolate(flat) selected: u32, // nonzero when the solid is selected
}

@vertex
// Corners of visible verified closed solids; everything else lands off screen.
fn vs_count(in: VsIn, @builtin(instance_index) plane: u32) -> CountOut {
    return count_corner(in, plane);
}

@vertex
// Corners of instanced closed solids: the row and plane come per instance.
fn vs_count_placed(in: VsIn, @location(4) plane: u32) -> CountOut {
    return count_corner(in, plane);
}

// One corner of a closed solid for plane `plane`, off screen unless visible, closed and no plane.
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

// Crossings of the view ray behind a plane, and the samples they cover.
struct CountColor {
    @location(0) count: vec4<f32>, // x: one crossing, SELECTED_CROSSING for a selected solid
    @builtin(sample_mask) mask: u32, // samples behind the plane
}

// Distance past plane `plane` as the eye sees it: positive behind the plane.
fn behind(plane: u32, p: vec3<f32>) -> f32 {
    return -clip_eye_side(plane) * clip_distance(plane, p);
}

// +1 where the view ray leaves a solid; a back face, unless the solid is mirrored or inward.
fn leaving(in: CountOut, front: bool) -> bool {
    return front == (in.flip != 0u);
}

@fragment
// +1 where the ray leaves a solid behind the plane, -1 where it enters one; the sum is inside.
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

@group(3) @binding(5) var<storage, read_write> pick_caps: array<atomic<u32>>; // per plane and pick pixel: count, id sum, nearest exit, owner

// First word of plane `plane`'s record at pick pixel `px`.
fn pick_cap_word(plane: u32, px: vec2<f32>) -> u32 {
    return clip_pick_word(plane, px, vec2<u32>(u32(line.vp_w), u32(line.vp_h)));
}

@fragment
// One crossing behind the plane into the pick's count and signed id sum; exits race for the nearest.
fn fs_pick_count(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0) {
        discard;
    }

    let leaves = leaving(in, front);
    let sign = select(-1, 1, leaves);
    let word = pick_cap_word(in.plane, in.pos.xy);
    atomicAdd(&pick_caps[word], bitcast<u32>(sign));
    atomicAdd(&pick_caps[word + 1u], bitcast<u32>(sign * i32(in.inst_id + 1u)));

    if (leaves) {
        atomicMax(&pick_caps[word + 2u], bitcast<u32>(in.pos.z));
    }

    return vec4<f32>(0.0);
}

@fragment
// The nearest exit behind the plane names the owner where solids nest.
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
