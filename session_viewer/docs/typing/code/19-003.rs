
use crate::app::scene::SheetInit;
use crate::app::walk::sheet::SheetRows;

/// The next slice of sheet `idx`.
pub struct SheetChunk {
    pub idx: usize,      // which sheet
    pub rows: SheetRows, // the new segments
    pub to: u32,         // segments loaded so far
}

/// Add a sheet's first segments and keep loading the rest.
#[cfg(target_arch = "wasm32")]
fn start_sheet(state: &mut State, init: Box<SheetInit>) {
    let (url, fields, from) = (init.url.clone(), init.fields.clone(), init.resident);
    let idx = state.add_sheet(*init);
    app::loader::spawn_sheet_rest(app::loader::SheetCursor {
        idx,
        url,
        fields,
        from,
    });
}
