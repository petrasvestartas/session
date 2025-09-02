use crate::{Xform, Line, LineCloud, Mesh, Point, Pline};

/// Trait to generalize extraction of pipe transforms from any segment source.
/// Returns transforms mapping the canonical unit pipe (aligned +Z, length=1, radius=0.5)
/// onto the source's segments. XY scale is fixed at 1.0; the shader controls thickness in pixel space.
pub trait PipeFromSegments {
    fn pipe_transforms(&self) -> Vec<Xform>;
}

impl PipeFromSegments for Line {
    fn pipe_transforms(&self) -> Vec<Xform> {
        let mut out = Vec::new();
        if let Some(tf) = self.to_pipe_transform() { out.push(tf); }
        out
    }
}

impl PipeFromSegments for Vec<Line> {
    fn pipe_transforms(&self) -> Vec<Xform> {
        let mut out = Vec::new();
        out.reserve(self.len());
        for ln in self {
            if let Some(tf) = ln.to_pipe_transform() { out.push(tf); }
        }
        out
    }
}

impl PipeFromSegments for LineCloud {
    fn pipe_transforms(&self) -> Vec<Xform> {
        let mut out = Vec::new();
        out.reserve(self.lines.len());
        for ln in &self.lines {
            if let Some(tf) = ln.to_pipe_transform() { out.push(tf); }
        }
        out
    }
}

impl PipeFromSegments for Pline {
    fn pipe_transforms(&self) -> Vec<Xform> {
        let mut out = Vec::new();
        if self.points.len() < 2 { return out; }
        for i in 0..(self.points.len() - 1) {
            let a = &self.points[i];
            let b = &self.points[i + 1];
            let seg = Line::new(a.x, a.y, a.z, b.x, b.y, b.z);
            if let Some(tf) = seg.to_pipe_transform() { out.push(tf); }
        }
        out
    }
}

impl PipeFromSegments for Mesh {
    fn pipe_transforms(&self) -> Vec<Xform> {
        self.extract_edge_pipe_transforms()
    }
}

/// Trait to extract the points where spheres should be placed for a given segment source.
/// For meshes: boundary vertices. For polylines: all vertices. For lines: both endpoints.
pub trait SphereFromSegments {
    fn sphere_points(&self) -> Vec<Point>;
}

impl SphereFromSegments for Line {
    fn sphere_points(&self) -> Vec<Point> {
        vec![
            Point::new(self.x0, self.y0, self.z0),
            Point::new(self.x1, self.y1, self.z1),
        ]
    }
}

impl SphereFromSegments for Vec<Line> {
    fn sphere_points(&self) -> Vec<Point> {
        let mut out = Vec::with_capacity(self.len() * 2);
        for ln in self {
            out.push(Point::new(ln.x0, ln.y0, ln.z0));
            out.push(Point::new(ln.x1, ln.y1, ln.z1));
        }
        out
    }
}

impl SphereFromSegments for LineCloud {
    fn sphere_points(&self) -> Vec<Point> {
        let mut out = Vec::with_capacity(self.lines.len() * 2);
        for ln in &self.lines {
            out.push(Point::new(ln.x0, ln.y0, ln.z0));
            out.push(Point::new(ln.x1, ln.y1, ln.z1));
        }
        out
    }
}

impl SphereFromSegments for Pline {
    fn sphere_points(&self) -> Vec<Point> {
        self.points.clone()
    }
}

impl SphereFromSegments for Mesh {
    fn sphere_points(&self) -> Vec<Point> {
        let mut out = Vec::new();
        for vk in self.vertex.keys() {
            if let Some(p) = self.vertex_position(*vk) {
                out.push(p);
            }
        }
        out
    }
}

/// Helper to convert a set of points into translation transforms with epsilon-based deduplication.
pub fn dedupe_sphere_transforms<I>(points: I, eps: f32) -> Vec<Xform>
where
    I: IntoIterator<Item = Point>,
{
    use std::collections::HashSet;
    let mut keys: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut out: Vec<Xform> = Vec::new();
    for p in points.into_iter() {
        let k = (
            (p.x / eps).round() as i64,
            (p.y / eps).round() as i64,
            (p.z / eps).round() as i64,
        );
        if keys.insert(k) {
            out.push(Xform::translation(p.x, p.y, p.z));
        }
    }
    out
}
