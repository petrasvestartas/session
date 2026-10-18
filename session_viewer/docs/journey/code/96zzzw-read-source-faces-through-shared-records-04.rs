fn face_geometry(index:u32,object:ObjectSettings)->FaceOutput {
 let vertex=positions[indices[index+object.range.x]];
 let placed=geometry(packed_position(vertex),unpack4x8unorm(vertex.color),vec3<f32>(0.),object);
 return FaceOutput(crop.transform*placed.position,faces[index/3u]+source.base,placed.world,placed.clip_flags,object.selection.y);
}
@vertex fn face_vertex(@builtin(vertex_index) index:u32)->FaceOutput {return face_geometry(index,object_at(0u));}
@vertex fn face_record_vertex(@builtin(vertex_index) index:u32,@location(0) row:u32)->FaceOutput {return face_geometry(index,object_at(row));}
