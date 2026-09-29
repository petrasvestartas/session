fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
) -> Result<(), JsValue> {
