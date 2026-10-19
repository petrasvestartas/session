use std::ops::Range;
pub struct Plan {pub range:Range<u32>,at:Option<usize>}
pub fn plan(free:&[Range<u32>],count:u32,end:u32)->Option<Plan> {
 if count>0 {if let Some(at)=free.iter().position(|span|span.end-span.start>=count){let first=free[at].start;return Some(Plan {range:first..first+count,at:Some(at)});}}
 Some(Plan {range:end..end.checked_add(count)?,at:None})
}
impl Plan {
 pub fn take(self,free:&mut Vec<Range<u32>>)->Range<u32> {
  if let Some(at)=self.at {free[at].start=self.range.end;if free[at].is_empty(){free.remove(at);}}
  self.range
 }
}
pub fn retire(free:&mut Vec<Range<u32>>,range:Range<u32>,mut end:u32)->u32 {
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
 *free=crate::slot_space::coalesce(std::mem::take(free),end);
 while free.last().is_some_and(|span|span.end==end){end=free.pop().unwrap().start;}end
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn plans_reuse_holes_merge_free_tails_and_refuse_overflow() {
  let mut free=vec![3..6,9..12];let first=plan(&free,2,12).unwrap();assert_eq!(first.range,3..5);assert_eq!(free,[3..6,9..12]);assert_eq!(first.take(&mut free),3..5);assert_eq!(free,[5..6,9..12]);assert_eq!(plan(&free,3,12).unwrap().take(&mut free),9..12);assert_eq!(free,[5..6]);assert_eq!(plan(&free,4,12).unwrap().take(&mut free),12..16);assert_eq!(free,[5..6]);
  let zero=plan(&free,0,16).unwrap();assert_eq!(zero.take(&mut free),16..16);assert_eq!(free,[5..6]);assert!(plan(&[],3,u32::MAX-1).is_none());assert_eq!(plan(&[0..4],3,u32::MAX-1).unwrap().range,0..3);
  let vertex=plan(&free,1,16).unwrap();assert!(plan(&[],3,u32::MAX-1).is_none());assert_eq!(free,[5..6]);assert_eq!(vertex.take(&mut free),5..6);
  let mut free=Vec::new();assert_eq!(retire(&mut free,3..6,9),9);assert_eq!(free,[3..6]);assert_eq!(retire(&mut free,6..9,9),3);assert!(free.is_empty());assert_eq!(retire(&mut free,0..3,3),0);assert!(free.is_empty());
 }
}
