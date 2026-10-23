pub use crate::production_ribbon::{floats, line_uniform, pick_frame};
use crate::{
    pick_sources::{Part, Sources, Target},
    production_curve_walk as walk,
    scene::{ObjectId, Scene},
};
pub const SOURCE: &str = include_str!("production_joined.wgsl");
pub const PROVENANCE: &str = include_str!("production_vector_provenance.json");

#[derive(Default)]
pub struct Rows {
    pub segments: Vec<u8>,
    pub objects: Vec<u8>,
    pub translations: Vec<u8>,
    pub owners: Vec<u32>,
    pub ids: Vec<[u32; 2]>,
    pub edges: Vec<u32>,
    pub heads: Vec<u8>,
    pub object_ids: Vec<ObjectId>,
    pub pick_owners: Vec<u32>,
    pub line_count: usize,
}

pub fn object(
    scene: &Scene,
    id: ObjectId,
    model: &session_rust::Xform,
    anchor: [f64; 3],
    selected: bool,
    flags: u32,
) -> (Vec<u8>, Vec<u8>) {
    let mut local = model.to_f32();
    local[12..15].fill(0.0);
    let color = scene.face_rgba(id);
    let mut bytes = Vec::with_capacity(96);
    floats(&mut bytes, local);
    floats(&mut bytes, color.unwrap_or([1.0; 4]));
    bytes.extend(
        (flags | u32::from(selected) | if color.is_some() { 256 } else { 0 }).to_ne_bytes(),
    );
    bytes.extend([0; 12]);
    let mut translation = Vec::with_capacity(16);
    floats(
        &mut translation,
        (0..4).map(|i| {
            if i < 3 {
                (model.m[12 + i] - anchor[i]) as f32
            } else {
                0.0
            }
        }),
    );
    (bytes, translation)
}

pub fn segment_bytes(segment: &walk::StrokeSegment, out: &mut Vec<u8>) {
    let s = &segment.segment;
    floats(out, s.p0);
    floats(out, [s.radius]);
    floats(out, s.p1);
    out.extend(
        [
            s.instance_id,
            s.color,
            s.facing,
            segment.previous,
            segment.next,
        ]
        .into_iter()
        .flat_map(u32::to_ne_bytes),
    );
    floats(out, [segment.sag]);
}
pub fn head_bytes(head: &walk::VectorRow, out: &mut Vec<u8>) {
    floats(out, head.start);
    floats(out, [head.radius]);
    floats(out, head.end);
    out.extend(head.instance_id.to_ne_bytes());
    out.extend(head.color.to_ne_bytes());
    floats(out, [head.head]);
    out.extend(head.heads.to_ne_bytes());
    out.extend(head.pad.to_ne_bytes());
}
pub fn source_points(source: &crate::chain::Source) -> Vec<[f64; 3]> {
    match source {
        crate::chain::Source::Curve(curve, _) => walk::sample_nurbscurve(curve),
        _ => source.coordinates(),
    }
}
fn pick(
    scene: &Scene,
    sources: &Sources,
    id: ObjectId,
    part: Part,
) -> Result<[u32; 2], &'static str> {
    let owner = crate::feature_owner::display_owner(scene, id).map_or(0, ObjectId::value);
    if scene.forest.derived(id) {
        return Ok([owner, 0]);
    }
    if part == Part::Object {
        return Ok([owner, crate::sub_id::OBJECT_INK]);
    }
    let index = sources
        .edges
        .iter()
        .position(|t| *t == Target { owner: id, part })
        .ok_or("Missing original joined edge address")?;
    Ok([
        owner,
        crate::sub_id::encode(crate::sub_id::Kind::Edge, index)?,
    ])
}
pub fn rows(
    scene: &Scene,
    sources: &Sources,
    anchor: [f64; 3],
    selected: &[ObjectId],
    limit: u64,
) -> Result<Rows, &'static str> {
    let mut out = Rows::default();
    let mut segments = walk::SegRows::default();
    let mut heads = walk::VectorRows::default();
    let add = |out: &mut Rows, id: ObjectId, model: &session_rust::Xform, flags: u32| {
        let (object, translation) = object(scene, id, model, anchor, selected.contains(&id), flags);
        out.objects.extend(object);
        out.translations.extend(translation);
        out.object_ids.push(id);
        out.pick_owners
            .push(crate::feature_owner::display_owner(scene, id).map_or(0, ObjectId::value));
    };
    for row in scene.lines().iter().filter(|r| scene.visible(r.id)) {
        let index = out.object_ids.len() as u32;
        let result = walk::walk_line(&mut segments, &mut heads, &row.prepared.source, index);
        add(&mut out, row.id, &row.model, result.flags);
        out.ids.push(pick(scene, sources, row.id, Part::Curve)?);
        out.edges.push(u32::MAX);
        out.owners.push(row.id.value());
    }
    out.line_count = out.owners.len();
    for row in scene.paths().iter().filter(|r| scene.visible(r.id)) {
        let index = out.object_ids.len() as u32;
        let first = segments.ribbons.len();
        let result = match &row.prepared.source {
            crate::chain::Source::Line(source) => {
                walk::walk_line(&mut segments, &mut heads, source, index)
            }
            crate::chain::Source::Polyline(source) => {
                walk::walk_polyline(&mut segments, &mut heads, source, index)
            }
            crate::chain::Source::Curve(source, _) => {
                walk::walk_nurbscurve(&mut segments, &mut heads, source, index)
            }
        };
        add(&mut out, row.id, &row.model, result.flags);
        if let crate::chain::Source::Polyline(source) = &row.prepared.source {
            let points: Vec<_> = source
                .coords
                .chunks_exact(3)
                .map(|p| [p[0] as f32, p[1] as f32, p[2] as f32])
                .collect();
            for (edge, _) in points.windows(2).enumerate().filter(|(_, p)| p[0] != p[1]) {
                out.ids
                    .push(pick(scene, sources, row.id, Part::PathEdge(edge))?);
                out.edges.push(edge as u32);
            }
        } else {
            for _ in first..segments.ribbons.len() {
                out.ids.push(pick(scene, sources, row.id, Part::Curve)?);
                out.edges.push(u32::MAX);
            }
        }
        out.owners.extend(std::iter::repeat_n(
            row.id.value(),
            segments.ribbons.len() - first,
        ));
    }
    for row in scene
        .objects()
        .iter()
        .filter(|r| scene.visible(r.id) && r.cad.as_ref().is_some_and(|c| !c.wires.is_empty()))
    {
        let cad = row.cad.as_ref().unwrap();
        let index = out.object_ids.len() as u32;
        add(&mut out, row.id, &row.model, 0);
        let color = scene
            .view
            .edges
            .get(&row.id)
            .map_or([0.0, 0.0, 0.0, 1.0], |c| {
                [
                    c[0] as f32 / 255.0,
                    c[1] as f32 / 255.0,
                    c[2] as f32 / 255.0,
                    1.0,
                ]
            });
        let pen = walk::Pen {
            row: index,
            radius: crate::production_ribbon::encode_width(cad.source.width),
            color: crate::production_ribbon::pack_rgba(color),
        };
        for wire in &cad.wires {
            let edge = &cad.source.m_edges[wire.edge];
            if edge.degenerated || edge.curve_3d_index < 0 { continue; }
            let points: Vec<_> = walk::sample_nurbscurve(&cad.source.m_curves_3d[edge.curve_3d_index as usize])
                .into_iter().map(walk::render_position).collect();
            let first = segments.ribbons.len();
            walk::push_polyline(
                &mut segments,
                &points,
                &pen,
                &mut session_rust::AABB::empty(),
            );
            for _ in first..segments.ribbons.len() {
                out.ids.push(pick(scene, sources, row.id, Part::Object)?);
                out.edges.push(u32::MAX);
                out.owners.push(row.id.value());
            }
        }
    }
    for row in walk::joined_rows(
        &segments.ribbons,
        &segments.ribbon_chains,
        &segments.ribbon_heads,
        0,
        &[],
    ) {
        segment_bytes(&row, &mut out.segments);
    }
    for head in &heads.rows {
        head_bytes(head, &mut out.heads);
    }
    if out.ids.len() != out.owners.len() {
        return Err("Joined source IDs do not match production rows");
    }
    if [&out.segments, &out.objects, &out.translations, &out.heads]
        .iter()
        .any(|b| b.len() as u64 > limit)
        || out.ids.len() as u64 * 8 > limit
    {
        return Err("Production joined rows exceed the storage limit");
    }
    Ok(out)
}
