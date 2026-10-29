use session_rust::{AABB,Arrowhead,Line,NurbsCurve,Polyline};
use crate::production_ribbon::{encode_width,pack_rgba};
pub const FACING_UNKNOWN:u32=u32::MAX;
pub const HEAD_MARK:u32=u32::MAX-1;
pub struct Pen {pub row:u32,pub radius:f32,pub color:u32}
pub struct Instance;
impl Instance {pub const FLAG_HEADS:u32=32768;}
pub struct Row {pub flags:u32,pub bounds:AABB}
impl Row {fn thin(bounds:AABB)->Self {Self{flags:0,bounds}}}
#[repr(C)]
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CylinderSegment {pub p0:[f32;3],pub radius:f32,pub p1:[f32;3],pub instance_id:u32,pub color:u32,pub facing:u32}
#[repr(C)]
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct StrokeSegment {pub segment:CylinderSegment,pub previous:u32,pub next:u32,pub sag:f32}
#[repr(C)]
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct VectorRow {pub start:[f32;3],pub radius:f32,pub end:[f32;3],pub instance_id:u32,pub color:u32,pub head:f32,pub heads:u32,pub pad:u32}
impl VectorRow {pub const HEAD_END:u32=1;pub const HEAD_START:u32=2;pub const HEAD_ONLY:u32=4;}
#[derive(Default)]
pub struct VectorRows {pub rows:Vec<VectorRow>}
#[derive(Default)]
pub struct SegRows {pub ribbons:Vec<CylinderSegment>,pub ribbon_chains:Vec<std::ops::Range<u32>>,pub ribbon_heads:Vec<(u32,u32)>}
const _:()={assert!(std::mem::size_of::<CylinderSegment>()==40);assert!(std::mem::size_of::<StrokeSegment>()==52);assert!(std::mem::size_of::<VectorRow>()==48);};
/// One segment per pair of neighbours; grows `bounds`.
pub(crate) fn push_polyline(seg: &mut SegRows, pts: &[[f32; 3]], pen: &Pen, bounds: &mut AABB) {
    let first = seg.ribbons.len() as u32;
    seg.ribbons.reserve(pts.len().saturating_sub(1));

    for w in pts.windows(2) {
        // skip a repeated point
        if w[0] == w[1] {
            continue;
        }
        bounds.union_with_point(w[0][0] as f64, w[0][1] as f64, w[0][2] as f64);
        seg.ribbons.push(CylinderSegment {
            p0: w[0],
            radius: pen.radius,
            p1: w[1],
            instance_id: pen.row,
            color: pen.color,
            facing: FACING_UNKNOWN, // no face orientation
        });
    }

    seg.ribbon_chains.push(first..seg.ribbons.len() as u32); // one joined stroke

    if let Some(last) = pts.last() {
        bounds.union_with_point(last[0] as f64, last[1] as f64, last[2] as f64);
    }
}

/// A head-only vector row at the end of `segment`, aimed along it.
fn head_row(segment: &CylinderSegment) -> VectorRow {
    VectorRow {
        start: segment.p0,
        radius: segment.radius,
        end: segment.p1,
        instance_id: segment.instance_id,
        color: segment.color,
        head: 0.0,
        heads: VectorRow::HEAD_END | VectorRow::HEAD_ONLY,
        pad: 0,
    }
}

/// Heads on the ribbons from `first` on: a vector row per headed end, that end marked so the ribbon stops under it; the row flags.
pub(crate) fn push_heads(seg: &mut SegRows, lanes: &mut VectorRows, first: usize, arrowhead: Arrowhead) -> u32 {
    let (start, end) = match arrowhead {
        Arrowhead::NONE => return 0,
        Arrowhead::START => (true, false),
        Arrowhead::END => (false, true),
        Arrowhead::BOTH => (true, true),
    };
    let (Some(&head), Some(&tail)) = (seg.ribbons.get(first), seg.ribbons.last()) else {
        return 0;
    };

    // a zero-length line has no direction to aim along
    if tail.p0 == tail.p1 {
        return 0;
    }

    let rows = &mut lanes.rows;

    if start {
        let flipped = CylinderSegment {
            p0: head.p1,
            p1: head.p0,
            ..head
        };
        rows.push(head_row(&flipped));
        seg.ribbon_heads.push((first as u32, VectorRow::HEAD_START));
    }

    if end {
        rows.push(head_row(&tail));
        seg.ribbon_heads
            .push((seg.ribbons.len() as u32 - 1, VectorRow::HEAD_END));
    }

    Instance::FLAG_HEADS
}

/// A line as one segment.
pub fn walk_line(seg: &mut SegRows, lanes: &mut VectorRows, l: &Line, row: u32) -> Row {
    let p0 = [l[0] as f32, l[1] as f32, l[2] as f32];
    let p1 = [l[3] as f32, l[4] as f32, l[5] as f32];
    let mut bounds = AABB::empty();
    bounds.union_with_point(p0[0] as f64, p0[1] as f64, p0[2] as f64);
    bounds.union_with_point(p1[0] as f64, p1[1] as f64, p1[2] as f64);
    let first = seg.ribbons.len();
    seg.ribbons.push(CylinderSegment {
        p0,
        radius: encode_width(l.width),
        p1,
        instance_id: row,
        color: pack_rgba(l.linecolor.to_f32()),
        facing: FACING_UNKNOWN, // no face orientation
    });
    Row {
        flags: push_heads(seg, lanes, first, l.arrowhead),
        ..Row::thin(bounds)
    }
}

/// A polyline as one segment per span.
pub fn walk_polyline(seg: &mut SegRows, lanes: &mut VectorRows, pl: &Polyline, row: u32) -> Row {
    let mut pts: Vec<[f32; 3]> = Vec::with_capacity(pl.coords.len() / 3);

    for c in pl.coords.chunks_exact(3) {
        pts.push([c[0] as f32, c[1] as f32, c[2] as f32]);
    }

    let pen = Pen {
        row,
        radius: encode_width(pl.width),
        color: pack_rgba(pl.linecolor.to_f32()),
    };
    let mut bounds = AABB::empty();
    let first = seg.ribbons.len();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row {
        flags: push_heads(seg, lanes, first, pl.arrowhead),
        ..Row::thin(bounds)
    }
}

/// Degrees of turning one chord may span.
const CHORD_DEGREES: f64 = 5.0;

/// Control point `i` with its weight divided out.
fn control_position(c: &NurbsCurve, i: usize) -> Option<[f64; 3]> {
    let p = c.cv(i)?;
    let w = if c.m_is_rat && p.len() > 3 && p[3] != 0.0 {
        p[3]
    } else {
        1.0
    };
    Some([p[0] / w, p[1] / w, p[2] / w])
}

/// f64 point to the f32 the GPU takes.
pub(crate) fn render_position(point: [f64; 3]) -> [f32; 3] {
    [point[0] as f32, point[1] as f32, point[2] as f32]
}

/// Total turning of the control polygon in degrees.
pub(crate) fn turning_degrees(c: &NurbsCurve) -> f64 {
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

        let u = [d[0] / len, d[1] / len, d[2] / len]; // unit direction

        if let Some(q) = prev {
            let dot = (q[0] * u[0] + q[1] * u[1] + q[2] * u[2]).clamp(-1.0, 1.0);
            total += dot.acos().to_degrees(); // angle between neighbours
        }

        prev = Some(u);
    }

    total
}

/// Sample the curve, one chord per `CHORD_DEGREES`.
pub(crate) fn sample_nurbscurve(c: &NurbsCurve) -> Vec<[f64; 3]> {
    if c.m_cv_count < 2 {
        return Vec::new();
    }

    let spans = c.span_count().max(1);
    let n = ((turning_degrees(c) / CHORD_DEGREES).ceil() as usize).clamp(spans, 512); // chord count

    let (t0, t1) = c.domain();
    let mut pts: Vec<[f64; 3]> = Vec::with_capacity(n + 1);

    for i in 0..=n {
        let point = c.point_at(t0 + (t1 - t0) * i as f64 / n as f64);
        pts.push([point[0], point[1], point[2]]);
    }

    pts
}

/// A curve as a sampled polyline.
pub fn walk_nurbscurve(seg: &mut SegRows, lanes: &mut VectorRows, c: &NurbsCurve, row: u32) -> Row {
    walk_sampled_curve(seg,lanes,c,&sample_nurbscurve(c),row)
}
pub fn walk_sampled_curve(seg: &mut SegRows, lanes: &mut VectorRows, c: &NurbsCurve, points:&[[f64;3]], row:u32)->Row {
    let pts:Vec<_>=points.iter().copied()
        .map(render_position)
        .collect();
    let color = c
        .linecolors
        .first()
        .map(session_rust::Color::to_f32)
        .unwrap_or([0.0, 0.0, 0.0, 1.0]); // black when unset
    let pen = Pen {
        row,
        radius: encode_width(c.width),
        color: pack_rgba(color),
    };
    let mut bounds = AABB::empty();
    let first = seg.ribbons.len();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row {
        flags: push_heads(seg, lanes, first, c.arrowhead),
        ..Row::thin(bounds)
    }
}

pub fn joined_rows(
    rows: &[CylinderSegment],
    chains: &[std::ops::Range<u32>],
    heads: &[(u32, u32)],
    base: u32,
    sags: &[f32],
) -> Vec<StrokeSegment> {
    let mut result = Vec::with_capacity(rows.len());

    for (index, segment) in rows.iter().enumerate() {
        result.push(StrokeSegment {
            segment: *segment,
            previous: u32::MAX,
            next: u32::MAX,
            sag: sags.get(index).copied().unwrap_or(0.0),
        });
    }

    for chain in chains {
        if chain.end > rows.len() as u32 || chain.end.saturating_sub(chain.start) < 2 {
            continue;
        }

        for index in chain.clone() {
            // a closed chain wraps around
            let next = if index + 1 == chain.end {
                chain.start
            } else {
                index + 1
            };
            let a = &rows[index as usize];
            let b = &rows[next as usize];

            // join only where the ends meet and look the same
            if a.p1 == b.p0
                && a.instance_id == b.instance_id
                && a.color == b.color
                && a.radius == b.radius
            {
                result[index as usize].next = next + base;
                result[next as usize].previous = index + base;
            }
        }
    }

    for &(index, end) in heads {
        let Some(row) = result.get(index as usize).copied() else {
            continue;
        };

        // a closed chain opens where the head sits
        if end == VectorRow::HEAD_END {
            if row.next < HEAD_MARK {
                result[(row.next - base) as usize].previous = u32::MAX;
            }

            result[index as usize].next = HEAD_MARK;
        } else {
            if row.previous < HEAD_MARK {
                result[(row.previous - base) as usize].next = u32::MAX;
            }

            result[index as usize].previous = HEAD_MARK;
        }
    }

    result
}

