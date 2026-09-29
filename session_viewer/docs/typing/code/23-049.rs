
/// Show a scene opened from a `.session` file.
pub(super) fn install_saved_scene(scene: super::scene::Scene) {
    LOAD_GENERATION.set(LOAD_GENERATION.get().wrapping_add(1));
    GENERATION.set(GENERATION.get().wrapping_add(1));
    reset_range_gate(); // register:stream
    RESIDENT.set(0);
    SHEET_RESIDENT.set(0);
    post(Msg::SavedScene(Box::new(scene)));
}
