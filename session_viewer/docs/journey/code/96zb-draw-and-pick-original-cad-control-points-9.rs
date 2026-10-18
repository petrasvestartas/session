camera=crate::camera::Camera::default();let pen_rows=|pixels:Vec<u8>|pixels.chunks_exact(320*4).filter(|row|row[160*4..][..3].iter().all(|v|*v<30)).count();renderer.sheet_camera(&camera);let first=pen_rows(colour_pixels_at(&mut renderer,camera.uniform()));assert!(first>20);camera.frame=[0.,0.,1.,0.2];renderer.sheet_camera(&camera);let shifted=pen_rows(colour_pixels_at(&mut renderer,camera.uniform()));assert_eq!(shifted,first,"A panel shifts the frame without changing the original world pen");
}

#[test]
fn actual_mesh_control_pixel_keeps_sparse_vertex_key_at_both_densities() {
    let mut mesh=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.5,-0.5,0.25]),(30,[0.,0.5,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![10,20,30],Some(37));let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();scene.show_controls(true);
    let source=crate::source_edit::original(&scene,owner).unwrap();let model=session_rust::Xform::translation(0.125,0.25,0.);scene.place(owner,model).unwrap();
    for density in [1.,2.] {
        let mut renderer=renderer(&scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);renderer.request_pick([-0.375,-0.25],identity()).unwrap();
        let pixel=finish(&mut renderer).expect("Actual placed control pixel");
        assert_eq!(renderer.resolve_pick(&scene,pixel),Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::SourceControl(crate::source_controls::ControlId::Vertex(10))}));
        scene.show_controls(false);renderer.set_scene(&scene,None);renderer.request_pick([-0.375,-0.25],identity()).unwrap();assert!(finish(&mut renderer).is_none());scene.show_controls(true);
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,owner).unwrap()));
}
