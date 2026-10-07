pub fn prepare(renderer: &mut crate::renderer::Renderer, scene: &crate::scene::Scene,
    selected: Option<crate::scene::ObjectId>, fault: crate::gpu_fault::Fault,
) {
    let started = crate::browser_phase::now();
    let before = renderer.uploaded_bytes();
    renderer.set_scene(scene, selected);
    let bytes = renderer.uploaded_bytes() - before;
    crate::browser_phase::finish("upload preparation", started, bytes, "GPU");
    if bytes == 0 { return; }
    let submitted = crate::browser_phase::now();
    renderer.queue.submit([]);
    renderer.queue.on_submitted_work_done(move || {
        if crate::browser_runtime::owns(&fault) && fault.message().is_none() {
            crate::browser_phase::finish("GPU upload ready", submitted, bytes, "GPU");
        }
    });
}
