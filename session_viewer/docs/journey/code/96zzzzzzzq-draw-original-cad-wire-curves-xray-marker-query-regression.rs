cement(id).unwrap();s.place(id,Xform::translation(1_000_000_000.125,0.,0.)).unwrap();let prepared=Sources::anchored(&s,[1_000_000_000.,0.,0.]).unwrap();assert_eq!(prepared.mesh_markers[0].center[0],0.125,"Transform after anchoring preserves the source endpoint fraction");s.place(id,old).unwrap();
}

#[test]
fn actual_xray_rear_marker_queries_follow_opacity_before_color_and_restore() {
    let mut scene=Scene::empty();
    let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
    let hidden=session_rust::Line::new(-0.4,0.,-0.9,0.4,0.,-0.9);
    scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(hidden)).unwrap()).unwrap();
    let original=source(&scene);
    let mut camera=crate::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.fit(&scene.bounds().unwrap());
    let matrix=camera.uniform();
    let mut gpu=renderer(&scene);gpu.set_lighting(false,false);gpu.set_outlines(false);gpu.set_ambient(false);
    gpu.set_ink_visibility(crate::ink_ranges::Visibility{edges:true,lines:false,points:false,vertices:true});
    gpu.set_display_settings(crate::display_settings::Settings{thickness:4.,feather:1.,..Default::default()});
    let uploads=gpu.stats()[0];let mut opaque=None;
    for opacity in [1.,0.5,0.,1.] {
        gpu.set_opacity(opacity);
        for point in [[-0.6,-0.6,-0.8],[0.6,0.6,-0.8]] {
            assert_eq!(pick(&mut gpu,&scene,matrix,point,Mode::Object),Some(Target{owner,part:Part::Object}),"First original query must follow actual opacity {opacity}");
        }
        let image=pixels_at(&gpu,matrix);
        if opacity==1. {if let Some(before)=&opaque {assert_eq!(&image,before,"Returning from Xray restores exact opaque pixels");}else{opaque=Some(image);}}
        assert_eq!(source(&scene),original);assert_eq!(gpu.stats()[0],uploads);
    }
}
