use crate::packed_vertex::{pack,oct32,NO_NORMAL};
fn decode(word:u32)->[f64;3]{
 let x=(word as u16 as i16)as f64/32767.;let y=((word>>16)as u16 as i16)as f64/32767.;let z=1.-x.abs()-y.abs();
 let(x,y)=if z<0.{((1.-y.abs())*if x<0.{-1.}else{1.},(1.-x.abs())*if y<0.{-1.}else{1.})}else{(x,y)};
 let length=(x*x+y*y+z*z).sqrt();[x/length,y/length,z/length]
}
#[test]
fn compact_vertex_preserves_position_bits_and_current_color_quantization(){
 let v=[-0.,123456.25,-0.0009765625,1.2,0.5,-0.2];let before=v;let row=pack(v,[0.;3]);
 assert_eq!(std::mem::size_of_val(&row),20);assert_eq!(&row[..3],&v[..3].iter().map(|v|v.to_bits()).collect::<Vec<_>>());assert_eq!(row[3],NO_NORMAL);assert_eq!(row[4].to_le_bytes(),[255,128,0,255]);assert_eq!(v,before);
}
#[test]
fn octahedral_words_reserve_missing_normals_and_bound_direction_error(){
 assert_eq!(oct32([0.;3]),NO_NORMAL);assert_eq!(oct32([f32::NAN,0.,0.]),NO_NORMAL);
 for x in -8..=8{for y in -8..=8{for z in -8..=8{if x==0&&y==0&&z==0{continue}let normal=[x as f32,y as f32,z as f32];let word=oct32(normal);assert_ne!(word,NO_NORMAL);let decoded=decode(word);let length=((x*x+y*y+z*z)as f64).sqrt();let dot=decoded[0]*x as f64/length+decoded[1]*y as f64/length+decoded[2]*z as f64/length;assert!(dot>0.99999998,"direction error for {normal:?}");}}}
}
