fn ambient_vertex(position:vec3<f32>,object:ObjectSettings)->Vertex {
 let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,object.selection.z,bitcast<f32>(object.range.z));
}
@vertex fn vs_main(@location(0) position:vec3<f32>)->Vertex {return ambient_vertex(position,object_at(0u));}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,object);}
