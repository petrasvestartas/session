impl Default for Editor {
    fn default() -> Self {
        Self {
            ink_visibility:Default::default(),element:None,element_attributes:false,element_interactions:true,opacity:1.,arctic:false,lit:false,backfaces:false,outlines:false,plane_size:crate::production_plane::DEFAULT_SIZE,display:Default::default(),clipping_enabled:true,clipping_solid:true,interaction_epoch:0,lasso:None,trim:None,last_text_height:0.,mark:None,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
