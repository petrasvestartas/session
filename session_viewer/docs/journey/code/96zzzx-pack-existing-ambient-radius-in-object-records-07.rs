fn ambient_vertex(position:vec3<f32>,normal:vec3<f32>,object:ObjectSettings,radius:f32)->Vertex {
 let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(normal,0.)).xyz,object.selection.z,radius);
}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:u32)->Vertex {return ambient_vertex(position,oct32_decode(normal),object_at(0u),ambient_radius.x);}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,input.normal,object,bitcast<f32>(object.range.z));}
