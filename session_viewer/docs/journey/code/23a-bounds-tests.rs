use crate::bounds::Bounds;

#[test]
fn bounds_measure_vertices_and_define_the_empty_case() {
    let mut bounds = Bounds::point([-2.0, 1.0, 0.0]);
    bounds.include([2.0, 3.0, 2.0]);
    assert_eq!(bounds.centre(), [0.0, 2.0, 1.0]);
    assert!((bounds.radius() - 6.0_f64.sqrt()).abs() < 1.0e-12);
    let mut scene = crate::scene::Scene::demo();
    assert!(scene.bounds().is_some());
    while let Some(id) = scene.next(None) { scene.remove(id); }
    assert!(scene.bounds().is_none());
}
