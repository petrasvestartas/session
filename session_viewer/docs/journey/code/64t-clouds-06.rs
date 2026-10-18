    editor.apply(Action::SelectAll).unwrap();assert_eq!(editor.selected_objects().len(),2);editor.apply(Action::Delete).unwrap();assert!(editor.scene.clouds().is_empty());editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.clouds().len(),2);editor.apply(Action::Close).unwrap();drop(history);assert!(weak.upgrade().is_none());
}

#[test]
fn cloud_radius_adapter_matches_world_units_in_perspective_and_orthographic_views() {
    let run=crate::cloud_lod::Range {first:0,count:3,spacing:12.,tile:true};
    let ortho=crate::cloud_gpu::radius_factor(&run,6.,2.,100.)*200.;
    let perspective=crate::cloud_gpu::radius_factor(&run,6.,2.,0.)*200./(100./std::f32::consts::FRAC_PI_6.tan());
    assert!((ortho-perspective).abs()<1e-5);assert!((ortho-24.).abs()<1e-5);
}
