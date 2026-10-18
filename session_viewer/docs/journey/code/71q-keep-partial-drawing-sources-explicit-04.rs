                .any(|page| page.entities.contains(&id))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_sheet_columns_remain_unpadded_while_display_defaults_match_current_source() {
        let original = Rc::new(proto::Sheet {
            guid: "sheet".into(),
            coords: vec![0., 0., 0., 1., 0., 0., 0., 1., 0., 1., 1., 0.],
            segment_count: 2,
            entity_count: 10,
            colors: vec![0xff00_00ff],
            widths: vec![0.5],
            source_ids: vec![9],
            ..Default::default()
        });
        let prepared = Prepared::new(original.clone()).unwrap();
        assert!(Rc::ptr_eq(&original, &prepared.original));
        assert_eq!(prepared.original.colors.len(), 1);
        assert_eq!(prepared.page.colors, [0xff00_00ff, 0xff00_0000]);
        assert_eq!(prepared.page.entities, [9, u32::MAX]);
        assert_eq!(prepared.page.widths, [0.5, 0.]);
        let mut bad = original.as_ref().clone();
        bad.segment_count = 3;
        assert!(Prepared::new(Rc::new(bad)).is_err());
    }
}
