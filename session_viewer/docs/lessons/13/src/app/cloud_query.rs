// --8<-- [start:step-3a]
use super::stream::{CloudFields, CloudLod};
use session_rust::Xform;
use std::cell::Cell;
use std::ops::Range;
use std::rc::Rc;

/// Points read per page.
pub const PAGE_POINTS: u32 = 65_536;

/// A point's original id from its four stored bytes.
pub fn original_id(raw: &[u8]) -> Result<u32, String> {
    let Ok(bytes): Result<[u8; 4], _> = raw.try_into() else {
        return Err("Original point ID range must contain exactly four bytes".to_string());
    };
    Ok(u32::from_le_bytes(bytes))
}

/// The click and the camera it was made with.
#[derive(Clone)]
pub struct QueryView {
    pub matrix: Xform,  // cloud space to clip space
    pub size: [f64; 2], // viewport in pixels
    pub at: [u32; 2],   // click pixel
    pub radius: f64,    // pick radius in pixels
}

impl QueryView {
    /// A point in clip space, before the divide.
    fn clip(&self, point: [f64; 3]) -> [f64; 4] {
        let mut clip = [0.0; 4];

        for (row, value) in clip.iter_mut().enumerate() {
            *value = self.matrix.m[row] * point[0]
                + self.matrix.m[4 + row] * point[1]
                + self.matrix.m[8 + row] * point[2]
                + self.matrix.m[12 + row];
        }

        clip
    }

    /// A point as pixel x, y and depth; None when off screen.
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
    // --8<-- [end:step-3a]
// --8<-- [start:step-3b]

    /// True when the cube may reach the click window.
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

// --8<-- [end:step-3b]
// --8<-- [start:step-3c]
/// True when a clip point is not finite.
fn invalid_clip(point: &[f64; 4]) -> bool {
    !point.iter().copied().all(f64::is_finite)
}

/// True when a clip point is behind the eye.
fn behind_eye(point: &[f64; 4]) -> bool {
    point[3] <= 0.0
}

/// True when a clip point is past the far plane.
fn beyond_far(point: &[f64; 4]) -> bool {
    point[2] < 0.0
}

/// True when a clip point is before the near plane.
fn beyond_near(point: &[f64; 4]) -> bool {
    point[2] > point[3]
}

/// Corner `corner` of a cube.
fn cube_corner(min: [f64; 3], size: f64, corner: usize) -> [f64; 3] {
    [
        min[0] + if corner & 1 == 0 { 0.0 } else { size },
        min[1] + if corner & 2 == 0 { 0.0 } else { size },
        min[2] + if corner & 4 == 0 { 0.0 } else { size },
    ]
}

/// Grow a pixel box to include a point.
fn grow_pixel_bounds(lo: &mut [f64; 2], hi: &mut [f64; 2], point: [f64; 2]) {
    for axis in 0..2 {
        lo[axis] = lo[axis].min(point[axis]);
        hi[axis] = hi[axis].max(point[axis]);
    }
}

// --8<-- [end:step-3c]
// --8<-- [start:step-3d]
/// One point near the click.
#[derive(Clone, Debug)]
pub struct Candidate {
    pub local: u32,         // index in the source file
    pub position: [f64; 3], // exact source position
}

/// Order ranges by start.
fn source_range_order(left: &Range<u32>, right: &Range<u32>) -> std::cmp::Ordering {
    left.start.cmp(&right.start)
}

/// Merge overlapping ranges.
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

/// Every row, as page-sized ranges.
fn all_rows(total: u32) -> Vec<Range<u32>> {
    std::iter::once(0..total).collect()
}

/// The point ranges of every octree node near the click.
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

// --8<-- [end:step-3d]
// --8<-- [start:step-3e]
/// One pick in a streamed cloud, page by page.
pub struct Query {
    pub id: u64,                    // pick number
    pub parent: u32,                // the cloud's object row
    pub url: String,                // the cloud file
    pub fields: CloudFields,        // where the arrays are in the file
    pub view: QueryView,            // the click
    pub cancelled: Rc<Cell<bool>>,  // set when a newer pick replaces this
    pub revision: Option<String>,   // file ETag every read must match
    pub candidates: Vec<Candidate>, // points near the click so far
    pub best: Option<u32>,          // winning candidate index so far
    pub checked: u32,               // points examined so far
    pub total: u32,                 // points to examine
    ranges: Vec<Range<u32>>,        // pages still to read
    next: usize,                    // next range
    pub awaiting_gpu: bool,         // a page is on the GPU for ranking
}

impl Query {
    /// A new pick over the cloud's eligible ranges.
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

    /// The next page to read, None when done.
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
// --8<-- [end:step-3e]
// --8<-- [start:step-3f]

impl Drop for Query {
    /// Cancel the pick.
    fn drop(&mut self) {
        self.cancelled.set(true);
    }
}

/// One page's answer.
pub struct Batch {
    pub query: u64, // which pick
    pub count: u32, // points examined
    pub result: Result<(Vec<Candidate>, Option<String>), String>, // candidates and the ETag
}

/// The final answer of a pick.
pub struct Resolved {
    pub query: u64,                                // which pick
    pub result: Result<(u32, [f64; 3]), String>, // original id and position
}

// --8<-- [end:step-3f]
// --8<-- [start:step-3g]
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::loader::post;

    use crate::app::fetch::fetch_range as range;

    /// What one page read needs from the pick.
    struct SourceRequest {
        query: u64,                // which pick
        url: String,               // the cloud file
        fields: CloudFields,       // array positions
        cancelled: Rc<Cell<bool>>, // the pick's cancel flag
        revision: Option<String>,  // ETag to match
    }

    /// Copy what a page read needs.
    fn source_request(query: &Query) -> SourceRequest {
        SourceRequest {
            query: query.id,
            url: query.url.clone(),
            fields: query.fields.clone(),
            cancelled: query.cancelled.clone(),
            revision: query.revision.clone(),
        }
    }

    /// Start reading one page; the answer arrives as a message.
    pub fn fetch_page(query: &Query, page: Range<u32>) {
        wasm_bindgen_futures::spawn_local(post_page(
            source_request(query),
            query.view.clone(),
            page,
        ));
    }

    /// Read the page and post the answer unless cancelled.
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

    /// Read one page and keep the points near the click.
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

    /// A point from its 24 stored bytes.
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

    /// Points of one cloud chunk whose screen position is near the click.
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
                        // --8<-- [end:step-3g]
                    // --8<-- [start:step-3h]
                    });
                }
            }
        }

        Ok(candidates)
    }

    /// Read the winner's id and position.
    pub fn resolve_id(query: &Query, local: u32) {
        wasm_bindgen_futures::spawn_local(post_source(source_request(query), local));
    }

    /// Read and post the final answer unless cancelled.
    async fn post_source(source: SourceRequest, local: u32) {
        let result = read_source(&source, local).await;

        if !source.cancelled.get() {
            post(crate::Msg::CloudQueryResolved(Resolved {
                query: source.query,
                result,
            }));
        }
    }

    /// Read one point's position and id.
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

    /// A 100x100 view clicked at its centre.
    fn view() -> QueryView {
        QueryView {
            matrix: Xform::identity(),
            size: [100.0; 2],
            at: [50; 2],
            radius: 6.0,
        }
    }

    /// An id keeps all 32 bits; a short read is refused.
    #[test]
    fn original_fixed32_ids_preserve_all_bits_and_reject_partial_ranges() {
        for value in [0, 127, 128, 65535, u32::MAX, 42] {
            assert_eq!(original_id(&value.to_le_bytes()).unwrap(), value);
        }

        assert!(original_id(&[1, 2, 3]).is_err());
        assert!(original_id(&[1, 2, 3, 4, 5]).is_err());
    }

    /// Nodes past six million points are still read; points in no node are scanned.
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

    /// Cubes and points project to the same pixel centre.
    #[test]
    fn cube_eligibility_uses_the_same_pixel_center_as_candidate_projection() {
        let v = view();
        // 56.4 is within 6 of the pixel centre 50.5
        assert!(v.intersects([0.128, 0.0, 0.5], 0.0));
        assert!(!v.intersects([0.132, 0.0, 0.5], 0.0));
    }

    /// A cube crossing the near plane stays eligible.
    #[test]
    fn near_plane_crossing_cube_is_conservative() {
        let mut v = view();
        v.matrix.m[3] = 1.0;
        v.matrix.m[15] = 0.0;
        assert!(v.intersects([-1.0, 0.0, 0.0], 2.0));
    }

    /// Pages cover every row; a cancel stops them.
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

    /// Dropping a pick sets its cancel flag.
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
// --8<-- [end:step-3h]
