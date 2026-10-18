pub fn relative(translation:[f64;3],anchor:[f64;3])->[f32;3] {std::array::from_fn(|i|(translation[i]-anchor[i])as f32)}
pub fn placed(place:&session_rust::Xform,anchor:[f64;3])->session_rust::Xform {
 let mut local=place.clone();for i in 0..3 {local.m[12+i]-=anchor[i];}local
}
#[derive(Default)]
pub struct Origin {anchor:Option<[f64;3]>,last_ms:f64}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rebase {pub anchor:[f64;3],pub moved:bool,pub pending:bool}
impl Origin {
 pub fn anchor(&self)->[f64;3] {self.anchor.unwrap_or([0.;3])}
 pub fn forget(&mut self){self.anchor=None;}
 pub fn retry_after_ms(&self,now:f64)->Option<u32>{
  now.is_finite().then(||(self.last_ms+201.-now).ceil().clamp(1.,201.)as u32)
 }
 pub fn update(&mut self,target:[f64;3],distance:f64,now:f64)->Option<Rebase>{
  if target.iter().any(|v|!v.is_finite())||!distance.is_finite()||distance<=0.||!now.is_finite(){return None;}
  let threshold=(distance*0.25).clamp(1000.,100000.);
  let need=self.anchor.is_none_or(|a|(0..3).map(|i|(a[i]-target[i]).powi(2)).sum::<f64>().sqrt()>threshold);
  let moved=need&&(self.anchor.is_none()||now-self.last_ms>200.);
  if moved {self.anchor=Some(target);self.last_ms=now;}
  Some(Rebase {anchor:self.anchor.unwrap(),moved,pending:need&&!moved})
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn origin_threshold_throttle_reset_and_invalid_inputs_keep_one_anchor(){
  let mut origin=Origin::default();assert_eq!(origin.update([0.;3],4000.,0.),Some(Rebase{anchor:[0.;3],moved:true,pending:false}));assert_eq!(origin.update([1000.,0.,0.],4000.,1.),Some(Rebase{anchor:[0.;3],moved:false,pending:false}));assert_eq!(origin.update([1000.25,0.,0.],4000.,200.),Some(Rebase{anchor:[0.;3],moved:false,pending:true}));assert_eq!(origin.update([1000.25,0.,0.],4000.,200.01),Some(Rebase{anchor:[1000.25,0.,0.],moved:true,pending:false}));assert!(!origin.update([99000.,0.,0.],1e15,300.).unwrap().moved);assert!(origin.update([200000.,0.,0.],1e15,500.).unwrap().moved);
  for (target,distance,now) in [([f64::NAN,0.,0.],1.,501.),([0.;3],0.,501.),([0.;3],f64::INFINITY,501.),([0.;3],1.,f64::NAN)]{assert!(origin.update(target,distance,now).is_none());}assert_eq!(origin.update([200000.,0.,0.],1.,501.).unwrap().anchor,[200000.,0.,0.]);origin.forget();assert!(origin.update([1e12,-1e12,1e12],1.,501.).unwrap().moved);
 }
}

#[cfg(test)]
mod delay_tests {
 use super::*;
 #[test]
 fn retry_delay_waits_past_the_strict_origin_throttle(){
  let mut origin=Origin::default();origin.update([0.;3],3.,0.).unwrap();assert!(origin.update([2000.,0.,0.],3.,100.).unwrap().pending);assert_eq!(origin.retry_after_ms(100.),Some(101));assert_eq!(origin.retry_after_ms(200.),Some(1));assert_eq!(origin.retry_after_ms(-100.),Some(201));assert_eq!(origin.retry_after_ms(f64::NAN),None);assert_eq!(origin.retry_after_ms(f64::INFINITY),None);assert!(origin.update([2000.,0.,0.],3.,201.).unwrap().moved);
 }
}
