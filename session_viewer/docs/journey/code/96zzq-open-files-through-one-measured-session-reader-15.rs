    let previous=candidate.all_ids();let url=location.as_ref().map(|location|location.value().to_owned());
    crate::scene_document::import_at(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.,location)?;
    if let Some(url)=url {candidate.sheet_import_url(&url,&previous);}