fn present_latest(
    window: &web_sys::Window, canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>, config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer, editor: &mut Editor, panel: &mut crate::panel::Panel,
    budget: Option<u32>, forced: Option<u32>, math_view:&mut crate::math_view::Presentation,
) -> Result<(), JsValue> {
    if !resize(window, canvas, surface, config, renderer, editor, budget, forced) { return Ok(()); }
    panel.update(None, canvas)?;
    let rect = canvas.get_bounding_client_rect();
    if let Some(frame) = panel.free_frame([rect.width(), rect.height()]) {
        if editor.camera.frame(frame) { renderer.cancel_pick(); let _ = editor.apply(Action::CanvasCancel); }
    }
    crate::browser_rebase::prepare(renderer, editor).map(|frame|{if frame.moved {math_view.invalidate(editor.display_scene());}frame})?;
    renderer.cloud_camera(&editor.camera);
    let pending=if editor.draft.is_some() || editor.gizmo_edit.is_some() {math_view.invalidate(editor.display_scene());false} else {math_view.refresh(renderer,editor.display_scene(),&editor.camera,[config.width,config.height],editor.whole_selection())};
        if math_view.uploaded {let view=math_view.scene(editor.display_scene());let _=renderer.set_highlight(&view,editor.selection());let _=renderer.select_objects(&view,&editor.selected_objects());}
        let _=canvas.set_attribute("data-math-display",&serde_json::json!({"pending":pending,"bytes":math_view.bytes,"generations":math_view.generations,"limited":math_view.limited,"qualities":math_view.qualities()}).to_string());
        if pending {math_view.schedule(&window,&canvas);}
    inspect(editor, renderer)?;
    present(surface, renderer, &editor.background, &renderer.camera_matrix(&editor.camera), editor.normal_view, panel)
}

fn present(
