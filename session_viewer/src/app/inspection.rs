//! Read-only, opt-in browser measurements (`?inspect=1`); no mutation or production UI.
#[cfg(target_arch = "wasm32")]
use crate::State;
mod source_memory;

thread_local! {
    static SOURCE_MEMORY: std::cell::RefCell<source_memory::SourceCache> = Default::default();
}

/// Publish the last submitted frame and source selection for repeatable browser checks.
#[cfg(target_arch = "wasm32")]
pub fn publish(state: &State) {
    if super::route::query("inspect").as_deref() != Some("1") {
        return;
    }
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(canvas) = document.get_element_by_id("canvas") else {
        return;
    };
    let (buffers, textures) = state.gpu.allocated_bytes();
    // The thread-local adapter forwards only; geometry traversal occurs on document changes.
    let source_memory = SOURCE_MEMORY.with_borrow_mut(|cache| cache.snapshot(&state.scene.docs));
    let parent = state.scene.selected;
    let model = match parent {
        Some(row) => state.gpu.objects.anchored_model(row),
        None => None,
    };
    let identity = selected_identity(state);
    let snapshot = serde_json::json!({
        "submitted_at_ms": crate::engine::performance::now_ms(),
        "frames": state.gpu.performance.frames,
        "draw_calls": state.gpu.performance.draws,
        "selected": parent,
        "identity": identity,
        "selection": state.selection,
        "controls": state.inspected_controls(),
        "markers": state.gpu.controls.dot_count(),
        "control_segments": state.gpu.control_net.ribbon_count(),
        "pick_busy": state.gpu.pick.busy(),
        "objects": state.scene.object_count(),
        "cloud_points": state.gpu.cloud.resident(),
        "vertices": state.gpu.arena.vert_count(),
        "pipes": state.gpu.segments.pipe_count(),
        "ribbons": state.gpu.segments.ribbon_count(),
        "mvp": state.gpu.frame.mvp_f32,
        "model": model,
        "origin": state.gpu.objects.anchor(),
        "canvas": [state.gpu.config.width, state.gpu.config.height],
        "logical_canvas": state.gpu.logical_size,
        "samples": state.gpu.targets.samples,
        "outlines": state.gpu.view.show_outlines,
        "source_cpu_known_payload_bytes": source_memory.known_bytes(),
        "source_cpu_known_payload": source_memory,
        "source_cpu_scope": "retained Session arrays/strings/values; Rc objects deduplicated; not RSS or total heap",
        "source_cpu_exclusions": "allocator/Rc/map overhead and spare map slots, private cloud LOD/capacity, GUID allocations, private nested metadata/element/BVH caches, tree/graph/component-extra payloads, streamed descriptors, upload staging and loader buffers",
        "text_cpu_raster_image_capacity_bytes": state.gpu.text.stats.raster_image_capacity_bytes,
        "text_cpu_scope": "Swash image byte-vector capacity only; font/shaper/layout/hash metadata excluded",
        "gpu_buffer_capacity_bytes": buffers,
        "gpu_texture_estimate_bytes": textures,
        "glyphon_private_gpu_capacity": "not exposed by pinned dependency; separate from totals",
        "text": state.gpu.text.stats,
        "text_labels": text_labels(state),
        "wasm_capacity_bytes": crate::engine::performance::heap_mb() * 1_048_576.0,
    });
    let _ = canvas.set_attribute("data-viewer-inspection", &snapshot.to_string());
}

/// Resolve the selected row through its owning document before copying the display GUID.
#[cfg(target_arch = "wasm32")]
fn selected_identity(state: &State) -> Option<(usize, String)> {
    let row = state.scene.selected?;
    let (document, guid) = state.scene.identity_of(row)?;
    Some((document, guid.to_string()))
}

/// Expose the actual shaped label contract for centered-nameplate pixel regressions.
#[cfg(target_arch = "wasm32")]
fn text_labels(state: &State) -> Vec<serde_json::Value> {
    use crate::engine::text::TextPlacement;
    let mut labels = Vec::new();
    for run in &state.gpu.text.document.runs {
        let (kind, world, padding) = match run.label.placement {
            TextPlacement::Screen { .. } => ("screen", None, None),
            TextPlacement::Anchor { world, .. } => ("anchor", Some(world), None),
            TextPlacement::WorldBillboard { world, .. } => ("world_billboard", Some(world), None),
            TextPlacement::WorldPlane { world, .. } => ("world_plane", Some(world), None),
            TextPlacement::Nameplate { world, padding, .. } => {
                ("nameplate", Some(world), Some(padding))
            }
        };
        let mut width = 0.0f32;
        let mut height = 0.0f32;
        for line in run.buffer.layout_runs() {
            width = width.max(line.line_w);
            height = height.max(line.line_top + line.line_height);
        }
        labels.push(serde_json::json!({
            "id": run.label.id,
            "object": run.label.object.map(|object| object.row),
            "text": run.label.text,
            "font_size": run.label.font_size,
            "line_height": run.label.line_height,
            "color": run.label.ink_color(),
            "placement": kind,
            "world": world,
            "world_height": match run.label.placement {
                TextPlacement::WorldBillboard { world_height, .. } => Some(world_height),
                _ => None,
            },
            "padding": padding,
            "rounded": matches!(run.label.placement, TextPlacement::Nameplate { rounded: true, .. }),
            "line_box": [width, height],
            "plane": match run.label.placement {
                TextPlacement::WorldPlane { right, up, world_height, .. } => Some((right, up, world_height)),
                _ => None,
            },
        }));
    }
    labels
}
