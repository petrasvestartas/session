struct ViewSettings {transform:mat4x4f,viewport:vec4f,display:vec4f,eye:vec4f}
@group(0) @binding(0) var<uniform> view:ViewSettings;
@group(1) @binding(0) var<uniform> object:ObjectSettings;
@group(3) @binding(0) var<storage,read> mesh_values:array<PackedVertex>;
@group(3) @binding(2) var<storage,read> mesh_indices:array<u32>;
struct PrintOutput {@builtin(position) position:vec4f,@location(0) color:vec4f,@location(1) @interpolate(flat) owner:u32}
fn print_vertex(occurrence:u32)->PrintOutput {
    let vertex=mesh_values[mesh_indices[object.range.x+occurrence]];
    let color=unpack4x8unorm(vertex.color);
    let tint=select(vec3f(1.),object.face_colour.rgb,object.face_colour.w>0.5);
    let world=object.model*vec4f(packed_position(vertex),1.);
    return PrintOutput(view.transform*world,vec4f(select(color.rgb*tint,SELECTION_COLOR,(object.selection.x&1u)!=0u),color.a),object.selection.y);
}
@vertex fn vertex(@builtin(vertex_index) occurrence:u32)->PrintOutput {return print_vertex(occurrence);}
fn print_cut(input:PrintOutput,origin:vec2f)->bool {
    return clip_active()&&clip_cut_ndc(0u,clip_ndc(input.position.xy+origin,view.viewport.xy,input.position.z));
}
@fragment fn fragment(input:PrintOutput)->@location(0) vec4f {
    if(print_cut(input,vec2f(0.))){discard;}
    return input.color;
}
