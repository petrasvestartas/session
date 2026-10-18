pub fn relative(translation:[f64;3],anchor:[f64;3])->[f32;3] {std::array::from_fn(|i|(translation[i]-anchor[i])as f32)}
#[derive(Default)]
pub struct Origin {anchor:Option<[f64;3]>,last_ms:f64}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rebase {pub anchor:[f64;3],pub moved:bool,pub pending:bool}
impl Origin {
 pub fn forget(&mut self){self.anchor=None;}
 pub fn update(&mut self,target:[f64;3],distance:f64,now:f64)->Option<Rebase>{
  if target.iter().any(|v|!v.is_finite())||!distance.is_finite()||distance<=0.||!now.is_finite(){return None;}
  let threshold=(distance*0.25).clamp(1000.,100000.);
  let need=self.anchor.is_none_or(|a|(0..3).map(|i|(a[i]-target[i]).powi(2)).sum::<f64>().sqrt()>threshold);
  let moved=need&&(self.anchor.is_none()||now-self.last_ms>200.);
  if moved {self.anchor=Some(target);self.last_ms=now;}
  Some(Rebase {anchor:self.anchor.unwrap(),moved,pending:need&&!moved})
 }
}
