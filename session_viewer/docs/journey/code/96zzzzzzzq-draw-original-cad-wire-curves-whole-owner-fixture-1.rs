fn actual_control_net_pixel_addresses_parent_without_a_fake_edge_or_control() {
    let mut editor=crate::camera_defaults_tests::editor_fixture();editor.apply(crate::editor::Action::Close).unwrap();editor.apply(crate::editor::Action::AddCurve).unwrap();editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let owner=editor.scene.paths()[0].id;let nets=editor.scene.control_nets();assert_eq!(nets.len(),2);let markers=editor.scene.control_markers();
    for(parent,line)in &nets{assert_eq!(*parent,owner);assert!(markers.iter().any(|m|m.display.center==line.start));assert!(markers.iter().any(|m|m.display.center==line.end));assert_eq!(line.width,1.);}
    let source=crate::source_edit::original(&editor.scene,owner).unwrap();
    for density in [1.,2.] {
        let mut renderer=renderer(&editor.scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});renderer.set_pick_mode(crate::pick_sources::Mode::Object);renderer.request_pick([-0.3,0.1],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(pixel,[owner.value(),crate::sub_id::OBJECT_INK]);assert_eq!(renderer.resolve_pick(&editor.scene,pixel),Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Object}));
        for mode in [crate::pick_sources::Mode::Edge,crate::pick_sources::Mode::Control]{renderer.set_pick_mode(mode);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert!(finish(&mut renderer).is_none());}
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,owner).unwrap()));editor.scene.show_controls(false);assert!(editor.scene.control_nets().is_empty());
}
