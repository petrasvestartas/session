editor:&mut crate::editor::Editor)->Result<crate::render_space::Rebase,JsValue>{
    renderer.grow_camera(&mut editor.camera);