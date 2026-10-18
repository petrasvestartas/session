fn ambient_vertex(position:vec3<f32>,colour:vec4<f32>,normal:vec3<f32>,object:ObjectSettings)->Vertex {
 let input=geometry(position,colour,normal,object);return Vertex(input.position,input.world,input.clip_flags,bitcast<f32>(object.range.z));
}