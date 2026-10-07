use crate::{scene::Scene, prepared::PreparedMesh, stroke::PreparedLine, chain::PreparedChain, marker::PreparedPoint, camera::Camera};
use session_rust::{Mesh, Line, Polyline, NurbsCurve, Point};
use std::rc::Rc;

pub fn hidden_scene() -> Scene {
    let mut scene = Scene::demo(); scene.clear();
    let mut mesh = Mesh::create_box(1.0, 1.0, 1.0); mesh.is_visible = false; scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();
    let mut line = Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0); line.is_visible = false;
    scene.insert_line(PreparedLine::new(Rc::new(line)).unwrap()).unwrap();
    let points = vec![Point::new(-0.6, -0.4, 0.0), Point::new(0.0, 0.6, 0.0), Point::new(0.6, -0.4, 0.0)];
    let mut polyline = Polyline::new(points.clone()); polyline.is_visible = false;
    scene.insert_path(PreparedChain::polyline(Rc::new(polyline)).unwrap()).unwrap();
    let mut curve = NurbsCurve::create(false, 2, &points); curve.is_visible = false;
    scene.insert_path(PreparedChain::curve(Rc::new(curve)).unwrap()).unwrap();
    let mut point = Point::new(0.0, 0.0, 0.0); point.is_visible = false;
    scene.insert_point(PreparedPoint::new(Rc::new(point)).unwrap()).unwrap(); scene.show_controls(true); scene
}

#[test]
fn invisible_original_sources_remain_owned_but_cannot_fit_or_pick() {
    let scene = hidden_scene(); assert_eq!(scene.objects().len(), 1); assert_eq!(scene.lines().len(), 1);
    assert_eq!(scene.paths().len(), 2); assert_eq!(scene.points().len(), 1); assert!(scene.bounds().is_none()); assert!(scene.next(None).is_none());
    assert!(scene.control_markers().is_empty()); assert!(scene.selected_bounds(scene.objects()[0].id).is_none());
    let ray = Camera::default().ray([0.0, 0.0]).unwrap(); assert!(crate::picking::pick(&scene, &ray).is_none());
    assert_eq!(crate::memory::curves([&scene])[1], 1); assert_eq!(crate::memory::points([&scene])[1], 1);
}

#[test]
fn selection_cycles_past_hidden_sources_without_changing_their_rows() {
    let mut scene = hidden_scene(); let hidden = scene.objects()[0].id;
    let visible = scene.insert(PreparedMesh::new(Mesh::create_box(1.0, 1.0, 1.0)).unwrap()).unwrap();
    assert_eq!(scene.next(None), Some(visible)); assert_eq!(scene.next(Some(hidden)), Some(visible));
    assert_eq!(scene.next(Some(visible)), Some(visible));
    let ray = Camera::default().ray([0.0, 0.0]).unwrap(); assert_eq!(crate::picking::pick(&scene, &ray), Some(visible));
}
