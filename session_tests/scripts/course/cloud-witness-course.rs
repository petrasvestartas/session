use super::*;
use serde_json::{Value, json};

impl Lane {
    pub fn witness_snapshot(&self) -> Result<Value, &'static str> {
        let Some(key) = &self.key else {
            return Ok(json!({"prepared": false}));
        };
        let view = View {
            anchor: key.anchor.map(f64::from_bits),
            eye: key.eye.map(f32::from_bits),
            ortho_h: f32::from_bits(key.ortho),
            lod_px: f32::from_bits(key.lod),
        };
        let mut draws = Vec::new();
        for draw in &self.draws {
            if !draw.visible {
                continue;
            }
            let model = crate::render_space::placed(&draw.row.model, view.anchor).to_f32();
            let mut walk = LodWalk::default();
            walk.select(
                &Projection {
                    eye: view.eye,
                    ortho_h: view.ortho_h,
                    height_px: key.size[1],
                    lod_px: if draw.row.controls_visible {
                        0.0
                    } else {
                        view.lod_px
                    },
                    nodes: &draw.row.nodes,
                },
                &draw.row.descriptor,
                &model,
            );
            walk.ranges.retain_mut(|range| {
                let start = range.first.max(draw.row.rows.start);
                let end = (range.first + range.count).min(draw.row.rows.end);
                range.first = start;
                range.count = end.saturating_sub(start);
                range.count > 0
            });
            let (bytes, count) = record_bytes(
                draw,
                key.transform,
                f32::from_bits(key.density),
                view,
                &walk.ranges,
            )?;
            if count != draw.total {
                return Err("diagnostic replay differs from actual draw count");
            }
            let records: Vec<_> = bytes[16..].chunks_exact(160).map(|record| {
                let words: Vec<_> = record.chunks_exact(4).map(|word| u32::from_ne_bytes(word.try_into().unwrap())).collect();
                json!({
                    "mvp_model": words[..16].iter().map(|v| f32::from_bits(*v)).collect::<Vec<_>>(),
                    "minimum_radius": f32::from_bits(words[19]), "first": words[20],
                    "count": words[21], "cum": words[22], "k": f32::from_bits(words[23]),
                    "flags": words[38], "word39": words[39], "cpu_replayed_bytes": record,
                })
            }).collect();
            draws.push(json!({
                "source_rows": [draw.row.rows.start, draw.row.rows.end], "page_address": draw.row.address,
                "spacing": draw.row.descriptor.spacing, "point_px_css": draw.row.point_px,
                "resident": draw.row.descriptor.resident, "records": records,
            }));
        }
        Ok(json!({
            "prepared": true, "record_evidence": "production course record_bytes replay with retained actual draw/key; not GPU readback",
            "matrix_f32": key.transform, "viewport": key.size, "density": f32::from_bits(key.density),
            "eye": view.eye, "ortho_half_height": view.ortho_h, "lod_px": view.lod_px,
            "edl": self.edl, "samples": self.samples, "draws": draws,
        }))
    }
}
