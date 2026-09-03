//! Streaming a point cloud: HTTP Range in, GPU rows out, nothing large in between - the
//! whole-file path peaks at bytes + proto + kernel object + rows, this one never holds more
//! than a slice. Two wire facts make it possible (checked on a real scan): every hop
//! Session.3 -> Objects.8 -> PointCloud.3/.4 is length-delimited, and `coords` is packed
//! DOUBLE, so its length prefix gives the exact point count before a byte of payload is read.
//! Colours are packed VARINTS, so their slices carry a split varint's tail across (`ColorRun`).

use super::fetch::fetch_range;

/// One Range read: 8 MiB. The coords loop rounds it down to whole points; the colour loop
/// takes it as is, since `ColorRun` carries a split varint over the boundary.
pub const SLICE_BYTES: u64 = 8 * 1024 * 1024;

/// Where the two packed arrays live in the file, as absolute byte offsets.
pub struct CloudFields {
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,
    pub count: u32,
}

/// One protobuf varint. Returns the value and how many bytes it ate.
fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;
    loop {
        let byte = *b.get(i)?;
        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;
        if byte & 0x80 == 0 { return Some((v, i - start)) }
        shift += 7;
        if shift > 63 { return None }
    }
}

/// Walk `head` (the first few KB of the file) down Session.3 -> Objects.8 -> PointCloud, and
/// report where `coords` starts. Descends into exactly the three fields it cares about and
/// skips every other one by its length - no allocation, no decoding.
///
/// Returns `None` for anything that is not a single-cloud file, which is the signal to fall
/// back to the whole-file prost path.
fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
    let mut i = 0usize;
    let mut end = head.len();
    for want in [3u32, 8u32] {
        let mut found = false;
        while i < end {
            let (tag, n) = varint(head, i)?;
            i += n;
            let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
            if wire != 2 { return None } // every hop we care about is length-delimited
            let (len, n) = varint(head, i)?;
            i += n;
            if field == want { end = i + len as usize; found = true; break }
            i += len as usize; // skip this sub-message whole
        }
        if !found { return None }
    }
    // inside PointCloud now: find field 3 (coords)
    while i < end {
        let (tag, n) = varint(head, i)?;
        i += n;
        let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
        if wire != 2 {
            // point_size is a fixed64, everything else we skip by wire type
            i += match wire { 0 => varint(head, i)?.1, 1 => 8, 5 => 4, _ => return None };
            continue;
        }
        let (len, n) = varint(head, i)?;
        i += n;
        if field == 3 { return Some((i as u64, len)) }
        if field == 4 { return None } // colours before coords - not a layout we can size from
        i += len as usize;
    }
    None
}

/// Convert one already-fetched coords slice to f32 triples.
pub fn positions_from(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 8);
    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()) as f32);
    }
    out
}

/// Locate both packed arrays with two small reads: one at the head for `coords`, then one at
/// the end of the coords payload, where the `colors` header must be.
pub async fn cloud_fields(url: &str) -> Option<CloudFields> {
    let head = fetch_range(url, 0, 8192).await.ok()?;
    let (coords_at, coords_len) = walk_to_coords(&head)?;
    if coords_len == 0 || coords_len % 24 != 0 { return None }

    let hdr = fetch_range(url, coords_at + coords_len, 16).await.ok()?;
    let (tag, n) = varint(&hdr, 0)?;
    if (tag >> 3) != 4 || (tag & 7) != 2 { return None } // expected the colours field next
    let (colors_len, n2) = varint(&hdr, n)?;
    Some(CloudFields {
        coords_at,
        coords_len,
        colors_at: coords_at + coords_len + (n + n2) as u64,
        colors_len,
        count: (coords_len / 24) as u32,
    })
}

/// The `colors` run decoded slice by slice. Packed uint32 is VARINT on the wire - not
/// memcpy-able the way `coords` is - so a slice boundary can fall inside a point's four
/// varints: the undecoded tail of every slice is carried into the next. Fetching the run
/// whole cost 148 MB of transient on a 14M-point scan; a slice costs 8 MiB.
pub struct ColorRun {
    carry: Vec<u8>,
    left: u32,
}

impl ColorRun {
    /// `count` points still to decode, nothing carried.
    pub fn new(count: u32) -> Self {
        Self { carry: Vec::new(), left: count }
    }

    /// Every WHOLE point in the carried tail + `raw`, packed to RGBA8; what is left over (at
    /// most one point's bytes) waits for the next slice. Empty once `count` points are out.
    pub fn decode(&mut self, raw: &[u8]) -> Vec<u32> {
        self.carry.extend_from_slice(raw);
        let buf = std::mem::take(&mut self.carry);
        let mut out = Vec::with_capacity((buf.len() / 4).min(self.left as usize));
        let mut i = 0usize;
        while self.left > 0 {
            let Some((rgba, n)) = point_rgba(&buf, i) else { break };
            out.push(rgba);
            i += n;
            self.left -= 1;
        }
        self.carry = buf[i..].to_vec();
        out
    }
}

/// One point's four varints at `i`, packed RGBA8, and the bytes they took; `None` when the
/// buffer ends inside them - the caller carries the tail.
fn point_rgba(b: &[u8], mut i: usize) -> Option<(u32, usize)> {
    let start = i;
    let mut rgba = [255u8; 4];
    for k in 0..4 {
        let (v, n) = varint(b, i)?;
        i += n;
        rgba[k] = (v & 255) as u8;
    }
    Some((u32::from_le_bytes(rgba), i - start))
}
