pub use viewer_core::cloud_query::*;

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::fetch::fetch_range as range;
    use crate::app::loader::post;
    use crate::app::stream::CloudFields;
    use std::cell::Cell;
    use std::ops::Range;
    use std::rc::Rc;

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
