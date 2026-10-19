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
