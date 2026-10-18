    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes
}