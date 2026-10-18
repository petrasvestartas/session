use std::ops::Range;
pub const EDGES:u8=1;
pub const LINES:u8=2;
pub const POINTS:u8=4;
pub const CONTROLS:u8=8;
pub const VERTICES:u8=16;

pub struct Runs {parts:Vec<(u8,Range<u32>)>,present:u8,pub mask:u8}
impl Default for Runs {fn default()->Self{Self{parts:Vec::new(),present:0,mask:u8::MAX}}}
impl Runs {
 pub fn push(&mut self,kind:u8,start:u32,end:u32){
  if start>=end{return;}self.present|=kind;
  if let Some((previous,range))=self.parts.last_mut()&&*previous==kind&&range.end==start {range.end=end;return;}
  self.parts.push((kind,start..end));
 }
 pub fn visible(&self,count:u32)->impl Iterator<Item=Range<u32>>+'_ {
  let all=self.mask!=0&&self.mask&self.present==self.present;
  let parts:&[_]=if all||self.mask&self.present==0 {&[]}else{&self.parts};
  all.then_some(0..count).into_iter().chain(parts.iter().filter(|(kind,_)|self.mask&kind!=0).map(|(_,range)|range.clone()))
 }
}
