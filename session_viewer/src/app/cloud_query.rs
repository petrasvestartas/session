//! Exhaustive source-cloud selection with bounded range reads. The display retains its LOD
//! residency budget; a click examines every intersecting source node, including nonresident
//! nodes. Only one page of candidates exists at a time and visibility stays in the GPU picker.

use super::stream::{CloudFields, CloudLod};
use crate::math::Mat4;
use std::cell::Cell;
use std::ops::Range;
use std::rc::Rc;

/// Coordinates fetched per page (1.5 MiB of wire doubles), independent of cloud size.
pub const PAGE_POINTS: u32 = 65_536;
/// Original point IDs are packed fixed32 (schema field 15), deliberately range-addressable.
pub fn original_id(raw: &[u8]) -> Result<u32, String> {
    let Ok(bytes): Result<[u8; 4], _> = raw.try_into() else {
        return Err("Original point ID range must contain exactly four bytes".to_string());
    };
    Ok(u32::from_le_bytes(bytes))
}

/// The click's immutable projection in source coordinates and physical pixels.
#[derive(Clone)]
pub struct QueryView {
    pub matrix: Mat4,
    pub size: [f64; 2],
    pub at: [u32; 2],
    pub radius: f64,
}
impl QueryView {
    /// Homogeneous source-to-clip projection; division waits until the eye plane is checked.
    fn clip(&self, point: [f64; 3]) -> [f64; 4] {
        let mut clip = [0.0; 4];
        for (row, value) in clip.iter_mut().enumerate() {
            *value = self.matrix[row] * point[0]
                + self.matrix[4 + row] * point[1]
                + self.matrix[8 + row] * point[2]
                + self.matrix[12 + row];
        }
        clip
    }
    /// Pixel center and reversed depth; candidates behind the eye or clip planes are absent.
    pub fn project(&self, point: [f64; 3]) -> Option<[f64; 3]> {
        let p = self.clip(point);
        if invalid_clip(&p) || p[3] <= 0.0 || p[2] < 0.0 || p[2] > p[3] {
            return None;
        }
        Some([
            (p[0] / p[3] + 1.0) * self.size[0] * 0.5,
            (1.0 - p[1] / p[3]) * self.size[1] * 0.5,
            p[2] / p[3],
        ])
    }
    /// A conservative projected cube test. A cube crossing the eye plane must be examined.
    fn intersects(&self, min: [f64; 3], size: f64) -> bool {
        if !size.is_finite() || size < 0.0 || !min.into_iter().all(f64::is_finite) {
            return true;
        }
        let mut corners = [[0.0; 4]; 8];
        for (corner, clip) in corners.iter_mut().enumerate() {
            *clip = self.clip(cube_corner(min, size, corner));
        }
        if corners.iter().any(invalid_clip) {
            return true;
        }
        if corners.iter().all(behind_eye) {
            return false;
        }
        if corners.iter().any(behind_eye) {
            return true;
        }
        if corners.iter().all(beyond_far) || corners.iter().all(beyond_near) {
            return false;
        }
        let mut lo = [f64::INFINITY; 2];
        let mut hi = [f64::NEG_INFINITY; 2];
        for p in corners {
            let xy = [
                (p[0] / p[3] + 1.0) * self.size[0] * 0.5,
                (1.0 - p[1] / p[3]) * self.size[1] * 0.5,
            ];
            grow_pixel_bounds(&mut lo, &mut hi, xy);
        }
        for axis in 0..2 {
            let center = self.at[axis] as f64 + 0.5;
            if lo[axis] > center + self.radius || hi[axis] < center - self.radius {
                return false;
            }
        }
        true
    }
}

/// Nonfinite homogeneous coordinates cannot safely exclude a source cube.
fn invalid_clip(point: &[f64; 4]) -> bool {
    !point.iter().copied().all(f64::is_finite)
}
/// The perspective eye plane separates forward and backward homogeneous coordinates.
fn behind_eye(point: &[f64; 4]) -> bool {
    point[3] <= 0.0
}
/// Reversed-Z far clipping remains at clip z=0.
fn beyond_far(point: &[f64; 4]) -> bool {
    point[2] < 0.0
}
/// Reversed-Z near clipping remains at clip z=w.
fn beyond_near(point: &[f64; 4]) -> bool {
    point[2] > point[3]
}
/// Enumerate a source cube corner without constructing a separate geometry object.
fn cube_corner(min: [f64; 3], size: f64, corner: usize) -> [f64; 3] {
    [
        min[0] + if corner & 1 == 0 { 0.0 } else { size },
        min[1] + if corner & 2 == 0 { 0.0 } else { size },
        min[2] + if corner & 4 == 0 { 0.0 } else { size },
    ]
}
/// Expand both pixel axes; keeping this loop named avoids nesting it in the corner walk.
fn grow_pixel_bounds(lo: &mut [f64; 2], hi: &mut [f64; 2], point: [f64; 2]) {
    for axis in 0..2 {
        lo[axis] = lo[axis].min(point[axis]);
        hi[axis] = hi[axis].max(point[axis]);
    }
}

/// One exact source point; no sampled/local display ID is substituted for `local`.
#[derive(Clone, Debug)]
pub struct Candidate {
    pub local: u32,
    pub position: [f64; 3],
}

/// Source pages advance in file order, independent of octree traversal order.
fn source_range_order(left: &Range<u32>, right: &Range<u32>) -> std::cmp::Ordering {
    left.start.cmp(&right.start)
}

/// Merge overlapping source ranges, preserving every row exactly once.
fn merge(mut ranges: Vec<Range<u32>>) -> Vec<Range<u32>> {
    ranges.sort_unstable_by(source_range_order);
    let mut out: Vec<Range<u32>> = Vec::new();
    for range in ranges {
        if range.is_empty() {
            continue;
        }
        if let Some(last) = out.last_mut()
            && range.start <= last.end
        {
            last.end = last.end.max(range.end);
        } else {
            out.push(range);
        }
    }
    out
}

/// A fallback scan still streams one bounded page at a time instead of allocating all rows.
fn all_rows(total: u32) -> Vec<Range<u32>> {
    std::iter::once(0..total).collect()
}

/// Select all intersecting source nodes, with no resident-prefix or LOD-level cutoff.
/// Missing/malformed coverage falls back to a bounded scan of the complete source array.
pub fn eligible_ranges(lod: &CloudLod, total: u32, view: &QueryView) -> Vec<Range<u32>> {
    let n = lod.len();
    if n == 0 || lod.first.len() < n || lod.count.len() < n || lod.min.len() < n * 3 {
        return all_rows(total);
    }
    let mut coverage = Vec::new();
    let mut eligible = Vec::new();
    for node in 0..n {
        let (Ok(first), Ok(count)) = (
            u32::try_from(lod.first[node]),
            u32::try_from(lod.count[node]),
        ) else {
            return all_rows(total);
        };
        let Some(end) = first.checked_add(count) else {
            return all_rows(total);
        };
        if end > total {
            return all_rows(total);
        }
        coverage.push(first..end);
        let at = node * 3;
        let min = [lod.min[at], lod.min[at + 1], lod.min[at + 2]];
        if view.intersects(min, lod.size[node]) {
            eligible.push(first..end);
        }
    }
    if merge(coverage) != all_rows(total) {
        return all_rows(total);
    }
    merge(eligible)
}

/// Sequential query state. Superseding input cancels the token even while HTTP awaits.
pub struct Query {
    pub id: u64,
    pub parent: u32,
    pub url: String,
    pub fields: CloudFields,
    pub view: QueryView,
    pub cancelled: Rc<Cell<bool>>,
    pub revision: Option<String>,
    pub candidates: Vec<Candidate>,
    pub best: Option<u32>,
    pub checked: u32,
    pub total: u32,
    ranges: Vec<Range<u32>>,
    next: usize,
    pub awaiting_gpu: bool,
}
impl Query {
    /// Freeze the source descriptor and all eligible ranges for this camera generation.
    pub fn new(id: u64, cloud: &super::scene::StreamedCloud, view: QueryView) -> Self {
        let ranges = eligible_ranges(&cloud.lod, cloud.total, &view);
        let mut total = 0;
        for range in &ranges {
            total += range.end - range.start;
        }
        Self {
            id,
            parent: cloud.row,
            url: cloud.url.clone(),
            fields: cloud.fields.clone(),
            view,
            cancelled: Rc::new(Cell::new(false)),
            revision: cloud.fields.revision.clone(),
            candidates: Vec::new(),
            best: None,
            checked: 0,
            total,
            ranges,
            next: 0,
            awaiting_gpu: false,
        }
    }
    /// Advance one bounded page; only called after the previous GPU answer was collected.
    pub fn next_page(&mut self) -> Option<Range<u32>> {
        if self.cancelled.get() {
            return None;
        }
        let range = self.ranges.get_mut(self.next)?;
        let page = range.start..range.end.min(range.start.saturating_add(PAGE_POINTS));
        range.start = page.end;
        if range.start >= range.end {
            self.next += 1;
        }
        Some(page)
    }
}
impl Drop for Query {
    /// Retire callbacks when the query completes, fails, or is replaced by new input.
    fn drop(&mut self) {
        self.cancelled.set(true);
    }
}

/// One bounded range callback, including failures, scoped to its query generation.
pub struct Batch {
    pub query: u64,
    pub count: u32,
    pub result: Result<(Vec<Candidate>, Option<String>), String>,
}
/// Final original identity and exact source position; never a provisional page winner.
pub struct Resolved {
    pub query: u64,
    pub result: Result<(u32, [f64; 3]), String>,
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::{
        fetch::{GetOpts, get},
        loader::post,
    };

    /// Validate the exact ranged body and any exposed revision; never consume a whole-file 200.
    async fn range(
        url: &str,
        at: u64,
        len: u64,
        revision: &Option<String>,
    ) -> Result<(Vec<u8>, Option<String>), String> {
        let reply = get(
            url,
            &GetOpts {
                range: Some((at, len)),
                revalidate: true,
                ..Default::default()
            },
        )
        .await?;
        if reply.status != 206 || reply.bytes.len() as u64 != len {
            return Err(format!(
                "Source range failed (HTTP {}, {} of {len} bytes)",
                reply.status,
                reply.bytes.len()
            ));
        }
        if revision.is_some() && revision != &reply.etag {
            return Err("Source changed during point selection; reload the cloud".to_string());
        }
        Ok((reply.bytes, reply.etag))
    }

    /// Owned request data outlives the event-loop borrow, with one shared cancellation token.
    struct SourceRequest {
        query: u64,
        url: String,
        fields: CloudFields,
        cancelled: Rc<Cell<bool>>,
        revision: Option<String>,
    }

    /// Capture only the source identity and lifetime needed by one asynchronous range read.
    fn source_request(query: &Query) -> SourceRequest {
        SourceRequest {
            query: query.id,
            url: query.url.clone(),
            fields: query.fields.clone(),
            cancelled: query.cancelled.clone(),
            revision: query.revision.clone(),
        }
    }

    /// Schedule a bounded page; the named task posts its completion to the event loop.
    pub fn fetch_page(query: &Query, page: Range<u32>) {
        wasm_bindgen_futures::spawn_local(post_page(
            source_request(query),
            query.view.clone(),
            page,
        ));
    }

    /// Complete or fail exactly one page, unless superseding input retired its generation.
    async fn post_page(source: SourceRequest, view: QueryView, page: Range<u32>) {
        let count = page.end - page.start;
        let result = read_page(&source, &view, page).await;
        if !source.cancelled.get() {
            post(crate::Msg::CloudQueryBatch(Batch {
                query: source.query,
                count,
                result,
            }));
        }
    }

    /// Fetch source doubles, then retain only points whose marker can reach the click window.
    async fn read_page(
        source: &SourceRequest,
        view: &QueryView,
        page: Range<u32>,
    ) -> Result<(Vec<Candidate>, Option<String>), String> {
        let at = source.fields.coords_at + u64::from(page.start) * 24;
        let length = u64::from(page.end - page.start) * 24;
        let (raw, revision) = range(&source.url, at, length, &source.revision).await?;
        if source.cancelled.get() {
            return Err("Point query cancelled".to_string());
        }
        Ok((page_candidates(&raw, page.start, view)?, revision))
    }

    /// An exact finite source triple in world-file units, representable by the GPU row.
    fn source_position(raw: &[u8]) -> Result<[f64; 3], String> {
        if raw.len() != 24 {
            return Err("Source coordinate range must contain exactly 24 bytes".to_string());
        }
        let mut position = [0.0; 3];
        for (axis, value) in position.iter_mut().enumerate() {
            let bytes: [u8; 8] = raw[axis * 8..axis * 8 + 8]
                .try_into()
                .expect("exact triple checked above");
            *value = f64::from_le_bytes(bytes);
            if !value.is_finite() || !(*value as f32).is_finite() {
                return Err(
                    "Source coordinates are nonfinite or outside the GPU coordinate range"
                        .to_string(),
                );
            }
        }
        Ok(position)
    }

    /// One bounded page's candidates; visibility and cross-page ranking remain on the GPU.
    fn page_candidates(raw: &[u8], first: u32, view: &QueryView) -> Result<Vec<Candidate>, String> {
        let mut candidates = Vec::new();
        for (offset, xyz) in raw.chunks_exact(24).enumerate() {
            let position = source_position(xyz)?;
            if let Some(point) = view.project(position) {
                let distance = (point[0] - view.at[0] as f64 - 0.5).powi(2)
                    + (point[1] - view.at[1] as f64 - 0.5).powi(2);
                if distance <= view.radius.powi(2) {
                    candidates.push(Candidate {
                        local: first + offset as u32,
                        position,
                    });
                }
            }
        }
        Ok(candidates)
    }

    /// Resolve only the final accumulated winner, with two tiny revision-checked ranges.
    pub fn resolve_id(query: &Query, local: u32) {
        wasm_bindgen_futures::spawn_local(post_source(source_request(query), local));
    }

    /// Post the original identity and exact position only while this query remains current.
    async fn post_source(source: SourceRequest, local: u32) {
        let result = read_source(&source, local).await;
        if !source.cancelled.get() {
            post(crate::Msg::CloudQueryResolved(Resolved {
                query: source.query,
                result,
            }));
        }
    }

    /// Re-read the chosen source position and its packed fixed32 ID under one source revision.
    async fn read_source(source: &SourceRequest, local: u32) -> Result<(u32, [f64; 3]), String> {
        let fields = &source.fields;
        let (coords, _) = range(
            &source.url,
            fields.coords_at + u64::from(local) * 24,
            24,
            &source.revision,
        )
        .await?;
        if source.cancelled.get() {
            return Err("Point query cancelled".to_string());
        }
        let position = source_position(&coords)?;
        let original = if fields.ids_len == 0 {
            local
        } else {
            if fields.ids_len != u64::from(fields.count) * 4 {
                return Err("Original point ID count differs from source coordinates".to_string());
            }
            let (raw, _) = range(
                &source.url,
                fields.ids_at + u64::from(local) * 4,
                4,
                &source.revision,
            )
            .await?;
            original_id(&raw)?
        };
        Ok((original, position))
    }
}
#[cfg(target_arch = "wasm32")]
pub use web::{fetch_page, resolve_id};

#[cfg(test)]
mod tests {
    use super::*;
    fn view() -> QueryView {
        QueryView {
            matrix: session_rust::Xform::identity().m,
            size: [100.0; 2],
            at: [50; 2],
            radius: 6.0,
        }
    }
    #[test]
    fn original_fixed32_ids_preserve_all_bits_and_reject_partial_ranges() {
        for value in [0, 127, 128, 65535, u32::MAX, 42] {
            assert_eq!(original_id(&value.to_le_bytes()).unwrap(), value);
        }
        assert!(original_id(&[1, 2, 3]).is_err());
        assert!(original_id(&[1, 2, 3, 4, 5]).is_err());
    }
    #[test]
    fn nodes_after_six_million_remain_eligible_and_uncovered_rows_are_scanned() {
        let lod = CloudLod {
            min: vec![10.0, 10.0, 0.0, -0.01, -0.01, 0.4],
            size: vec![1.0, 0.02],
            first: vec![0, 6_000_000],
            count: vec![6_000_000, 100],
            ..Default::default()
        };
        assert_eq!(
            eligible_ranges(&lod, 6_000_100, &view()),
            std::iter::once(6_000_000..6_000_100).collect::<Vec<_>>()
        );
        assert_eq!(
            eligible_ranges(&lod, 6_000_101, &view()),
            std::iter::once(0..6_000_101).collect::<Vec<_>>()
        );
    }
    #[test]
    fn cube_eligibility_uses_the_same_pixel_center_as_candidate_projection() {
        let v = view();
        // x=56.4 lies inside radius6 around pixel center50.5, beyond integer cursor50+6.
        assert!(v.intersects([0.128, 0.0, 0.5], 0.0));
        assert!(!v.intersects([0.132, 0.0, 0.5], 0.0));
    }

    #[test]
    fn near_plane_crossing_cube_is_conservative() {
        let mut v = view();
        v.matrix[3] = 1.0;
        v.matrix[15] = 0.0;
        assert!(v.intersects([-1.0, 0.0, 0.0], 2.0));
    }
    #[test]
    fn bounded_pages_visit_the_entire_range_and_cancellation_stops_advancement() {
        let mut query = Query {
            id: 1,
            parent: 0,
            url: String::new(),
            fields: CloudFields {
                end: 0,
                coords_at: 0,
                coords_len: 0,
                colors_at: 0,
                colors_len: 0,
                count: 1,
                ids_at: 0,
                ids_len: 0,
                revision: None,
            },
            view: view(),
            cancelled: Rc::new(Cell::new(false)),
            revision: None,
            candidates: Vec::new(),
            best: None,
            checked: 0,
            total: PAGE_POINTS * 2 + 7,
            ranges: std::iter::once(6_000_000..6_000_000 + PAGE_POINTS * 2 + 7).collect(),
            next: 0,
            awaiting_gpu: false,
        };
        assert_eq!(query.next_page(), Some(6_000_000..6_000_000 + PAGE_POINTS));
        assert_eq!(
            query.next_page(),
            Some(6_000_000 + PAGE_POINTS..6_000_000 + PAGE_POINTS * 2)
        );
        assert_eq!(
            query.next_page(),
            Some(6_000_000 + PAGE_POINTS * 2..6_000_000 + PAGE_POINTS * 2 + 7)
        );
        assert_eq!(query.next_page(), None);
        query.next = 0;
        query.ranges = std::iter::once(0..100).collect();
        query.cancelled.set(true);
        assert_eq!(query.next_page(), None);
        assert_eq!(query.ranges[0], 0..100);
    }

    #[test]
    fn cancellation_token_invalidates_inflight_work_on_drop() {
        let token = Rc::new(Cell::new(false));
        let query = Query {
            id: 1,
            parent: 0,
            url: String::new(),
            fields: CloudFields {
                end: 0,
                coords_at: 0,
                coords_len: 0,
                colors_at: 0,
                colors_len: 0,
                count: 1,
                ids_at: 0,
                ids_len: 0,
                revision: None,
            },
            view: view(),
            cancelled: token.clone(),
            revision: None,
            candidates: Vec::new(),
            best: None,
            checked: 0,
            total: 1,
            ranges: std::iter::once(0..1).collect(),
            next: 0,
            awaiting_gpu: false,
        };
        drop(query);
        assert!(token.get());
    }
}
