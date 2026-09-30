
/// Undo steps held by every document.
#[cfg(target_arch = "wasm32")]
fn undo_depth(state: &State) -> serde_json::Value {
    serde_json::json!(
        state
            .scene
            .docs
            .iter()
            .map(|doc| doc.session.history.depth())
            .sum::<usize>()
    )
}
