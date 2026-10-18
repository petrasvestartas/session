    if crate::origin::FileVersion::of(bytes) != origin.version { return Err("Reloaded file version changed"); }
    crate::document_source::prepare_at(bytes,&session_rust::Xform::identity(),0.,origin.location.clone())
        .map(|source|source.document).map_err(|_|"Cannot restore source session")