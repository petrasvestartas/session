use crate::{cap_gpu_tests::{renderer,pixels,at,wait,matrix},ink_ranges::{EDGES,LINES,POINTS}};
fn generic_markers(r:&mut crate::renderer::Renderer,scene:&Scene){
    r.strokes.points_visible=false;
    r.markers.set_points(&r.device,scene.points().iter().map(|row|row.marker())).unwrap();
    r.markers.set_ids(&r.device,r.sources.marker_ids.iter().copied()).unwrap();
}
