
// A sheet's arrays are found the same way, then each slice reads coords, colours, widths and ids as four ranges at once.
#[cfg(target_arch = "wasm32")]
pub use web_sheets::*;

#[cfg(target_arch = "wasm32")]
mod web_sheets {
    use super::web::start_range;
    use super::*;
    use crate::app::fetch::Reply;
    use crate::app::walk::sheet::SheetRows;

    /// Find where a sheet's arrays are in the file, from its `probe`.
    pub async fn sheet_fields(url: &str, probe: &Reply) -> Option<SheetFields> {
        if probe.status != 206 {
            return None;
        }

        let (coords_at, coords_len, end) = sheet_layout(&probe.bytes)?;

        if coords_len == 0 || !coords_len.is_multiple_of(SheetFields::SEGMENT_BYTES) {
            return None;
        }

        let mut fields = SheetFields {
            end,
            coords_at,
            coords_len,
            count: u32::try_from(coords_len / SheetFields::SEGMENT_BYTES).ok()?,
            revision: probe.etag.clone(),
            ..Default::default()
        };
        let mut at = body_end(coords_at, coords_len, end)?;
        let mut window = MetadataWindow::default();

        while at < end {
            let header = window
                .read(url, at, 64.min(end - at), end, &fields.revision)
                .await?;
            let f = field_at(header, at, end)?;
            let body = if (f.field, f.wire) == (8, 2) {
                if f.value > NAME_BYTES {
                    return None;
                }

                window
                    .read(url, f.body, f.value, end, &fields.revision)
                    .await?
            } else {
                &[]
            };

            if !fields.set(&f, body) {
                return None;
            }

            at = f.next;
        }

        Some(fields)
    }

    /// The byte range of entries `[from, to)` of one fixed-width array.
    fn sheet_range(
        fields: &SheetFields,
        (at, len): (u64, u64),
        from: u32,
        to: u32,
        stride: u64,
    ) -> Option<(u64, u64)> {
        if len == 0 {
            return Some((0, 0));
        }

        let start = at.checked_add(u64::from(from).checked_mul(stride)?)?;
        let length = u64::from(to - from).checked_mul(stride)?;
        body_end(start, length, body_end(at, len, fields.end)?)?;
        Some((start, length))
    }

    /// Segments `[from, to)` of a sheet, its four arrays read at once.
    pub async fn fetch_sheet_slice(
        url: &str,
        fields: &SheetFields,
        from: u32,
        to: u32,
    ) -> Option<SheetRows> {
        if from > to || to > fields.count {
            return None;
        }

        let coords = (fields.coords_at, fields.coords_len);
        let coords = sheet_range(fields, coords, from, to, SheetFields::SEGMENT_BYTES)?;
        // colour, width and id are 4 bytes a segment
        let colors = sheet_range(fields, (fields.colors_at, fields.colors_len), from, to, 4)?;
        let widths = sheet_range(fields, (fields.widths_at, fields.widths_len), from, to, 4)?;
        let ids = sheet_range(fields, (fields.ids_at, fields.ids_len), from, to, 4)?;
        let reads =
            [coords, colors, widths, ids].map(|range| start_range(url, range, &fields.revision));
        let [coords, colors, widths, ids] = reads;
        let positions = checked_doubles(&coords.wait().await??, u64::from(to - from) * 6);
        let colors = packed_u32(&colors.wait().await??);
        let widths = packed_f32(&widths.wait().await??);
        let ids = packed_u32(&ids.wait().await??);
        Some(SheetRows {
            positions: positions?,
            colors,
            widths,
            ids,
        })
    }
}
