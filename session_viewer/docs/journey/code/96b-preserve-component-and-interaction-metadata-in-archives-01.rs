        let header = proto::Session { name: message.name.clone(), guid: message.guid.clone(),
            tree: message.tree.clone(), graph: message.graph.clone(),
            objects:crate::source_metadata::components(message),interactions:message.interactions.clone(),
            definitions:crate::source_metadata::unused_definitions(message),
            bvh_boxes: message.bvh_boxes.clone(), xforms: message.xforms.clone(), ..Default::default() };
        Self { id: uuid::Uuid::new_v4(), header, version: FileVersion::of(bytes), location: None }
