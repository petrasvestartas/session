if (matrix_hidden(in.pos, 0u)) {
discard;
}
return vec2<u32>(course_owners[words[in.block + INSTANCE]], 0u);
}

@group(2) @binding(2) var scene_depth_single: texture_depth_2d;
