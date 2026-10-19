    Ok((at, length))
}

#[cfg(test)]
mod bounded_tests {
    use super::*;
    #[test]
    fn original_metadata_queries_bound_record_offsets_counts_sizes_and_file_ends() {
        let raw = [12u64, 32]
            .into_iter()
            .flat_map(u64::to_le_bytes)
            .collect::<Vec<_>>();
        assert_eq!(blob_range(3, 2, &raw, 100), Ok((68, 32)));
        assert!(blob_range(3, 3, &raw, 100).is_err());
        assert!(blob_range(3, 2, &raw, 99).is_err());
        let overflow = [u64::MAX, 1]
            .into_iter()
            .flat_map(u64::to_le_bytes)
            .collect::<Vec<_>>();
        assert!(blob_range(3, 2, &overflow, u64::MAX).is_err());
        let large = [0u64, MAX_BLOB + 1]
            .into_iter()
            .flat_map(u64::to_le_bytes)
            .collect::<Vec<_>>();
        assert!(blob_range(3, 2, &large, u64::MAX).is_err());
    }
}
