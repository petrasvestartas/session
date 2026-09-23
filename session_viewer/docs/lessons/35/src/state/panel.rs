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
            // --8<-- [start:step-21a]
            let parts: Vec<_> = value.split('/').collect();

            if parts.len() == 3
                && let Ok(index) = parts[0].parse::<usize>()
                && index < self.hierarchy.nodes.len()
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

                for row in self.hierarchy.targets(index) {
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
                        // --8<-- [end:step-21a]
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
                // --8<-- [start:step-21b]
                "select" | "add" => {
                // --8<-- [end:step-21b]
                    let rows = self.hierarchy.targets(index);

                    // while splitting, a click picks cutters
                    if self.pending_split.is_some() {
                        for row in rows {
                            self.pick_split_cutter(row);
                        }

                        return;
                    }

                    // --8<-- [start:step-21c]
                    self.select_rows(rows, action == "add");
                    // --8<-- [end:step-21c]
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
                key: format!("select/{index}"),
                label: node.label.clone(),
                count,
                hidden,
                locked,
                // --8<-- [start:step-21d]
                selected: targets.iter().any(|r| {
                    self.scene.selected == Some(*r) || self.hierarchy.selected.contains(r)
                }),
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
                // --8<-- [end:step-21d]
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
