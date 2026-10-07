// Vectors: one instance per VectorRow, its corners from arrow.wgsl.
#include "arrow.wgsl"

@group(3) @binding(0) var<storage, read> vectors: array<VectorRow>; // one row per vector

// Corner `vid` of 18 of vector `row`.
@vertex
fn vs_main(@builtin(vertex_index) vid: u32, @builtin(instance_index) row: u32) -> VsOut {
    return arrow_corner(vectors[row], vid, false);
}
