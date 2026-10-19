        }
    }
    for row in scene.objects().iter().filter(|r| scene.visible(r.id)) {
        let mut seen = std::collections::HashSet::new();
        if let Some(source) = row.geometry() { for key in source.vertices() {
