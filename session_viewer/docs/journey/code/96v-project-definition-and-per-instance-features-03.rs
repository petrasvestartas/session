(matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_)))
            || candidate.instances.contains_key(id)).then_some(*id))