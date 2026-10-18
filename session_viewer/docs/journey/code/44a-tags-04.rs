    assert!(Parts::new(&source, 3).is_err());
}

#[test]
fn tagged_zero_addresses_and_largest_addresses_roundtrip_without_kind_collisions() {
    for kind in [crate::sub_id::Kind::Face, crate::sub_id::Kind::Control, crate::sub_id::Kind::Edge] {
        for address in [0, crate::sub_id::LIMIT as usize] {
            let encoded = crate::sub_id::encode(kind, address).unwrap(); assert_ne!(encoded, 0);
            assert_eq!(crate::sub_id::decode(encoded), Some((kind, address)));
            assert_eq!(crate::sub_id::surface(encoded), kind == crate::sub_id::Kind::Face);
        }
        assert!(crate::sub_id::encode(kind, crate::sub_id::LIMIT as usize + 1).is_err());
    }
    assert!(crate::sub_id::decode(0).is_none()); assert!(crate::sub_id::decode(1).is_none());
}
