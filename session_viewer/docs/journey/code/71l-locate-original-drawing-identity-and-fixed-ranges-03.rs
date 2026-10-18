    Some((start, bytes))
}
#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;
    #[test]
    fn ranged_drawing_identity_and_fixed_column_offsets_match_the_original_protobuf() {
        let raw = crate::sheet_example::bytes();
        let original = crate::sheet_example::source();
        assert_eq!(identity(&raw), Some((original.guid, original.name)));
        let (at, length, end) = crate::stream::sheet_layout(&raw).unwrap();
        let fields = SheetFields {
            count: 5,
            end,
            coords_at: at,
            coords_len: length,
            ..Default::default()
        };
        assert_eq!(slice(&fields, at, length, 2..4, 48), Some((at + 96, 96)));
        assert!(slice(&fields, at, length, 4..6, 48).is_none());
        assert!(slice(&fields, u64::MAX, 48, 1..2, 48).is_none());
        let mut mixed = session_rust::proto::Session::decode(raw.as_slice()).unwrap();
        mixed
            .objects
            .as_mut()
            .unwrap()
            .points
            .push(session_rust::Point::new(0., 0., 0.).to_proto());
        assert!(identity(&mixed.encode_to_vec()).is_none());
        let mut repeated = session_rust::proto::Session::decode(raw.as_slice()).unwrap();
        repeated
            .objects
            .as_mut()
            .unwrap()
            .sheets
            .push(crate::sheet_example::source());
        assert!(identity(&repeated.encode_to_vec()).is_none());
    }
}

