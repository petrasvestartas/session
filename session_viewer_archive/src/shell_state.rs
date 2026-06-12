pub struct ShellState {
    pub egui_ctx: egui::Context,
    pub egui_renderer: egui_wgpu::Renderer,
    pub egui_state: egui_winit::State,
    pub cmd_input: String,
    pub cmd_log: Vec<String>,
    pub cmd_counter: u32,
    pub cmd_history: Vec<String>,
    pub cmd_history_idx: Option<usize>,
    pub cmd_history_saved: String,
    pub tree_search: String,
    /// Right panel visibility (collapse toggle); the 3D viewport expands when hidden.
    pub panel_visible: bool,
}
