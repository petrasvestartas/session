    let mut nearest = ray.max_distance;
    let mut hit = None;
    for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
        let vertices = object.mesh.vertices();
        for triangle in object.mesh.indices().chunks_exact(3) {
