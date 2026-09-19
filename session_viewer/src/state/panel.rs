use crate::app::layers::Layer;
use crate::state::State;

impl State {
    pub fn selected_group_count(&self) -> usize {
        self.hierarchy.selected.len()
    }

    pub fn panel_action(&mut self, key: &str) {
        if let Some(layer) = Layer::from_key(key) {
            self.toggle_layer(layer);
            return;
        }

        if let Some(value) = key.strip_prefix("color/") {
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
                    if !self.hierarchy.open.remove(&index) {
                        self.hierarchy.open.insert(index);
                    }
                }
                "select" => {
                    let rows = self.hierarchy.targets(index);

                    if self.pending_split.is_some() {
                        for row in rows {
                            self.pick_split_cutter(row);
                        }

                        return;
                    }

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
                    let lock = rows.iter().any(|row| self.scene.selectable(*row));

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

    /// Apply visibility to sorted object rows while preserving unrelated selection.
    pub(super) fn set_rows_hidden(&mut self, rows: &[u32], hide: bool) {
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

    pub(super) fn hierarchy_labels(&mut self, rows: &mut Vec<crate::app::feedback::LayerRow>) {
        use crate::app::feedback::LayerRow;
        use crate::app::hierarchy::PAGE_SIZE;
        let visible = self.hierarchy.visible();
        self.hierarchy.page = self
            .hierarchy
            .page
            .min(visible.len().saturating_sub(1) / PAGE_SIZE);
        let first = self.hierarchy.page * PAGE_SIZE;

        for &index in visible.iter().skip(first).take(PAGE_SIZE) {
            let node = &self.hierarchy.nodes[index];
            let count = node.rows.len();
            let hidden = self.hierarchy.rows[node.rows.clone()].iter().all(|row| {
                self.scene
                    .identity_of(*row)
                    .is_some_and(|id| self.scene.hidden.contains(&id))
            });
            let targets = &self.hierarchy.rows[node.rows.clone()];
            let locked = targets.iter().all(|row| !self.scene.selectable(*row));
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
                expanded: (node.end > index + 1).then(|| self.hierarchy.open.contains(&index)),
            });
        }

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
