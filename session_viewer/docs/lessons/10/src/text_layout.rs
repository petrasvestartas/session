//! A check on shaping alone, before any GPU text: one string at five sizes, sent to text-layout.html as JSON.
use crate::engine::text::{FONT_FAMILY, TextDocument, TextLabel, TextPlacement};
use wasm_bindgen::prelude::*;

/// JavaScript calls this as `module.text_layout()`; an Err arrives there as a thrown error.
#[wasm_bindgen]
pub fn text_layout() -> Result<String, JsValue> {
    layout_report().map_err(js_error)
}

/// Shape once, then move and recolor every label: the shape count must stay the same.
fn layout_report() -> anyhow::Result<String> {
    let mut document = TextDocument::new();
    let mut labels = Vec::new();

    for (index, size) in [12.0, 14.0, 16.0, 18.0, 24.0].into_iter().enumerate() {
        labels.push(TextLabel {
            id: index as u32,
            // kerning (AV, To), a ligature (ffi), é typed two ways, and ⚙ from a fallback font
            text: "AVATAR To office ffi • é e\u{301} • Ø 25 ± 0.1 mm • ⚙".into(),
            font_size: size,
            line_height: size * 1.5,
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

    // color and position are not part of the shape, so this must reuse it
    for label in &mut labels {
        label.color = [255, 255, 0, 255];
        label.placement = TextPlacement::Screen {
            left: 10.0,
            top: 20.0,
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

/// JavaScript receives the error message as a plain string.
fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}
