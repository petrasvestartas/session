shader.replace("#include \"near_clip.wgsl\"", include_str!("near_clip.wgsl"))
        .replace("#include \"projected_triangle.wgsl\"", geometry)