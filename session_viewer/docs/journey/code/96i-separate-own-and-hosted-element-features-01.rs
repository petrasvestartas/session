use session_rust::{Element, Geometry, Session};
use session_rust::element::ElementFeature;

#[derive(Debug, PartialEq)]
pub struct Hosted {
    pub label: String,
    pub features: Vec<usize>,
}

pub fn interaction(feature: &ElementFeature) -> bool {
    matches!(feature.feature_type.as_str(), "contact" | "joint" | "cut" | "drill")
}

pub fn own(element: &Element) -> Vec<usize> {
    element.features.iter().enumerate()
        .filter_map(|(i, feature)| (!interaction(feature)).then_some(i)).collect()
}

pub fn label(feature: &ElementFeature) -> String {
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

fn source_name<'a>(session: &'a Session, guid: &'a str) -> &'a str {
    match session.lookup.get(guid) {
        Some(Geometry::Element(element)) if !element.name.is_empty() => &element.name,
        _ => guid,
    }
}
