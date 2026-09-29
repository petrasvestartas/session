
// A sheet is a drawing's flat linework, up to millions of two-point segments, streamed by range like a cloud.
use super::scene::SheetInit;
use super::stream::{SheetFields, fetch_sheet_slice, sheet_fields};
use super::walk::sheet::SheetRows;
use crate::SheetChunk;

/// A sheet read by range: its first segments go out now.
async fn start_sheet(cx: &mut ItemCx<'_>, head: &Reply, slot: &Placement) -> Result<(), Step> {
    let remaining = if cx.replacement.is_some() {
        max_segments().saturating_sub(*cx.staged_segments)
    } else {
        sheet_budget_left()
    };
    let Some(init) = sheet_prefix(cx.url, slot, remaining, head).await else {
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
        *cx.staged_segments = cx.staged_segments.saturating_add(init.resident);
        cx.pending.push(PendingDocument::Sheet(Box::new(init)));
    } else {
        sheet_budget_spend(init.resident);
        post(Msg::Sheet(Box::new(init)));
    }

    Err(Step::Next)
}

/// `name` in the same folder as `url`.
fn sibling(url: &str, name: &str) -> String {
    let dir = url.rfind('/').map_or(0, |at| at + 1);
    format!("{}{name}", &url[..dir])
}

/// Read a sheet's first `share` segments by range; None when not a sheet file.
async fn sheet_prefix(url: &str, slot: &Placement, share: u32, head: &Reply) -> Option<SheetInit> {
    let name = slot.name.as_str();
    let fields = sheet_fields(url, head).await?;
    let meta_url = (!fields.meta.is_empty()).then(|| sibling(url, &fields.meta));
    let mut resident = SHEET_PREFIX_SEGMENTS.min(share).min(fields.count);
    let rows = match fetch_sheet_slice(url, &fields, 0, resident).await {
        Some(rows) if resident > 0 => rows,
        _ => {
            log::warn!(
                "'{name}': no sheet prefix - {resident} of {} segments allowed (?segments= to raise the ceiling) or the range read failed",
                fields.count
            );
            resident = 0;
            SheetRows {
                positions: Vec::new(),
                colors: Vec::new(),
                widths: Vec::new(),
                ids: Vec::new(),
            }
        }
    };
    log::info!(
        "sheet '{name}': {resident} of {} segments on screen, {} entities",
        fields.count,
        fields.entities
    );
    Some(SheetInit {
        name: name.to_string(),
        url: url.to_string(),
        meta_url,
        place: slot.place.clone(),
        rows,
        fields,
        resident,
    })
}

/// Where a sheet's streaming continues.
pub struct SheetCursor {
    pub idx: usize,          // the sheet's slot in the scene
    pub url: String,         // the sheet file
    pub fields: SheetFields, // array positions in the file
    pub from: u32,           // next segment to read
}

/// Keep reading a sheet's slices in the background.
pub fn spawn_sheet_rest(cursor: SheetCursor) {
    wasm_bindgen_futures::spawn_local(sheet_rest(cursor));
}

/// The slice loop behind `spawn_sheet_rest`.
async fn sheet_rest(c: SheetCursor) {
    let (url, idx, fields) = (c.url, c.idx, c.fields);
    let generation = GENERATION.get();
    let mut at = c.from;
    after_loads().await;

    while at < fields.count {
        if GENERATION.get() != generation {
            return;
        }

        let left = sheet_budget_left();

        if left == 0 {
            log::info!(
                "'{url}': {at} of {} segments resident - at the page's segment ceiling (?segments= to raise it)",
                fields.count
            );
            return;
        }

        let to = (at + SHEET_CHUNK_SEGMENTS.min(left)).min(fields.count);
        sheet_budget_spend(to - at);
        let Some(rows) = fetch_sheet_slice(&url, &fields, at, to).await else {
            if GENERATION.get() == generation {
                SHEET_RESIDENT.set(SHEET_RESIDENT.get().saturating_sub(to - at));
            }

            super::feedback::status("A sheet range failed; reload to retry the missing data");
            return;
        };

        if GENERATION.get() != generation {
            return;
        }

        if !post(Msg::SheetChunk(SheetChunk { idx, rows, to })) {
            return;
        }

        at = to;
    }
}
