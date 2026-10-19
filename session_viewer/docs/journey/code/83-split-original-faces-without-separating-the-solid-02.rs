use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
use session_rust::{BRep, Line, Point, Xform};
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn cutter(e: &mut Editor, a: &Point, b: &Point) -> crate::scene::ObjectId {
    e.scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(Line::from_points(a, b))).unwrap())
        .unwrap()
}
#[test]
fn split_face_keeps_solid_joined_original_area_identity_and_one_undo() {
    let mut e = editor();
    let brep = Rc::new(BRep::create_box(10., 10., 10.));
    let original_area = brep.face_meshes_q(Some((20., 0.005)))[0].area();
    let s = &brep.m_surfaces[0];
    let (a, u, v) = (
        s.get_cv(0, 0).unwrap(),
        s.get_cv(1, 0).unwrap(),
        s.get_cv(0, 1).unwrap(),
    );
    let p = |x: f64, y: f64| {
        Point::new(
            a[0] + x * (u[0] - a[0]) + y * (v[0] - a[0]),
            a[1] + x * (u[1] - a[1]) + y * (v[1] - a[1]),
            a[2] + x * (u[2] - a[2]) + y * (v[2] - a[2]),
        )
    };
    let target = e.scene.insert_cad(brep.clone()).unwrap();
    let cut = cutter(&mut e, &p(0.5, -1.), &p(0.5, 2.));
    let guid = e.scene.row_guid(target).unwrap().to_owned();
    e.select_objects([target], false);
    assert!(e.apply(Action::Split).is_err());
    assert!(e.split.is_none());
    assert!(crate::split_draft::Draft::new(&e.scene, target, Some(99)).is_err());
    e.apply(Action::PickedTarget(Some(crate::pick_sources::Target {
        owner: target,
        part: crate::pick_sources::Part::Face(0),
    })))
    .unwrap();
    e.apply(Action::Split).unwrap();
    assert_eq!(e.split.as_ref().unwrap().face, Some(0));
    e.apply(Action::SplitPick(Some(cut))).unwrap();
    e.apply(Action::Split).unwrap();
    let result = e.scene.objects()[0].cad.as_ref().unwrap().source.clone();
    assert_eq!(result.face_count(), 7);
    assert!(result.is_solid());
    assert_eq!(brep.face_count(), 6);
    assert_eq!(e.scene.row_guid(target), Some(guid.as_str()));
    assert_eq!(e.scene.objects().len(), 1);
    assert_eq!(e.scene.lines().len(), 1);
    let meshes = result.face_meshes_q(Some((20., 0.005)));
    assert!((meshes[0].area() - original_area / 2.).abs() < 1e-6);
    assert!((meshes[6].area() - original_area / 2.).abs() < 1e-6);
    let mut reopened = Scene::empty();
    crate::scene_document::open(
        &mut reopened,
        &crate::scene_archive::save(&e.scene).unwrap(),
    )
    .unwrap();
    assert_eq!(
        reopened.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
            .to_proto(),
        result.to_proto()
    );
    e.apply(Action::Undo).unwrap();
    assert!(Rc::ptr_eq(
        &brep,
        &e.scene.objects()[0].cad.as_ref().unwrap().source
    ));
    e.apply(Action::Redo).unwrap();
    assert_eq!(
        e.scene.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
            .face_count(),
        7
    );
}
#[test]
fn placed_original_surface_becomes_one_trimmed_brep_with_the_same_guid_and_exact_archive() {
    let mut e = editor();
    let surface = Rc::new(
        crate::patch_geometry::patch(&[
            Point::new(-2., -2., 0.),
            Point::new(2., -2., 0.),
            Point::new(2., 2., 0.),
            Point::new(-2., 2., 0.),
        ])
        .unwrap(),
    );
    let target = e.scene.insert_surface(surface.clone()).unwrap();
    let model = Xform::translation(100., 20., 3.);
    e.scene.place_owner(target, model.clone()).unwrap();
    let cut = cutter(
        &mut e,
        &Point::new(100., 17., 3.),
        &Point::new(100., 23., 3.),
    );
    let guid = e.scene.row_guid(target).unwrap().to_owned();
    e.select_objects([target], false);
    e.apply(Action::Split).unwrap();
    e.apply(Action::SplitPick(Some(cut))).unwrap();
    e.apply(Action::Split).unwrap();
    let row = &e.scene.objects()[0];
    assert_eq!(row.id, target);
    assert_eq!(row.model.m, model.m);
    assert!(row.surface.is_none());
    let brep = row.cad.as_ref().unwrap().source.clone();
    assert_eq!(brep.face_count(), 2);
    assert_eq!(brep.guid(), surface.guid());
    assert_eq!(e.scene.row_guid(target), Some(guid.as_str()));
    assert_eq!(e.scene.objects().len(), 1);
    let meshes = brep.face_meshes_q(Some((20., 0.005)));
    assert!((meshes.iter().map(|m| m.area()).sum::<f64>() - 16.).abs() < 1e-6);
    let mut reopened = Scene::empty();
    crate::scene_document::open(
        &mut reopened,
        &crate::scene_archive::save(&e.scene).unwrap(),
    )
    .unwrap();
    assert_eq!(
        reopened.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
            .to_proto(),
        brep.to_proto()
    );
    assert_eq!(reopened.objects()[0].model.m, model.m);
    e.apply(Action::Undo).unwrap();
    assert!(Rc::ptr_eq(
        &surface,
        &e.scene.objects()[0].surface.as_ref().unwrap().source
    ));
}
#[test]
fn off_surface_cutters_make_no_division_and_preserve_originals_and_redo() {
    let mut e = editor();
    let source = Rc::new(
        crate::patch_geometry::patch(&[
            Point::new(-2., -2., 0.),
            Point::new(2., -2., 0.),
            Point::new(2., 2., 0.),
            Point::new(-2., 2., 0.),
        ])
        .unwrap(),
    );
    let target = e.scene.insert_surface(source.clone()).unwrap();
    let cut = cutter(&mut e, &Point::new(0., -3., 1.), &Point::new(0., 3., 1.));
    e.apply(Action::AddPoint).unwrap();
    e.apply(Action::Undo).unwrap();
    e.select_objects([target], false);
    e.apply(Action::Split).unwrap();
    e.apply(Action::SplitPick(Some(cut))).unwrap();
    e.apply(Action::Split).unwrap();
    assert!(e.tool_status.starts_with("No division"));
    assert!(e.split.is_none());
    assert!(Rc::ptr_eq(
        &source,
        &e.scene.objects()[0].surface.as_ref().unwrap().source
    ));
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.points().len(), 1);
}
