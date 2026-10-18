pub(crate) fn changed_runs<T:PartialEq>(previous:&[T],next:&[T])->Vec<std::ops::Range<usize>> {
 let end=previous.len().min(next.len());let mut at=0;let mut runs=Vec::new();
 while at<end {
  if previous[at]==next[at]{at+=1;continue;}
  let first=at;while at<end&&previous[at]!=next[at]{at+=1;}
  runs.push(first..at);
 }
 runs
}
pub struct Slots