    Ok(out)
}
#[cfg(test)]
mod tests {
    #[test]
    fn drawing_buffer_chunks_keep_exact_original_intervals_within_device_limits() {
        assert_eq!(super::chunks(10, 128).unwrap(), vec![0..4, 4..8, 8..10]);
        assert!(super::chunks(1, 31).is_err());
        assert!(super::chunks(0, 32).unwrap().is_empty());
    }
}

fn settings(scene: &Scene, row: &crate::sheet_source::Row, base: u32, selected: bool) -> Vec<u8> {
    let mut bytes = row
