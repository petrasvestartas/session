use crate::{scene::Scene, prepared::PreparedMesh};
use std::rc::Rc;

#[test]
fn cycling_skips_locked_and_hidden_rows_and_wraps_across_geometry_lanes() {
    let mut scene=Scene::demo(); scene.clear();
    let mut mesh=session_rust::Mesh::create_box(1.,1.,1.); mesh.is_locked=true;
    let locked=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();
    let mesh=scene.insert(PreparedMesh::new(session_rust::Mesh::create_box(1.,1.,1.)).unwrap()).unwrap();
    let mut line=session_rust::Line::new(0.,0.,0.,1.,0.,0.); line.is_locked=true;
    scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();
    let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.,0.,0.))).unwrap()).unwrap();
    let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(0.,0.,0.),session_rust::Point::new(1.,0.,0.)]))).unwrap()).unwrap();
    let mut hidden=session_rust::Point::new(0.,0.,0.); hidden.is_visible=false;
    scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(hidden)).unwrap()).unwrap();
    assert_eq!(scene.next(None),Some(mesh)); assert_eq!(scene.next(Some(locked)),Some(mesh));
    assert_eq!(scene.next(Some(mesh)),Some(point)); assert_eq!(scene.next(Some(point)),Some(path));
    assert_eq!(scene.next(Some(path)),Some(mesh)); scene.clear(); assert_eq!(scene.next(None),None);
}
