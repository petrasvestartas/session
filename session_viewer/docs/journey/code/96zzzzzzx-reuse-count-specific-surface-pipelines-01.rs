pub struct Variants<T> {
 pub active:T,
 pub alternate:Option<T>,
 pub samples:u32,
}
impl<T> Variants<T> {
 pub fn new(active:T,samples:u32)->Self {
  assert!(matches!(samples,1|4));Self {active,alternate:None,samples}
 }
 pub fn select(&mut self,samples:u32,create:impl FnOnce()->T)->bool {
  assert!(matches!(samples,1|4));if self.samples==samples {return false;}
  let alternate=self.alternate.get_or_insert_with(create);
  std::mem::swap(&mut self.active,alternate);self.samples=samples;true
 }
}
