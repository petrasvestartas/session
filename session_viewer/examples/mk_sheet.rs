use std::collections::BTreeMap;
use std::path::Path;

use prost::Message;
use session_rust::{Color, Geometry, NurbsCurve, Session, Xform, proto};

const CHORD_DEGREES: f64 = 5.0;

struct Sheet {
    coords: Vec<f64>,
    colors: Vec<u32>,
    widths: Vec<f32>,
    ids: Vec<u32>,
    meta: Vec<Vec<u8>>,
}

fn quant8(v: f32) -> u32 {
    ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff
}

fn pack_rgba(c: [f32; 4]) -> u32 {
    quant8(c[0]) | quant8(c[1]) << 8 | quant8(c[2]) << 16 | quant8(c[3]) << 24
}

fn publish_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 {
        w as f32
    } else {
        0.0
    }
}

fn control_position(c: &NurbsCurve, i: usize) -> Option<[f64; 3]> {
    let p = c.cv(i)?;
    let w = if c.m_is_rat && p.len() > 3 && p[3] != 0.0 {
        p[3]
    } else {
        1.0
    };
    Some([p[0] / w, p[1] / w, p[2] / w])
}

fn turning_degrees(c: &NurbsCurve) -> f64 {
    let mut total = 0.0;
    let mut prev: Option<[f64; 3]> = None;
    for i in 1..c.m_cv_count {
        let (Some(a), Some(b)) = (control_position(c, i - 1), control_position(c, i)) else {
            continue;
        };
        let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if len < 1e-12 {
            continue;
        }
        let u = [d[0] / len, d[1] / len, d[2] / len];
        if let Some(q) = prev {
            let dot = (q[0] * u[0] + q[1] * u[1] + q[2] * u[2]).clamp(-1.0, 1.0);
            total += dot.acos().to_degrees();
        }
        prev = Some(u);
    }
    total
}

fn sample_nurbscurve(c: &NurbsCurve) -> Vec<f64> {
    if c.m_cv_count < 2 {
        return Vec::new();
    }
    let spans = c.span_count().max(1);
    let n = ((turning_degrees(c) / CHORD_DEGREES).ceil() as usize).clamp(spans, 512);
    let (t0, t1) = c.domain();
    let mut pts = Vec::with_capacity((n + 1) * 3);
    for i in 0..=n {
        let p = c.point_at(t0 + (t1 - t0) * i as f64 / n as f64);
        pts.extend_from_slice(&[p[0], p[1], p[2]]);
    }
    pts
}

impl Sheet {
    fn push(&mut self, pts: &[f64], guid: &str, name: &str, kind: &str, width: f64, color: &Color) {
        let id = self.meta.len() as u32;
        let rgba = pack_rgba(color.to_f32());
        let w = publish_width(width);
        for s in pts.windows(6).step_by(3) {
            self.coords.extend_from_slice(s);
            self.colors.push(rgba);
            self.widths.push(w);
            self.ids.push(id);
        }
        let record = serde_json::json!({
            "guid": guid, "name": name, "kind": kind, "width": width,
            "color": [color.r, color.g, color.b, color.a],
        });
        self.meta.push(record.to_string().into_bytes());
    }

    fn meta_bytes(&self) -> Vec<u8> {
        let mut out = b"SHM1".to_vec();
        out.extend_from_slice(&(self.meta.len() as u32).to_le_bytes());
        let mut offset = 0u64;
        for blob in &self.meta {
            out.extend_from_slice(&offset.to_le_bytes());
            out.extend_from_slice(&(blob.len() as u64).to_le_bytes());
            offset += blob.len() as u64;
        }
        for blob in &self.meta {
            out.extend_from_slice(blob);
        }
        out
    }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let src = a.first().expect("mk_sheet <in.pb> <out.pb>");
    let out = Path::new(a.get(1).expect("mk_sheet <in.pb> <out.pb>"));
    let meta_path = out.with_extension("meta");
    let meta_name = meta_path.file_name().unwrap().to_string_lossy().to_string();

    let session = Session::pb_load(src);
    let world = session.world_xforms();
    let identity = Xform::identity();
    let mut sheet = Sheet {
        coords: Vec::new(),
        colors: Vec::new(),
        widths: Vec::new(),
        ids: Vec::new(),
        meta: Vec::new(),
    };
    let mut skipped: BTreeMap<&str, usize> = BTreeMap::new();
    for guid in session.order() {
        let xf = world.get(&guid).unwrap_or(&identity);
        match &session.lookup[&guid] {
            Geometry::Line(l) => {
                let l = l.transformed(xf);
                let pts = [l[0], l[1], l[2], l[3], l[4], l[5]];
                sheet.push(&pts, &guid, &l.name, "line", l.width, &l.linecolor);
            }
            Geometry::Polyline(p) => {
                let p = p.transformed(xf);
                sheet.push(&p.coords, &guid, &p.name, "polyline", p.width, &p.linecolor);
            }
            Geometry::NurbsCurve(c) => {
                let c = c.transformed(xf);
                let color = c
                    .linecolors
                    .first()
                    .cloned()
                    .unwrap_or(Color::new(0.0, 0.0, 0.0, 1.0));
                sheet.push(
                    &sample_nurbscurve(&c),
                    &guid,
                    &c.name,
                    "nurbscurve",
                    c.width,
                    &color,
                );
            }
            g => *skipped.entry(kind_name(g)).or_default() += 1,
        }
    }

    let segments = sheet.ids.len();
    let entities = sheet.meta.len();
    let meta = sheet.meta_bytes();
    let pb = proto::Session {
        name: session.name.clone(),
        guid: session.guid().to_string(),
        objects: Some(proto::Objects {
            sheets: vec![proto::Sheet {
                guid: session_rust::Objects::new().guid().to_string(),
                name: session.name.clone(),
                coords: sheet.coords,
                colors: sheet.colors,
                widths: sheet.widths,
                segment_count: segments as u32,
                entity_count: entities as u32,
                meta: meta_name,
                source_ids: sheet.ids,
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
    .encode_to_vec();
    std::fs::write(out, &pb).expect("write pb");
    std::fs::write(&meta_path, &meta).expect("write meta");
    let skipped: Vec<String> = skipped.iter().map(|(k, n)| format!("{k} {n}")).collect();
    println!(
        "{}: {segments} segments, {entities} entities, {} B pb, {} B meta, skipped [{}]",
        out.display(),
        pb.len(),
        meta.len(),
        skipped.join(", ")
    );
}

fn kind_name(g: &Geometry) -> &'static str {
    match g {
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

/*
description: flatten every Line, Polyline and NurbsCurve of a session into one range-addressable Sheet plus its .meta side table.

directory: cd ~/code/code_cpp/wood_research/session/session_viewer
run: cargo run --example mk_sheet --target x86_64-unknown-linux-gnu --release -- assets/pb/view_local_sheet_querschnitt.pb /tmp/sheets/querschnitt.pb
view: https://petrasvestartas.github.io/session/
*/
