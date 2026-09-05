//! Reading a cloud by HTTP Range, without decoding the file whole. Two facts about the wire
//! format make it possible: every hop `Session.3 -> Objects.8 -> PointCloud` is
//! length-delimited, so the headers sit in the first few KB; and `coords` is packed double,
//! so its length prefix gives the exact point count before a byte of payload is read. The
//! byte-level parsing here is pure and tested natively; the fetching half is wasm-only.

/// Where the two packed arrays live in the file, as absolute byte offsets.
#[derive(Clone, Copy, Debug)]
pub struct CloudFields {
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,
    pub count: u32,
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

    /// Fill one of the seven arrays from its packed bytes; other fields are ignored.
    pub fn set_field(&mut self, field: usize, raw: &[u8]) {
        match field {
            8 => self.min = packed_f64(raw),
            9 => self.size = packed_f64(raw),
            10 => self.spacing = packed_f64(raw),
            11 => self.level = packed_i32(raw),
            12 => self.first = packed_i32(raw),
            13 => self.count = packed_i32(raw),
            14 => self.children = packed_i32(raw),
            _ => {}
        }
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
    let mut i = 0usize;
    let mut end = head.len();
    let mut outer_end = end;
    for want in [3u32, 8u32] {
        let mut found = false;
        while i < end {
            let (tag, n) = varint(head, i)?;
            i += n;
            let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
            if wire != 2 {
                return None;
            }
            let (len, n) = varint(head, i)?;
            i += n;
            if field == want {
                outer_end = end;
                end = i + len as usize;
                found = true;
                break;
            }
            if len > NAME_BYTES {
                return None;
            }
            i += len as usize;
        }
        if !found {
            return None;
        }
    }
    if end != outer_end {
        return None;
    }
    while i < end {
        let (tag, n) = varint(head, i)?;
        i += n;
        let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
        if wire != 2 {
            i += skip_scalar(head, i, wire)?;
            continue;
        }
        let (len, n) = varint(head, i)?;
        i += n;
        if field == 3 {
            return Some((i as u64, len));
        }
        if field == 4 {
            return None;
        }
        i += len as usize;
    }
    None
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

/// The fetching half: three small reads locate everything, then slices come down by range.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::fetch::fetch_range;

    /// Bytes per point of packed doubles.
    const POINT_BYTES: u64 = 24;

    /// Locate both packed arrays: one read at the head for `coords`, one at its end for the
    /// `colors` header.
    pub async fn cloud_fields(url: &str) -> Option<CloudFields> {
        let head = fetch_range(url, 0, 8192).await.ok()?;
        let (coords_at, coords_len) = walk_to_coords(&head)?;
        if coords_len == 0 || coords_len % POINT_BYTES != 0 {
            return None;
        }
        let after = coords_at + coords_len;
        let hdr = fetch_range(url, after, 16).await.ok()?;
        let (tag, n) = varint(&hdr, 0)?;
        let mut colors = (after, 0u64);
        if (tag >> 3) == 4 && (tag & 7) == 2 {
            let (len, n2) = varint(&hdr, n)?;
            colors = (after + (n + n2) as u64, len);
        }
        Some(CloudFields { coords_at, coords_len, colors_at: colors.0, colors_len: colors.1, count: (coords_len / POINT_BYTES) as u32 })
    }

    /// Bytes of one header read while walking the fields after the colours.
    const HEADER_READ: u64 = 64;

    /// The node table: walk the fields after the colours one tag/length header at a time,
    /// fetching ONLY the seven LOD arrays (8-14) and skipping normals (5) and point ids (15)
    /// by their length - on a 14 M cloud those two are 380 MB the table never needs.
    /// `None` = no octree.
    pub async fn cloud_lod(url: &str, f: &CloudFields) -> Option<CloudLod> {
        let mut at = f.colors_at + f.colors_len;
        let mut lod = CloudLod::default();
        let mut found = false;
        loop {
            // Past the end the store answers 416: the message is over.
            let Ok(head) = fetch_range(url, at, HEADER_READ).await else { break };
            if head.is_empty() {
                break;
            }
            let Some((tag, n)) = varint(&head, 0) else { break };
            let (field, wire) = ((tag >> 3) as usize, (tag & 7) as u32);
            if wire != 2 {
                let Some(skip) = skip_scalar(&head, n, wire) else { break };
                at += (n + skip) as u64;
                continue;
            }
            let Some((len, n2)) = varint(&head, n) else { break };
            let body_at = at + (n + n2) as u64;
            if (8..=14).contains(&field) {
                let raw = fetch_range(url, body_at, len).await.ok()?;
                lod.set_field(field, &raw);
                found = true;
            }
            if field >= 15 {
                break;
            }
            at = body_at + len;
        }
        if !found || lod.is_empty() { None } else { Some(lod) }
    }

    /// Points `[from, to)` of the coords run as f32 triples.
    pub async fn fetch_positions(url: &str, f: &CloudFields, from: u32, to: u32) -> Option<Vec<f32>> {
        let raw = fetch_range(url, f.coords_at + from as u64 * POINT_BYTES, (to - from) as u64 * POINT_BYTES).await.ok()?;
        Some(positions_from(&raw))
    }

    /// `count` colours starting at byte `at` of the colour run; returns them and where the
    /// run continues. 8 bytes per point is generous for four 0-255 varints.
    pub async fn fetch_colors(url: &str, f: &CloudFields, at: u64, count: u32) -> Option<(Vec<u32>, u64)> {
        let end = f.colors_at + f.colors_len;
        let want = (count as u64 * 8).min(end.saturating_sub(at));
        if want == 0 {
            return None;
        }
        let raw = fetch_range(url, at, want).await.ok()?;
        let (colors, used) = colors_from(&raw, count)?;
        Some((colors, at + used as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A varint round trip over the boundaries that matter.
    #[test]
    fn varint_reads_one_and_two_byte_values() {
        assert_eq!(varint(&[0x05], 0), Some((5, 1)));
        assert_eq!(varint(&[0xac, 0x02], 0), Some((300, 2)));
        assert_eq!(varint(&[0x80], 0), None);
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
