// Preserve the physical angular predicate while rejecting degenerate faces before rasterization.
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(0) @binding(1) var<uniform> line: LineUniform;
@group(0) @binding(2) var<storage, read> translations: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read> face_planes: array<FacePlane>;
@group(0) @binding(4) var<storage, read> source_indices: array<u32>;
@group(0) @binding(5) var<storage, read> vertex_faces: array<u32>;
@group(0) @binding(6) var<storage, read_write> filtered_indices: array<u32>;
@group(0) @binding(7) var<uniform> params: FaceFilterParams;

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
    feather: f32,
    occluder_rect: vec4<f32>,
    lit: f32,
};

struct FacePlane {
    point: vec3<f32>,
    instance_id: u32,
    normal: vec3<f32>,
    _pad: u32,
};
// The live prefix, not the storage binding's allocated capacity. Size 16, offsets 0/4/8.
struct FaceFilterParams {
    index_count: u32,
    row_width: u32,
    _pad: vec2<u32>,
};

// Float rounding and raster subpixel quantization can turn an edge-on CAD triangle into
// a sliver with arbitrary interpolated depth. Use an angular band of 32 single-precision
// epsilons around parallelism; this does not move either geometry or depth.
const PARALLEL_ROUNDOFF: f32 = 32.0 * 1.1920928955078125e-7;

fn has_projected_area(face: u32) -> u32 {
    if (face == 0u) { return 1u; }
    let plane = face_planes[face - 1u];
    let normal = plane.normal;
    var toward_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - (plane.point + translations[plane.instance_id].xyz);
    if (line.ortho_h > 0.0) {
        toward_eye = vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z);
    }
    let facing = dot(normal, toward_eye);
    return select(0u, 1u, facing * facing > PARALLEL_ROUNDOFF * PARALLEL_ROUNDOFF * dot(normal, normal) * dot(toward_eye, toward_eye));
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let triangle = gid.x + gid.y * params.row_width;
    if (triangle >= params.index_count / 3u) { return; }
    let at = 3u * triangle;
    let first = source_indices[at];
    // Flat(first) only reads this vertex. The other two may carry unrelated face tokens.
    let visible = has_projected_area(vertex_faces[first]) != 0u;
    filtered_indices[at] = first;
    filtered_indices[at + 1u] = select(first, source_indices[at + 1u], visible);
    filtered_indices[at + 2u] = select(first, source_indices[at + 2u], visible);
}
