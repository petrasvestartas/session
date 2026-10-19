    (count > 0).then(|| from..from + count)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sheet_prefix_chunks_ceiling_and_retained_page_charges_are_explicit() {
        assert_eq!(PREFIX, 25_000);
        assert_eq!(CHUNK, 500_000);
        assert_eq!(CEILING, 3_000_000);
        assert_eq!(next(25_000, 4_000_000, 900_000), Some(25_000..525_000));
        assert_eq!(
            next(u32::MAX - 3, u32::MAX, 10),
            Some(u32::MAX - 3..u32::MAX)
        );
        assert!(next(0, 10, 0).is_none());
        assert_eq!(temporary_bytes(CHUNK), Some(96_000_000));
        let budget = Budget::new(512);
        let source = Rc::new(Source {
            guid: "drawing".into(),
            name: "plan".into(),
            url: "plan.pb".into(),
            fields: crate::stream::SheetFields {
                count: 2,
                entities: 1,
                ..Default::default()
            },
        });
        let stream = Stream::new(source, budget.clone());
        let raw = [0f64, 0., 0., 1., 0., 0.]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect::<Vec<_>>();
        let page = Page::decode(0..1, 2, 1, &raw, None, None, None).unwrap();
        let held = stream.adopt(page, budget.reserve(192).unwrap()).unwrap();
        assert_eq!(budget.used(), 84);
        assert_eq!(stream.loaded.get(), 1);
        stream.pages.borrow_mut().clear();
        assert_eq!(budget.used(), 84);
        drop(held);
        assert_eq!(budget.used(), 0);
    }
}
