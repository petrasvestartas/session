            None if url.ends_with(".pb") && encoded.is_none() => (probe(&url).await, None), // register:stream
