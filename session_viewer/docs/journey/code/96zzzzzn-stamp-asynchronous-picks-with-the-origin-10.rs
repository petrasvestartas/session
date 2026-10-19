#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct View {transform:[u32;16],size:[u32;2],density:u32,origin:[u64;3]}

impl View {
    // Readback belongs to its matrix, physical viewport, CSS density and precise origin, even when the document has not changed.
    pub fn new(transform:[f32;16],size:[u32;2],density:f32)->Result<Self,&'static str>{Self::anchored(transform,size,density,[0.;3])}
    pub fn anchored(transform:[f32;16],size:[u32;2],density:f32,origin:[f64;3])->Result<Self,&'static str>{
        if origin.iter().any(|v|!v.is_finite())||transform.iter().any(|n| !n.is_finite()) || size.contains(&0) || !density.is_finite() || density <= 0.0 {
            return Err("Invalid pick view");
        }
        Ok(Self { transform: transform.map(f32::to_bits), size, density:density.to_bits(),origin:origin.map(f64::to_bits) })
    }
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn origin_bits_complete_the_pick_snapshot_and_reject_invalid_inputs(){
  let matrix=session_rust::Xform::identity().to_f32();let size=[320,240];let zero=View::new(matrix,size,1.).unwrap();assert_eq!(zero,View::anchored(matrix,size,1.,[0.;3]).unwrap());assert_ne!(zero,View::anchored(matrix,size,1.,[2000.,0.,0.]).unwrap());assert_ne!(zero,View::anchored(matrix,size,1.,[-0.,0.,0.]).unwrap());assert!(View::anchored(matrix,size,1.,[f64::NAN,0.,0.]).is_err());assert!(View::anchored(matrix,[0,240],1.,[0.;3]).is_err());assert!(View::anchored(matrix,size,0.,[0.;3]).is_err());let mut invalid=matrix;invalid[0]=f32::NAN;assert!(View::anchored(invalid,size,1.,[0.;3]).is_err());
 }
}
