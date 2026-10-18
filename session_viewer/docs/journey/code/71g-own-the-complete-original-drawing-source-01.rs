use session_rust::{Point, Xform, proto};
use std::rc::Rc;

pub struct Prepared {
    pub original: Rc<proto::Sheet>,
    pub page: Rc<crate::sheet_page::Page>,
}
impl Prepared {
    pub fn new(original: Rc<proto::Sheet>) -> Result<Self, &'static str> {
        if original.guid.is_empty()
            || original.coords.len() % 6 != 0
            || original.coords.len() / 6 != original.segment_count as usize
        {
            return Err("Original sheet count does not match its coordinates");
        }
        let count = original.segment_count as usize;
        // Missing authored columns use source defaults; present words stay original, including repeated entity IDs and hairline pens.
        fn column<T: Copy>(words: &[T], count: usize, default: T) -> Result<Vec<T>, &'static str> {
            if words.len() > count {
                return Err("Sheet attribute exceeds original segment count");
            }
            let mut words = words.to_vec();
            words.resize(count, default);
            Ok(words)
        }
        let coordinates = original
            .coords
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<_>>();
        let colors = column(&original.colors, count, 0xff00_0000)?
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>();
        let widths = column(&original.widths, count, 0.)?
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>();
        let ids = column(&original.source_ids, count, u32::MAX)?
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>();
        let page = crate::sheet_page::Page::decode(
            0..original.segment_count,
            original.segment_count,
            original.entity_count,
            &coordinates,
            Some(&colors),
            Some(&widths),
            Some(&ids),
        )?;
        Ok(Self {
            original,
            page: Rc::new(page),
        })
    }
}
