
/// The picked entity of the selected sheet, if known.
#[cfg(target_arch = "wasm32")]
fn sheet_entity(state: &State) -> Option<serde_json::Value> {
    let (id, meta) = state
        .scene
        .sheet_at(state.scene.selected?)?
        .resolved
        .as_ref()?;
    Some(serde_json::json!({"id": id, "guid": meta.guid, "name": meta.name, "kind": meta.kind}))
}
