        .replace("#include \"projected_triangle.wgsl:viewport-span\"", viewport)
        .replace("#include \"stroke_coverage.wgsl\"", include_str!("stroke_coverage.wgsl"))