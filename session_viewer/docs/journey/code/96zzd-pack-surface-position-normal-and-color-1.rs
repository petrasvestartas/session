pub const NO_NORMAL:u32=0x8000_8000;
pub fn oct32(normal:[f32;3])->u32{
 let length=normal.iter().map(|v|v.abs()).sum::<f32>();
 if length==0.||length.is_nan(){return NO_NORMAL;}
 let (x,y)=(normal[0]/length,normal[1]/length);
 let sign=|v:f32|if v<0.{-1.}else{1.};
 let (x,y)=if normal[2]<0.{((1.-y.abs())*sign(x),(1.-x.abs())*sign(y))}else{(x,y)};
 let half=|v:f32|u32::from((v*32767.).round().clamp(-32766.,32767.)as i16 as u16);
 half(x)|(half(y)<<16)
}
pub fn pack(vertex:[f32;6],normal:[f32;3])->[u32;5]{
 let color=[vertex[3],vertex[4],vertex[5],1.].map(|v|(v.clamp(0.,1.)*255.).round()as u8);
 [vertex[0].to_bits(),vertex[1].to_bits(),vertex[2].to_bits(),oct32(normal),u32::from_le_bytes(color)]
}
pub fn bytes(mesh:&crate::mesh::Mesh)->Vec<u8>{
 mesh.vertices().iter().zip(mesh.normals()).flat_map(|(v,n)|pack(*v,*n)).flat_map(u32::to_ne_bytes).collect()
}
