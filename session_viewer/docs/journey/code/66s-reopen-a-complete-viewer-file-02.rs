    scene.restore(candidate);Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_save_open_restores_original_sources_placements_layers_and_presentation() {
        let mut scene=Scene::demo();scene.clear();
        let cad=scene.insert_cad(Rc::new(crate::cad_display::example())).unwrap();
        let surface=scene.insert_surface(Rc::new(crate::surface::example())).unwrap();
        let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap()).unwrap();
        let copy=scene.copy_row(cloud,false).unwrap();scene.place_cloud(copy,Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
        scene.cloud_point_size(cloud,5.).unwrap();scene.cloud_point_size(copy,5.).unwrap();
        let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.123456789012345,0.,0.))).unwrap()).unwrap();
        let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-1.,0.,0.,1.,0.,0.))).unwrap()).unwrap();
        let label=crate::scene_label::example(crate::text::TextPlacement::WorldBillboard {world:[0.,0.,0.],world_height:0.1});
        let text=scene.insert_text(crate::scene_label::Prepared::new(Rc::new(label)).unwrap()).unwrap();scene.place_text(text,Xform::translation(0.123456789012345,0.,0.)).unwrap();
        let root=&scene.forest.documents[0];let current=format!("{}/{}",root.key,root.nodes[0].source.guid);
        let view=Rc::make_mut(&mut scene.view);view.current=Some(current);view.hide(&[cad,text],true);view.lock(&[surface,line,text],true);view.faces.insert(cad,[2,5,7]);view.edges.insert(cad,[0;3]);
        let mut restored=Scene::demo();let old_ids=restored.objects().iter().map(|row|row.id.value()).collect::<Vec<_>>();let bytes=crate::scene_archive::save(&scene).unwrap();open(&mut restored,&bytes).unwrap();
        assert_eq!(restored.objects().len(),2);assert_eq!(restored.objects().iter().filter(|row|row.cad.is_some()).count(),1);assert_eq!(restored.objects().iter().filter(|row|row.surface.is_some()).count(),1);
        assert_eq!(restored.clouds().len(),2);assert_eq!(restored.clouds()[1].model.m[12],1_000_000_000.125);assert_eq!(restored.clouds()[0].prepared.source.to_proto().coords,scene.clouds()[0].prepared.source.to_proto().coords);assert!(restored.clouds().iter().all(|row|row.point_px==5.&&row.prepared.source.point_size==3.));
        assert_eq!(restored.points()[0].prepared.source[0],scene.points()[0].prepared.source[0]);assert_eq!(restored.points()[0].prepared.source[0],0.123456789012345);let _=point;
        assert_eq!(restored.texts().len(),1);assert_eq!(restored.texts()[0].label(false,false).unwrap().placement,scene.texts()[0].label(false,false).unwrap().placement);assert!(!restored.visible(restored.texts()[0].id));assert!(restored.view.locked.contains(&restored.texts()[0].id));
        assert!(restored.view.current.is_some());assert!(restored.created.is_some());assert_eq!(restored.view.hidden.len(),2);assert_eq!(restored.view.locked.len(),3);assert_eq!(restored.view.faces[&restored.objects().iter().find(|row|row.cad.is_some()).unwrap().id],[2,5,7]);assert_eq!(restored.view.edges[&restored.objects().iter().find(|row|row.cad.is_some()).unwrap().id],[0;3]);assert!(restored.objects().iter().all(|row|!old_ids.contains(&row.id.value())));
        let again=crate::archive::Bundle::decode(&crate::scene_archive::save(&restored).unwrap()).unwrap();assert_eq!(again.documents.len(),2);assert_eq!(again.metadata.hidden.len(),2);assert_eq!(again.metadata.locked.len(),3);
    }
    #[test]
    fn repeated_guid_documents_reopen_with_independent_flags_and_original_meshes() {
        let mut scene=Scene::demo();scene.clear();let raw=crate::specimen::bytes();scene.import(crate::document::load(&raw).unwrap()).unwrap();let n=scene.objects().len();scene.import(crate::document::load(&raw).unwrap()).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[n].id;Rc::make_mut(&mut scene.view).hide(&[a],true);Rc::make_mut(&mut scene.view).lock(&[b],true);
        let saved=crate::scene_archive::save(&scene).unwrap();let mut restored=Scene::demo();open(&mut restored,&saved).unwrap();assert_eq!(restored.objects().len(),2*n);let a=&restored.objects()[0];let b=&restored.objects()[n];assert_ne!(a.guid,b.guid);assert_eq!(a.geometry().unwrap().guid(),b.geometry().unwrap().guid());assert!(!restored.visible(a.id));assert!(restored.visible(b.id));assert!(!restored.view.locked.contains(&a.id));assert!(restored.view.locked.contains(&b.id));
        let document=Rc::downgrade(&a.source().unwrap().document);let ids=restored.objects().iter().take(n).map(|row|row.id).collect::<Vec<_>>();for id in ids {assert!(restored.remove(id));}assert!(document.upgrade().is_none());
    }
    #[test]
    fn rejected_files_and_late_display_preparation_errors_leave_the_scene_unchanged() {
        let mut scene=Scene::demo();let ids=scene.objects().iter().map(|row|row.id).collect::<Vec<_>>();assert!(open(&mut scene,b"broken").is_err());assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
        let mut source=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();source.tree=None;source.objects.as_mut().unwrap().points.push(session_rust::Point::new(1e100,0.,0.).to_proto());let bytes=source.encode_to_vec();assert!(import(&mut scene,&bytes,"too large",&Xform::identity(),0.).is_err());assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);assert!(scene.points().is_empty());assert!(scene.forest.documents.iter().all(|doc|doc.label!="too large"));
        source.objects.as_mut().unwrap().points.clear();source.definitions=Some(proto::Objects::default());assert!(import(&mut scene,&source.encode_to_vec(),"unsupported",&Xform::identity(),0.).unwrap_err().contains("Definitions"));assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
    }
    #[test]
    fn document_placement_is_composed_without_mutating_original_source_coordinates() {
        let mut scene=Scene::demo();scene.clear();let cloud=crate::cloud::example();let _=cloud.guid();let original=cloud.to_proto();let source=proto::Session {name:"precise cloud".into(),objects:Some(proto::Objects {pointclouds:vec![original.clone()],..Default::default()}),..Default::default()};let raw=source.encode_to_vec();let place=Xform::translation(1_000_000_000.125,0.,0.);import(&mut scene,&raw,"placed",&place,7.).unwrap();assert_eq!(scene.clouds()[0].model.m[12],place.m[12]);assert_eq!(scene.clouds()[0].prepared.source.to_proto(),original);assert_eq!(scene.clouds()[0].point_px,7.);
    }
    #[test]
    fn repeated_roundtrips_keep_tree_depth_current_layer_source_header_and_precise_pose() {
        let mut source=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();source.guid="00000000-0000-0000-0000-000000000123".into();let raw=source.encode_to_vec();let mut scene=Scene::empty();import(&mut scene,&raw,"placed",&Xform::translation(0.123456789012345,0.,0.),0.).unwrap();let doc=&scene.forest.documents[0];let current=format!("{}/{}",doc.key,doc.nodes[0].source.guid);Rc::make_mut(&mut scene.view).current=Some(current);let nodes=doc.nodes.len();let depth=doc.nodes.iter().map(|node|node.depth).max().unwrap();
        for _ in 0..6 {let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();assert_eq!(proto::Session::decode(bundle.documents[0].as_slice()).unwrap().guid,source.guid);open(&mut scene,&saved).unwrap();assert_eq!(scene.forest.documents[0].nodes.len(),nodes);assert_eq!(scene.forest.documents[0].nodes.iter().map(|node|node.depth).max().unwrap(),depth);assert_eq!(scene.objects()[0].model.m[12],0.123456789012345);assert!(scene.view.current.is_some());}
    }
    #[test]
    fn moving_repeated_source_objects_into_one_layer_saves_distinct_editable_records() {
        let raw=crate::specimen::bytes();let mut scene=Scene::empty();import(&mut scene,&raw,"first",&Xform::identity(),0.).unwrap();let n=scene.objects().len();import(&mut scene,&raw,"second",&Xform::identity(),0.).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[n].id;let doc=&scene.forest.documents[0];let key=format!("{}/{}",doc.key,doc.nodes[0].source.guid);crate::layer_edit::apply(&mut scene,&key,crate::layer_edit::Edit::Move(vec![b])).unwrap();
        let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();let source=proto::Session::decode(bundle.documents[0].as_slice()).unwrap();let meshes=source.objects.unwrap().meshes;assert_eq!(meshes.len(),n+1);let guids=meshes.iter().map(|mesh|&mesh.guid).collect::<HashSet<_>>();assert_eq!(guids.len(),n+1);assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().guid(),scene.objects().iter().find(|row|row.id==b).unwrap().geometry().unwrap().guid());let mut restored=Scene::empty();open(&mut restored,&saved).unwrap();assert_eq!(restored.objects().len(),2*n);
    }

}
