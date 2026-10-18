use std::ops::Range;
pub struct Page {
    pub rows: Range<u32>,
    pub coordinates: Vec<f64>,
    pub display: Vec<f32>,
    pub colors: Vec<u32>,
    pub widths: Vec<f32>,
    pub entities: Vec<u32>,
}
impl Page {
    pub fn decode(
        rows: Range<u32>,
        total: u32,
        entities: u32,
        coordinates: &[u8],
        colors: Option<&[u8]>,
        widths: Option<&[u8]>,
        ids: Option<&[u8]>,
    ) -> Result<Self, &'static str> {
        if rows.start > rows.end || rows.end > total {
            return Err("Invalid original sheet row interval");
        }
        let count = (rows.end - rows.start) as usize;
        if coordinates.len() != count.checked_mul(48).ok_or("Sheet page size overflow")? {
            return Err("Original sheet coordinates do not match segment count");
        }
        let coordinates = coordinates
            .chunks_exact(8)
            .map(|c| f64::from_le_bytes(c.try_into().unwrap()))
            .collect::<Vec<_>>();
        let display = coordinates.iter().map(|c| *c as f32).collect::<Vec<_>>();
        if coordinates.iter().any(|c| !c.is_finite()) || display.iter().any(|c| !c.is_finite()) {
            return Err("Invalid original sheet coordinates");
        }
        fn words(raw: Option<&[u8]>, count: usize, default: u32) -> Result<Vec<u32>, &'static str> {
            let Some(raw) = raw else {
                return Ok(vec![default; count]);
            };
            if raw.len() != count * 4 {
                return Err("Original sheet attribute does not match segment count");
            }
            Ok(raw
                .chunks_exact(4)
                .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
                .collect())
        }
        let colors = words(colors, count, 0xff00_0000)?;
        let widths = words(widths, count, 0)?
            .into_iter()
            .map(f32::from_bits)
            .collect::<Vec<_>>();
        let ids = words(ids, count, u32::MAX)?;
        if ids.iter().any(|id| *id != u32::MAX && *id >= entities) {
            return Err("Original sheet entity is outside its side table");
        }
        Ok(Self {
            rows,
            coordinates,
            display,
            colors,
            widths,
            entities: ids,
        })
    }
}
