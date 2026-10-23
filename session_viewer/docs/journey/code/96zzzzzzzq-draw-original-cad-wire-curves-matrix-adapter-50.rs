    let bundle=crate::archive::Bundle::decode(bytes)?;
    let mut candidate=scene.clone();candidate.clear();let mut keys=Vec::new();
    for (source,meta) in bundle.documents.iter().zip(&bundle.metadata.documents) {keys.push(import(&mut candidate,source,&meta.name,&Xform::from_matrix(meta.place),meta.point_px)?);}
    if !bundle.metadata.texts.is_empty() {
        let generated=candidate.texts().iter().filter(|row|row.guid.starts_with("matrix-title/")).map(|row|row.id).collect::<Vec<_>>();
        for id in generated {candidate.remove(id);}
    }
    crate::archive_restore_texts::restore(&mut candidate,&bundle.metadata)?;
    crate::archive_restore_flags::restore(&mut candidate,&bundle.metadata,&keys)?;
    scene.restore(candidate);Ok(())
