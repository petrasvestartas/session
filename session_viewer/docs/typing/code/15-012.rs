
// A cloud's first 2 million points go out with the scene; the rest follow in the background, 2 million a slice, up to the page's 6 million.
use super::scene::StreamedInit;
use super::stream::{
    CloudFields, cloud_fields, cloud_lod, fetch_colors, fetch_normals, fetch_positions, plain,
    probe, reset_range_gate,
};
use super::walk::cloud::StreamRows;
use crate::CloudChunk;

/// A cloud read by range: its first `share` points go out now.
async fn start_cloud(cx: &mut ItemCx<'_>, head: &Reply, slot: &Placement) -> Result<(), Step> {
    let remaining = if cx.replacement.is_some() {
        max_points().saturating_sub(*cx.staged_points)
    } else {
        budget_left()
    };
    let share = cx.share.min(remaining.max(STREAM_MIN_PREFIX));
    let Some(init) = stream_prefix(cx.url, slot, share, head).await else {
        return Ok(());
    };

    if stale_load(cx.generation) {
        return Err(Step::Stop);
    }

    if init.resident == 0 {
        *cx.failed = true;
        return Err(Step::Next);
    }

    if cx.replacement.is_some() {
        *cx.staged_points = cx.staged_points.saturating_add(init.resident);
        cx.pending.push(PendingDocument::Streamed(Box::new(init)));
    } else {
        budget_spend(init.resident);
        post(Msg::StreamedCloud(Box::new(init)));
    }

    Err(Step::Next)
}

/// Read a cloud's first `share` points by range; None when it should load whole.
async fn stream_prefix(
    url: &str,
    slot: &Placement,
    share: u32,
    head: &Reply,
) -> Option<StreamedInit> {
    let (name, place, point_px) = (slot.name.as_str(), slot.place.clone(), slot.point_px);
    let mut fields = cloud_fields(url, head).await?;

    if fields.count <= STREAM_PREFIX_POINTS && fields.coords_len < STREAM_MIN_BYTES {
        return None;
    }

    let lod = cloud_lod(url, &mut fields).await?;
    let resident = STREAM_PREFIX_POINTS.min(share).min(fields.count);
    let Some(positions) = fetch_positions(url, &fields, 0, resident).await else {
        log::warn!(
            "'{name}': the prefix range read failed - the cloud stays off screen (a whole decode would take {:.0} MB)",
            fields.coords_len as f64 / 1.048576e6
        );
        return Some(StreamedInit {
            name: name.to_string(),
            url: url.to_string(),
            place,
            rows: StreamRows {
                positions: Vec::new(),
                colors: Vec::new(),
                normals: Vec::new(),
            },
            lod,
            col_at: fields.colors_at,
            fields,
            resident: 0,
            point_px,
            ceiling: max_points(),
        });
    };
    let (colors, col_at) = fetch_colors(url, &fields, fields.colors_at, resident)
        .await
        .unwrap_or((Vec::new(), fields.colors_at));
    let normals = fetch_normals(url, &fields, 0, resident)
        .await
        .unwrap_or_default();
    log::info!(
        "streamed '{name}': {resident} of {} points on screen, {} nodes",
        fields.count,
        lod.len()
    );
    Some(StreamedInit {
        name: name.to_string(),
        url: url.to_string(),
        place,
        rows: StreamRows {
            positions,
            colors,
            normals,
        },
        lod,
        fields,
        resident,
        point_px,
        col_at,
        ceiling: max_points(),
    })
}

/// Where a cloud's streaming continues.
pub struct StreamCursor {
    pub idx: usize,          // the cloud's slot in the scene
    pub url: String,         // the cloud file
    pub fields: CloudFields, // array positions in the file
    pub from: u32,           // next point to read
    pub col_at: u64,         // byte position of its colour
}

/// Keep reading a cloud's slices in the background.
pub fn spawn_stream_rest(cursor: StreamCursor) {
    wasm_bindgen_futures::spawn_local(stream_rest(cursor));
}

/// The slice loop behind `spawn_stream_rest`.
async fn stream_rest(c: StreamCursor) {
    let (url, idx, fields) = (c.url, c.idx, c.fields);
    let generation = GENERATION.get();
    let mut col_at = c.col_at;
    let mut at = c.from;

    while at < fields.count {
        if GENERATION.get() != generation {
            return;
        }

        let left = budget_left();

        if left == 0 {
            log::info!(
                "'{url}': {at} of {} points resident - at the page's point ceiling (?points= to raise it)",
                fields.count
            );
            return;
        }

        let to = (at + STREAM_CHUNK_POINTS.min(left)).min(fields.count);
        budget_spend(to - at); // reserve before reading, so two clouds cannot both take the last room
        let Some(positions) = fetch_positions(&url, &fields, at, to).await else {
            if GENERATION.get() == generation {
                RESIDENT.set(RESIDENT.get().saturating_sub(to - at));
            }

            super::feedback::status("A point-cloud range failed; reload to retry the missing data");
            return;
        };
        let (colors, next) = fetch_colors(&url, &fields, col_at, to - at)
            .await
            .unwrap_or((Vec::new(), col_at));
        col_at = next;
        let normals = fetch_normals(&url, &fields, at, to)
            .await
            .unwrap_or_default();

        if GENERATION.get() != generation {
            return;
        }

        if !post(Msg::CloudChunk(CloudChunk {
            idx,
            rows: StreamRows {
                positions,
                colors,
                normals,
            },
            to,
        })) {
            return;
        }
        at = to;
    }
}
