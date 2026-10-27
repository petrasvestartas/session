struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32,@location(4) @interpolate(flat) face:vec3<f32>}
fn ambient_vertex(occurrence:u32,object:ObjectSettings)->Vertex {
 let input=mesh_vertex(occurrence,object.range.x);let output=geometry(input.position,input.colour,input.normal,object);
 let base=occurrence/3u*3u;let origin=mesh_vertex(base,object.range.x).position;
 let a=(object.model*vec4<f32>(mesh_vertex(base+1u,object.range.x).position-origin,0.0)).xyz;
 let b=(object.model*vec4<f32>(mesh_vertex(base+2u,object.range.x).position-origin,0.0)).xyz;
 return Vertex(output.position,output.world,output.clip_flags,bitcast<f32>(object.range.z),cross(a,b));
}
@vertex fn vs_main(@builtin(vertex_index) occurrence:u32)->Vertex{return ambient_vertex(occurrence,object_at(0u));}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex{return ambient_vertex(occurrence,object_at(row));}
struct Source{@location(0) radius:f32,@location(1) normal_x:f32,@location(2) normal_y:f32,@location(3) normal_z:f32}
@fragment fn fs_main(in:Vertex)->Source{
 if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 return Source(in.radius,in.face.x,in.face.y,in.face.z);
}
