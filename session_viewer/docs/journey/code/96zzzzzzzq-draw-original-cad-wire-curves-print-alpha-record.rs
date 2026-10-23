    bytes[RANGE+4..RANGE+8].copy_from_slice(&primitive_base.to_ne_bytes());
    let alpha=style.colour.map_or(1.,|colour|colour[3]);
    bytes[crate::object_record::PRINT_ALPHA..BYTES].copy_from_slice(&alpha.to_ne_bytes());
