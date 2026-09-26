// --8<-- [start:layers-state]
// The layers panel: the document tree down the right edge, a bulb, lock and colour per row, the graph table under it.
use super::command_line::command_cursor_select;
use super::{Control, Output, record};
use crate::State;
use crate::app::feedback::{EdgeRow, LayerRow};
use std::cell::RefCell;

/// What the layers panel shows and remembers between frames.
#[derive(Default)]
pub(crate) struct Layers {
    pub(crate) layers_open: bool,        // layers panel shown
    pub(crate) rows: Vec<LayerRow>,      // its rows
    pub(crate) edges: Vec<EdgeRow>,      // graph table rows
    pub(crate) edge_total: usize,        // graph edges, listed or not
    pub(crate) graph_open: bool,         // graph table unfolded
    pub(crate) renaming: Option<Rename>, // a layer name edited in its row
    layers_collapsed: bool,              // layers panel folded to its title
    keyboard_rects: Vec<egui::Rect>,     // a layer name field or an item opening one
}

thread_local! { pub(crate) static STATE: RefCell<Layers> = RefCell::default(); } // kept between frames

/// A layer name being edited in its row.
pub(crate) struct Rename {
    pub node: String,  // the row's node index
    pub text: String,  // the name typed so far
    pub focused: bool, // the field has the keys
    pub done: bool,    // kept by Enter or a click elsewhere, applied after the frame
}
// --8<-- [end:layers-state]

// --8<-- [start:layers-hooks]
/// The layers panel, registered in PANELS.
pub(super) struct Hooks;

impl super::Panel for Hooks {
    fn show(&self, root: &mut egui::Ui, controls: &mut Option<Vec<Control>>, out: &mut Output) {
        STATE.with_borrow_mut(|model| draw(root, model, controls, out));
    }

    fn apply(&self, state: &mut State) -> bool {
        // a kept layer name goes in before the click that ended its edit
        let renamed = STATE.with_borrow_mut(|model| model.renaming.take_if(|rename| rename.done));

        if let Some(rename) = &renamed {
            state.panel_action(&format!("rename/{}/{}", rename.node, rename.text));
        }

        renamed.is_some()
    }

    fn keys_taken(&self) -> bool {
        STATE.with_borrow(|model| model.renaming.as_ref().is_some_and(|rename| rename.focused))
    }

    fn holds_escape(&self) -> bool {
        STATE.with_borrow(|model| model.renaming.is_some())
    }

    fn field(&self, edit: &mut dyn FnMut(&mut String)) -> Option<&'static str> {
        STATE.with_borrow_mut(|model| {
            model.renaming.as_mut().map(|rename| {
                edit(&mut rename.text);
                "layer-rename"
            })
        })
    }

    fn hit(&self, point: egui::Pos2) -> (bool, bool) {
        let inside =
            STATE.with_borrow(|model| model.keyboard_rects.iter().any(|r| r.contains(point)));
        (inside, false)
    }

    fn snapshot(&self, json: &mut serde_json::Map<String, serde_json::Value>) {
        STATE.with_borrow(|model| {
            json.insert("rows".into(), serde_json::json!(model.rows));
            json.insert("edges".into(), serde_json::json!(model.edges));
            json.insert("edge_total".into(), model.edge_total.into());
            json.insert("layers_open".into(), model.layers_open.into());
        });
    }
}
// --8<-- [end:layers-hooks]

// --8<-- [start:layers-draw]
/// The layers panel; a click sets `action`.
fn draw(
    root: &mut egui::Ui,                 // the panel area
    model: &mut Layers,                  // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    out: &mut Output,
) {
    let action = &mut out.action;
    model.keyboard_rects.clear();

    if !model.layers_open {
        return;
    }

    let collapsed = model.layers_collapsed;
    let width = if collapsed {
        32.0
    } else {
        (root.available_width() * 0.25).clamp(180.0, 310.0)
    };
    // two ids, so egui remembers the width dragged on the open panel apart from the 32 px strip
    egui::Panel::right(if collapsed {
        "session-layers-collapsed"
    } else {
        "session-layers"
    })
    .default_size(width)
    .size_range(if collapsed {
        32.0..=32.0
    } else {
        180.0..=360.0
    })
    .show_separator_line(false)
    .frame(
        egui::Frame::new()
            .fill(egui::Color32::from_gray(245))
            .inner_margin(4),
    )
    .resizable(!collapsed)
    .show_inside(root, |ui| {
        ui.set_min_width(ui.available_width());
        let collapse = ui
            .button(if collapsed { "+" } else { "–" })
            .on_hover_text("Collapse or expand panel");
        record(
            controls,
            "layers/collapse",
            "Collapse or expand panel",
            &collapse,
        );
        if collapse.clicked() {
            model.layers_collapsed = !model.layers_collapsed;
            ui.ctx().request_repaint();
        }
        if collapsed {
            return;
        }
        let height = ui.text_style_height(&egui::TextStyle::Body); // one compact row
        // the graph section sits below the tree once there is an edge, a table when unfolded
        let graph = !model.rows.is_empty() && model.edge_total > 0;
        let rest = (ui.available_height() - if graph { height + 4. } else { 0. }).max(0.);
        let tree = if graph && model.graph_open {
            rest * 0.6
        } else {
            rest
        };
        // one line per row
        egui::ScrollArea::vertical()
            .id_salt("layer-rows")
            .max_height(tree)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.;
                ui.spacing_mut().interact_size.y = height;
                ui.spacing_mut().button_padding.y = 0.;
                // borrow three fields apart, so the loop reads `rows` while it writes `renaming`
                let Layers {
                    rows,
                    renaming,
                    keyboard_rects,
                    ..
                } = &mut *model;

                for row in rows.iter() {
                    // reserve a place in the paint order now, filled once the row is laid out, so the strip sits under it
                    let strip = ui.painter().add(egui::Shape::Noop);
                    let line = ui.horizontal(|ui| {
                        if let Some(index) = row.key.strip_prefix("select/") {
                            layer_row(ui, row, index, height, controls, action, renaming)
                        } else {
                            let response = ui.button(&row.label);
                            record(controls, &row.key, &row.label, &response);

                            if response.clicked() {
                                *action = Some(row.key.clone());
                            }

                            Vec::new()
                        }
                    });
                    keyboard_rects.extend(line.inner);

                    // the whole strip, buttons included
                    if row.selected {
                        let rect = egui::Rect::from_x_y_ranges(
                            ui.max_rect().x_range(),
                            line.response.rect.y_range(),
                        );
                        ui.painter()
                            .set(strip, egui::Shape::rect_filled(rect, 0., SELECTED));
                    }
                }
            });

        if graph {
            super::graph::show(ui, model, height, controls, action);
        }
    });
}

/// The selection yellow, as in the scene.
pub(super) const SELECTED: egui::Color32 = egui::Color32::from_rgb(255, 255, 0);
// --8<-- [end:layers-draw]

// --8<-- [start:layers-row]
/// One tree row: arrow, name, bulb or check, lock, swatch; returns where a tap raises the keyboard.
fn layer_row(
    ui: &mut egui::Ui,
    row: &LayerRow,
    index: &str,
    height: f32,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
    renaming: &mut Option<Rename>,
) -> Vec<egui::Rect> {
    let mut keyboard = Vec::new();
    ui.spacing_mut().item_spacing.x = 2.;
    ui.add_space(row.depth.min(8) as f32 * 10.); // 10 px of indent per level, at most 8 levels
    let response = layer_icon(ui, "open", row, height);
    record(controls, &format!("open/{index}"), &row.label, &response);

    if response.clicked() && row.expanded.is_some() {
        *action = Some(format!("open/{index}"));
    }

    // the name takes what the three icons leave
    let width = (ui.available_width() - 3. * (height + 6.)).max(24.);

    if let Some(rename) = renaming.as_mut()
        && rename.node == index
        && !rename.done
    {
        let edit = ui.add_sized(
            [width, height],
            egui::TextEdit::singleline(&mut rename.text)
                .id(egui::Id::new("layer-rename"))
                .margin(egui::vec2(2., 0.)),
        );
        record(controls, &format!("rename/{index}"), &row.label, &edit);
        keyboard.push(edit.rect);

        // the whole name selected, so typing replaces it
        if !rename.focused {
            edit.request_focus();
            command_cursor_select(ui.ctx(), edit.id, 0, rename.text.chars().count());
            rename.focused = true;
        }

        // Escape cancels; Enter or a click elsewhere keeps a changed name
        if edit.lost_focus() {
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) || rename.text == row.label {
                *renaming = None;
            } else {
                rename.done = true;
            }
        }
    } else {
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
        let galley = egui::WidgetText::from(row.label.as_str()).into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            width - 2.,
            egui::TextStyle::Body,
        );
        let top = rect.center().y - galley.size().y / 2.;
        ui.painter().galley(
            egui::pos2(rect.left() + 2., top),
            galley,
            ui.visuals().text_color(),
        );
        let response = response.on_hover_text(format!("{} · {} objects", row.label, row.count));
        record(
            controls,
            &row.key,
            &format!("Select {}", row.label),
            &response,
        );

        // egui counts a quick third click as a triple, not a double
        if row.layer && (response.double_clicked() || response.triple_clicked()) {
            *action = Some(format!("current/{index}"));
        } else if response.clicked() {
            *action = Some(if ui.input(|i| i.modifiers.shift) {
                row.key.replacen("select/", "add/", 1)
            } else {
                row.key.clone()
            });
        }

        if row.layer {
            response.context_menu(|ui| {
                layer_menu(ui, row, index, controls, action, renaming, &mut keyboard)
            });
        }
    }

    for kind in ["hide", "lock"] {
        let response = layer_icon(ui, kind, row, height);
        let verb = if kind == "hide" {
            if row.current {
                "Current"
            } else if row.hidden {
                "Show"
            } else {
                "Hide"
            }
        } else if row.locked {
            "Unlock"
        } else {
            "Lock"
        };
        record(
            controls,
            &format!("{kind}/{index}"),
            &format!("{verb} {}", row.label),
            &response,
        );

        // the current layer cannot be hidden
        if response.clicked() && !(kind == "hide" && row.current) {
            *action = Some(format!("{kind}/{index}"));
        }
    }

    layer_color(ui, row, index, height, controls, action);
    keyboard
}
// --8<-- [end:layers-row]

// --8<-- [start:layers-menu]
/// The right-click menu of a layer row; items that open a name field go to `keyboard`.
fn layer_menu(
    ui: &mut egui::Ui,
    row: &LayerRow,
    index: &str,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
    renaming: &mut Option<Rename>,
    keyboard: &mut Vec<egui::Rect>, // a tap on these raises the phone keyboard
) {
    // the top layer of a document keeps its name and place
    let root = row.root.then_some("The top layer of a document stays");

    for (key, label) in [
        ("current", "Set Current"),
        ("new_layer", "New Layer"),
        ("new_sublayer", "New Sublayer"),
    ] {
        let item = menu_item(ui, &format!("{key}/{index}"), label, None, controls, action);

        // a new layer is named right away
        if key != "current" {
            keyboard.push(item.rect);
        }
    }

    let response = ui
        .add_enabled(root.is_none(), egui::Button::new("Rename Layer"))
        .on_disabled_hover_text(root.unwrap_or_default());
    record(
        controls,
        &format!("menu-rename/{index}"),
        "Rename Layer",
        &response,
    );

    if root.is_none() {
        keyboard.push(response.rect);
    }

    if response.clicked() {
        *renaming = Some(Rename {
            node: index.to_string(),
            text: row.label.clone(),
            focused: false,
            done: false,
        });
        ui.close();
    }

    let delete = format!("delete_layer/{index}");
    let refusal = if row.current {
        Some("The current layer cannot be deleted")
    } else {
        root
    };

    // a layer with objects asks first
    if refusal.is_some() || row.count == 0 {
        menu_item(ui, &delete, "Delete Layer", refusal, controls, action);
    } else {
        let response = ui
            .menu_button("Delete Layer", |ui| {
                let confirm = "Delete Layer and Objects";
                ui.label(format!("Also deletes its {} objects", row.count));
                menu_item(ui, &delete, confirm, None, controls, action);
            })
            .response;
        record(
            controls,
            &format!("menu-delete/{index}"),
            "Delete Layer",
            &response,
        );
    }

    let duplicate = format!("duplicate_layer/{index}");
    menu_item(ui, &duplicate, "Duplicate Layer", root, controls, action);

    for (key, label) in [
        ("change_layer", "Change Object Layer"),
        ("copy_layer", "Copy Object Layer"),
    ] {
        menu_item(ui, &format!("{key}/{index}"), label, None, controls, action);
    }
}

/// One menu button, greyed with its reason when refused; a click sets `action` and closes the menu.
fn menu_item(
    ui: &mut egui::Ui,
    key: &str,
    label: &str,
    refusal: Option<&str>,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
) -> egui::Response {
    let response = ui
        .add_enabled(refusal.is_none(), egui::Button::new(label))
        .on_disabled_hover_text(refusal.unwrap_or_default());
    record(controls, key, label, &response);

    if response.clicked() {
        *action = Some(key.to_string());
        ui.close();
    }

    response
}
// --8<-- [end:layers-menu]

// --8<-- [start:layers-icon]
/// One icon of a layer row: eye, lock or arrow.
fn layer_icon(ui: &mut egui::Ui, kind: &str, row: &LayerRow, height: f32) -> egui::Response {
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(height + 4., height), egui::Sense::click());
    let scale = height / 18.;
    let c = rect.center();
    // icon coordinates are pixels from the centre of an 18 px row, scaled to the real one
    let at = |x: f32, y: f32| c + egui::vec2(x, y) * scale;
    let ink = ui.visuals().text_color();
    let stroke = egui::Stroke::new(1.4_f32, ink);

    if response.hovered() {
        ui.painter()
            .rect_filled(rect.shrink(1.), 3., ui.visuals().widgets.hovered.bg_fill);
    }

    match kind {
        "open" => {
            if let Some(open) = row.expanded {
                let points = if open {
                    vec![at(-4., -2.), at(4., -2.), at(0., 3.)]
                } else {
                    vec![at(-2., -4.), at(-2., 4.), at(3., 0.)]
                };
                ui.painter()
                    .add(egui::Shape::convex_polygon(points, ink, egui::Stroke::NONE));
            }

            response.on_hover_text("Expand or collapse")
        }
        "hide" if row.current => {
            // a check mark instead of the bulb
            ui.painter().add(egui::Shape::line(
                vec![at(-5., 0.), at(-1.5, 4.), at(5., -5.)],
                egui::Stroke::new(2_f32, egui::Color32::BLACK),
            ));

            response.on_hover_text("Current layer: new objects go here")
        }
        "hide" => {
            let fill = if row.hidden {
                egui::Color32::TRANSPARENT
            } else {
                egui::Color32::from_rgb(255, 216, 80)
            };
            ui.painter().circle(at(0., -2.), 4. * scale, fill, stroke);

            for y in [3., 5.5] {
                ui.painter().line_segment([at(-2.5, y), at(2.5, y)], stroke);
            }

            if row.hidden {
                ui.painter()
                    .line_segment([at(-6., 7.), at(6., -8.)], stroke);
            }

            response.on_hover_text(if row.hidden {
                "Show object and children"
            } else {
                "Hide object and children"
            })
        }
        _ => {
            ui.painter().rect(
                egui::Rect::from_center_size(at(0., 2.5), egui::vec2(10., 8.) * scale),
                1.,
                if row.locked {
                    egui::Color32::from_rgb(225, 180, 90)
                } else {
                    egui::Color32::TRANSPARENT
                },
                stroke,
                egui::StrokeKind::Inside,
            );
            let x = if row.locked { 0. } else { 3. };
            ui.painter().add(egui::Shape::line(
                vec![
                    at(-2.5 + x, -1.5),
                    at(-2.5 + x, -5.5),
                    at(2.5 + x, -5.5),
                    at(2.5 + x, -1.5),
                ],
                stroke,
            ));
            response.on_hover_text(if row.locked {
                "Unlock object and children"
            } else {
                "Lock selection of object and children"
            })
        }
    }
}
// --8<-- [end:layers-icon]

// --8<-- [start:layers-color]
/// The colour swatches of a layer row.
fn layer_color(
    ui: &mut egui::Ui,
    row: &LayerRow,
    index: &str,
    height: f32,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
) {
    let mut color = row.color.unwrap_or([180, 180, 180]);
    // one icon wide and unframed, so the row keeps its width and a selected strip shows through
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(height + 4., height), egui::Sense::click());

    if response.hovered() {
        ui.painter()
            .rect_filled(rect.shrink(1.), 3., ui.visuals().widgets.hovered.bg_fill);
    }

    ui.painter().rect_filled(
        egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(height * 0.5)),
        1.,
        egui::Color32::from_rgb(color[0], color[1], color[2]),
    );
    // stays open while a slider is dragged, closes on a click outside
    let menu =
        egui::Popup::menu(&response).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
    menu.show(|ui| {
        let channel_id = egui::Id::new(("layer-color-channel", index));
        // egui's temporary memory, keyed by id, remembers Faces or Edges between frames without a field of ours
        let mut edge = ui
            .ctx()
            .data_mut(|data| data.get_temp::<bool>(channel_id).unwrap_or(false))
            && row.has_faces;

        if row.has_faces {
            ui.horizontal(|ui| {
                for (label, value) in [("Faces", false), ("Edges", true)] {
                    let response = ui.selectable_label(edge == value, label);
                    record(
                        controls,
                        &format!("color-channel/{index}/{label}"),
                        label,
                        &response,
                    );

                    if response.clicked() {
                        edge = value;
                    }
                }
            });
        } else {
            ui.label("Object and child colors");
        }

        ui.ctx().data_mut(|data| data.insert_temp(channel_id, edge));
        let channel = if edge { "edge" } else { "face" };
        color = if edge { row.edge_color } else { row.color }.unwrap_or([180; 3]);
        let response = ui
            .button("Original")
            .on_hover_text("Restore the source colors for this channel and its children");
        let key = format!("color/{index}/{channel}/original");
        record(controls, &key, "Original", &response);

        if response.clicked() {
            *action = Some(key);
            ui.close();
        }

        ui.separator();
        for colors in [
            [
                ("Red", [230, 65, 55]),
                ("Orange", [240, 145, 45]),
                ("Yellow", [240, 210, 60]),
            ],
            [
                ("Green", [60, 170, 100]),
                ("Blue", [65, 130, 225]),
                ("Violet", [160, 85, 210]),
            ],
            [
                ("White", [245, 245, 245]),
                ("Gray", [150, 150, 150]),
                ("Black", [35, 35, 35]),
            ],
        ] {
            ui.horizontal(|ui| {
                for (name, rgb) in colors {
                    let response = ui
                        .add_sized(
                            [48., 28.],
                            egui::Button::new(
                                egui::RichText::new("■")
                                    .color(egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2])),
                            ),
                        )
                        .on_hover_text(name);
                    let key = format!(
                        "color/{index}/{channel}/{:02x}{:02x}{:02x}",
                        rgb[0], rgb[1], rgb[2]
                    );
                    record(controls, &key, name, &response);

                    if response.clicked() {
                        *action = Some(key);
                        ui.close();
                    }
                }
            });
        }
        ui.separator();
        let mut changed = false;

        for (channel, value) in ["R", "G", "B"].into_iter().zip(color.iter_mut()) {
            changed |= ui
                .add(egui::Slider::new(value, 0..=255).text(channel))
                .changed();
        }

        if changed {
            *action = Some(format!(
                "color/{index}/{channel}/{:02x}{:02x}{:02x}",
                color[0], color[1], color[2]
            ));
        }
    });
    let response = response.on_hover_text("Change object and child colors");
    record(
        controls,
        &format!("color/{index}"),
        &format!("Color {}", row.label),
        &response,
    );
}
// --8<-- [end:layers-color]
