impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

