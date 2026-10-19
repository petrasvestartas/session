use crate::{
    editor::{Action, Editor},
    measure_mark::{Kind, Unit},
    scene::Scene,
};
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn command(e: &mut Editor, text: &str) -> Result<(), String> {
    let action = if let Some(a) = e.tool_line(text) {
        a.map_err(str::to_owned)?
    } else {
        let crate::commands::Command::Editor(a) = crate::commands::parse(text)? else {
            return Err("Editor action".into());
        };
        a
    };
    e.apply(action).map_err(|m| {
        if m == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            m.into()
        }
    })?;
    Ok(())
}
#[test]
fn bounding_box_is_an_original_wire_box_with_world_lengths_exact_archive_and_one_undo() {
    let mut e = editor();
    command(&mut e, "Line w1000000000.125,23,31 w1000000006.125,29,31").unwrap();
    let old = e.scene.lines()[0].prepared.source.clone();
    command(&mut e, "Select All").unwrap();
    command(&mut e, "Bounding Box").unwrap();
    let row = &e.scene.objects()[0];
    let target = row.id;
    let Some(Geometry::OBB(source)) = &row.original else {
        panic!("Original box")
    };
    assert_eq!(source.center[0], 1000000003.125);
    assert_eq!(
        [
            source.half_size[0],
            source.half_size[1],
            source.half_size[2]
        ],
        [3., 3., 0.]
    );
    assert_eq!(row.model.m, Xform::identity().m);
    assert!(row.mesh.indices().is_empty());
    assert_eq!(row.mesh.parts.edges.len(), 12);
    let proto = source.to_proto();
    command(&mut e, "Length BoundingBox").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "X 6 · Y 6 · Z 0 mm");
    assert_eq!(e.selected_objects(), [target]);
    assert!(Rc::ptr_eq(&old, &e.scene.lines()[0].prepared.source));
    let mut restored = Scene::empty();
    crate::scene_document::open(
        &mut restored,
        &crate::scene_archive::save(&e.scene).unwrap(),
    )
    .unwrap();
    let Some(Geometry::OBB(source)) = &restored.objects()[0].original else {
        panic!("Reopened original box")
    };
    assert_eq!(source.to_proto(), proto);
    assert!(restored.objects()[0].mesh.indices().is_empty());
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.objects().is_empty());
    assert!(e.mark.is_none());
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.objects().len(), 1);
}
#[test]
fn selection_measurements_use_placed_originals_and_preserve_scene_history_and_sources() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w3,4,0").unwrap();
    let line = e.scene.lines()[0].id;
    e.scene
        .place_owner(line, Xform::scale_xyz(2., 3., 1.))
        .unwrap();
    let old = e.scene.lines()[0].prepared.source.clone();
    command(&mut e, "Point w1,1,1").unwrap();
    command(&mut e, "Undo").unwrap();
    e.select_objects([line], false);
    command(&mut e, "Length").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "13.416 mm");
    assert!(Rc::ptr_eq(&old, &e.scene.lines()[0].prepared.source));
    command(&mut e, "Length BoundingBox").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "X 6 · Y 12 · Z 0 mm");
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    assert!(e.mark.is_none());
}
#[test]
fn area_volume_face_units_and_open_refusal_follow_original_geometry() {
    let mut e = editor();
    command(&mut e, "Box w0,0,0 2 2 0.4").unwrap();
    let target = e.scene.objects()[0].id;
    e.select_objects([target], false);
    command(&mut e, "Area").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "11.2 mm²");
    command(&mut e, "Volume").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "1.6 mm³");
    e.camera.unit = Unit::Meters;
    command(&mut e, "Volume").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "1.6 m³");
    let source = e.scene.objects()[0].cad.as_ref().unwrap().source.clone();
    e.apply(Action::PickedTarget(Some(crate::pick_sources::Target {
        owner: target,
        part: crate::pick_sources::Part::Face(0),
    })))
    .unwrap();
    command(&mut e, "Area").unwrap();
    let area =
        crate::measure_area::compute_area(&Geometry::BRep(source), &Xform::identity(), Some(0))
            .unwrap();
    assert_eq!(
        e.mark.as_ref().unwrap().label,
        format!("{} m²", crate::measure_math::to_text(area))
    );
    let surface = Rc::new(
        crate::patch_geometry::patch(&[
            Point::new(3., 0., 0.),
            Point::new(4., 0., 0.),
            Point::new(4., 1., 0.),
            Point::new(3., 1., 0.),
        ])
        .unwrap(),
    );
    let id = e.scene.insert_surface(surface).unwrap();
    e.select_objects([id], false);
    assert!(command(&mut e, "Volume").is_err());
    assert!(e.mark.is_none());
    assert!(
        e.scene
            .objects()
            .iter()
            .find(|r| r.id == id)
            .unwrap()
            .surface
            .is_some()
    );
}
#[test]
fn distance_accepts_world_relative_and_plane_inputs_without_creating_originals() {
    let mut e = editor();
    command(&mut e, "CPlane XZ").unwrap();
    command(&mut e, "Measure Distance w0,0,0 w3,4,0").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "5 mm");
    assert!(e.tool_status.contains("dx 3 · dy 4 · dz 0"));
    assert!(e.scene.lines().is_empty());
    assert!(e.scene.objects().is_empty());
    command(&mut e, "Snap Off").unwrap();
    command(&mut e, "Measure Distance").unwrap();
    assert!(e.snapping.objects);
    command(&mut e, "0,0,0").unwrap();
    command(&mut e, "@3,4,0").unwrap();
    assert_eq!(
        e.mark.as_ref().unwrap().points,
        vec![[0., 0., 0.], [3., 0., 4.]]
    );
    assert_eq!(e.mark.as_ref().unwrap().label, "5 mm");
    assert!(e.draft.is_none());
    e.apply(Action::Zoom(1.1)).unwrap();
    assert!(e.mark.is_some());
    command(&mut e, "Escape").unwrap();
    assert!(e.mark.is_none());
    assert!(e.scene.lines().is_empty());
}
#[test]
fn invalid_measurement_arguments_keep_pending_answers_and_redo_and_no_source_proxy_is_measured() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w1,0,0").unwrap();
    command(&mut e, "Point w1,1,1").unwrap();
    command(&mut e, "Undo").unwrap();
    e.select_objects([], false);
    assert!(command(&mut e, "BoundingBox").is_err());
    assert!(command(&mut e, "Length").is_err());
    assert!(crate::commands::parse("Length BoundingBox extra").is_err());
    command(&mut e, "Curve").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "Measure Distance bad").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    let id = crate::bounds_original::insert(
        &mut e.scene,
        Rc::new(crate::bounds_original::box_geometry(
            &crate::bounds::Bounds {
                min: [0.; 3],
                max: [1.; 3],
            },
        )),
    )
    .unwrap();
    e.select_objects([id], false);
    assert!(command(&mut e, "Area").is_err());
    assert!(command(&mut e, "Volume").is_err());
    assert!(command(&mut e, "Length").is_err());
}
#[test]
fn temporary_annotation_ids_do_not_collide_with_scene_labels_or_gain_pick_owners() {
    let (mark, _) = crate::measure_mark::distance([0.; 3], [3., 4., 0.], Unit::Millimeters);
    let mut existing = mark.labels(&[]);
    existing[0].id = u32::MAX;
    let labels = mark.labels(&existing);
    assert_eq!(labels.len(), 3);
    assert!(labels.iter().all(|l| l.object.is_none()));
    let ids = labels
        .iter()
        .map(|l| l.id)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(ids.len(), 3);
    assert!(
        labels
            .iter()
            .all(|l| existing.iter().all(|old| old.id != l.id))
    );
    assert_eq!(labels[0].text, "5 mm");
    assert_eq!(labels[1].text, "■");
}

#[test]
fn one_camera_unit_drives_measurements_without_editing_originals_or_history(){
 let mut e=editor();command(&mut e,"Line w0,0,0 w3,4,0").unwrap();let owner=e.scene.lines()[0].id;let source=e.scene.lines()[0].prepared.source.clone();let original=source.to_proto();e.select_objects([owner],false);command(&mut e,"Length").unwrap();assert_eq!(e.mark.as_ref().unwrap().label,"5 mm");e.camera.unit=crate::camera::Unit::Meters;command(&mut e,"Length").unwrap();assert_eq!(e.mark.as_ref().unwrap().label,"5 m");assert_eq!(e.camera.unit,Unit::Meters);assert_eq!(e.camera.unit.suffix(2),"m²");assert_eq!(e.camera.unit.to_meters(),1.);assert!(e.camera.view_projection().inverse().is_some());assert!(e.camera.ray([0.,0.]).is_some());assert!(Rc::ptr_eq(&e.scene.lines()[0].prepared.source,&source));assert_eq!(source.to_proto(),original);command(&mut e,"Undo").unwrap();assert!(e.scene.lines().is_empty());assert_eq!(e.camera.unit,Unit::Meters);command(&mut e,"Redo").unwrap();assert_eq!(e.scene.lines()[0].id,owner);assert_eq!(e.scene.lines()[0].prepared.source.to_proto(),original);e.select_objects([owner],false);command(&mut e,"Length").unwrap();assert_eq!(e.mark.as_ref().unwrap().label,"5 m");
}
