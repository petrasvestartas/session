use super::State;
use crate::app::selection::SelectionMode;
use crate::app::sheet_query::{Query, Resolved};

impl State {
    /// Select one sheet entity; the same one again clears it.
    pub(super) fn apply_sheet_pick(&mut self, row: u32, entity: u32) {
        let Some(slot) = self.scene.sheet_slot(row) else {
            return;
        };

        // already selected: clear
        if self.selection
            == (SelectionMode::Edge {
                parent: row,
                edge: entity,
            })
        {
            self.select(None);
            self.status("");
            return;
        }

        self.select(Some(row));
        self.gpu.set_selected(row, false);
        self.selection.select_edge(row, entity);
        self.gpu
            .segments
            .set_edge(&self.gpu.ctx, Some((row, entity)));
        self.scene.sheets[slot].resolved = None;
        self.features.sheet_generation = self.features.sheet_generation.wrapping_add(1); // new query id
        let query = Query::new(self.features.sheet_generation, row, entity);

        // fetch name and kind from the side table, if any
        match self.scene.sheets[slot].meta_url.clone() {
            None => self.status(&format!("Selected entity {entity}")),
            Some(url) => {
                self.status(&format!("Selected entity {entity}, fetching…"));
                #[cfg(target_arch = "wasm32")]
                crate::app::sheet_query::fetch_entity(
                    &query,
                    url,
                    self.scene.sheets[slot].table.clone(),
                    self.scene.sheets[slot].fields.entities,
                );
                #[cfg(not(target_arch = "wasm32"))]
                let _ = url;
            }
        }

        self.features.sheet_query = Some(query);
        self.touch();
    }

    /// The entity's name and kind arrived: show them.
    pub fn sheet_entity(&mut self, resolved: Resolved) {
        let Some(query) = self.features.sheet_query.as_ref() else {
            return;
        };

        // an old query's answer
        if query.id != resolved.query || query.cancelled.get() {
            return;
        }

        let query = self.features.sheet_query.take().unwrap();
        let Some(slot) = self.scene.sheet_slot(query.row) else {
            return;
        };

        match resolved.result {
            Ok((meta, table)) => {
                self.status(&format!(
                    "Selected entity {}: {} ({})",
                    query.entity, meta.name, meta.kind
                ));
                let sheet = &mut self.scene.sheets[slot];
                sheet.table = Some(table);
                sheet.resolved = Some((query.entity, meta));
                self.update_label();
                self.touch();
            }
            Err(error) => self.status(&format!("Entity {}: {error}", query.entity)),
        }
    }
}
