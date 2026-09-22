use crate::app::layers::Layer;
use crate::state::State;

impl State {
    /// How many rows are selected together.
    pub fn selected_group_count(&self) -> usize {
        self.hierarchy.selected.len()
    }

    /// A click in the layers panel, by its key.
    pub fn panel_action(&mut self, key: &str) {
        // a layer row: hide or show it
        if let Some(layer) = Layer::from_key(key) {
            self.toggle_layer(layer);
            return;
        }

        // color/<node>/<face|edge>/<hex or original>
        if let Some(value) = key.strip_prefix("color/") {
            if let Some((index, hex)) = value.split_once('/')
                && let (Ok(index), Ok(rgb)) = (index.parse::<usize>(), u32::from_str_radix(hex, 16))
                && index < self.hierarchy.nodes.len()
            {
                let color = [(rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8];

                for row in self.hierarchy.targets(index) {
                    if let Some(id) = self.scene.identity_of(row) {
                        self.scene.colors.insert(id, color);
                        self.gpu.set_object_color(row, color);
                    }
                }

                self.refresh_layers();
                self.touch();
            }

            return;
        }

        // <action>/<node index>
        let Some((action, index)) = key.split_once('/') else {
            return;
        };
        let Ok(index) = index.parse::<usize>() else {
            return;
        };

        if action == "page" {
            self.hierarchy.page = index;
        } else if index < self.hierarchy.nodes.len() {
            match action {
                "open" => {
                    // fold or unfold the node
                    if !self.hierarchy.open.remove(&index) {
                        self.hierarchy.open.insert(index);
                    }
                }
                "select" => {
                    let rows = self.hierarchy.targets(index);
// --8<-- [start:step-13]

                    // while splitting, a click picks cutters
                    if self.pending_split.is_some() {
                        for row in rows {
                            self.pick_split_cutter(row);
                        }

                        return;
                    }

// --8<-- [end:step-13]
                    self.select(None);

                    for row in rows {
                        if self.scene.identity_of(row).is_some_and(|id| {
                            !self.scene.hidden.contains(&id) && !self.scene.locked.contains(&id)
                        }) {
                            self.gpu.set_selected(row, true);
                            self.hierarchy.selected.push(row);
                        }
                    }

                    if self.hierarchy.selected.len() == 1 {
                        let row = self.hierarchy.selected[0];
                        self.select(Some(row));
                    }
                }
                "lock" => {
                    let rows = self.hierarchy.targets(index);
                    let lock = rows.iter().any(|row| self.scene.selectable(*row)); // anything unlocked: lock all

                    // a locked row cannot stay selected
                    if lock
                        && (self
                            .scene
                            .selected
                            .is_some_and(|row| rows.binary_search(&row).is_ok())
                            || self
                                .hierarchy
                                .selected
                                .iter()
                                .any(|row| rows.binary_search(row).is_ok()))
                    {
                        self.select(None);
                    }

                    for row in rows {
                        if let Some(id) = self.scene.identity_of(row) {
                            if lock {
                                self.scene.locked.insert(id);
                            } else {
                                self.scene.locked.remove(&id);
                            }
                        }
                    }
                }
                "hide" => {
                    let rows = self.hierarchy.targets(index);
                    // anything visible: hide all
                    let hide = rows.iter().any(|row| {
                        self.scene
                            .identity_of(*row)
                            .is_some_and(|id| !self.scene.hidden.contains(&id))
                    });
                    self.set_rows_hidden(&rows, hide);
                    return;
                }
                _ => return,
            }
        }

        self.refresh_layers();
        self.touch();
    }

    /// Hide or show sorted rows.
    pub(super) fn set_rows_hidden(&mut self, rows: &[u32], hide: bool) {
        // a hidden row cannot stay selected
        if hide
            && (self
                .scene
                .selected
                .is_some_and(|row| rows.binary_search(&row).is_ok())
                || self
                    .hierarchy
                    .selected
                    .iter()
                    .any(|row| rows.binary_search(row).is_ok()))
        {
            self.select(None);
        }

        for row in rows {
            let Some(id) = self.scene.identity_of(*row) else {
                continue;
            };
            let changed = if hide {
                self.scene.hidden.insert(id)
            } else {
                self.scene.hidden.remove(&id)
            };

            if changed {
                self.gpu.set_hidden(*row, hide);
            }
        }

        self.refresh_layers();
        self.update_label();
        self.touch();
    }

    /// The rows of the layers panel for the current page.
    pub(super) fn hierarchy_labels(&mut self, rows: &mut Vec<crate::app::feedback::LayerRow>) {
        use crate::app::feedback::LayerRow;
        use crate::app::hierarchy::PAGE_SIZE;
        let visible = self.hierarchy.visible(); // unfolded nodes
        self.hierarchy.page = self
            .hierarchy
            .page
            .min(visible.len().saturating_sub(1) / PAGE_SIZE); // keep the page in range
        let first = self.hierarchy.page * PAGE_SIZE;

        // one panel row per visible node on this page
        for &index in visible.iter().skip(first).take(PAGE_SIZE) {
            let node = &self.hierarchy.nodes[index];
            let count = node.rows.len();
            // every row hidden?
            let hidden = self.hierarchy.rows[node.rows.clone()].iter().all(|row| {
                self.scene
                    .identity_of(*row)
                    .is_some_and(|id| self.scene.hidden.contains(&id))
            });
            let targets = &self.hierarchy.rows[node.rows.clone()];
            let locked = targets.iter().all(|row| !self.scene.selectable(*row)); // every row locked?
            // the shared color, when every row has the same one
            let first_color = targets
                .first()
                .and_then(|row| self.scene.identity_of(*row))
                .and_then(|id| self.scene.colors.get(&id).copied());
            let color = first_color.filter(|first| {
                targets.iter().all(|row| {
                    self.scene
                        .identity_of(*row)
                        .is_some_and(|id| self.scene.colors.get(&id) == Some(first))
                })
            });
            rows.push(LayerRow {
                key: format!("select/{index}"), // the select button key
                label: node.label.clone(),
                count,
                hidden,
                locked,
                color,
                depth: node.depth,
                expanded: (node.end > index + 1).then(|| self.hierarchy.open.contains(&index)),
            });
        }

        // Previous and Next rows when there are more pages
        for (label, page) in [
            ("Previous", self.hierarchy.page.checked_sub(1)),
            (
                "Next",
                (first + PAGE_SIZE < visible.len()).then_some(self.hierarchy.page + 1),
            ),
        ] {
            if let Some(page) = page {
                rows.push(LayerRow {
                    key: format!("page/{page}"),
                    label: label.into(),
                    count: visible.len(),
                    hidden: false,
                    ..Default::default()
                });
            }
        }

        if self.hierarchy.truncated {
            rows.push(LayerRow {
                key: String::new(),
                label: "Tree exceeds panel capacity".into(),
                count: 0,
                hidden: false,
                ..Default::default()
            });
        }
    }
}
