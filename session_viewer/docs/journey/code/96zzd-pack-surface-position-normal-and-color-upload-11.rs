let mut bytes = crate::packed_vertex::bytes(&source);
        if bytes.is_empty() { bytes.resize(20, 0); }