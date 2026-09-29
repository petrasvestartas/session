
/// Shared WGSL for ink: the visibility test and the projected triangles it reads.
pub const INK: &str = shader!("ink_visibility.wgsl");

/// An ink shader's full text: its own code, the ink code, then the prelude.
pub fn ink_source(source: &str) -> String {
    scene_source(&format!("{source}\n{INK}"))
}

/// An ink shader: scene code plus the ink code.
pub fn ink_module(ctx: &GpuCtx, label: &str, source: &str) -> Shader {
    module(ctx, label, &ink_source(source))
}
