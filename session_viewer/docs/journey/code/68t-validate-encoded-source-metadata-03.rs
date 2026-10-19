        if bytes>crate::archive::LIMIT||self.decoded.is_some_and(|size|size!=bytes) {return Err("Decoded source size does not match its manifest".into());}Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encoded_sources_require_supported_encoding_and_exact_bounded_decoded_bytes() {
        let gzip=Encoding::new("gzip",24).unwrap();assert!(gzip.check(24).is_ok());assert!(gzip.check(23).is_err());assert!(Encoding::new("gzip",0).is_err());assert!(Encoding::new("gzip",crate::archive::LIMIT as u64+1).is_err());assert!(Encoding::new("brotli",24).is_err());assert!(Encoding::new("",0).unwrap().check(24).is_ok());
    }
}
