// --8<-- [start:inspection-publish]
// Inspection = a JSON snapshot of the viewer written onto the canvas, so browser tests read counters without a debugger.
#[cfg(target_arch = "wasm32")]
use crate::State;
mod source_memory; // register:release

/// Write the viewer state onto the canvas for browser tests.
#[cfg(target_arch = "wasm32")]
pub fn publish(state: &State) {
    if super::route::query("inspect").as_deref() != Some("1") {
        return; // only with ?inspect=1
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
    let source_memory = SOURCE_MEMORY.with_borrow_mut(|cache| cache.snapshot(&state.scene.docs)); // register:release
    let parent = state.scene.selected;
    let model = match parent {
        Some(row) => state.gpu.objects.anchored_model(row),
        None => None,
    };
    let identity = selected_identity(state);
    // `json!` builds a JSON value in JSON syntax; any Rust expression can fill a value slot
    let mut snapshot = serde_json::json!({
        "submitted_at_ms": crate::engine::performance::now_ms(),
        "frames": state.gpu.performance.frames,
        "draw_calls": state.gpu.performance.draws,
        "selected": parent,
        "hidden_count": state.scene.hidden.len(),
        "identity": identity,
        "selection": state.selection,
        "controls": state.inspected_controls(),
        "sheet_entity": sheet_entity(state), // register:sheets
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
        "source_cpu_known_payload_bytes": source_memory.known_bytes(), // register:release
        "source_cpu_known_payload": source_memory, // register:release
        "source_cpu_scope": "retained Session arrays/strings/values; Rc objects deduplicated; not RSS or total heap", // register:release
        "source_cpu_exclusions": "allocator/Rc/map overhead and spare map slots, private cloud LOD/capacity, GUID allocations, private nested metadata/element/BVH caches, tree/graph/component-extra payloads, streamed descriptors, upload staging and loader buffers", // register:release
        "text_cpu_raster_image_capacity_bytes": state.gpu.text.stats.raster_image_capacity_bytes,
        "text_cpu_scope": "Swash image byte-vector capacity only; font/shaper/layout/hash metadata excluded",
        "gpu_buffer_capacity_bytes": buffers,
        "gpu_texture_estimate_bytes": textures,
        "egui_private_gpu_capacity": "renderer buffers and font atlas are managed by egui; excluded from totals",
        "glyphon_private_gpu_capacity": "not exposed by pinned dependency; separate from totals",
        "text": state.gpu.text.stats,
        "text_labels": text_labels(state),
        "wasm_capacity_bytes": crate::engine::performance::heap_mb() * 1_048_576.0,
    });
    snapshot["selected_rows"] = serde_json::json!(state.selected_rows());
    snapshot["selected_models"] = serde_json::json!(
        state
            .selected_rows()
            .iter()
            .map(|r| state.gpu.objects.anchored_model(*r))
            .collect::<Vec<_>>()
    );
    snapshot["drawing"] = state.drawing_status(); // register:commands
    snapshot["tool"] = state.tool_status(); // register:tools
    snapshot["mark"] = state.mark_status(); // register:annotate
    snapshot["clipping"] = state.clipping_status(); // register:clipping
    snapshot["object_drag"] = state.object_drag_status(); // register:editing
    snapshot["number_box"] = number_box(state); // register:editing
    snapshot["undo_depth"] = undo_depth(state); // register:document
    snapshot["snap_enabled"] = serde_json::json!(state.features.snap.enabled); // register:editing
    snapshot["snap_modes"] = serde_json::json!(state.features.snap.modes); // register:editing
    snapshot["snap_bar"] = serde_json::json!(state.features.snap.bar); // register:editing
    snapshot["ssao"] = serde_json::json!(state.gpu.view.ssao);
    snapshot["locked_count"] = serde_json::json!(state.scene.locked.len());
    snapshot["color_count"] = serde_json::json!(state.scene.colors.len());
    snapshot["edge_color_count"] = serde_json::json!(state.scene.edge_colors.len());
    snapshot["current_layer"] = serde_json::json!(state.scene.current_layer()); // register:editing
    snapshot["source_faces"] =
        serde_json::json!(parent.and_then(|row| match state.scene.geometry(row)? {
            session_rust::Geometry::BRep(brep) => Some(brep.face_count()),
            session_rust::Geometry::Element(element) => match element.geometry() {
                session_rust::element::ElementGeometry::BRep(brep) => Some(brep.face_count()),
                _ => None,
            },
            _ => None,
        }));
    snapshot["selected_instance"] = // register:instancing
        serde_json::json!(parent.and_then(|row| state.scene.instance_name(row))); // register:instancing
    snapshot["selected_kind"] =
        serde_json::json!(parent.and_then(|row| state.scene.geometry(row).map(kind)));
    snapshot["selected_bounds"] = serde_json::json!(parent.and_then(|row| {
        let b = state.gpu.objects.row_bounds(row)?;
        Some([
            [b.cx - b.hx, b.cy - b.hy, b.cz - b.hz],
            [b.cx + b.hx, b.cy + b.hy, b.cz + b.hz],
        ])
    }));
    snapshot["selected_geometry"] = parent.map_or(serde_json::Value::Null, |row| shape(state, row)); // register:editing
    snapshot["scene_revision"] = serde_json::json!(state.scene.row_revision);
    let (dead_rows, free_rows, dead_bytes, graves, compactions) = state.scene.row_counters(); // register:document
    snapshot["dead_rows"] = serde_json::json!(dead_rows); // register:document
    snapshot["free_rows"] = serde_json::json!(free_rows); // register:document
    snapshot["dead_bytes"] = serde_json::json!(dead_bytes); // register:document
    snapshot["graves"] = serde_json::json!(graves); // register:document
    snapshot["compactions"] = serde_json::json!(compactions); // register:document
    let (tombs, tomb_bytes) = state.scene.tomb_counters(); // register:document
    snapshot["tombs"] = serde_json::json!(tombs); // register:document
    snapshot["tomb_bytes"] = serde_json::json!(tomb_bytes); // register:document
    snapshot["row_table_bytes"] = serde_json::json!(state.scene.row_table_bytes()); // register:document
    snapshot["preview_cache_bytes"] = serde_json::json!(state.scene.preview_cache_bytes()); // register:editing
    let docs = &state.scene.docs;
    let slots = &state.gpu.arena.source_faces.slots;
    snapshot["instancing"] = serde_json::json!({
        "definitions": docs.iter().map(|d| d.session.definition_lookup.len()).sum::<usize>(),
        "instances": docs.iter().map(|d| d.session.instance_lookup.len()).sum::<usize>(),
        "definition_uploads": state.scene.instancing.batch_rows(), // register:instancing
        "shared_instance_rows": state.scene.instancing.shared_rows(), // register:instancing
        "gpu_draws": slots.draws().len(),
        "gpu_instances": slots.instances(),
    });
    let _ = canvas.set_attribute("data-viewer-inspection", &snapshot.to_string()); // a test reads it with getAttribute
}
// --8<-- [end:inspection-publish]

// --8<-- [start:inspection-helpers]
/// Document index and guid of the selected row.
#[cfg(target_arch = "wasm32")]
fn selected_identity(state: &State) -> Option<(usize, String)> {
    let row = state.scene.selected?;
    let (document, guid) = state.scene.identity_of(row)?;
    Some((document, guid.to_string()))
}

/// Every drawn text label, as JSON.
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

/// The geometry's variant name, e.g. `BRep`.
#[cfg(target_arch = "wasm32")]
fn kind(geometry: &session_rust::Geometry) -> &'static str {
    use session_rust::Geometry;

    match geometry {
        Geometry::OBB(_) => "OBB",
        Geometry::BRep(_) => "BRep",
        Geometry::Element(_) => "Element",
        Geometry::Line(_) => "Line",
        Geometry::Mesh(_) => "Mesh",
        Geometry::NurbsCurve(_) => "NurbsCurve",
        Geometry::NurbsSurface(_) => "NurbsSurface",
        Geometry::Plane(_) => "Plane",
        Geometry::Point(_) => "Point",
        Geometry::PointCloud(_) => "PointCloud",
        Geometry::Polyline(_) => "Polyline",
    }
}
// --8<-- [end:inspection-helpers]

// --8<-- [start:16-source-memory]
// --8<-- [start:source-memory]
thread_local! {
    static SOURCE_MEMORY: std::cell::RefCell<source_memory::SourceCache> = Default::default();
}
// --8<-- [end:source-memory]
// --8<-- [end:16-source-memory]

// --8<-- [start:19-sheet-entity]
// --8<-- [start:sheet-entity]
/// The picked entity of the selected sheet, if known.
#[cfg(target_arch = "wasm32")]
fn sheet_entity(state: &State) -> Option<serde_json::Value> {
    let (id, meta) = state
        .scene
        .sheet_at(state.scene.selected?)?
        .resolved
        .as_ref()?;
    Some(serde_json::json!({"id": id, "guid": meta.guid, "name": meta.name, "kind": meta.kind}))
}
// --8<-- [end:sheet-entity]
// --8<-- [end:19-sheet-entity]

// --8<-- [start:20-undo-depth]
// --8<-- [start:undo-depth]
/// Undo steps held by every document.
#[cfg(target_arch = "wasm32")]
fn undo_depth(state: &State) -> serde_json::Value {
    serde_json::json!(
        state
            .scene
            .docs
            .iter()
            .map(|doc| doc.session.history.depth())
            .sum::<usize>()
    )
}
// --8<-- [end:undo-depth]
// --8<-- [end:20-undo-depth]

// --8<-- [start:21-edit-snapshot]
// --8<-- [start:edit-snapshot]
/// The open number box: its title, unit and place.
#[cfg(target_arch = "wasm32")]
fn number_box(state: &State) -> serde_json::Value {
    serde_json::json!(state.number_prompt().map(|prompt| {
        serde_json::json!({"title": prompt.title, "unit": prompt.unit, "at": prompt.at})
    }))
}

/// A surface's degrees, control counts, closure and world middle; a BRep's faces and solidity.
#[cfg(target_arch = "wasm32")]
fn shape(state: &State, row: u32) -> serde_json::Value {
    let place = state.scene.placement_of(row).unwrap_or_default();

    match state.scene.geometry(row) {
        Some(session_rust::Geometry::NurbsSurface(surface)) => {
            let middle = surface
                .domain(0)
                .zip(surface.domain(1))
                .and_then(|(u, v)| surface.point_at((u.0 + u.1) / 2.0, (v.0 + v.1) / 2.0));
            serde_json::json!({
                "kind": "NurbsSurface",
                "degree": [surface.degree(0), surface.degree(1)],
                "cv_count": [surface.cv_count(0), surface.cv_count(1)],
                "closed": [surface.is_closed(0), surface.is_closed(1)],
                "mid": middle.map(|p| {
                    let p = p.transformed(&place);
                    [p[0], p[1], p[2]]
                }),
            })
        }
        Some(session_rust::Geometry::BRep(brep)) => serde_json::json!({
            "kind": "BRep",
            "faces": brep.face_count(),
            "solid": brep.is_solid(),
        }),
        Some(geometry) => serde_json::json!({ "kind": kind(geometry) }),
        None => serde_json::Value::Null,
    }
}
// --8<-- [end:edit-snapshot]
// --8<-- [end:21-edit-snapshot]
