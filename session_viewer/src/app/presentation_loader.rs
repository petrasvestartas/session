use super::post;
use crate::Msg;
use crate::app::decode::{Body, session_from_body};
use crate::app::fetch::{fetch_buffer, fetch_bytes};
use crate::app::manifest::TextItem;
use crate::app::presentation::Presentation;
use crate::app::route::{SceneRoute, join};
use crate::app::scene::FileDoc;
use session_rust::{Matrix, Text, Xform};
use std::rc::Rc;

fn number(value: f64) -> String {
    let mut text = format!("{value:.2}");
    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    if text == "-0" {
        text = "0".into();
    }
    text
}

fn matrix_items(
    session: &session_rust::Session,
    authored: &std::collections::HashSet<String>,
) -> Result<Vec<TextItem>, String> {
    let mut items = Vec::new();
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    for component in &session.objects.components {
        if component.type_name != "Matrix" {
            continue;
        }
        let matrix = Matrix::from_component(component)
            .map_err(|error| format!("invalid Matrix component '{}': {error}", component.name))?;
        if matrix.rows * matrix.cols > 64 {
            continue;
        }
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                let name = format!("{}[{row},{col}]", component.name);
                if authored.contains(&name) {
                    continue;
                }
                let local = session_rust::Point::new(
                    col as f64 * matrix.cell,
                    row as f64 * matrix.cell,
                    0.0,
                );
                let position = local.transformed(&session.world_xform(&component.guid));
                let text = number(matrix[(row, col)]);
                items.push(TextItem {
                    height: crate::app::math::matrix_value_height(&text, matrix.cell),
                    text,
                    math: true,
                    serif: true,
                    center: true,
                    center_ink: true,
                    fixed: true,
                    max_width: 0.0,
                    name,
                    outline: 0.0,
                    at: [position[0], position[1], position[2]],
                    offset: [0.0; 2],
                    right: [1.0, 0.0, 0.0],
                    up: [0.0, 1.0, 0.0],
                    camera_facing: false,
                    spatial: false,
                    plate: false,
                    color: Some([
                        channel(matrix.textcolor.r),
                        channel(matrix.textcolor.g),
                        channel(matrix.textcolor.b),
                        channel(matrix.textcolor.a),
                    ]),
                });
            }
        }
    }
    Ok(items)
}

/// Decode kernel texts and leave animated objects at their initial placement.
fn prepare_session(
    mut session: session_rust::Session,
    height: f64,
) -> Result<(session_rust::Session, Vec<TextItem>), String> {
    let animated: Vec<String> = session
        .graph
        .get_vertices()
        .into_iter()
        .filter(|vertex| vertex.attributes.contains_key("xform_ms"))
        .map(|vertex| vertex.name.clone())
        .collect();
    for guid in animated {
        session.xforms.remove(&guid);
    }
    let mut items = Vec::new();
    for component in &session.objects.components {
        if component.type_name != "Text" {
            continue;
        }
        let text = Text::from_component(component)
            .map_err(|error| format!("invalid Text component '{}': {error}", component.name))?;
        let position = text.at.transformed(&session.world_xform(&component.guid));
        let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
        items.push(TextItem {
            text: text.text,
            math: text.math,
            serif: text.serif,
            center: true,
            center_ink: true,
            fixed: text.spatial || component.name.contains('['),
            max_width: 0.0,
            name: text.name,
            outline: text.outline as f32,
            at: [
                position[0],
                if text.spatial {
                    position[1]
                } else {
                    height - position[1]
                },
                position[2],
            ],
            offset: [0.0; 2],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            height: text.height,
            camera_facing: text.spatial,
            spatial: text.spatial,
            plate: false,
            color: Some([
                channel(text.color.r),
                channel(text.color.g),
                channel(text.color.b),
                channel(text.color.a),
            ]),
        });
    }
    let authored = items.iter().map(|item| item.name.clone()).collect();
    items.extend(matrix_items(&session, &authored)?);
    Ok((session, items))
}

/// Fetch and parse the deck before installing it; a bad deck never clears the current scene.
pub async fn open_presentation(route: SceneRoute) {
    let result = match fetch_bytes(&route.manifest).await {
        Ok(bytes) => Presentation::parse(&bytes),
        Err(error) => Err(format!("cannot fetch presentation: {error}")),
    };
    match result {
        Ok(deck) => {
            post(Msg::BeginPresentation(deck, route.base));
        }
        Err(error) => super::super::feedback::error(&error),
    }
}

/// Load a topic through the requested last step, prefetching later reveals hidden.
pub fn spawn_presentation(
    deck: Presentation,
    base: String,
    topic: usize,
    from: usize,
    through: usize,
    generation: u64,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let Some(group) = deck.topics.get(topic) else {
            return;
        };
        let height = deck.height;
        for index in from..=through {
            let Some(step) = group.steps.get(index) else {
                continue;
            };
            for item in &step.items {
                let url = join(&base, &item.file);
                let doc = async {
                    let body = fetch_buffer(&url).await.map(Body::Js)?;
                    let session = session_from_body(&url, body, true).await?;
                    prepare_session(session, height)
                }
                .await;
                match doc {
                    Ok((session, texts)) => {
                        let name = if item.name.is_empty() {
                            if session.name.is_empty() {
                                item.file.clone()
                            } else {
                                session.name.clone()
                            }
                        } else {
                            item.name.clone()
                        };
                        let doc = FileDoc {
                            name,
                            session: Rc::new(session),
                            place: item.placement().unwrap_or_else(Xform::identity),
                            point_px: item.point_size as f32,
                            display_only: false,
                        };
                        if !post(Msg::PresentationPart {
                            generation,
                            topic,
                            index,
                            doc: Some(doc),
                            texts,
                            last: false,
                        }) {
                            return;
                        }
                    }
                    Err(error) => {
                        post(Msg::PresentationError {
                            generation,
                            message: format!("{} / {}: {error}", step.name, item.file),
                        });
                        return;
                    }
                }
            }
            if !post(Msg::PresentationPart {
                generation,
                topic,
                index,
                doc: None,
                texts: step.texts.clone(),
                last: true,
            }) {
                return;
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_text_becomes_flat_or_fixed_size_spatial_labels() {
        let mut session = session_rust::Session::new("text");
        let mut flat = Text::new("q=1", session_rust::Point::new(200.0, 100.0, 0.0));
        flat.name = "flat".to_string();
        flat.math = true;
        let mut spatial = Text::new("n_0", session_rust::Point::new(3.0, 4.0, 5.0));
        spatial.name = "spatial".to_string();
        spatial.math = true;
        spatial.spatial = true;
        spatial.height = 18.0;
        spatial.outline = -1.5;
        let mut entry = Text::new("1", session_rust::Point::new(20.0, 30.0, 0.0));
        entry.name = "C[0,0]".to_string();
        entry.math = true;
        session.add_text(&flat, None);
        session.add_text(&spatial, None);
        session.add_text(&entry, None);
        let (_, items) = prepare_session(session, 760.0).unwrap();

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].at, [200.0, 660.0, 0.0]);
        assert_eq!(items[0].height, 28.0);
        assert!(!items[0].fixed && !items[0].spatial && !items[0].camera_facing);
        assert_eq!(items[1].at, [3.0, 4.0, 5.0]);
        assert_eq!(items[1].height, 18.0);
        assert_eq!(items[1].outline, -1.5);
        assert!(items[1].fixed && items[1].spatial && items[1].camera_facing);
        assert!(items[2].fixed);
        assert!(items.iter().all(|item| item.center && item.center_ink));
    }

    #[test]
    fn animated_transform_does_not_move_the_initial_geometry() {
        let mut session = session_rust::Session::new("animation");
        let point = session_rust::Point::new(0.0, 0.0, 0.0);
        let guid = point.guid().to_string();
        session.add_point(point, None);
        session.animate_xform(&guid, Xform::translation(2.0, 0.0, 0.0), 900.0, 150.0);
        let (prepared, _) = prepare_session(session, 760.0).unwrap();

        assert!(!prepared.xforms.contains_key(&guid));
        assert_eq!(
            prepared.graph.vertex_attribute(&guid, "xform_ms"),
            Some(900.0)
        );
        assert_eq!(
            prepared.graph.vertex_attribute(&guid, "delay_ms"),
            Some(150.0)
        );
    }
}
