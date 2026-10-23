if (coverage(in) < 0.5 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, 0u)) {
discard;
}
return vec2<u32>(course_owners[in.inst_id], 1u);
}
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
