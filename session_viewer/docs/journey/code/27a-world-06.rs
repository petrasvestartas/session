use session_rust::Xform;

#[test]
fn bounds_and_picking_follow_the_placed_triangle() {
    let mut scene = Scene::demo();
    let id = scene.objects()[0].id;
    let before = scene.selected_bounds(id).unwrap();
    scene.place(id, Xform::translation(2.0, -1.0, 0.5)).unwrap();
    let bounds = scene.selected_bounds(id).unwrap();
    for i in 0..3 {
        assert!((bounds.min[i] - before.min[i] - [2.0, -1.0, 0.5][i]).abs() < 1e-12);
        assert!((bounds.max[i] - before.max[i] - [2.0, -1.0, 0.5][i]).abs() < 1e-12);
    }
    let object = &scene.objects()[0];
    let mut centre = [0.0; 3];
    for v in object.mesh.vertices() {
        let world = object.world_point(*v);
        for i in 0..3 { centre[i] += world[i] / 3.0; }
    }
    let mut camera = crate::camera::Camera::default();
    camera.fit(&bounds);
    let point = session_rust::Point::new(centre[0], centre[1], centre[2]);
    let screen = camera.view_projection().transform_point(&point);
    let ray = camera.ray([screen[0] as f32, screen[1] as f32]).unwrap();
    assert_eq!(crate::picking::pick(&scene, &ray), Some(id));
    let all = scene.bounds().unwrap();
    assert!(all.max[0] >= bounds.max[0] && all.min[1] <= bounds.min[1]);
}
