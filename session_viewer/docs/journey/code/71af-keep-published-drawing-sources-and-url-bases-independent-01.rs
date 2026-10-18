    if bytes.starts_with(crate::archive::MAGIC) {crate::scene_document::open(&mut candidate,bytes)?;return Ok(candidate);}
    if replace {candidate.clear();}
    let url=location.as_ref().map(|location|location.value().to_owned());
    match crate::document::load_at(bytes,location) {
        Ok(loaded)=>candidate.import(loaded).map_err(str::to_owned)?,
        Err(_)=>{let previous=candidate.all_ids();crate::scene_document::import(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.)?;if let Some(url)=url {candidate.sheet_import_url(&url,&previous);}}
    }
    Ok(candidate)
