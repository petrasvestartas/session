//! Layout decides where a label sits - line breaks, alignment, the box it occupies - before a single pixel is drawn.
use crate::engine::text::{FONT_FAMILY, TextDocument, TextLabel, TextPlacement};
use wasm_bindgen::prelude::*;

/// Shape five sizes; return clusters and advances.
#[wasm_bindgen]
/// Shape the sample text at five sizes; returns JSON.
pub fn text_layout() -> Result<String, JsValue> {
    layout_report().map_err(js_error)
}

/// Source strings and clusters for the DOM reference.
fn layout_report() -> anyhow::Result<String> {
    let mut document = TextDocument::new();
    let mut labels = Vec::new();

    for (index, size) in [12.0, 14.0, 16.0, 18.0, 24.0].into_iter().enumerate() {
        labels.push(TextLabel {
            id: index as u32, // one per size
            text: "AVATAR To office ffi • é e\u{301} • Ø 25 ± 0.1 mm • ⚙".into(),
            font_size: size, // CSS pixels
            line_height: size * 1.5, // CSS pixels
            color: [255; 4],
            placement: TextPlacement::Screen {
                left: 0.0,
                top: 0.0,
            },
            clip: None,
        });
    }

    document.set_labels(labels.clone())?;
    let shaped = document.shape_count;

    for label in &mut labels {
        label.color = [255, 255, 0, 255];
        label.placement = TextPlacement::Screen {
            left: 10.0, // CSS pixels from the left
            top: 20.0, // CSS pixels from the top
        };
    }

    document.set_labels(labels)?;
    anyhow::ensure!(
        document.shape_count == shaped,
        "placement/color unexpectedly reshaped"
    );
    let mut rows = Vec::new();

    for run in &document.runs {
        rows.push(serde_json::json!({"id":run.label.id,"text":run.label.text,
            "size":run.label.font_size,"lineHeight":run.label.line_height}));
    }

    Ok(
        serde_json::json!({"family":FONT_FAMILY,"rows":rows,"glyphs":document.diagnostics(),
        "shapeCount":document.shape_count,"shapedBeforePlacementChange":shaped,
        "revision":document.revision,"fontRevision":document.font_revision})
        .to_string(),
    )
}

/// Preserve layout/font validation failures at the JavaScript boundary.
fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}
