pub fn load_at(bytes: &[u8], location: Option<Rc<crate::reload_url::ReloadUrl>>) -> Result<Loaded, &'static str> {
    load_observed(bytes, location, &mut crate::load_measure::now, &mut |phase| {
        #[cfg(target_arch = "wasm32")]
        crate::browser_phase::completed(phase, "document");
        #[cfg(not(target_arch = "wasm32"))]
        let _ = phase;
    })
}

pub fn load_observed(bytes: &[u8], location: Option<Rc<crate::reload_url::ReloadUrl>>,
    clock: &mut impl FnMut() -> f64, observe: &mut impl FnMut(crate::load_measure::Completed<'_>),
) -> Result<Loaded, &'static str> {
    use crate::load_measure::run;
    let size = bytes.len() as u64;
    let message = run("decode", size, clock, observe, || decode_raw(bytes))?;
    run("validation", size, clock, observe, || validate(&message))?;
    let mut origin = crate::origin::Origin::new(&message, bytes); origin.location = location;
    let origin = Rc::new(origin);
    let document = run("kernel", size, clock, observe, || {
        Session::from_proto(message).map(Rc::new).map_err(|_| "Cannot construct session")
    })?;
    let meshes = run("display walk", size, clock, observe, || {
        let mut meshes = Vec::new();
        for mesh in document.objects.meshes.iter() {
            let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
            prepared.model = document.xforms.get(mesh.guid()).cloned().unwrap_or_else(session_rust::Xform::identity);
            prepared.source = Some(Source {
                document: Rc::clone(&document), guid: mesh.guid().to_owned(), origin: Rc::clone(&origin),
            });
            meshes.push(prepared);
        }
        Ok(meshes)
    })?;
    Ok(Loaded { document, meshes })
}

fn decode_raw(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")
}

fn decode(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    let message = decode_raw(bytes)?;
    validate(&message)?;
    Ok(message)
}
