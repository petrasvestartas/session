use crate::{editor::{Action,Editor},pick_sources::{Part,Sources},source_controls::ControlId};

#[test]
fn original_families_reuse_control_addresses_and_keep_the_source_unchanged() {
    for action in [Action::AddBox,Action::AddCad,Action::AddSurface] {
        let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(action).unwrap();
        let owner=editor.scene.objects()[0].id;let source=crate::source_edit::original(&editor.scene,owner).unwrap();
        editor.apply(Action::Controls(true)).unwrap();let markers=editor.scene.control_markers();assert!(!markers.is_empty());
        let sources=Sources::new(&editor.scene).unwrap();assert_eq!(sources.controls.len(),markers.len());
        for (index,target) in sources.controls.iter().enumerate() {
            assert_eq!(target.owner,owner);assert!(matches!(target.part,Part::SourceControl(_)));assert!(target.valid(&editor.scene));
            let pixel=[owner.value(),crate::sub_id::encode(crate::sub_id::Kind::Control,index).unwrap()];
            assert_eq!(sources.resolve(&editor.scene,pixel,crate::pick_sources::Mode::Control),Some(*target));
        }
        assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,owner).unwrap()));
        let old=sources.controls[0];editor.scene.show_controls(false);assert!(!old.valid(&editor.scene));assert!(editor.scene.control_markers().is_empty());
        editor.scene.show_controls(true);std::rc::Rc::make_mut(&mut editor.scene.view).hide(&[owner],true);assert!(editor.scene.control_markers().is_empty());assert!(!old.valid(&editor.scene));
        editor.apply(Action::Close).unwrap();assert!(editor.scene.control_owners.is_empty());
    }
}

#[test]
fn element_mesh_controls_keep_the_original_element_owner() {
    let mut scene=crate::scene::Scene::empty();let mut element=session_rust::Element::new("controls");element.set_geometry(session_rust::Mesh::create_box(1.,1.,1.));
    let source=std::rc::Rc::new(element);let owner=crate::element_original::insert(&mut scene,source.clone()).unwrap();scene.show_controls(true);
    let markers=scene.control_markers();assert!(!markers.is_empty());assert!(markers.iter().all(|m|m.parent==owner&&matches!(m.part,Part::SourceControl(ControlId::Vertex(_)))));
    assert_eq!(scene.objects().len(),1);assert!(matches!(&scene.objects()[0].original,Some(session_rust::Geometry::Element(current)) if std::rc::Rc::ptr_eq(current,&source)));
    scene.remove(owner);assert!(scene.control_owners.is_empty());assert!(scene.control_markers().is_empty());
}
