use crate::{colour::{self, Mode}, mesh::Mesh};
use session_rust::{Color, mesh::ColorMode};

#[test]
fn source_modes_preserve_topology_and_resolve_complete_colour_arrays() {
    let faces = colour::specimen(Mode::Faces); let original = faces.to_proto(); let display = Mesh::from_kernel(&faces).unwrap();
    assert_eq!(faces.number_of_vertices(), 6); assert_eq!(display.vertices().len(), 12); assert_eq!(display.indices().len(), 12);
    assert!(display.vertices()[..6].iter().all(|p| p[3..] == [1.0, 0.0, 0.0]));
    assert!(display.vertices()[6..].iter().all(|p| p[3..] == [0.0, 0.0, 1.0])); assert_eq!(faces.to_proto(), original);
    let points = colour::specimen(Mode::Points); let display = Mesh::from_kernel(&points).unwrap();
    assert_eq!(display.vertices().len(), 6); assert_eq!(display.vertices()[1][3..], [0.0, 1.0, 0.0]);
    assert_eq!(points.get_pointcolors().len(), 6); assert_eq!(points.number_of_faces(), 2);
    let mut object = colour::specimen(Mode::Points); object.color_mode = ColorMode::OBJECTCOLOR;
    let display = Mesh::from_kernel(&object).unwrap(); assert!(display.vertices().iter().all(|p| p[3..] == [0.8, 0.6, 0.1]));
    assert_eq!(object.get_pointcolors().len(), 6);
}

#[test]
fn incomplete_active_arrays_use_object_colour_without_source_mutation() {
    for points in [true, false] {
        let mut source = colour::specimen(Mode::Object);
        if points { source.set_pointcolors(vec![Color::new(1.0, 0.0, 0.0, 1.0)]); }
        else { source.set_facecolors(vec![Color::new(1.0, 0.0, 0.0, 1.0)]); }
        let display = Mesh::from_kernel(&source).unwrap(); assert!(display.vertices().iter().all(|p| p[3..] == [0.8, 0.6, 0.1]));
        assert_eq!(if points { source.get_pointcolors().len() } else { source.get_facecolors().len() }, 1);
    }
}
