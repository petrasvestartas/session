use crate::engine::gpu::Upload;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::lane::LaneRows;
use crate::engine::gpu::segments::SegRows;
use brep::{walk_brep, walk_surface};
use cloud::walk_cloud;
use curves::{walk_line, walk_nurbscurve, walk_polyline};
use frames::{walk_obb, walk_plane};
use mesh::{MeshCx, MeshOpts, walk_mesh};
use mesh_ink::Ink;
use points::walk_point;
use session_rust::AABB;
use session_rust::Arrowhead;
use session_rust::Element;
use session_rust::Geometry;
use session_rust::element::{ElementFeature, ElementGeometry};

pub mod bounds;
pub mod brep;
pub mod brep_edges;
pub mod brep_orient;
pub mod brep_shapes;
pub mod cloud;
pub mod curves;
pub mod encode;
pub mod frames;
pub mod mesh;
pub mod mesh_ink;
pub mod mesh_topology;
pub mod plane;
pub mod points;
pub mod sheet; // register:sheets

/// The row tables one object writes into.
pub struct Walk<'a> {
    pub arena: &'a mut ArenaRows, // triangles of faces
    pub seg: &'a mut SegRows,     // line segments
    pub glyph: &'a mut GlyphRows, // dots and labels
    pub cloud: &'a mut CloudRows, // point cloud points
    pub lanes: &'a mut LaneRows,  // registered lanes, e.g. arrowheads
}

impl<'a> Walk<'a> {
    /// Borrow every table of one upload.
    pub fn of(t: &'a mut Upload) -> Self {
        Self {
            arena: &mut t.arena,
            seg: &mut t.seg,
            glyph: &mut t.glyph,
            cloud: &mut t.cloud,
            lanes: &mut t.lanes,
        }
    }

    /// Tables a solid needs: faces plus its edge ink.
    fn solid(&mut self) -> (&mut ArenaRows, Ink<'_>) {
        (
            self.arena,
            Ink {
                seg: self.seg,
                glyph: self.glyph,
            },
        )
    }
}

/// Where one object's rows go.
pub struct WalkCx {
    pub vert_base: u32,   // arena vertices already on the GPU
    pub cloud_px: f32,    // point size override in px, 0 = file's own
    pub row: u32,         // this object's row index
    pub attributes: bool, // draw element features inside its row
}

/// What a producer reports for one object row.
pub struct Row {
    pub bounds: AABB, // local bounding box
    pub spacing: f32, // point or vertex spacing
    pub flags: u32,   // row flag bits
    pub faces: bool,  // row drew faces
}

impl Row {
    /// A row with only a box: lines, points, frames.
    pub fn thin(bounds: AABB) -> Self {
        Self {
            bounds,
            spacing: 0.0,
            flags: 0,
            faces: false,
        }
    }
}

const ATTRIBUTE_LINE_PX: f64 = 2.0; // twice the 1 px pen
const ATTRIBUTE_DOT_PX: f64 = 12.0; // twice the 6 px point
const CONTACT_COLOR: [f32; 4] = [0.9, 0.1, 0.1, 1.0 / 255.0]; // contacts fill red; alpha 1/255 tells the face shader to draw it opaque and unlit, apart from every other feature
const CONTACT_LIFT: f64 = 0.5; // mm off both sides of the face, so neither touching element hides the fill

/// Draw an element's visible features, thick, into its own row.
fn walk_attributes(w: &mut Walk, cx: &WalkCx, e: &Element, bounds: &mut AABB) {
    walk_features(w, cx, e.features(), bounds);
}

/// Draw visible features, thick, into the row of `cx`: an element's own or an instance's; contacts always, the rest while attributes are on.
pub fn walk_features(w: &mut Walk, cx: &WalkCx, features: &[ElementFeature], bounds: &mut AABB) {
    for feature in features {
        if !feature.visible || (!cx.attributes && feature.feature_type != "contact") {
            continue;
        }

        for outline in &feature.outlines {
            // a contact polygon is a red fill, not a line
            if feature.feature_type == "contact"
                && let Some(fill) = walk_contact(w.arena, cx, outline)
            {
                bounds.union_with(&fill);
                continue;
            }

            // Coincident contact endpoints also represent a point.
            let point = outline.get_point(0).filter(|p| {
                outline
                    .coords
                    .chunks_exact(3)
                    .all(|q| [q[0] as f32, q[1] as f32, q[2] as f32] == p.to_f32())
            });
            let r = if let Some(mut p) = point {
                p.pointcolor = outline.linecolor.clone();
                p.width = ATTRIBUTE_DOT_PX;
                walk_point(w.glyph, &p, cx.row)
            } else {
                let mut outline = outline.clone();
                outline.width = ATTRIBUTE_LINE_PX;
                outline.arrowhead = Arrowhead::NONE; // the feature row carries no head flag
                walk_polyline(w.seg, w.lanes, &outline, cx.row)
            };
            bounds.union_with(&r.bounds);
        }
    }
}

/// A contact polygon as red triangles just off both sides of its plane, in the row of `cx`; none for fewer than three corners.
fn walk_contact(arena: &mut ArenaRows, cx: &WalkCx, outline: &session_rust::Polyline) -> Option<AABB> {
    use crate::engine::gpu::faces::FaceSource;
    use session_rust::Point;

    let mut points: Vec<[f64; 3]> = outline.coords.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
    if points.len() > 1 && points.first() == points.last() {
        points.pop();
    }
    if points.len() < 3 {
        return None;
    }

    // the polygon's normal, Newell's sum
    let mut normal = [0.0; 3];
    for (k, a) in points.iter().enumerate() {
        let b = points[(k + 1) % points.len()];
        normal[0] += (a[1] - b[1]) * (a[2] + b[2]);
        normal[1] += (a[2] - b[2]) * (a[0] + b[0]);
        normal[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
    if length < 1e-12 {
        return None;
    }

    let mut bounds = AABB::empty();
    for side in [1.0, -1.0] {
        let lift = CONTACT_LIFT * side / length;
        let lifted: Vec<Point> = points
            .iter()
            .map(|p| Point::new(p[0] + normal[0] * lift, p[1] + normal[1] * lift, p[2] + normal[2] * lift))
            .collect();
        let render = session_rust::Mesh::from_polylines(vec![lifted], None).to_render();
        let base = cx.vert_base + arena.verts.len() as u32;
        for vertex in &render.vertices {
            bounds.union_with_point(vertex.position[0] as f64, vertex.position[1] as f64, vertex.position[2] as f64);
            arena.verts.push(session_rust::render_mesh::RenderVertex { color: CONTACT_COLOR, ..*vertex });
            arena.vids.push(cx.row);
        }
        arena.idx.extend(render.indices.iter().map(|&i| base + i));
        let address = arena.face_sources.len() as u32;
        arena.face_sources.push(FaceSource { parent: cx.row, face: 0 });
        arena.face_ids.extend(std::iter::repeat_n(address, render.indices.len() / 3));
    }

    Some(bounds)
}

/// An element without geometry gets no row.
pub fn is_drawable(geom: &Geometry) -> bool {
    match geom {
        Geometry::Element(e) => !matches!(e.geometry(), ElementGeometry::None),
        _ => true,
    }
}

/// Write one object into the tables and report its row.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
    match geom {
        Geometry::Mesh(m) => {
            let (arena, mut ink) = w.solid();
            walk_mesh(
                arena,
                &mut ink,
                m,
                &MeshCx {
                    cx,
                    opts: &MeshOpts::OBJECT,
                },
            )
        }
        Geometry::BRep(b) => {
            let (arena, mut ink) = w.solid();
            walk_brep(arena, &mut ink, b, cx)
        }
        Geometry::NurbsSurface(s) => {
            let (arena, mut ink) = w.solid();
            walk_surface(arena, &mut ink, s, cx)
        }
        Geometry::Line(l) => walk_line(w.seg, w.lanes, l, cx.row),
        Geometry::Polyline(pl) => walk_polyline(w.seg, w.lanes, pl, cx.row),
        Geometry::NurbsCurve(c) => walk_nurbscurve(w.seg, w.lanes, c, cx.row),
        Geometry::Plane(p) if plane::is_clipping(p) => plane::walk(w.seg, p, cx.row),
        Geometry::Plane(p) => walk_plane(w.seg, p, cx.row),
        Geometry::OBB(b) => walk_obb(w.seg, b, cx.row),
        Geometry::Point(p) => walk_point(w.glyph, p, cx.row),
        Geometry::PointCloud(pc) => walk_cloud(w.cloud, pc, cx),
        Geometry::Element(e) => {
            let mut row = match e.geometry() {
                ElementGeometry::Mesh(m) => {
                    let (arena, mut ink) = w.solid();
                    walk_mesh(
                        arena,
                        &mut ink,
                        m,
                        &MeshCx {
                            cx,
                            opts: &MeshOpts::ELEMENT,
                        },
                    )
                }
                ElementGeometry::BRep(b) => {
                    let (arena, mut ink) = w.solid();
                    walk_brep(arena, &mut ink, b, cx)
                }
                ElementGeometry::None => Row::thin(AABB::empty()),
            };

            walk_attributes(w, cx, e, &mut row.bounds);

            row
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Mesh;
    use session_rust::Point;
    use session_rust::Polyline;
    use session_rust::element::ElementFeature;

    /// A box element with an axis, a section dot, a joint and a hidden joint.
    fn walk_element(attributes: bool) -> (Upload, Row) {
        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let axis = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(100.0, 0.0, 0.0)]);
        element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
        let dot = Polyline::new(vec![Point::new(5.0, 5.0, 5.0)]);
        element.add_feature(ElementFeature::new("section", -1, vec![dot], "section"));
        let joint = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 50.0)]);
        element.add_feature(ElementFeature::new("joint", 0, vec![joint], "joint"));
        let contact = Polyline::new(vec![Point::new(2.0, 3.0, 5.0), Point::new(2.0, 3.0, 5.0)]);
        element.add_feature(ElementFeature::new("contact", 0, vec![contact], "touch"));
        let empty = Polyline::new(vec![]);
        element.add_feature(ElementFeature::new("contact", 0, vec![empty], "empty"));
        let far = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(0.0, 200.0, 0.0)]);
        let mut hidden = ElementFeature::new("joint", 1, vec![far], "hidden");
        hidden.visible = false;
        element.add_feature(hidden);
        let mut up = Upload::default();
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row: 4,
            attributes,
        };
        let row = walk_geometry(
            &mut Walk::of(&mut up),
            &cx,
            &Geometry::Element(std::rc::Rc::new(element)),
        );
        (up, row)
    }

    /// Visible features add two ribbons and two dots to the element's own row, the contact dot even with attributes off; the hidden one adds nothing.
    #[test]
    fn attributes_join_the_element_row() {
        let (off, row_off) = walk_element(false);
        let (on, row_on) = walk_element(true);
        assert_eq!(on.seg.ribbons.len(), off.seg.ribbons.len() + 2);
        assert_eq!(on.glyph.dots.len(), off.glyph.dots.len() + 1);
        assert_eq!(off.glyph.dots.last().map(|d| d.instance_id), Some(4));
        assert!(on.seg.ribbons.iter().all(|r| r.instance_id == 4));
        assert_eq!(on.glyph.dots.last().map(|d| d.instance_id), Some(4));
        assert_eq!(row_off.bounds.max_point()[0], 5.0);
        assert_eq!(row_on.bounds.max_point()[0], 100.0);
        assert_eq!(row_on.bounds.max_point()[1], 5.0);
    }

    /// A contact polygon fills red triangles on both sides of its face, in the element's own row, instead of a ribbon.
    #[test]
    fn contact_polygons_fill_red() {
        let mut element = Element::new("plate");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let square = Polyline::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(4.0, 0.0, 5.0), Point::new(4.0, 4.0, 5.0), Point::new(0.0, 4.0, 5.0), Point::new(0.0, 0.0, 5.0)]);
        element.add_feature(ElementFeature::new("contact", 0, vec![square], "side_side"));
        let mut up = Upload::default();
        let cx = WalkCx { vert_base: 0, cloud_px: 0.0, row: 7, attributes: true };
        let before = {
            let mut bare = Upload::default();
            let mut plain = element.clone();
            plain.set_features(vec![]);
            walk_geometry(&mut Walk::of(&mut bare), &cx, &Geometry::Element(std::rc::Rc::new(plain)));
            bare.arena.idx.len()
        };
        walk_geometry(&mut Walk::of(&mut up), &cx, &Geometry::Element(std::rc::Rc::new(element)));
        assert_eq!(up.arena.idx.len(), before + 12, "two triangles per side, both sides");
        assert!(up.arena.verts.iter().rev().take(8).all(|v| v.color == CONTACT_COLOR));
        assert!(up.arena.vids.iter().rev().take(8).all(|&row| row == 7));
        assert_eq!(up.seg.ribbons.len(), 0);
    }
}
