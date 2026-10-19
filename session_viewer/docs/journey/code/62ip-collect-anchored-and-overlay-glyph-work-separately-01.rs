#[path = "text_plane.rs"]
mod plane;

struct PreparedText<'a> {
    overlays: Vec<TextArea<'a>>,
    anchors: Vec<TextArea<'a>>,
    depths: HashMap<usize, f32>,
    plates: Vec<plate::Rectangle>,
    overlay: bool,
    anchored: bool,
}

fn areas<'a>(
    runs: &'a [TextRun],
    frame: &TextFrame,
    scale: f32,
    stats: &mut TextStats,
    raster_keys: &mut HashSet<glyphon::CacheKey>,
) -> PreparedText<'a> {
    let mut overlay = false;
    let mut anchored = false;
    let mut overlays = Vec::new();
    let mut anchors = Vec::new();
    let mut depths = HashMap::new();
    let mut plates = Vec::new();
    stats.new_raster_keys = 0;
    stats.requested_glyphs = 0;
    stats.missing_glyphs = 0;

    for run in runs {
        let Some(mut placed) = place(&run.label, frame, scale) else {
            continue;
        };

        if let Some(rectangle) = text_rectangle(run, &mut placed, frame, scale) {
            plates.push(rectangle);
        }

        let area = TextArea {
            buffer: &run.buffer,
            left: placed.left,
            top: placed.top,
            scale: placed.scale,
            bounds: clip_bounds(&run.label, frame, scale),
            default_color: Color::rgba(
                run.label.ink_color()[0],
                run.label.ink_color()[1],
                run.label.ink_color()[2],
                run.label.ink_color()[3],
            ),
            custom_glyphs: &[],
        };

        for line in run.buffer.layout_runs() {
            for glyph in line.glyphs {
                stats.requested_glyphs += 1;
                stats.missing_glyphs += usize::from(glyph.glyph_id == 0);
                let physical = glyph.physical((placed.left, placed.top), placed.scale);
                stats.new_raster_keys += usize::from(raster_keys.insert(physical.cache_key));
            }
        }

        if !run.label.text.is_empty() {
            match run.label.placement {
                TextPlacement::Screen { .. } | TextPlacement::Nameplate { .. } => overlay = true,
                _ => anchored = true,
            }
        }

        if let Some(depth) = placed.depth {
            depths.insert(run.label.id as usize, depth);
            anchors.push(area);
        } else {
            overlays.push(area);
        }
    }
    PreparedText {
        overlays,
        anchors,
        depths,
        plates,
        overlay,
        anchored,
    }
}
