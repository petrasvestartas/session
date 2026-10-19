        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        if raw.chunks_exact(8).any(|bytes|!f64::from_le_bytes(bytes.try_into().unwrap()).is_finite()) {return None;}
        (raw.len() as u64 == length).then(|| normals_from(&raw))
    }
