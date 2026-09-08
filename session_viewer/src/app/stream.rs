//! Reading a cloud by HTTP Range, without decoding the file whole. Two facts about the wire
//! format make it possible: every hop `Session.3 -> Objects.8 -> PointCloud` is
//! length-delimited, so the headers sit in the first few KB; and `coords` is packed double,
//! so its length prefix gives the exact point count before a byte of payload is read. The
//! byte-level parsing here is pure and tested natively; the fetching half is wasm-only.

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
        if field == 4 || length > NAME_BYTES {
            return None;
        }
        at = usize::try_from(next).ok()?;
    }
    None
}

/// Descend one length-delimited envelope, skipping only small names before its target field.
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
            if want == 8 && parent_end != Some(next) {
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
    if raw.len() as u64 != u64::from(count).checked_mul(24)? {
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

    /// Read ahead 64 KiB inside the cloud; larger packed arrays retain the existing cap.
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

    const POINT_BYTES: u64 = 24;
    const MAX_TABLE_BYTES: u64 = 128 * 1024 * 1024;

    impl MetadataWindow {
        /// Reuse adjacent headers/arrays under the same ETag; replace the window on a jump.
        async fn read(
            &mut self,
            url: &str,
            at: u64,
            length: u64,
            fields: &CloudFields,
        ) -> Option<&[u8]> {
            body_end(at, length, fields.end)?;
            if self.slice(at, length).is_none() {
                let read_length = Self::read_length(at, length, fields.end)?;
                self.bytes = source_range(url, at, read_length, &fields.revision).await?;
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
                .read(url, at, 64.min(fields.end - at), fields)
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
                let raw = window.read(url, body, length, fields).await?;
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

    /// Colours decode sequentially and report the boundary the next slice starts on.
    #[test]
    fn colors_decode_and_report_their_end() {
        let raw = [127u8, 0, 100, 127, 1, 2, 3, 4, 9, 9];
        let (c, used) = colors_from(&raw, 2).unwrap();
        assert_eq!(c, [0x7f64_007f, 0x0403_0201]);
        assert_eq!(used, 8);
    }
}
