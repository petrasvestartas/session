//! Reading a cloud or a sheet by HTTP Range, without decoding the file whole. Two facts about
//! the wire format make it possible: every hop `Session.3 -> Objects.8 -> PointCloud` (or
//! `Objects.17 -> Sheet`) is length-delimited, so the headers sit in the first few KB; and
//! `coords` is packed double, so its length prefix gives the exact count before a byte of
//! payload is read. The byte-level parsing here is pure and tested natively; the fetching
//! half is wasm-only.

/// Where the packed source arrays live in the file, as absolute byte offsets.
#[derive(Clone, Debug)]
pub struct CloudFields {
    /// End of the enclosing PointCloud message, exclusive.
    pub end: u64,
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,
    pub count: u32,
    /// Packed fixed32 original point IDs (field 15); absent means identity IDs.
    pub ids_at: u64,
    pub ids_len: u64,
    /// Source validator captured with the first metadata range when the server exposes it.
    pub revision: Option<String>,
}

/// Where a sheet's fixed-width arrays live in the file, as absolute byte offsets; a zero
/// length means the field is absent (default colour, hairline pens, no entity ids).
#[derive(Clone, Debug, Default)]
pub struct SheetFields {
    /// End of the enclosing Sheet message, exclusive.
    pub end: u64,
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,
    pub widths_at: u64,
    pub widths_len: u64,
    pub ids_at: u64,
    pub ids_len: u64,
    /// Segments, from the coords length; `segment_count` (field 6) must agree.
    pub count: u32,
    /// Records in the side table (field 7).
    pub entities: u32,
    /// Side-table file name relative to the sheet URL; empty = none.
    pub meta: String,
    pub revision: Option<String>,
}

/// One field header inside a message: what it is, its varint value or body length, where
/// the body starts and where the next field does.
#[derive(Clone, Copy)]
pub struct Field {
    pub field: u32,
    pub wire: u32,
    pub value: u64,
    pub body: u64,
    pub next: u64,
}

/// The field at `at`, whose header is the start of `header`; bodies never cross `end`.
pub fn field_at(header: &[u8], at: u64, end: u64) -> Option<Field> {
    let (tag, used) = varint(header, 0)?;
    let (field, wire) = (u32::try_from(tag >> 3).ok()?, (tag & 7) as u32);
    if field == 0 {
        return None;
    }
    let body = body_end(at, used as u64, end)?;
    if wire != 2 {
        let (value, _) = if wire == 0 {
            varint(header, used)?
        } else {
            (0, 0)
        };
        let skip = skip_scalar(header, used, wire)?;
        return Some(Field {
            field,
            wire,
            value,
            body,
            next: body_end(body, skip as u64, end)?,
        });
    }
    let (length, extra) = varint(header, used)?;
    let body = body_end(body, extra as u64, end)?;
    Some(Field {
        field,
        wire,
        value: length,
        body,
        next: body_end(body, length, end)?,
    })
}

impl SheetFields {
    /// Bytes per segment in `coords`: six doubles.
    pub const SEGMENT_BYTES: u64 = 48;

    /// Record one field found after `coords`; `body` is the bytes of a small string field.
    /// False for a repeated array, a count that disagrees with `coords`, or a field of the
    /// wrong wire type.
    pub fn set(&mut self, f: &Field, body: &[u8]) -> bool {
        let per_segment = u64::from(self.count) * 4;
        match (f.field, f.wire) {
            (4, 2) if self.colors_len == 0 && f.value == per_segment => {
                (self.colors_at, self.colors_len) = (f.body, f.value)
            }
            (5, 2) if self.widths_len == 0 && f.value == per_segment => {
                (self.widths_at, self.widths_len) = (f.body, f.value)
            }
            (15, 2) if self.ids_len == 0 && f.value == per_segment => {
                (self.ids_at, self.ids_len) = (f.body, f.value)
            }
            (6, 0) => return f.value == u64::from(self.count),
            (7, 0) => match u32::try_from(f.value) {
                Ok(entities) => self.entities = entities,
                Err(_) => return false,
            },
            (8, 2) if f.value <= NAME_BYTES => match std::str::from_utf8(body) {
                Ok(meta) => self.meta = meta.to_string(),
                Err(_) => return false,
            },
            (1..=8, _) | (15, _) => return false,
            _ => {}
        }
        true
    }
}

/// One cloud's LOD node table, read from the file's tail without touching a point.
/// `first`/`count` index the cloud's rows, which are stored in octree order.
#[derive(Clone, Default)]
pub struct CloudLod {
    pub min: Vec<f64>,
    pub size: Vec<f64>,
    pub spacing: Vec<f64>,
    pub level: Vec<i32>,
    pub first: Vec<i32>,
    pub count: Vec<i32>,
    pub children: Vec<i32>,
}

impl CloudLod {
    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.size.len()
    }

    /// Decode one complete packed LOD array; false for malformed or unknown fields.
    pub fn set_field(&mut self, field: usize, raw: &[u8]) -> bool {
        if (8..=10).contains(&field) && !raw.len().is_multiple_of(8) {
            return false;
        }
        if (11..=14).contains(&field) {
            let mut at = 0;
            while at < raw.len() {
                let Some((_, bytes)) = varint(raw, at) else {
                    return false;
                };
                at += bytes;
            }
        }
        match field {
            8 => self.min = packed_f64(raw),
            9 => self.size = packed_f64(raw),
            10 => self.spacing = packed_f64(raw),
            11 => self.level = packed_i32(raw),
            12 => self.first = packed_i32(raw),
            13 => self.count = packed_i32(raw),
            14 => self.children = packed_i32(raw),
            _ => return false,
        }
        true
    }

    /// Validate the complete table before the scene indexes its parallel arrays or children.
    pub fn valid(&self, total: u32) -> bool {
        let n = self.len();
        let Some(triples) = n.checked_mul(3) else {
            return false;
        };
        let Some(children) = n.checked_mul(8) else {
            return false;
        };
        if n == 0
            || self.min.len() != triples
            || self.spacing.len() != n
            || self.level.len() != n
            || self.first.len() != n
            || self.count.len() != n
            || self.children.len() != children
            || self.level[0] != 0
        {
            return false;
        }
        let mut parents = vec![false; n];
        for node in 0..n {
            if !self.valid_node(node, total) || !self.valid_children(node, &mut parents) {
                return false;
            }
        }
        !parents[1..].contains(&false)
    }

    /// One node's finite GPU cube, spacing, level and source row range.
    fn valid_node(&self, node: usize, total: u32) -> bool {
        let size = self.size[node];
        let spacing = self.spacing[node];
        if !finite_float(size)
            || size < 0.0
            || !finite_float(spacing)
            || spacing < 0.0
            || self.level[node] < 0
        {
            return false;
        }
        for &min in &self.min[node * 3..node * 3 + 3] {
            if !finite_float(min) || !finite_float(min + size) {
                return false;
            }
        }
        let (Ok(first), Ok(count)) = (
            u32::try_from(self.first[node]),
            u32::try_from(self.count[node]),
        ) else {
            return false;
        };
        let Some(end) = first.checked_add(count) else {
            return false;
        };
        end <= total
    }

    /// Every child has one parent at the preceding level, preventing cycles and aliases.
    fn valid_children(&self, node: usize, parents: &mut [bool]) -> bool {
        for &child in &self.children[node * 8..node * 8 + 8] {
            if child == -1 {
                continue;
            }
            let Ok(child) = usize::try_from(child) else {
                return false;
            };
            if child == 0
                || child >= parents.len()
                || parents[child]
                || self.level[node].checked_add(1) != Some(self.level[child])
            {
                return false;
            }
            parents[child] = true;
        }
        true
    }

    /// True when the file carried no octree.
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}

/// One protobuf varint at `i`: the value and how many bytes it took.
pub fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;
    loop {
        let byte = *b.get(i)?;
        if shift == 63 && byte > 1 {
            return None;
        }
        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;
        if byte & 0x80 == 0 {
            return Some((v, i - start));
        }
        shift += 7;
        if shift > 63 {
            return None;
        }
    }
}

/// Bytes a non-length-delimited field of wire type `wire` occupies at `i`.
fn skip_scalar(b: &[u8], i: usize, wire: u32) -> Option<usize> {
    match wire {
        0 => Some(varint(b, i)?.1),
        1 => Some(8),
        5 => Some(4),
        _ => None,
    }
}

/// A skipped field longer than this is geometry, not a name: the file holds more than one
/// cloud and is not streamed.
const NAME_BYTES: u64 = 256;

/// Walk `head` down `Session.3 -> Objects.8 -> PointCloud` and report where `coords` (field
/// 3) starts and how long it is. `None` for anything that is not a single-cloud file: a
/// geometry-sized field before the cloud, or bytes after it inside the objects message.
pub fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
    let (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}

/// Read only header bytes with a usize cursor; message bounds remain u64 even on wasm32.
fn cloud_layout(head: &[u8]) -> Option<(u64, u64, u64)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 8)?;
    first_array(head, at, end)
}

/// The same walk down `Session.3 -> Objects.17 -> Sheet`: where `coords` starts, how long it
/// is, and where the sheet message ends.
pub fn sheet_layout(head: &[u8]) -> Option<(u64, u64, u64)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 17)?;
    first_array(head, at, end)
}

/// The message's `coords` (field 3), which only small names may precede: a later field or a
/// geometry-sized one before it means the file is not laid out for ranged reads.
fn first_array(head: &[u8], mut at: usize, end: u64) -> Option<(u64, u64, u64)> {
    while (at as u64) < end {
        let (tag, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        let (field, wire) = (u32::try_from(tag >> 3).ok()?, (tag & 7) as u32);
        if field == 0 {
            return None;
        }
        if wire != 2 {
            at = at.checked_add(skip_scalar(head, at, wire)?)?;
            if at as u64 > end {
                return None;
            }
            continue;
        }
        let (length, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        let next = (at as u64).checked_add(length)?;
        if next > end {
            return None;
        }
        if field == 3 {
            return Some((at as u64, length, end));
        }
        if field > 3 || length > NAME_BYTES {
            return None;
        }
        at = usize::try_from(next).ok()?;
    }
    None
}

/// Descend one length-delimited envelope, skipping only small names before its target field.
/// Inside a bounded parent (`Objects`) the target must close the parent: a single-object file.
fn descend_message(head: &[u8], at: &mut usize, parent_end: Option<u64>, want: u32) -> Option<u64> {
    loop {
        let (tag, used) = varint(head, *at)?;
        *at = (*at).checked_add(used)?;
        let (field, wire) = (u32::try_from(tag >> 3).ok()?, tag & 7);
        if field == 0 || wire != 2 {
            return None;
        }
        let (length, used) = varint(head, *at)?;
        *at = (*at).checked_add(used)?;
        let next = (*at as u64).checked_add(length)?;
        if let Some(end) = parent_end
            && next > end
        {
            return None;
        }
        if field == want {
            if let Some(end) = parent_end
                && end != next
            {
                return None;
            }
            return Some(next);
        }
        if length > NAME_BYTES {
            return None;
        }
        *at = usize::try_from(next).ok()?;
    }
}

/// A finite value that remains finite when uploaded to the f32 GPU tables.
fn finite_float(value: f64) -> bool {
    value.is_finite() && (value as f32).is_finite()
}

/// Exact packed triples only; truncated reads and nonrepresentable coordinates never upload.
#[cfg(any(target_arch = "wasm32", test))]
fn checked_positions(raw: &[u8], count: u32) -> Option<Vec<f32>> {
    checked_doubles(raw, u64::from(count).checked_mul(3)?)
}

/// Exactly `n` finite doubles as f32; anything else never uploads.
#[cfg(any(target_arch = "wasm32", test))]
fn checked_doubles(raw: &[u8], n: u64) -> Option<Vec<f32>> {
    if raw.len() as u64 != n.checked_mul(8)? {
        return None;
    }
    let mut out = Vec::with_capacity(raw.len() / 8);
    for bytes in raw.chunks_exact(8) {
        let value = f64::from_le_bytes(bytes.try_into().ok()?);
        if !finite_float(value) {
            return None;
        }
        out.push(value as f32);
    }
    Some(out)
}

/// One metadata/position/color request is bounded before any network allocation.
#[cfg(any(target_arch = "wasm32", test))]
fn bounded_range(at: u64, length: u64) -> bool {
    length <= 64 * 1024 * 1024 && at.checked_add(length).is_some()
}

/// Checked body bounds, shared by metadata, position and color ranges.
#[cfg(any(target_arch = "wasm32", test))]
fn body_end(at: u64, length: u64, end: u64) -> Option<u64> {
    let next = at.checked_add(length)?;
    if next <= end { Some(next) } else { None }
}

/// One bounded metadata window; skipped geometry fields never determine its allocation.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Default)]
struct MetadataWindow {
    at: u64,
    bytes: Vec<u8>,
}

#[cfg(any(target_arch = "wasm32", test))]
impl MetadataWindow {
    /// Borrow an exact cached range, including a valid empty range at the window's end.
    fn slice(&self, at: u64, length: u64) -> Option<&[u8]> {
        let start = usize::try_from(at.checked_sub(self.at)?).ok()?;
        let length = usize::try_from(length).ok()?;
        self.bytes.get(start..start.checked_add(length)?)
    }

    /// Read ahead 64 KiB inside the message; larger packed arrays retain the existing cap.
    fn read_length(at: u64, length: u64, end: u64) -> Option<u64> {
        if !bounded_range(at, length) {
            return None;
        }
        body_end(at, length, end)?;
        Some(length.max(64 * 1024).min(end - at))
    }
}

/// A packed `int32` (varint) array in full.
pub fn packed_i32(raw: &[u8]) -> Vec<i32> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < raw.len() {
        let Some((v, n)) = varint(raw, i) else { break };
        out.push(v as i32);
        i += n;
    }
    out
}

/// A packed `double` array in full.
pub fn packed_f64(raw: &[u8]) -> Vec<f64> {
    let mut out = Vec::with_capacity(raw.len() / 8);
    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()));
    }
    out
}

/// A packed `fixed32` array in full.
pub fn packed_u32(raw: &[u8]) -> Vec<u32> {
    let mut out = Vec::with_capacity(raw.len() / 4);
    for c in raw.chunks_exact(4) {
        out.push(u32::from_le_bytes(c.try_into().unwrap()));
    }
    out
}

/// A packed `float` array in full; a non-finite value becomes 0 (the hairline pen).
pub fn packed_f32(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 4);
    for c in raw.chunks_exact(4) {
        let v = f32::from_le_bytes(c.try_into().unwrap());
        out.push(if v.is_finite() { v } else { 0.0 });
    }
    out
}

/// An already-fetched coords slice as f32 triples.
pub fn positions_from(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 8);
    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()) as f32);
    }
    out
}

/// `count` RGBA colours decoded from packed varints in `raw`, and the byte offset just past
/// the last one (so the next slice starts on a varint boundary).
pub fn colors_from(raw: &[u8], count: u32) -> Option<(Vec<u32>, usize)> {
    let mut out = Vec::with_capacity(count as usize);
    let mut i = 0usize;
    for _ in 0..count {
        let mut rgba = [255u8; 4];
        for k in &mut rgba {
            let (v, n) = varint(raw, i)?;
            i += n;
            *k = (v & 255) as u8;
        }
        out.push(u32::from_le_bytes(rgba));
    }
    Some((out, i))
}

#[cfg(target_arch = "wasm32")]
pub use web::*;

/// The fetching half: bounded metadata windows locate arrays, then slices arrive by range.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::fetch::{GetOpts, get};
    use crate::app::walk::sheet::SheetRows;

    const POINT_BYTES: u64 = 24;
    const MAX_TABLE_BYTES: u64 = 128 * 1024 * 1024;

    impl MetadataWindow {
        /// Reuse adjacent headers/arrays under the same ETag; replace the window on a jump.
        async fn read(
            &mut self,
            url: &str,
            at: u64,
            length: u64,
            end: u64,
            revision: &Option<String>,
        ) -> Option<&[u8]> {
            body_end(at, length, end)?;
            if self.slice(at, length).is_none() {
                let read_length = Self::read_length(at, length, end)?;
                self.bytes = source_range(url, at, read_length, revision).await?;
                self.at = at;
            }
            self.slice(at, length)
        }
    }

    /// Exact bounded 206 bodies, tied to the metadata's exposed source validator.
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
        let reply = get(
            url,
            &GetOpts {
                range: Some((at, length)),
                revalidate: true,
                ..Default::default()
            },
        )
        .await
        .ok()?;
        if reply.status != 206 || reply.bytes.len() as u64 != length {
            return None;
        }
        if revision.is_some() && revision != &reply.etag {
            return None;
        }
        Some(reply.bytes)
    }

    /// Locate the coordinate/color runs without crossing the enclosing cloud message.
    pub async fn cloud_fields(url: &str) -> Option<CloudFields> {
        let reply = get(
            url,
            &GetOpts {
                range: Some((0, 8192)),
                revalidate: true,
                ..Default::default()
            },
        )
        .await
        .ok()?;
        if reply.status != 206 || reply.bytes.len() > 8192 {
            return None;
        }
        let (coords_at, coords_len, end) = cloud_layout(&reply.bytes)?;
        if coords_len == 0 || !coords_len.is_multiple_of(POINT_BYTES) {
            return None;
        }
        let after = body_end(coords_at, coords_len, end)?;
        let mut colors = (after, 0);
        if after < end {
            let header = source_range(url, after, 16.min(end - after), &reply.etag).await?;
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
            count: u32::try_from(coords_len / POINT_BYTES).ok()?,
            ids_at: 0,
            ids_len: 0,
            revision: reply.etag,
        })
    }

    /// Read bounded LOD arrays and record fixed32 IDs, validating the table before upload.
    pub async fn cloud_lod(url: &str, fields: &mut CloudFields) -> Option<CloudLod> {
        let mut at = body_end(fields.colors_at, fields.colors_len, fields.end)?;
        let mut lod = CloudLod::default();
        let mut seen = [false; 7];
        let mut table_bytes = 0u64;
        let mut ids = None;
        let mut window = MetadataWindow::default();
        while at < fields.end {
            let header = window
                .read(
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
                let raw = window
                    .read(url, body, length, fields.end, &fields.revision)
                    .await?;
                if !lod.set_field(field, raw) {
                    return None;
                }
                seen[field - 8] = true;
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
        Some(lod)
    }

    /// Exact finite points `[from, to)` of the source coordinate run, under one revision.
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

    /// A bounded sequential packed color range under the same source revision as positions.
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
        let length = (u64::from(count) * 8).min(end - at);
        if length == 0 {
            return None;
        }
        let raw = source_range(url, at, length, &fields.revision).await?;
        let (colors, used) = colors_from(&raw, count)?;
        Some((colors, body_end(at, used as u64, end)?))
    }

    /// Locate a sheet's arrays: one head read finds `coords`, one bounded window walk after it
    /// records the rest without reading a segment.
    pub async fn sheet_fields(url: &str) -> Option<SheetFields> {
        let reply = get(
            url,
            &GetOpts {
                range: Some((0, 8192)),
                revalidate: true,
                ..Default::default()
            },
        )
        .await
        .ok()?;
        if reply.status != 206 || reply.bytes.len() > 8192 {
            return None;
        }
        let (coords_at, coords_len, end) = sheet_layout(&reply.bytes)?;
        if coords_len == 0 || !coords_len.is_multiple_of(SheetFields::SEGMENT_BYTES) {
            return None;
        }
        let mut fields = SheetFields {
            end,
            coords_at,
            coords_len,
            count: u32::try_from(coords_len / SheetFields::SEGMENT_BYTES).ok()?,
            revision: reply.etag,
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

    /// Segments `[from, to)` of one fixed-width array: `stride` bytes each, by one range read.
    async fn sheet_array(
        url: &str,
        fields: &SheetFields,
        (at, len): (u64, u64),
        from: u32,
        to: u32,
        stride: u64,
    ) -> Option<Vec<u8>> {
        if len == 0 {
            return Some(Vec::new());
        }
        let start = at.checked_add(u64::from(from).checked_mul(stride)?)?;
        let length = u64::from(to - from).checked_mul(stride)?;
        body_end(start, length, body_end(at, len, fields.end)?)?;
        source_range(url, start, length, &fields.revision).await
    }

    /// Segments `[from, to)` of a sheet: four range reads under one revision. An absent array
    /// comes back empty and the walk pads it.
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
        let raw = sheet_array(url, fields, coords, from, to, SheetFields::SEGMENT_BYTES).await?;
        let positions = checked_doubles(&raw, u64::from(to - from) * 6)?;
        let colors = (fields.colors_at, fields.colors_len);
        let colors = packed_u32(&sheet_array(url, fields, colors, from, to, 4).await?);
        let widths = (fields.widths_at, fields.widths_len);
        let widths = packed_f32(&sheet_array(url, fields, widths, from, to, 4).await?);
        let ids = (fields.ids_at, fields.ids_len);
        let ids = packed_u32(&sheet_array(url, fields, ids, from, to, 4).await?);
        Some(SheetRows {
            positions,
            colors,
            widths,
            ids,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Adjacent metadata borrows one window; jumps and boundary crossings force a refill.
    #[test]
    fn metadata_window_checks_ranges_without_copying_or_following_large_skips() {
        let window = MetadataWindow {
            at: 100,
            bytes: vec![1, 2, 3, 4],
        };
        assert_eq!(window.slice(101, 2), Some(&[2, 3][..]));
        assert_eq!(window.slice(104, 0), Some(&[][..]));
        assert!(window.slice(99, 1).is_none());
        assert!(window.slice(103, 2).is_none());
        assert!(window.slice(u64::MAX, 2).is_none());
        assert_eq!(
            MetadataWindow::read_length(100, 64, 1_000_000_000),
            Some(65_536)
        );
        assert_eq!(MetadataWindow::read_length(100, 64, 170), Some(70));
        assert_eq!(
            MetadataWindow::read_length(100, 70_000, 1_000_000),
            Some(70_000)
        );
        assert_eq!(MetadataWindow::read_length(100, 64, 150), None);
        assert_eq!(
            MetadataWindow::read_length(0, 64 * 1024 * 1024 + 1, u64::MAX),
            None
        );
        assert_eq!(MetadataWindow::read_length(u64::MAX - 1, 2, u64::MAX), None);
    }

    fn valid_lod() -> CloudLod {
        CloudLod {
            min: vec![0.0; 6],
            size: vec![10.0, 1.0],
            spacing: vec![1.0, 0.5],
            level: vec![0, 1],
            first: vec![0, 1],
            count: vec![1, 1],
            children: [vec![1], vec![-1; 15]].concat(),
        }
    }

    #[test]
    fn lod_parallel_arrays_bounds_ranges_and_child_graph_are_validated() {
        assert!(valid_lod().valid(2));
        let mut lod = valid_lod();
        lod.min.pop();
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.spacing[0] = f64::NAN;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.size[0] = f64::MAX;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.min[0] = f32::MAX as f64;
        lod.size[0] = f32::MAX as f64;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.first[1] = 2;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.count[1] = -1;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.children[0] = 2;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.children[8] = 0;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.children[0] = -1;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.children[1] = 1;
        assert!(!lod.valid(2));
    }

    #[test]
    fn packed_metadata_and_positions_reject_partial_or_nonfinite_values() {
        let mut lod = CloudLod::default();
        assert!(!lod.set_field(8, &[0; 7]));
        assert!(!lod.set_field(14, &[0x80]));
        let raw: Vec<_> = [1.0f64, 2.0, 3.0]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect();
        assert_eq!(checked_positions(&raw, 1).unwrap(), [1.0, 2.0, 3.0]);
        assert!(checked_positions(&raw[..23], 1).is_none());
        assert!(checked_positions(&raw, 2).is_none());
        for bad in [f64::NAN, f64::INFINITY, f64::MAX] {
            let raw: Vec<_> = [bad, 0.0, 0.0]
                .into_iter()
                .flat_map(f64::to_le_bytes)
                .collect();
            assert!(checked_positions(&raw, 1).is_none());
        }
    }

    #[test]
    fn source_offsets_do_not_wrap_or_cross_the_enclosing_message() {
        assert!(bounded_range(0, 64 * 1024 * 1024));
        assert!(!bounded_range(0, 64 * 1024 * 1024 + 1));
        assert!(!bounded_range(u64::MAX - 1, 3));
        assert_eq!(body_end(u64::MAX - 1, 3, u64::MAX), None);
        assert_eq!(body_end(50, 51, 100), None);
        assert_eq!(body_end(50, 50, 100), Some(100));
        // The cloud's coords field claims 24 bytes inside a two-byte cloud message.
        assert_eq!(walk_to_coords(&[0x1a, 4, 0x42, 2, 0x1a, 24]), None);
    }

    /// A varint round trip over the boundaries that matter.
    #[test]
    fn varint_reads_one_and_two_byte_values() {
        assert_eq!(varint(&[0x05], 0), Some((5, 1)));
        assert_eq!(varint(&[0xac, 0x02], 0), Some((300, 2)));
        assert_eq!(varint(&[0x80], 0), None);
        assert_eq!(
            varint(
                &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 1],
                0
            ),
            Some((u64::MAX, 10))
        );
        assert_eq!(
            varint(
                &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 2],
                0
            ),
            None
        );
    }

    /// One protobuf varint.
    fn uvarint(mut v: u64) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let byte = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                out.push(byte);
                return out;
            }
            out.push(byte | 0x80);
        }
    }

    /// One length-delimited field: tag, length, body.
    fn bytes_field(field: u32, body: &[u8]) -> Vec<u8> {
        let mut out = uvarint(u64::from(field << 3 | 2));
        out.extend(uvarint(body.len() as u64));
        out.extend_from_slice(body);
        out
    }

    /// One varint field.
    fn uint_field(field: u32, value: u32) -> Vec<u8> {
        let mut out = uvarint(u64::from(field << 3));
        out.extend(uvarint(u64::from(value)));
        out
    }

    /// A one-segment sheet with every array present, wrapped in Objects and Session.
    fn sheet_file() -> Vec<u8> {
        let coords: Vec<u8> = [0.0f64, 1.0, 2.0, 3.0, 4.0, 5.0]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect();
        let mut sheet = bytes_field(1, b"guid");
        sheet.extend(bytes_field(2, b"plan"));
        sheet.extend(bytes_field(3, &coords));
        sheet.extend(bytes_field(4, &0xff00_00ffu32.to_le_bytes()));
        sheet.extend(bytes_field(5, &0.35f32.to_le_bytes()));
        sheet.extend(uint_field(6, 1));
        sheet.extend(uint_field(7, 9));
        sheet.extend(bytes_field(8, b"plan.shm"));
        sheet.extend(bytes_field(15, &7u32.to_le_bytes()));
        let objects = bytes_field(17, &sheet);
        bytes_field(3, &objects)
    }

    /// The tail scan the fetching half runs, over bytes already in memory.
    fn scan_tail(file: &[u8], fields: &mut SheetFields, mut at: u64) -> bool {
        while at < fields.end {
            let Some(f) = field_at(&file[at as usize..], at, fields.end) else {
                return false;
            };
            let body = if f.wire == 2 {
                &file[f.body as usize..(f.body + f.value.min(NAME_BYTES)) as usize]
            } else {
                &[]
            };
            if !fields.set(&f, body) {
                return false;
            }
            at = f.next;
        }
        true
    }

    /// The sheet walk finds Objects field 17 and records every fixed-width array after coords.
    #[test]
    fn sheet_layout_walks_objects_field_17_and_locates_every_array() {
        let file = sheet_file();
        let (coords_at, coords_len, end) = sheet_layout(&file).unwrap();
        assert_eq!(coords_len, 48);
        assert_eq!(end, file.len() as u64);
        assert_eq!(
            packed_f64(&file[coords_at as usize..(coords_at + 48) as usize])[5],
            5.0
        );
        let mut fields = SheetFields {
            end,
            coords_at,
            coords_len,
            count: 1,
            ..Default::default()
        };
        assert!(scan_tail(&file, &mut fields, coords_at + coords_len));
        assert_eq!(fields.colors_len, 4);
        assert_eq!(fields.widths_len, 4);
        assert_eq!(fields.ids_len, 4);
        assert_eq!(fields.entities, 9);
        assert_eq!(fields.meta, "plan.shm");
        assert_eq!(
            packed_u32(&file[fields.colors_at as usize..][..4]),
            [0xff00_00ff]
        );
        assert_eq!(packed_f32(&file[fields.widths_at as usize..][..4]), [0.35]);
        assert_eq!(packed_u32(&file[fields.ids_at as usize..][..4]), [7]);
        assert_eq!(fields.ids_at + 4, end);
        // The cloud walk does not accept a sheet, and a sheet followed by another object is
        // not a single-object file.
        assert!(walk_to_coords(&file).is_none());
        let mut two = bytes_field(17, &file[4..]);
        two.extend(bytes_field(17, b""));
        assert!(sheet_layout(&bytes_field(3, &two)).is_none());
        // A segment_count that disagrees with coords is refused.
        let mut wrong = fields.clone();
        wrong.count = 2;
        assert!(!scan_tail(&file, &mut wrong, coords_at + coords_len));
    }

    /// Colours decode sequentially and report the boundary the next slice starts on.
    #[test]
    fn colors_decode_and_report_their_end() {
        let raw = [127u8, 0, 100, 127, 1, 2, 3, 4, 9, 9];
        let (c, used) = colors_from(&raw, 2).unwrap();
        assert_eq!(c, [0x7f64_007f, 0x0403_0201]);
        assert_eq!(used, 8);
    }
}
