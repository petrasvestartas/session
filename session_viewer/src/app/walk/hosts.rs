//! Associate separately authored coplanar strokes with the actual rasterized mesh faces.
//! This runs once in f64; the GPU compares integer identities instead of depth tolerances.

use std::ops::Range;
use session_rust::Geometry;
use crate::engine::gpu::segments::InkSupport;
use crate::math::{Mat4, xform_point_f64};
use super::{curves::sample_nurbscurve, mesh_ink::Ink};

/// One logical face's original f64 triangles and the token written by its raster vertices.
pub struct HostFace {
    pub face: u32,
    pub triangles: Vec<[[f64; 3]; 3]>,
}

/// One placed physical triangle, retained only while its file's rows are being walked.
struct HostTriangle {
    face: u32,
    points: [[f64; 3]; 3],
    normal: [f64; 3],
    tolerance: f64,
}

/// Mesh faces available to host the linework in one file.
#[derive(Default)]
pub struct Hosts {
    triangles: Vec<HostTriangle>,
}

/// One object's placement and the newly emitted ribbon range.
pub struct Association<'a> {
    pub place: &'a Mat4,
    pub ribbons: Range<usize>,
    pub dots: Range<usize>,
}

/// Subtract two vectors.
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }

/// Dot product of two vectors.
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

/// Cross product of two vectors.
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// Test the actual bounded triangle, including its boundary, without a world-distance lift.
fn contains(triangle: &HostTriangle, point: [f64; 3]) -> bool {
    let [a, b, c] = triangle.points;
    if dot(triangle.normal, sub(point, a)).abs() > triangle.tolerance { return false; }
    let u = sub(b, a);
    let v = sub(c, a);
    let w = sub(point, a);
    let (uu, uv, vv, wu, wv) = (dot(u, u), dot(u, v), dot(v, v), dot(w, u), dot(w, v));
    let det = uu * vv - uv * uv;
    if det <= 0.0 { return false; }
    let x = (vv * wu - uv * wv) / det;
    let y = (uu * wv - uv * wu) / det;
    let epsilon = 128.0 * f64::EPSILON;
    x >= -epsilon && y >= -epsilon && x + y <= 1.0 + epsilon
}

impl Hosts {
    /// Place emitted face triangles using exactly the object's final transform.
    pub fn extend(&mut self, faces: Vec<HostFace>, place: &Mat4) {
        for face in faces {
            for local in face.triangles {
                let points = local.map(|point| xform_point_f64(place, point));
                let n = cross(sub(points[1], points[0]), sub(points[2], points[0]));
                let length = dot(n, n).sqrt();
                if length == 0.0 { continue; }
                let normal = n.map(|value| value / length);
                let scale = points.iter().flatten().fold(1.0f64, |old, value| old.max(value.abs()));
                self.triangles.push(HostTriangle { face: face.face, points, normal, tolerance: scale * 128.0 * f64::EPSILON });
            }
        }
    }

    /// All exact coplanar face regions containing both ends, even across tessellation edges.
    fn supporting(&self, endpoints: [[f64; 3]; 2]) -> Vec<u32> {
        let mut first = Vec::new();
        let mut second = Vec::new();
        for triangle in &self.triangles {
            if contains(triangle, endpoints[0]) && !first.contains(&triangle.face) { first.push(triangle.face); }
            if contains(triangle, endpoints[1]) && !second.contains(&triangle.face) { second.push(triangle.face); }
        }
        first.retain(|face| second.contains(face));
        first
    }

    /// Associate original f64 strokes and points after all physical faces have been walked.
    pub fn associate(&self, ink: &mut Ink, geometry: &Geometry, cx: &Association) {
        if self.triangles.is_empty() { return; }
        let mut endpoints = Vec::new();
        match geometry {
            Geometry::Polyline(polyline) => {
                for pair in polyline.coords.chunks_exact(3).collect::<Vec<_>>().windows(2) {
                    endpoints.push([[pair[0][0], pair[0][1], pair[0][2]], [pair[1][0], pair[1][1], pair[1][2]]]);
                }
            }
            Geometry::Line(line) => endpoints.push([[line[0], line[1], line[2]], [line[3], line[4], line[5]]]),
            Geometry::NurbsCurve(curve) => {
                for pair in sample_nurbscurve(curve).windows(2) { endpoints.push([pair[0], pair[1]]); }
            }
            Geometry::Point(point) => {
                assert_eq!(cx.dots.len(), 1, "free point must emit exactly one dot");
                let world = xform_point_f64(cx.place, [point[0], point[1], point[2]]);
                let supporting = self.supporting([world, world]);
                let dot = &mut ink.glyph.dots[cx.dots.start];
                dot.support_start = ink.glyph.supports.len() as u32;
                dot.support_count = supporting.len() as u32;
                for face in supporting { ink.glyph.supports.push(InkSupport { face, region: 0 }); }
                return;
            }
            _ => return,
        }
        assert_eq!(endpoints.len(), cx.ribbons.len(), "authored span order disagrees with ribbon walk");
        for (index, pair) in cx.ribbons.clone().zip(endpoints) {
            let world = pair.map(|point| xform_point_f64(cx.place, point));
            let supporting = self.supporting(world);
            let segment = &mut ink.seg.ribbons[index];
            segment.support_start = ink.seg.supports.len() as u32;
            segment.support_count = supporting.len() as u32;
            for face in supporting { ink.seg.supports.push(InkSupport { face, region: 0 }); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Color, Mesh, NurbsCurve, Point, Session, Xform};
    use crate::app::scene::{FileDoc, Scene};
    use std::rc::Rc;

    /// A non-f32-representable plane requires original curve/point coordinates for support;
    /// a distinct nearby parallel plane must not acquire that support after float rounding.
    #[test]
    fn scene_associates_original_curve_samples_and_free_dots() {
        let height = 0.123456789;
        let mut session = Session::new("authored ink support");
        for offset in [0.0, -0.0001] {
            let color = if offset == 0.0 { Color::blue() } else { Color::magenta() };
            let mut curve = NurbsCurve::create(false, 2, &[
                Point::new(-2.0, -1.0, height + offset), Point::new(0.0, 2.0, height + offset), Point::new(2.0, -1.0, height + offset),
            ]);
            curve.linecolors = vec![color.clone()];
            session.add_nurbscurve(curve, None);
            let mut point = Point::new(0.0, 0.0, height + offset);
            point.pointcolor = color;
            session.add_point(point, None);
        }
        session.add_mesh(Mesh::from_vertices_and_faces(vec![
            Point::new(-10.0, -10.0, height), Point::new(10.0, -10.0, height), Point::new(0.0, 10.0, height),
        ], vec![vec![0, 1, 2]]), None);
        let mut place = Xform::translation(1000.2, -400.1, 12.5);
        place.m[0] = 2.0; place.m[5] = 0.5; place.m[10] = 3.0; place.m[4] = 0.3;
        let mut scene = Scene::new();
        scene.add_file(FileDoc { name: "support".into(), session: Rc::new(session), place, point_px: 0.0, display_only: false });
        assert_eq!(scene.tables.glyph.dots.len(), 2);
        assert!(!scene.tables.seg.ribbons.is_empty());
        for dot in &scene.tables.glyph.dots {
            assert_eq!(dot.support_count > 0, dot.color == Color::blue().to_f32());
        }
        for segment in &scene.tables.seg.ribbons {
            let blue = segment.color == crate::app::walk::encode::pack_rgba(Color::blue().to_f32());
            assert_eq!(segment.support_count > 0, blue);
            assert!(scene.ribbon_range(segment.instance_id).is_some());
        }
        assert!((height as f32 as f64 - height).abs() > 1e-10, "fixture must detect association from rounded GPU positions");
    }

    /// Exact face membership distinguishes close parallel surfaces under nonuniform scale.
    #[test]
    fn support_is_face_specific_after_placement() {
        let local = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [0.0, 10.0, 0.0]];
        let mut place = Xform::identity().m;
        place[0] = 2.0;
        place[5] = 3.0;
        place[10] = 0.5;
        let mut hosts = Hosts::default();
        hosts.extend(vec![HostFace { face: 11, triangles: vec![local] }], &place);
        let shifted = local.map(|p| [p[0], p[1], p[2] + 0.0001]);
        hosts.extend(vec![HostFace { face: 12, triangles: vec![shifted] }], &place);
        let endpoints = [[1.0, 1.0, 0.0], [2.0, 2.0, 0.0]].map(|p| xform_point_f64(&place, p));
        assert_eq!(hosts.supporting(endpoints), vec![11]);
        let between = endpoints.map(|p| [p[0], p[1], p[2] + 0.000025]);
        assert!(hosts.supporting(between).is_empty());
        assert!(hosts.supporting([[18.0, 27.0, 0.0], [19.0, 28.0, 0.0]]).is_empty(), "inside the bounding box is not inside the triangle");
        assert!(hosts.supporting([[50.0, 50.0, 0.0], [60.0, 60.0, 0.0]]).is_empty());
    }
}
