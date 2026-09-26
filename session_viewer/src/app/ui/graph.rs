use super::layers::SELECTED;
use super::{Control, Model, record};

/// The graph section: a header folding a table of edges; a row click selects both objects.
pub(super) fn show(
    ui: &mut egui::Ui,
    model: &Model,
    height: f32,
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    action: &mut Option<String>,
) {
    let (rect, header) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height + 4.),
        egui::Sense::click(),
    );
    let ink = ui.visuals().text_color();
    let c = rect.left_center() + egui::vec2(8., 0.);
    // the same arrow as a tree row
    let points = if model.graph_open {
        vec![
            c + egui::vec2(-4., -2.),
            c + egui::vec2(4., -2.),
            c + egui::vec2(0., 3.),
        ]
    } else {
        vec![
            c + egui::vec2(-2., -4.),
            c + egui::vec2(-2., 4.),
            c + egui::vec2(3., 0.),
        ]
    };
    ui.painter()
        .add(egui::Shape::convex_polygon(points, ink, egui::Stroke::NONE));
    let title = format!("Graph · {} edges", model.edge_total);
    ui.painter().text(
        rect.left_center() + egui::vec2(18., 0.),
        egui::Align2::LEFT_CENTER,
        &title,
        egui::TextStyle::Body.resolve(ui.style()),
        ink,
    );
    record(controls, "graph/toggle", &title, &header);

    if header.clicked() {
        *action = Some("graph/toggle".into());
    }

    if !model.graph_open {
        return;
    }

    let half = ui.available_width() / 2.;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );

    for (title, left) in [("From", rect.left()), ("To", rect.left() + half)] {
        ui.painter().text(
            egui::pos2(left + 2., rect.center().y),
            egui::Align2::LEFT_CENTER,
            title,
            egui::TextStyle::Body.resolve(ui.style()),
            egui::Color32::from_gray(110),
        );
    }

    egui::ScrollArea::vertical()
        .id_salt("graph-edges")
        .auto_shrink([false, false])
        .show_rows(ui, height, model.edges.len(), |ui, range| {
            ui.spacing_mut().item_spacing.y = 0.;

            for edge in &model.edges[range] {
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), height),
                    egui::Sense::click(),
                );

                if edge.selected {
                    ui.painter().rect_filled(rect, 0., SELECTED);
                }

                // from and to, each truncated to its column
                for (text, left) in [(&edge.from, rect.left()), (&edge.to, rect.left() + half)] {
                    let galley = egui::WidgetText::from(text.as_str()).into_galley(
                        ui,
                        Some(egui::TextWrapMode::Truncate),
                        half - 6.,
                        egui::TextStyle::Body,
                    );
                    let top = rect.center().y - galley.size().y / 2.;
                    ui.painter().galley(
                        egui::pos2(left + 2., top),
                        galley,
                        ui.visuals().text_color(),
                    );
                }

                let response = response.on_hover_text(&edge.guids);
                record(
                    controls,
                    &edge.key,
                    &format!("{} → {}", edge.from, edge.to),
                    &response,
                );

                if response.clicked() {
                    *action = Some(edge.key.clone());
                }
            }
        });
}
