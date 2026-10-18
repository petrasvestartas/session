    }
}

pub fn hosted(session: &Session, element: &Element) -> Vec<Hosted> {
    let mut edges: Vec<_> = session.graph.edges.get(element.guid())
        .into_iter().flatten().collect();
    edges.sort_by_key(|(_, edge)| edge.index);
    let mut claimed = vec![false; element.features.len()];
    let mut groups = Vec::new();
    for (other, edge) in edges {
        for relation in session.interactions.get(edge.guid()).into_iter().flatten() {
            let features: Vec<_> = element.features.iter().enumerate()
                .filter(|(i, feature)| !claimed[*i] && interaction(feature) && feature.has_guid()
                    && (feature.guid() == relation.guid()
                        || relation.interaction_type_name().starts_with("InteractionFeature")
                            && feature.guid().starts_with(other.as_str())))
                .map(|(i, _)| i).collect();
            if features.is_empty() { continue; }
            for &i in &features { claimed[i] = true; }
            let name = if relation.name().is_empty() {
                source_name(session, other)
            } else { relation.name() };
            groups.push(Hosted {
                label: format!("{} {name}", relation.interaction_type_name()), features,
            });
        }
    }
    for (i, feature) in element.features.iter().enumerate() {
        if !claimed[i] && interaction(feature) {
            groups.push(Hosted { label: label(feature), features: vec![i] });
        }
    }
    groups
}
