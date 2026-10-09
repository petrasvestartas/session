//! What other elements do to an element, read from the session's interactions: a line per
//! interaction the element hosts, its full type name first, holding the features it drew there.
use session_rust::element::{Element, ElementFeature};
use session_rust::{Geometry, Session};

/// One interaction an element hosts: its label and the element's features it drew.
#[derive(Debug, PartialEq)]
pub struct Hosted {
    pub label: String, // the full type name, then the interaction's name or its source's
    pub features: Vec<usize>, // indices into the element's features
}

/// True for a feature another element puts on this one: a contact, a joint, a cut or a drill; the rest, an axis or a section, are the element's own attributes.
pub fn is_interaction(feature: &ElementFeature) -> bool {
    matches!(
        feature.feature_type.as_str(),
        "contact" | "joint" | "cut" | "drill"
    )
}

/// The interactions element `guid` hosts, in edge order, each with the features it drew; an interaction feature no interaction claims gets a line of its own.
pub fn hosted(session: &Session, guid: &str, element: &Element) -> Vec<Hosted> {
    let features = element.features();
    let mut claimed = vec![false; features.len()];
    let mut out = Vec::new();

    // the edges in the order they were made, so the lines keep their order
    let mut edges: Vec<_> = session.graph.edges.get(guid).into_iter().flatten().collect();
    edges.sort_by_key(|(_, edge)| edge.index);

    for (other, edge) in edges {
        let Some(interactions) = session.interactions.get(edge.guid()) else {
            continue;
        };

        for interaction in interactions {
            let id = interaction.guid();
            // a contact draws under its own guid, a joint's cut and drills under its source's
            let mine = |f: &ElementFeature| {
                f.guid() == id
                    || (interaction
                        .interaction_type_name()
                        .starts_with("InteractionFeature")
                        && f.guid().starts_with(other.as_str()))
            };
            let drawn: Vec<usize> = (0..features.len())
                .filter(|&i| !claimed[i] && is_interaction(&features[i]) && mine(&features[i]))
                .collect();

            if drawn.is_empty() {
                continue;
            }

            for &i in &drawn {
                claimed[i] = true;
            }

            let name = if interaction.name().is_empty() {
                element_name(session, other)
            } else {
                interaction.name()
            };
            out.push(Hosted {
                label: format!("{} {name}", interaction.interaction_type_name()),
                features: drawn,
            });
        }
    }

    // features of an interaction the file does not hold
    for (i, feature) in features.iter().enumerate() {
        if !claimed[i] && is_interaction(feature) {
            out.push(Hosted {
                label: feature_label(feature),
                features: vec![i],
            });
        }
    }

    out
}

/// The element's own features, the ones no other element put there.
pub fn own(element: &Element) -> Vec<usize> {
    (0..element.features().len())
        .filter(|&i| !is_interaction(&element.features()[i]))
        .collect()
}

/// A feature's line: its type, its name when it adds one, and its face.
pub fn feature_label(feature: &ElementFeature) -> String {
    let mut label = feature.feature_type.clone();

    if !feature.name.is_empty() && feature.name != feature.feature_type {
        label.push(' ');
        label.push_str(&feature.name);
    }

    if feature.face_index >= 0 {
        label.push_str(&format!(" · face {}", feature.face_index));
    }

    label
}

/// The name of the object `guid`, or the guid when it has none.
fn element_name<'a>(session: &'a Session, guid: &'a str) -> &'a str {
    match session.lookup.get(guid) {
        Some(Geometry::Element(element)) if !element.name.is_empty() => &element.name,
        _ => guid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Mesh, Point, Polyline};

    /// A contact and a joint's drills hang under the interactions they came from, each line led by the full type; the axis is the element's own.
    #[test]
    fn features_group_under_their_interactions() {
        let mut session = Session::new("floor");
        let mut rib = Element::new("rib");
        rib.set_geometry(Mesh::create_box(1.0, 1.0, 1.0));
        let line = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
        let mut connector = Element::new("connector_column_plate_0");
        connector.set_geometry(Mesh::create_box(1.0, 1.0, 1.0));
        let connector_guid = connector.guid().to_string();
        let contact = session_rust::interaction::InteractionUnknown::new(
            "InteractionContactFace",
            &[],
            "column_plate_0_0",
        );
        let contact_guid = session_rust::Interaction::guid(&contact).to_string();
        let mut touch = ElementFeature::new("contact", 9, vec![line.clone()], "side_side");
        touch.set_guid(contact_guid);
        rib.add_feature(ElementFeature::new("axis", -1, vec![line.clone()], "axis"));
        rib.add_feature(touch);

        for hole in 0..2 {
            let mut drill = ElementFeature::new(
                "drill",
                -1,
                vec![line.clone()],
                "connector_column_plate_0 d50",
            );
            drill.set_guid(format!("{connector_guid}-drill-{hole}"));
            rib.add_feature(drill);
        }

        let rib_guid = rib.guid().to_string();
        let column = session
            .add_element(Element::new("column"), None)
            .borrow()
            .name
            .clone();
        session.add_element(rib.clone(), None);
        session.add_element(connector, None);
        session.graph.add_edge(&column, &rib_guid, "");
        session.graph.add_edge(&connector_guid, &rib_guid, "");
        let edge = |a: &str, b: &str| session.graph.edges[a][b].guid().to_string();
        let (touching, cutting) = (edge(&column, &rib_guid), edge(&connector_guid, &rib_guid));
        session
            .interactions
            .insert(touching, vec![Box::new(contact)]);
        session.interactions.insert(
            cutting,
            vec![Box::new(
                session_rust::interaction::InteractionUnknown::new(
                    "InteractionFeatureSolid",
                    &[],
                    "",
                ),
            )],
        );

        assert_eq!(
            hosted(&session, &rib_guid, &rib),
            vec![
                Hosted {
                    label: "InteractionContactFace column_plate_0_0".into(),
                    features: vec![1]
                },
                Hosted {
                    label: "InteractionFeatureSolid connector_column_plate_0".into(),
                    features: vec![2, 3]
                },
            ]
        );
        assert_eq!(own(&rib), vec![0]);
        assert_eq!(
            feature_label(&rib.features()[1]),
            "contact side_side · face 9"
        );
    }
}
