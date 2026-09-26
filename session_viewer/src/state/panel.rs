use crate::app::layers::Layer;
use crate::state::State;
use std::rc::Rc;

impl State {
    /// How many rows are selected together.
    pub fn selected_group_count(&self) -> usize {
        self.features.hierarchy.selected.len()
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
            let parts: Vec<_> = value.split('/').collect();

            if parts.len() == 3
                && let Ok(index) = parts[0].parse::<usize>()
                && index < self.features.hierarchy.nodes.len()
                && matches!(parts[1], "face" | "edge")
            {
                let edge = parts[1] == "edge";
                let color = if parts[2] == "original" {
                    None
                } else {
                    let Ok(rgb) = u32::from_str_radix(parts[2], 16) else {
                        return;
                    };
                    Some([(rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8])
                };

                for row in self.features.hierarchy.targets(index) {
                    // an edge color needs faces to sit on
                    if edge
                        && !self.gpu.objects.row(row).is_some_and(|r| {
                            r.flags & crate::engine::gpu::Instance::FLAG_HAS_FACES != 0
                        })
                    {
                        continue;
                    }

                    if let Some(id) = self.scene.identity_of(row) {
                        let colors = if edge {
                            &mut self.scene.edge_colors
                        } else {
                            &mut self.scene.colors
                        };

                        if let Some(color) = color {
                            colors.insert(id, color);
                        } else {
                            colors.remove(&id);
                        }

                        self.gpu.set_object_color(row, edge, color);
                    }
                }

                self.refresh_layers();
                self.touch();
            }

            return;
        }

        // rename/<node>/<new name>
        if let Some(value) = key.strip_prefix("rename/") {
            if let Some((index, to)) = value.split_once('/')
                && let Ok(index) = index.parse::<usize>()
                && let Some((doc, name)) = self.layer_of(index)
            {
                let to = to.trim().to_string();
                let keep = self.selected_identities();
                let result = self.scene.rename_layer(doc, &name, &to).map(|_| {
                    // the fold and the highlight follow the new name
                    self.features.hierarchy.nodes[index].name.clone_from(&to);
                    format!("Renamed {name} to {to}")
                });
                self.after_layer_edit(result, keep);
            }

            return;
        }

        // pair/<row>/<row>: a graph edge selects both ends
        if let Some(value) = key.strip_prefix("pair/") {
            let rows: Vec<u32> = value
                .split('/')
                .filter_map(|row| row.parse().ok())
                .collect();
            self.select_rows(rows, false);
            return;
        }

        // the graph table folds; its rows are made only while it is open
        if key == "graph/toggle" {
            crate::app::feedback::toggle_graph();
            self.refresh_layers();
            self.touch();
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
            self.features.hierarchy.page = index;
        } else if index < self.features.hierarchy.nodes.len() {
            match action {
                "open" => {
                    // fold or unfold the node
                    if !self.features.hierarchy.open.remove(&index) {
                        self.features.hierarchy.open.insert(index);
                    }
                }
                "select" | "add" => {
                    let rows = self.features.hierarchy.targets(index);

                    // a tool picking objects takes the rows
                    if self.tool_picks() {
                        for row in rows {
                            self.tool_picked(Some(row));
                        }

                        return;
                    }

                    // while splitting, a click picks cutters
                    if self.features.pending_split.is_some() {
                        for row in rows {
                            self.pick_split_cutter(row);
                        }

                        return;
                    }

                    // the clicked layers stay highlighted; any other selection drops them
                    let mut active = if action == "add" {
                        std::mem::take(&mut self.features.hierarchy.active)
                    } else {
                        Vec::new()
                    };

                    if self.features.hierarchy.nodes[index].layer {
                        active.push(index);
                    }

                    self.select_rows(rows, action == "add");
                    self.features.hierarchy.active = active;
                }
                "lock" => {
                    let rows = self.features.hierarchy.targets(index);
                    let lock = rows.iter().any(|row| self.scene.selectable(*row)); // anything unlocked: lock all

                    // a locked row cannot stay selected
                    if lock
                        && (self
                            .scene
                            .selected
                            .is_some_and(|row| rows.binary_search(&row).is_ok())
                            || self
                                .features
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
                "current" | "new_layer" | "new_sublayer" | "delete_layer" | "duplicate_layer"
                | "change_layer" | "copy_layer" => {
                    self.layer_action(action, index);
                    return;
                }
                "hide" => {
                    let node = &self.features.hierarchy.nodes[index];
                    // the current layer, a layer holding it or its document line
                    let holds = match self.layer_of(index) {
                        Some((doc, name)) => self.scene.holds_current(doc, &name),
                        None => {
                            node.depth == 0
                                && self
                                    .scene
                                    .current_layer()
                                    .is_some_and(|(doc, _)| doc == node.doc)
                        }
                    };

                    if holds {
                        self.status("The current layer cannot be hidden");
                        return;
                    }

                    let rows = self.features.hierarchy.targets(index);
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

    /// Unfold the panel down to layer `name` of `doc` and open it, while the panel is shown.
    pub(crate) fn reveal_layer(&mut self, doc: usize, name: &str) {
        if !crate::app::feedback::layers_open() {
            return;
        }

        self.features.hierarchy.refresh(&self.scene);

        if let Some(node) = self.features.hierarchy.index_of(doc, name) {
            self.features.hierarchy.reveal(node);
            self.features.hierarchy.open.insert(node);
        }
    }

    /// The (document, tree node) of a layer row.
    fn layer_of(&self, index: usize) -> Option<(usize, String)> {
        let node = self.features.hierarchy.nodes.get(index)?;
        node.layer.then(|| (node.doc, node.name.clone()))
    }

    /// One layer menu action on node `index`.
    fn layer_action(&mut self, action: &str, index: usize) {
        let Some((doc, name)) = self.layer_of(index) else {
            return;
        };
        let rows = self.selected_rows();
        // what stays selected, by identity
        let mut keep = self.selected_identities();
        let mut made = None; // a new layer, named right away
        let result = match action {
            "current" => self.scene.set_current_layer(doc, &name).map(|_| {
                let rows = self.features.hierarchy.targets(index);

                // a hidden layer comes back when made current
                if !rows.is_empty()
                    && rows.iter().all(|row| {
                        self.scene
                            .identity_of(*row)
                            .is_some_and(|id| self.scene.hidden.contains(&id))
                    })
                {
                    self.set_rows_hidden(&rows, false);
                }

                format!("Current layer: {name}")
            }),
            "new_layer" | "new_sublayer" => self
                .scene
                .new_layer(doc, &name, action == "new_sublayer")
                .map(|layer| {
                    let message = format!("Added {layer}");
                    made = Some(layer);
                    message
                }),
            "delete_layer" => self
                .scene
                .delete_layer(doc, &name)
                .map(|count| format!("Deleted {name} and {count} objects")),
            "duplicate_layer" => self
                .scene
                .duplicate_layer(doc, &name)
                .map(|made| format!("Added {made}")),
            "change_layer" => {
                self.scene
                    .change_object_layer(&rows, doc, &name)
                    .map(|moved| {
                        let message = format!("Moved {} objects to {name}", moved.len());
                        self.features.hierarchy.active.clear(); // the clicked layers no longer hold them
                        keep = moved;
                        message
                    })
            }
            _ => self
                .scene
                .copy_object_layer(&rows, doc, &name)
                .map(|copies| format!("Copied {} objects to {name}", copies.len())),
        };
        self.after_layer_edit(result, keep);

        // a new layer is named right away
        if let Some(layer) = made
            && let Some(node) = self.features.hierarchy.index_of(doc, &layer)
        {
            self.features.hierarchy.reveal(node);
            self.refresh_layers();
            crate::app::feedback::rename_row(node, &layer);
        }
    }

    /// The selected objects by identity, which outlives their rows.
    fn selected_identities(&self) -> Vec<(usize, Rc<str>)> {
        self.selected_rows()
            .iter()
            .filter_map(|row| self.scene.identity_of(*row))
            .collect()
    }

    /// Report a layer edit and sync what it changed; `keep` stays selected, found by identity.
    fn after_layer_edit(&mut self, result: Result<String, String>, keep: Vec<(usize, Rc<str>)>) {
        match result {
            Ok(message) => {
                // the clicked layers by name, found again in the rebuilt panel
                let active: Vec<(usize, String)> = self
                    .features
                    .hierarchy
                    .active
                    .iter()
                    .filter_map(|index| self.features.hierarchy.nodes.get(*index))
                    .map(|node| (node.doc, node.name.clone()))
                    .collect();
                self.commit_rows();
                let mut rows: Vec<u32> = keep
                    .iter()
                    .filter_map(|(doc, guid)| self.scene.row_of(*doc, guid))
                    .collect();
                rows.sort_unstable();

                // objects moved to another document have new rows
                if rows != self.selected_rows() {
                    self.select_rows(rows, false);
                }

                self.features.hierarchy.active = active
                    .iter()
                    .filter_map(|(doc, name)| self.features.hierarchy.index_of(*doc, name))
                    .collect();
                self.status(&message);
            }
            Err(error) => {
                self.commit_rows();
                self.status(&error);
            }
        }

        self.refresh_layers();
        self.update_label();
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
                    .features
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
        let visible = self.features.hierarchy.visible(); // unfolded nodes
        self.features.hierarchy.page = self
            .features
            .hierarchy
            .page
            .min(visible.len().saturating_sub(1) / PAGE_SIZE); // keep the page in range
        let first = self.features.hierarchy.page * PAGE_SIZE;
        let current = self.scene.current_layer();
        let chosen = |row: &u32| {
            self.scene.selected == Some(*row)
                || self.features.hierarchy.selected.binary_search(row).is_ok()
        };
        let hidden = |row: &u32| {
            self.scene
                .identity_of(*row)
                .is_some_and(|id| self.scene.hidden.contains(&id))
        };

        // one panel row per visible node on this page
        for &index in visible.iter().skip(first).take(PAGE_SIZE) {
            let node = &self.features.hierarchy.nodes[index];
            let count = node.rows.len();
            let targets = &self.features.hierarchy.rows[node.rows.clone()];
            let locked = count > 0 && targets.iter().all(|row| !self.scene.selectable(*row)); // every row locked?
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
            // a clicked layer or sublayer while all it can select is, an object while it is selected
            let clicked =
                self.features.hierarchy.active.iter().any(|&layer| {
                    layer <= index && index < self.features.hierarchy.nodes[layer].end
                });
            let selected = if node.layer {
                clicked
                    && targets
                        .iter()
                        .all(|row| chosen(row) || hidden(row) || !self.scene.selectable(*row))
            } else {
                targets.iter().any(chosen)
            };
            rows.push(LayerRow {
                key: format!("select/{index}"), // the select button key
                label: node.label.clone(),
                count,
                hidden: count > 0 && targets.iter().all(hidden), // every row hidden, and at least one
                locked,
                selected,
                color,
                edge_color: targets
                    .first()
                    .and_then(|row| self.scene.identity_of(*row))
                    .and_then(|id| self.scene.edge_colors.get(&id).copied())
                    .filter(|first| {
                        targets.iter().all(|row| {
                            self.scene
                                .identity_of(*row)
                                .is_some_and(|id| self.scene.edge_colors.get(&id) == Some(first))
                        })
                    }),
                has_faces: targets.iter().any(|row| {
                    self.gpu.objects.row(*row).is_some_and(|r| {
                        r.flags & crate::engine::gpu::Instance::FLAG_HAS_FACES != 0
                    })
                }),
                depth: node.depth,
                expanded: (node.end > index + 1)
                    .then(|| self.features.hierarchy.open.contains(&index)),
                layer: node.layer,
                current: node.layer
                    && current
                        .as_ref()
                        .is_some_and(|(doc, name)| *doc == node.doc && *name == node.name),
                root: node.layer
                    && self.scene.docs[node.doc]
                        .session
                        .tree
                        .root()
                        .is_some_and(|root| root.borrow().name == node.name),
            });
        }

        // Previous and Next rows when there are more pages
        for (label, page) in [
            ("Previous", self.features.hierarchy.page.checked_sub(1)),
            (
                "Next",
                (first + PAGE_SIZE < visible.len()).then_some(self.features.hierarchy.page + 1),
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

        if self.features.hierarchy.truncated {
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
