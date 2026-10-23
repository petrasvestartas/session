if (coverage(in) < 0.5 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, 0u)) {
discard;
}
return vec2<u32>(course_owners[in.inst_id], 1u);
}

@group(2) @binding(2) var scene_depth_single: texture_depth_2d;
