}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sheet_pages_preserve_original_doubles_pens_and_repeated_full_u32_entity_ids() {
        let coordinates = [
            1_000_000_000.125f64,
            2.,
            3.,
            1_000_000_000.375,
            4.,
            5.,
            0.,
            0.,
            0.,
            1.,
            1.,
            0.,
        ]
        .into_iter()
        .flat_map(f64::to_le_bytes)
        .collect::<Vec<_>>();
        let ids = [u32::MAX - 1, u32::MAX - 1]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>();
        let widths = [0.5f32, f32::NAN]
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>();
        let page = Page::decode(
            3..5,
            8,
            u32::MAX,
            &coordinates,
            None,
            Some(&widths),
            Some(&ids),
        )
        .unwrap();
        assert_eq!(page.coordinates[0], 1_000_000_000.125);
        assert_eq!(page.coordinates[3], 1_000_000_000.375);
        assert_eq!(page.display[0], page.display[3]);
        assert_eq!(page.entities, [u32::MAX - 1; 2]);
        assert_eq!(page.colors, [0xff00_0000; 2]);
        assert_eq!(page.radius(0), 0.25);
        assert_eq!(page.radius(1), 0.);
        assert!(page.widths[1].is_nan());
        assert_eq!(page.bounds().unwrap().max, [1_000_000_000.375, 4., 5.]);
        assert_eq!(page.bytes(), 168);
    }
    #[test]
    fn sheet_pages_refuse_partial_columns_invalid_coordinates_and_out_of_range_entities() {
        let coordinates = [0f64; 6]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect::<Vec<_>>();
        assert!(Page::decode(0..1, 1, 2, &coordinates[..47], None, None, None).is_err());
        assert!(Page::decode(0..1, 1, 2, &coordinates, Some(&[0; 3]), None, None).is_err());
        let ids = 2u32.to_le_bytes();
        assert!(Page::decode(0..1, 1, 2, &coordinates, None, None, Some(&ids)).is_err());
        let invalid = [f64::INFINITY; 6]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect::<Vec<_>>();
        assert!(Page::decode(0..1, 1, 2, &invalid, None, None, None).is_err());
        assert_eq!(
            Page::decode(0..1, 1, 0, &coordinates, None, None, None)
                .unwrap()
                .entities,
            [u32::MAX]
        );
    }
}
