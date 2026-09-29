                selected = camera.ray(screen).and_then(|ray| crate::picking::pick(&scene, &ray));
