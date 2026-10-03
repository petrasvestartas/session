use crate::scene::Scene;
use session_rust::Xform;

#[test]
fn placement_keeps_the_shared_local_mesh() {
    let mut scene = Scene::demo();
    let object = &scene.objects()[0];
    let id = object.id;
    let mesh = object.mesh.clone();
    let local = mesh.vertices()[0];
    scene.place(id, Xform::translation(2.0, -1.0, 0.5)).unwrap();
    let object = &scene.objects()[0];
    assert!(std::rc::Rc::ptr_eq(&mesh, &object.mesh));
    assert_eq!(object.mesh.vertices()[0], local);
    assert_eq!(object.world_point(local), [local[0] as f64 + 2.0, local[1] as f64 - 1.0, local[2] as f64 + 0.5]);
}

#[test]
fn failed_placement_changes_nothing() {
    let mut scene = Scene::demo();
    let id = scene.objects()[0].id;
    let before = scene.objects()[0].model.m;
    assert!(scene.place(id, Xform::translation(f64::NAN, 0.0, 0.0)).is_err());
    let mut projective = Xform::identity();
    projective.m[3] = 0.1;
    assert!(scene.place(id, projective).is_err());
    assert_eq!(scene.objects()[0].model.m, before);
    scene.remove(id);
    assert!(scene.place(id, Xform::identity()).is_err());
}
