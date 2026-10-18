3,0.1],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([owner.value(),0]));
        for mode in [crate::pick_sources::Mode::Edge,crate::pick_sources::Mode::Control]{renderer.set_pick_mode(mode);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert!(finish(&mut renderer).is_none());}
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,owner).unwrap()));editor.scene.show_controls(false);assert!(editor.scene.control_nets().is_empty());
}

#[test]
fn actual_edited_element_controls_move_their_integer_pixels_and_undo() {
    use crate::pick_sources::{Mode,Part,Target};
    let mut mesh=session_rust::Mesh::new();for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.5,-0.5,0.25]),(30,[0.,0.5,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}mesh.add_face(vec![10,20,30],Some(37));
    let mut element=session_rust::Element::new("edited control pixel");element.set_geometry(mesh);let mut base=Scene::empty();let owner=crate::element_original::insert(&mut base,std::rc::Rc::new(element)).unwrap();base.owner_controls(owner).unwrap();let original=crate::source_edit::original(&base,owner).unwrap();let target=Target{owner,part:Part::SourceControl(crate::source_controls::ControlId::Vertex(10))};
    for density in [1.,2.] {
        let mut scene=base.clone();let mut renderer=renderer(&scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});renderer.set_pick_mode(Mode::Control);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
        let mut preview=crate::part_preview::Preview::new(&scene,target).unwrap();preview.set(session_rust::Xform::translation(0.25,0.,0.)).unwrap();renderer.set_scene(&preview.scene,None);renderer.request_pick([-0.25,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&preview.scene,pixel),Some(target));renderer.request_pick([-0.5,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());
        let mut history=crate::history::History::default();assert!(preview.commit(&mut scene,&mut history).unwrap());renderer.set_scene(&scene,None);renderer.request_pick([-0.25,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
        assert!(history.undo(&mut scene));renderer.set_scene(&scene,None);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));assert!(crate::source_edit::same(&original,&crate::source_edit::original(&scene,owner).unwrap()));
    }
}
