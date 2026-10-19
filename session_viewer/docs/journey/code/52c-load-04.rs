        }
        Ok(meshes)
    })?;
    Ok(Loaded { document, meshes, tree })
}

fn decode_raw(bytes: &[u8]) -> Result<proto::Session, &'static str> {
