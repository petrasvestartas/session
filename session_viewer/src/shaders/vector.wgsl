// Vectors: one instance per VectorRow, its corners from arrow.wgsl.
#include "arrow.wgsl"

@group(3) @binding(0) var<storage, read> vectors: array<VectorRow>; // one row per vector

// Corner `vid` of 18 of vector `row`.
@vertex
fn vs_main(@builtin(vertex_index) vid: u32, @builtin(instance_index) row: u32) -> VsOut {
    return arrow_corner(
        vectors[row].start,
        vectors[row].end,
        vectors[row].radius,
        vectors[row].instance_id,
        vectors[row].color,
        vectors[row].head,
        vectors[row].heads,
        vectors[row].start_trim,
        vectors[row].end_trim,
        vid,
        false,
    );
}
