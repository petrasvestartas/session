}

pub fn save(scene:&Scene)->Result<Vec<u8>,String> {
    let mut namespaces=HashSet::new();let mut documents=Vec::new();let mut bytes_used=0usize;
    let mut metadata=Metadata {created_doc:None,current_layer:None,documents:vec![],hidden:vec![],locked:vec![],colors:vec![],edge_colors:Some(vec![]),texts:vec![],groups:vec![]};
    for (index,namespace) in inventory(scene).iter().enumerate() {
        for id in &namespace.ids {if !namespaces.insert(*id) {return Err("Source owner occurs in two saved documents".into());}}
        let (source,names,point_px)=crate::archive_document::prepare(scene,namespace)?;crate::source_validate::session(&source)?;let bytes=source.encode_to_vec();bytes_used=bytes_used.checked_add(bytes.len()).ok_or("Session size overflow")?;if bytes_used>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
        crate::archive_presentation::append(&mut metadata,index,namespace,scene,&names)?;
        documents.push(bytes);metadata.documents.push(Document {name:namespace.name.clone(),place:session_rust::Xform::identity().m,point_px});
    }
    for row in scene.texts() {let mut label=row.label(false,false).map_err(str::to_owned)?;label.object=None;label.id=row.prepared.source.label.id;metadata.texts.push((row.guid.clone(),label,scene.visible(row.id)));}
    metadata.hidden.sort();metadata.locked.sort();metadata.colors.sort();metadata.edge_colors.as_mut().unwrap().sort();metadata.texts.sort_by(|a,b|a.0.cmp(&b.0));Bundle {documents,metadata}.encode()
}
