    Ok(scene)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_manifest_instances_share_original_sources_and_display_allocations() {
        let manifest=Manifest::parse(r#"items: [{file: cloud.pb, at: [0,0,0], point_size: 4}, {file: cloud.pb, at: [1000000000.125,0,0], point_size: 7}]
texts: [{text: 'CAD Ø25', at: [0,0,0], height: 0.1}]
"#.as_bytes()).unwrap();let mut source=Scene::demo();source.clear();source.insert_cloud(crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap()).unwrap();source.insert(crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[1.,0.,0.]).unwrap()).unwrap();let sources=[("cloud.pb".into(),source.clone())].into_iter().collect();let scene=prepare(&Scene::demo(),&manifest,&["cloud.pb".into(),"cloud.pb".into()],&sources).unwrap();
        assert_eq!(scene.clouds().len(),2);assert!(Rc::ptr_eq(&scene.clouds()[0].prepared.source,&scene.clouds()[1].prepared.source));assert!(Rc::ptr_eq(&scene.clouds()[0].prepared.nodes,&scene.clouds()[1].prepared.nodes));assert!(Rc::ptr_eq(&scene.objects()[0].mesh,&scene.objects()[1].mesh));assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),scene.objects()[1].geometry().unwrap()));assert_ne!(scene.clouds()[0].id,scene.clouds()[1].id);assert_ne!(scene.clouds()[0].guid,scene.clouds()[1].guid);assert_eq!(scene.clouds()[1].model.m[12],1_000_000_000.125);assert_eq!(scene.clouds()[0].point_px,4.);assert_eq!(scene.clouds()[1].point_px,7.);assert_eq!(scene.clouds()[0].prepared.source.point_size,3.);assert_eq!(scene.texts()[0].prepared.source.label.text,"CAD Ø25");
        let saved=crate::scene_archive::save(&scene).unwrap();let mut restored=Scene::demo();crate::scene_document::open(&mut restored,&saved).unwrap();assert_eq!(restored.clouds().iter().map(|row|row.point_px).collect::<Vec<_>>(),vec![4.,7.]);assert_eq!(source.clouds()[0].model.m,session_rust::Xform::identity().m);
    }
    #[test]
    fn invalid_manifest_adoption_and_overflowing_placements_leave_prior_scene_intact() {
        let prior=Scene::demo();let ids=prior.objects().iter().map(|row|row.id).collect::<Vec<_>>();let manifest=Manifest::parse(r#"items: [{file: absent.pb}]"#.as_bytes()).unwrap();assert!(prepare(&prior,&manifest,&["absent.pb".into()],&HashMap::new()).is_err());assert_eq!(prior.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
        let manifest=Manifest::parse(r#"items: [{file: mesh.pb, at: [1e100,0,0]}]"#.as_bytes()).unwrap();let sources=[("mesh.pb".into(),Scene::demo())].into_iter().collect();assert!(prepare(&prior,&manifest,&["mesh.pb".into()],&sources).is_err());assert_eq!(prior.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
    }
}
