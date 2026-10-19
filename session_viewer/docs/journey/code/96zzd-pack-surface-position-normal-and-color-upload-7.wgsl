struct PackedVertex{x:f32,y:f32,z:f32,normal:u32,color:u32}
fn packed_position(vertex:PackedVertex)->vec3<f32>{return vec3<f32>(vertex.x,vertex.y,vertex.z);}
fn oct32_decode(