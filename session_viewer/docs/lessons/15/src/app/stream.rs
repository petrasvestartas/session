/// Byte positions of a cloud's arrays in its file.
#[derive(Clone, Debug)]
pub struct CloudFields {
    pub end: u64,                 // end of the cloud message
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,          // their length
    pub count: u32,               // points in the cloud
    pub ids_at: u64,              // start of the original ids, 0 = none
    pub ids_len: u64,             // their length
    pub revision: Option<String>, // file ETag every read must match
}

/// A cloud's octree node table.
#[derive(Clone, Default)]
pub struct CloudLod {
    pub min: Vec<f64>,     // cube corner, three per node
    pub size: Vec<f64>,    // cube size per node
    pub spacing: Vec<f64>, // point spacing per node
    pub level: Vec<i32>,   // depth per node
    pub first: Vec<i32>,   // first point per node
    pub count: Vec<i32>,   // points per node
    pub children: Vec<i32>, // eight child indices per node, -1 = none
}

impl CloudLod {
    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.size.len()
    }

    /// Fill one array from a packed field; false when unknown.
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

    /// True when every node and child index is sound.
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

    /// True when one node's values are sound.
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

    /// True when a node's children are one level down and unclaimed.
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

/// The varint at `i` and its byte length.
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

/// Byte length of a scalar field of wire type `wire`.
fn skip_scalar(b: &[u8], i: usize, wire: u32) -> Option<usize> {
    match wire {
        0 => Some(varint(b, i)?.1),
        1 => Some(8),
        5 => Some(4),
        _ => None,
    }
}

/// A skipped field longer than this is geometry, not a name.
const NAME_BYTES: u64 = 256;

/// Start and length of a single cloud's `coords`; None otherwise.
pub fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
    let (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}

/// Start, length of `coords` and end of the cloud message.
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

/// Enter field `want` of the message at `at`; returns its end.
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

/// True when the value fits an f32.
fn finite_float(value: f64) -> bool {
    value.is_finite() && (value as f32).is_finite()
}

/// `count` xyz triples as f32; None when short or not finite.
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

/// True when a range is small enough to read.
#[cfg(any(target_arch = "wasm32", test))]
fn bounded_range(at: u64, length: u64) -> bool {
    length <= 64 * 1024 * 1024 && at.checked_add(length).is_some()
}

/// Checked body bounds, shared by metadata, position and color ranges.
#[cfg(any(target_arch = "wasm32", test))]
/// `at + length` when it stays within `end`.
fn body_end(at: u64, length: u64, end: u64) -> Option<u64> {
    let next = at.checked_add(length)?;

    if next <= end { Some(next) } else { None }
}

// --8<-- [start:step-1a]
/// A cached slice of the file's header bytes.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Default)]
struct MetadataWindow {
    at: u64,        // file position of `bytes[0]`
    bytes: Vec<u8>, // the cached bytes
}

#[cfg(any(target_arch = "wasm32", test))]
impl MetadataWindow {
    /// The cached bytes at `at`, if all present.
    fn slice(&self, at: u64, length: u64) -> Option<&[u8]> {
        let start = usize::try_from(at.checked_sub(self.at)?).ok()?;
        let length = usize::try_from(length).ok()?;
        self.bytes.get(start..start.checked_add(length)?)
    }

    /// How much to read at once: at least `length`, up to 64 KiB.
    fn read_length(at: u64, length: u64, end: u64) -> Option<u64> {
        if !bounded_range(at, length) {
            return None;
        }

        body_end(at, length, end)?;
        Some(length.max(64 * 1024).min(end - at))
    }
}

/// A packed `int32` (varint) array in full.
// --8<-- [end:step-1a]
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

/// `count` colours from packed varints, and where they ended.
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

// --8<-- [start:step-1b]
/// The reads: find the arrays, then fetch slices by range.
// --8<-- [end:step-1b]
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::fetch::{GetOpts, fetch_range, get};

    const POINT_BYTES: u64 = 24; // three doubles

    const MAX_TABLE_BYTES: u64 = 128 * 1024 * 1024; // largest node table read

    // --8<-- [start:step-1c]
    impl MetadataWindow {
        /// The bytes at `at`, reading more when not cached.
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

    /// Read one range of the file.
// --8<-- [end:step-1c]
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

        Some(fetch_range(url, at, length, revision).await.ok()?.0)
    }

    /// Find where a cloud's arrays are in the file.
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

    /// Read the cloud's node table.
    pub async fn cloud_lod(url: &str, fields: &mut CloudFields) -> Option<CloudLod> {
        let mut at = body_end(fields.colors_at, fields.colors_len, fields.end)?;
        let mut lod = CloudLod::default();
        let mut seen = [false; 7];
        let mut table_bytes = 0u64;
        let mut ids = None;
        // --8<-- [start:step-1d]
        let mut window = MetadataWindow::default();

        while at < fields.end {
            let header = window
                .read(url, at, 64.min(fields.end - at), fields)
                .await?;
            let (tag, used) = varint(header, 0)?;
            // --8<-- [end:step-1d]
            let (field, wire) = (usize::try_from(tag >> 3).ok()?, (tag & 7) as u32);

            if field == 0 {
                return None;
            }

            if wire != 2 {
                if (8..=15).contains(&field) {
                    return None;
                }

                // --8<-- [start:step-1e]
                let skip = skip_scalar(header, used, wire)?;
                at = body_end(at, (used + skip) as u64, fields.end)?;
                continue;
            }

            let (length, extra) = varint(header, used)?;
            // --8<-- [end:step-1e]
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

                // --8<-- [start:step-1f]
                let raw = window.read(url, body, length, fields).await?;

                if !lod.set_field(field, raw) {
                // --8<-- [end:step-1f]
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

#[cfg(test)]
mod tests {
    use super::*;

    // --8<-- [start:step-1g]
    /// The window serves nearby reads from cache.
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

    /// A two-node table.
// --8<-- [end:step-1g]
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

    /// Bad node tables are refused.
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

    /// Short or non-finite arrays are refused.
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

    /// Ranges never overflow or leave the message.
    #[test]
    fn source_offsets_do_not_wrap_or_cross_the_enclosing_message() {
        assert!(bounded_range(0, 64 * 1024 * 1024));
        assert!(!bounded_range(0, 64 * 1024 * 1024 + 1));
        assert!(!bounded_range(u64::MAX - 1, 3));
        assert_eq!(body_end(u64::MAX - 1, 3, u64::MAX), None);
        assert_eq!(body_end(50, 51, 100), None);
        assert_eq!(body_end(50, 50, 100), Some(100));
        // a coords field longer than its message
        assert_eq!(walk_to_coords(&[0x1a, 4, 0x42, 2, 0x1a, 24]), None);
    }

    /// Varints of one and two bytes read back.
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

    /// Colours decode and report where they end.
    #[test]
    fn colors_decode_and_report_their_end() {
        let raw = [127u8, 0, 100, 127, 1, 2, 3, 4, 9, 9];
        let (c, used) = colors_from(&raw, 2).unwrap();
        assert_eq!(c, [0x7f64_007f, 0x0403_0201]);
        assert_eq!(used, 8);
    }
}
