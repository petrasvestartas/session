
/// Replace the rows of the layers panel.
#[cfg(target_arch = "wasm32")]
pub fn layers_panel(rows: &[LayerRow]) {
    super::ui::layers::STATE.with_borrow_mut(|model| model.rows = rows.to_vec());
}

/// Replace the rows of the graph table; `total` counts the edges not listed too.
#[cfg(target_arch = "wasm32")]
pub fn graph_panel(edges: Vec<EdgeRow>, total: usize) {
    super::ui::layers::STATE.with_borrow_mut(|model| {
        model.edges = edges;
        model.edge_total = total;
    });
}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn graph_panel(_edges: Vec<EdgeRow>, _total: usize) {}

/// Whether the graph table is unfolded.
#[cfg(target_arch = "wasm32")]
pub fn graph_open() -> bool {
    super::ui::layers::STATE.with_borrow(|model| model.graph_open)
}

/// Fold or unfold the graph table.
#[cfg(target_arch = "wasm32")]
pub fn toggle_graph() {
    super::ui::layers::STATE.with_borrow_mut(|model| model.graph_open = !model.graph_open);
}

/// Start editing the name of layer row `index`.
#[cfg(target_arch = "wasm32")]
pub fn rename_row(index: usize, label: &str) {
    super::ui::layers::STATE.with_borrow_mut(|model| {
        model.renaming = Some(super::ui::layers::Rename {
            node: index.to_string(),
            text: label.to_string(),
            focused: false,
            done: false,
        });
    });
}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn graph_open() -> bool {
    false
}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn toggle_graph() {}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn rename_row(_index: usize, _label: &str) {}

/// Show or hide the layers panel.
#[cfg(target_arch = "wasm32")]
pub fn layers_visible(open: bool) {
    super::ui::layers::STATE.with_borrow_mut(|model| {
        model.layers_open = open;

        if !open {
            model.rows.clear();
            model.edges.clear();
        }
    });
}

/// Whether the layers panel is open.
#[cfg(target_arch = "wasm32")]
pub fn layers_open() -> bool {
    super::ui::layers::STATE.with_borrow(|model| model.layers_open)
}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn layers_panel(_rows: &[LayerRow]) {}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn layers_visible(_open: bool) {}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn layers_open() -> bool {
    false
}
