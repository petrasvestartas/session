    let elements = owners.iter().filter_map(|(guid, id, _)|
        matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_))).then_some(*id))
        .collect::<std::collections::HashSet<_>>();
    candidate.forest.classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) = source.document.lookup.get(guid) {
            crate::feature_display::append(&mut candidate, *id, &source.document, element)
                .map_err(str::to_owned)?;
        }
    }
    let colours=candidate.forest.colours(key);