use crate::{editor::{Action, Editor}, pick_sources::{Mode, Part, Sources}, sub_id::{self, Kind}};

#[test]
fn source_addresses_require_the_correct_kind_owner_and_current_row() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::AddCurve).unwrap(); editor.apply(Action::Controls(true)).unwrap();
    let sources = Sources::new(&editor.scene).unwrap(); assert_eq!(sources.faces.len(), 6); assert_eq!(sources.mesh_ink.len(), 12);
    let face = sources.faces[0]; let pixel = [face.owner.value(), sub_id::encode(Kind::Face, 0).unwrap()];
    assert_eq!(sources.resolve(&editor.scene, pixel, Mode::Face), Some(face)); assert_eq!(face.part, Part::Face(0));
    assert!(sources.resolve(&editor.scene, pixel, Mode::Edge).is_none());
    assert_eq!(sources.resolve(&editor.scene, pixel, Mode::Object).unwrap().part, Part::Object);
    let control = sources.controls[1]; let pixel = [control.owner.value(), sub_id::encode(Kind::Control, 1).unwrap()];
    assert_eq!(sources.resolve(&editor.scene, pixel, Mode::Control), Some(control)); assert_eq!(control.part, Part::Control(1));
    assert!(sources.resolve(&editor.scene, [face.owner.value(), pixel[1]], Mode::Control).is_none());
    assert_eq!(sources.path_ids.len(), editor.scene.paths()[0].segments().len());
    editor.apply(Action::Close).unwrap(); assert!(sources.resolve(&editor.scene, pixel, Mode::Control).is_none());
}

#[test]
fn degenerate_polyline_segments_preserve_the_original_nonzero_segment_index() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    let line = session_rust::Polyline::new(vec![session_rust::Point::new(0., 0., 0.), session_rust::Point::new(0., 0., 0.), session_rust::Point::new(1., 0., 0.)]);
    let id = editor.scene.insert_path(crate::chain::PreparedChain::polyline(std::rc::Rc::new(line)).unwrap()).unwrap();
    let sources = Sources::new(&editor.scene).unwrap(); assert_eq!(sources.path_ids.len(), 1);
    let target = sources.resolve(&editor.scene, sources.path_ids[0], Mode::Edge).unwrap();
    assert_eq!(target.owner, id); assert_eq!(target.part, Part::PathEdge(1));
}
