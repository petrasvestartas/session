pub use viewer_core::stream::*;

#[cfg(target_arch = "wasm32")]
pub use web::*;

/// The reads: find the arrays, then fetch slices by range.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::fetch::{GetOpts, PROBE_BYTES, Reply, Task, fetch_range, get, retryable};
    use crate::app::manifest::immutable_key;
    use crate::app::range_gate::{RANGE_READS, RangeGate};
    use crate::app::validate::varint;
    use std::cell::RefCell;
    use std::rc::Rc;

    const POINT_BYTES: u64 = 24; // three doubles

    const MAX_TABLE_BYTES: u64 = 128 * 1024 * 1024; // largest node table read

    thread_local! {
        /// The scene's streaming range reads queue here.
        static GATE: RefCell<Rc<RangeGate>> = RefCell::new(RangeGate::new(RANGE_READS));
    }

    /// A new scene: turn the old scene's waiting reads away and start an empty queue.
    pub fn reset_range_gate() {
        GATE.with_borrow_mut(|gate| {
            gate.close();
            *gate = RangeGate::new(RANGE_READS);
        });
    }

    pub(super) async fn read_window<'a>(
        window: &'a mut MetadataWindow,
        url: &str,
        at: u64,
        length: u64,
        end: u64,
        revision: &Option<String>,
    ) -> Option<&'a [u8]> {
        body_end(at, length, end)?;

        if window.slice(at, length).is_none() {
            let read_length = MetadataWindow::read_length(at, length, end)?;
            window.bytes = source_range(url, at, read_length, revision).await?;
            window.at = at;
        }

        window.slice(at, length)
    }

    /// Read one range of the file.
    async fn source_range(
        url: &str,
        at: u64,
        length: u64,
        revision: &Option<String>,
    ) -> Option<Vec<u8>> {
        if !bounded_range(at, length) {
            return None;
        }

        if length == 0 {
            return Some(Vec::new());
        }

        // the stall timer starts once the read holds a slot, so queued reads never time out
        let gate = GATE.with_borrow(Rc::clone);
        let _permit = gate.enter().await?;

        match fetch_range(url, at, length, revision).await {
            Ok((bytes, _)) => Some(bytes),
            Err(error) if retryable(&error) && !gate.is_closed() => {
                log::warn!("{url}: {error}; retrying the range once");
                Some(fetch_range(url, at, length, revision).await.ok()?.0)
            }
            Err(_) => None,
        }
    }

    /// Start reading one range of the file.
    pub(super) fn start_range(
        url: &str,
        (at, length): (u64, u64),
        revision: &Option<String>,
    ) -> Task<Option<Vec<u8>>> {
        let (url, revision) = (url.to_string(), revision.clone());
        Task::start(async move { source_range(&url, at, length, &revision).await })
    }

    /// The first 8 KB of a file, read once for every layout check and the file's size.
    pub async fn probe(url: &str) -> Option<Reply> {
        let reply = get(
            url,
            &GetOpts {
                range: Some((0, PROBE_BYTES)),
                revalidate: !immutable_key(url),
                ..Default::default()
            },
        )
        .await
        .ok()?;
        (reply.bytes.len() as u64 <= PROBE_BYTES).then_some(reply)
    }

    /// Find where a cloud's arrays are in the file, from its `probe`.
    pub async fn cloud_fields(url: &str, probe: &Reply) -> Option<CloudFields> {
        if probe.status != 206 {
            return None;
        }

        let (coords_at, coords_len, end) = cloud_layout(&probe.bytes)?;

        if coords_len == 0 || !coords_len.is_multiple_of(POINT_BYTES) {
            return None;
        }

        let after = body_end(coords_at, coords_len, end)?;
        let mut colors = (after, 0);

        if after < end {
            let header = source_range(url, after, 16.min(end - after), &probe.etag).await?;
            let (tag, used) = varint(&header, 0)?;

            if tag >> 3 == 4 && tag & 7 == 2 {
                let (length, extra) = varint(&header, used)?;
                let body = after.checked_add((used + extra) as u64)?;
                body_end(body, length, end)?;
                colors = (body, length);
            }
        }

        Some(CloudFields {
            end,
            coords_at,
            coords_len,
            colors_at: colors.0,
            colors_len: colors.1,
            normals_at: 0,
            normals_len: 0,
            count: u32::try_from(coords_len / POINT_BYTES).ok()?,
            ids_at: 0,
            ids_len: 0,
            revision: probe.etag.clone(),
        })
    }

    /// Read the cloud's node table.
    pub async fn cloud_lod(url: &str, fields: &mut CloudFields) -> Option<CloudLod> {
        let mut at = body_end(fields.colors_at, fields.colors_len, fields.end)?;
        let mut lod = CloudLod::default();
        let mut seen = [false; 7];
        let mut table_bytes = 0u64;
        let mut ids = None;
        let mut normals = (0, 0);
        let mut window = MetadataWindow::default();

        while at < fields.end {
            let header = read_window(
                &mut window,
                url,
                at,
                64.min(fields.end - at),
                fields.end,
                &fields.revision,
            )
            .await?;
            let (tag, used) = varint(header, 0)?;
            let (field, wire) = (usize::try_from(tag >> 3).ok()?, (tag & 7) as u32);

            if field == 0 {
                return None;
            }

            if wire != 2 {
                if (8..=15).contains(&field) {
                    return None;
                }

                let skip = skip_scalar(header, used, wire)?;
                at = body_end(at, (used + skip) as u64, fields.end)?;
                continue;
            }

            let (length, extra) = varint(header, used)?;
            let body = body_end(at, (used + extra) as u64, fields.end)?;
            let next = body_end(body, length, fields.end)?;

            if (8..=14).contains(&field) {
                if seen[field - 8] {
                    return None;
                }

                table_bytes = table_bytes.checked_add(length)?;

                if table_bytes > MAX_TABLE_BYTES {
                    return None;
                }

                let raw = read_window(&mut window, url, body, length, fields.end, &fields.revision)
                    .await?;

                if !lod.set_field(field, raw) {
                    return None;
                }

                seen[field - 8] = true;
            }

            if field == 5 && length == fields.coords_len {
                normals = (body, length);
            }

            if field == 15 {
                if length != u64::from(fields.count) * 4 || ids.is_some() {
                    return None;
                }

                ids = Some((body, length));
            }

            at = next;
        }

        if seen.contains(&false) || !lod.valid(fields.count) {
            return None;
        }

        (fields.ids_at, fields.ids_len) = ids.unwrap_or((0, 0));
        (fields.normals_at, fields.normals_len) = normals;
        Some(lod)
    }

    /// Points `[from, to)` of the cloud.
    pub async fn fetch_positions(
        url: &str,
        fields: &CloudFields,
        from: u32,
        to: u32,
    ) -> Option<Vec<f32>> {
        if from > to || to > fields.count {
            return None;
        }

        let at = fields
            .coords_at
            .checked_add(u64::from(from).checked_mul(POINT_BYTES)?)?;
        let length = u64::from(to - from).checked_mul(POINT_BYTES)?;
        body_end(
            at,
            length,
            body_end(fields.coords_at, fields.coords_len, fields.end)?,
        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        checked_positions(&raw, to - from)
    }

    /// Packed normals of points `[from, to)`; None when the cloud has none.
    pub async fn fetch_normals(
        url: &str,
        fields: &CloudFields,
        from: u32,
        to: u32,
    ) -> Option<Vec<u32>> {
        if fields.normals_len == 0 || from > to || to > fields.count {
            return None;
        }

        let at = fields
            .normals_at
            .checked_add(u64::from(from).checked_mul(POINT_BYTES)?)?;
        let length = u64::from(to - from).checked_mul(POINT_BYTES)?;
        body_end(
            at,
            length,
            body_end(fields.normals_at, fields.normals_len, fields.end)?,
        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        (raw.len() as u64 == length).then(|| normals_from(&raw))
    }

    /// Colours of `count` points starting at byte `at`.
    pub async fn fetch_colors(
        url: &str,
        fields: &CloudFields,
        at: u64,
        count: u32,
    ) -> Option<(Vec<u32>, u64)> {
        let end = body_end(fields.colors_at, fields.colors_len, fields.end)?;

        if at < fields.colors_at || at > end || count > fields.count {
            return None;
        }

        // a colour is at most 8 bytes of varints
        let length = (u64::from(count) * 8).min(end - at);

        if length == 0 {
            return None;
        }

        let raw = source_range(url, at, length, &fields.revision).await?;
        let (colors, used) = colors_from(&raw, count)?;
        Some((colors, body_end(at, used as u64, end)?))
    }
}

#[cfg(target_arch = "wasm32")]
pub use web_sheets::*;

#[cfg(target_arch = "wasm32")]
mod web_sheets {
    use super::web::{read_window, start_range};
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
            let header = read_window(
                &mut window,
                url,
                at,
                64.min(end - at),
                end,
                &fields.revision,
            )
            .await?;
            let f = field_at(header, at, end)?;
            let body = if (f.field, f.wire) == (8, 2) {
                if f.value > NAME_BYTES {
                    return None;
                }

                read_window(&mut window, url, f.body, f.value, end, &fields.revision).await?
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
