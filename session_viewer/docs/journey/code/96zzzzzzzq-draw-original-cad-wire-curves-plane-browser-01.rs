fn display(renderer:&mut Renderer,editor:&Editor)->Result<(),&'static str>{
    renderer.set_clipping(editor.clipping_enabled,editor.clipping_solid);
    renderer.set_opacity(editor.opacity);renderer.set_lighting(editor.lit,editor.backfaces);
    renderer.set_ambient(editor.arctic);renderer.set_outlines(editor.outlines);renderer.set_plane_size(editor.plane_size).expect("Validated plane size");renderer.set_ink_visibility(editor.ink_visibility);
    renderer.set_grid_on(editor.camera.grid,editor.grid_plane())
}

