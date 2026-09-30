
use crate::app::scene::StreamedInit;
use crate::app::walk::cloud::StreamRows;

/// The next slice of streamed cloud `idx`.
pub struct CloudChunk {
    pub idx: usize,       // which cloud
    pub rows: StreamRows, // the new points
    pub to: u32,          // rows loaded so far
}

/// Add a streamed cloud's first rows and keep loading the rest.
#[cfg(target_arch = "wasm32")]
fn start_stream(state: &mut State, init: Box<StreamedInit>) {
    let (url, fields, from, col_at) = (
        init.url.clone(),
        init.fields.clone(),
        init.resident,
        init.col_at,
    );
    let idx = state.add_streamed(*init);
    app::loader::spawn_stream_rest(app::loader::StreamCursor {
        idx,
        url,
        fields,
        from,
        col_at,
    });
}
