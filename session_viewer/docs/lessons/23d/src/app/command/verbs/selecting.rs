use crate::State;
use crate::engine::gpu::Instance;

/// Rows a selection command may take: shown, not hidden by Hide or a layer, not locked; ascending. Shared by every select verb.
pub fn candidates(state: &State) -> Vec<u32> {
    let scene = &state.scene;
    (0..scene.row_count() as u32)
        .filter(|&row| {
            scene.selectable(row)
                && scene
                    .identity_of(row)
                    .is_some_and(|id| !scene.hidden.contains(&id))
                && state
                    .gpu
                    .objects
                    .row(row)
                    .is_some_and(|object| object.flags & Instance::FLAG_HIDDEN == 0)
        })
        .collect()
}

/// Select `found` (ascending) instead of the selection, added to it, or taken out of it; the count selected after.
pub fn apply(state: &mut State, found: Vec<u32>, add: bool, remove: bool) -> usize {
    if remove {
        let mut keep = state.selected_rows();
        keep.retain(|row| found.binary_search(row).is_err()); // `found` is ascending, so binary_search checks a row in a few steps
        state.select_rows(keep, false);
    } else {
        state.select_rows(found, add);
    }

    state.selected_rows().len()
}
