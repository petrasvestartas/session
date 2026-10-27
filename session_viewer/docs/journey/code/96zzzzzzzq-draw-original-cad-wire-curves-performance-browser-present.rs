fn present_latest(
    window: &web_sys::Window, canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>, config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer, editor: &mut Editor, panel: &mut crate::panel::Panel,
    budget: Option<u32>, forced: Option<u32>,
) -> Result<(), JsValue> {
    if !resize(window, canvas, surface, config, renderer, editor, budget, forced) { return Ok(()); }
    panel.update(None, canvas)?;
    let rect = canvas.get_bounding_client_rect();
    if let Some(frame) = panel.free_frame([rect.width(), rect.height()]) {
        if editor.camera.frame(frame) { renderer.cancel_pick(); let _ = editor.apply(Action::CanvasCancel); }
    }
    crate::browser_rebase::prepare(renderer, editor)?;
    renderer.cloud_camera(&editor.camera);
    inspect(editor, renderer)?;
    present(surface, renderer, &editor.background, &renderer.camera_matrix(&editor.camera), editor.normal_view, panel)
}

fn present(
