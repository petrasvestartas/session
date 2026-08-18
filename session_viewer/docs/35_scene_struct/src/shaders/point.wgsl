// The RAW cloud lane. A scanned cloud is not a list of decorated dots - it is a clump of
// millions of samples, and the only thing it needs per point is a pixel.
//
// One vertex per point, PrimitiveTopology::PointList, opaque. That is the whole design, and
// each half of it is load-bearing:
//   - ONE vertex, not the three a triangle needs for a round dot: at 13.8M points that is
//     13.8M vertices instead of 41.4M.
//   - OPAQUE, no discard and no alpha ramp: blending forces every fragment to be shaded in
//     submission order with early-Z off. A 3 px blended dot covers ~38 px, so 13.8M of them
//     asked for ~520M blended fragments per frame against 2.5M screen pixels - 200x overdraw,
//     measured at ~100 ms/frame. Depth-tested opaque points reject occluded samples before the
//     fragment shader ever runs.
// WebGPU rasterises a point-list point as EXACTLY one pixel (WGSL has no gl_PointSize), which
// is why the sized round dots live on in glyph.wgsl for small clouds - see scene.rs.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// TWO flat arrays, not one interleaved row of structs. In the STORAGE address space
// array<f32> has element stride 4 and array<u32> stride 4 - no vec3 16-byte padding, which is
// a uniform-address-space rule and does not apply here - so a point costs exactly 12 + 4 = 16 B
// against the old CloudPoint's 32. Splitting them is also what lets a file stream in one
// forward pass: coords and colours are separate contiguous runs on the protobuf wire, and
// queue.write_buffer cannot do strided writes.
@group(3) @binding(0) var<storage, read> positions: array<f32>;  // 3 per point
@group(3) @binding(1) var<storage, read> colors: array<u32>;     // RGBA8, 1 per point

const FLAG_HIDDEN: u32 = 2u;   // Instance::FLAG_HIDDEN, bit 1

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

// vid IS the point index, and it is ABSOLUTE: WebGPU counts vertex_index from the draw's
// first_vertex, so one shared buffer holding every cloud needs no per-point base. The cloud's
// instance rides on instance_index the same way - one draw call per cloud carries it, which is
// how the per-point instance_id (4 B x 13.8M) left the row.
@vertex
fn vs_main(@builtin(vertex_index) vid: u32, @builtin(instance_index) iid: u32) -> VsOut{
    let i = vid * 3u;
    let local = vec3<f32>(positions[i], positions[i + 1u], positions[i + 2u]);
    let inst = instances[iid];
    let world = (inst.model * vec4<f32>(local, 1.0)).xyz;

    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    // A hidden cloud leaves the same way hidden geometry leaves every other lane: pushed behind
    // the near plane so the clip stage drops it, no per-fragment test.
    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        o.pos = vec4<f32>(0.0, 0.0, -1.0, 1.0);
    }
    o.color = unpack4x8unorm(colors[vid]) * inst.color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // No SDF, no AA ramp, no discard, and deliberately NO sample_mask output: the shader mask is
    // ANDed with rasterizer coverage on every backend, so it can only clear bits, never set them
    // - writing all-ones is inert, and merely declaring it makes coverage shader-dependent, which
    // is what demotes an early-Z rejection to late-Z. Early-Z is the entire point of this lane.
    // Under MSAA a point still covers one sample per lattice, so it stays one pixel of energy.
    return vec4<f32>(in.color.rgb, 1.0);
}
