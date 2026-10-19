    metadata.hidden.sort();metadata.locked.sort();metadata.colors.sort();metadata.edge_colors.as_mut().unwrap().sort();metadata.texts.sort_by(|a,b|a.0.cmp(&b.0));Bundle {documents,metadata}.encode()
}

#[cfg(test)]
mod tests {
    use super::*;
    use {session_rust::proto,std::collections::HashMap,std::rc::Rc};
    #[test]
    fn mixed_archive_keeps_original_source_types_copy_ids_and_precise_models() {
        let mut scene=Scene::demo();scene.clear();let cad=scene.insert_cad(Rc::new(crate::cad_display::example())).unwrap();let cloud=Rc::new(crate::cloud::example());let _=cloud.guid();let before=cloud.to_proto();let cloud_id=scene.insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap()).unwrap();let copy=scene.copy_row(cloud_id,false).unwrap();scene.place_cloud(copy,session_rust::Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
        Rc::make_mut(&mut scene.view).hide(&[cad],true);let saved=Bundle::decode(&save(&scene).unwrap()).unwrap();let mut found=HashMap::new();
        for (index,bytes) in saved.documents.iter().enumerate() {let source=proto::Session::decode(bytes.as_slice()).unwrap();let objects=source.objects.unwrap();assert!(objects.meshes.is_empty());for c in objects.pointclouds {found.insert(c.guid.clone(),c);}if !objects.breps.is_empty() {assert_eq!(objects.breps[0].is_visible,Some(false));assert!(saved.metadata.hidden.iter().any(|(doc,guid)|*doc==index&&guid==scene.row_guid(cad).unwrap()));}if let Some(entry)=source.xforms.iter().find(|entry|entry.guid==scene.row_guid(copy).unwrap()) {assert_eq!(entry.xform.as_ref().unwrap().matrix[12],1_000_000_000.125);}}
        assert_eq!(found.len(),2);assert_eq!(found[scene.row_guid(cloud_id).unwrap()].coords,before.coords);assert_eq!(found[scene.row_guid(copy).unwrap()].point_ids,before.point_ids);assert_eq!(cloud.to_proto(),before);
    }
    #[test]
    fn repeated_imported_documents_keep_independent_guid_namespaces_and_flags() {
        let mut scene=Scene::demo();scene.clear();let bytes=crate::specimen::bytes();scene.import(crate::document::load(&bytes).unwrap()).unwrap();let per_document=scene.objects().len();scene.import(crate::document::load(&bytes).unwrap()).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[per_document].id;assert_ne!(scene.row_guid(a),scene.row_guid(b));assert_eq!(scene.objects()[0].geometry().unwrap().guid(),scene.objects()[per_document].geometry().unwrap().guid());
        Rc::make_mut(&mut scene.view).hide(&[a],true);Rc::make_mut(&mut scene.view).lock(&[b],true);let bundle=Bundle::decode(&save(&scene).unwrap()).unwrap();assert_eq!(bundle.documents.len(),2);assert_eq!(bundle.metadata.hidden.len(),1);assert_eq!(bundle.metadata.locked.len(),1);assert_ne!(bundle.metadata.hidden[0].0,bundle.metadata.locked[0].0);assert_eq!(bundle.metadata.hidden[0].1,bundle.metadata.locked[0].1);
    }
}
