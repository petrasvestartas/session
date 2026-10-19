use super::*;
use crate::{cap_gpu_tests::{renderer,pixels,at,wait,matrix},ink_ranges::{EDGES,LINES,POINTS}};
#[test]
fn actual_filtered_instances_keep_absolute_colour_and_original_query_addresses(){
 for legacy in [false,true] {
  let mut scene=Scene::empty();for i in 0..3 {let mut p=session_rust::Point::new((i as f64-1.)*0.5,0.,-0.5);p.width=12.;p.pointcolor=session_rust::Color::new(if i==0{1.}else{0.},if i==1{1.}else{0.},if i==2{1.}else{0.},1.);scene.insert_point(crate::marker::PreparedPoint::new(std::rc::Rc::new(p)).unwrap()).unwrap();}
  let original=scene.points().iter().map(|r|r.prepared.source.to_proto()).collect::<Vec<_>>();let ids=scene.points().iter().map(|r|r.id).collect::<Vec<_>>();let mut r=renderer(&scene);if legacy{r.markers.use_legacy_fixture();}r.markers.ranges.push(EDGES,0,1);r.markers.ranges.push(LINES,1,2);r.markers.ranges.push(POINTS,2,3);
  for count in [1,4] {r.set_samples(count);let all=pixels(&r);assert!(at(&all,80,120)[0]>240&&at(&all,160,120)[1]>240&&at(&all,240,120)[2]>240);let usage=r.point_usage();let source_ids=r.sources.marker_ids.clone();r.markers.ranges.mask=LINES;let filtered=pixels(&r);assert_eq!(at(&filtered,80,120),at(&filtered,20,120));assert_eq!(at(&filtered,240,120),at(&filtered,20,120));assert_eq!(at(&filtered,160,120),at(&all,160,120),"A nonzero first_instance retains the middle source colour");
   for (x,owner)in [(0.,Some(ids[1].value())),(-0.5,None),(0.5,None)] {r.request_pick([x,0.],matrix()).unwrap();wait(&r);assert_eq!(r.take_pick(matrix()).unwrap().unwrap().map(|v|v[0]),owner,"Integer queries use the same absolute source range as colour");}
   assert_eq!(r.point_usage(),usage,"Presentation filtering keeps the original GPU marker upload");assert_eq!(r.sources.marker_ids,source_ids);assert_eq!(scene.points().iter().map(|r|r.prepared.source.to_proto()).collect::<Vec<_>>(),original);r.markers.ranges.mask=u8::MAX;assert!(pixels(&r)==all,"Returning the mask restores exact pixels without repacking instances");assert_eq!(r.point_usage(),usage);
  }
 }
}
