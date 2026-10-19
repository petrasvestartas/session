@group(3) @binding(0) var<storage,read> cap_vertices:array<PackedVertex>;
@group(3) @binding(1) var<uniform> cap_plane:vec4<u32>;
@group(3) @binding(2) var<storage,read> cap_indices:array<u32>;
@vertex fn vs_record_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=cap_vertices[cap_indices[occurrence+object.range.x]];return count_corner(packed_position(vertex),cap_plane.x,object);}
fn behind(plane: u32, p: vec3<f32>) -> f32 {