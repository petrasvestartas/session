pub fn expand(shader: &str) -> String {
    let (geometry, viewport) = include_str!("projected_triangle.wgsl")
        .split_once("// Pixels per tile side for this canvas.").unwrap();
    shader.replace("#include \"projected_triangle.wgsl\"", geometry)
        .replace("#include \"projected_triangle.wgsl:viewport-span\"", viewport)
}

pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }

