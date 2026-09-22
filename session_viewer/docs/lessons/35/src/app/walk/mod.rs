use crate::engine::gpu::Upload;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use brep::{walk_brep, walk_surface};
use cloud::walk_cloud;
use curves::{walk_line, walk_nurbscurve, walk_polyline};
use frames::{walk_obb, walk_plane};
use mesh::{MeshCx, MeshOpts, walk_mesh};
use mesh_ink::Ink;
use points::walk_point;
use session_rust::AABB;
// --8<-- [start:step-7a]
use session_rust::Element;
// --8<-- [end:step-7a]
use session_rust::Geometry;
use session_rust::element::ElementGeometry;

pub mod bounds;
pub mod brep;
pub mod brep_edges;
pub mod brep_orient;
pub mod cloud;
pub mod curves;
pub mod encode;
pub mod frames;
pub mod mesh;
pub mod mesh_ink;
pub mod mesh_topology;
pub mod points;
pub mod sheet;

/// The four row tables one object writes into.
pub struct Walk<'a> {
    pub arena: &'a mut ArenaRows, // triangles of faces
    pub seg: &'a mut SegRows,     // line segments
    pub glyph: &'a mut GlyphRows, // dots and labels
    pub cloud: &'a mut CloudRows, // point cloud points
}

impl<'a> Walk<'a> {
    /// Borrow every table of one upload.
    pub fn of(t: &'a mut Upload) -> Self {
        Self {
            arena: &mut t.arena,
            seg: &mut t.seg,
            glyph: &mut t.glyph,
            cloud: &mut t.cloud,
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
    // --8<-- [start:step-7b]
    pub attributes: bool, // draw element features inside its row
    // --8<-- [end:step-7b]
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
        // --8<-- [start:step-7c]
        }
    }
}

/// Feature types `Attributes On` draws on the element.
const ATTRIBUTE_FEATURES: [&str; 4] = ["outline", "axis", "section", "centroid"];

/// Draw an element's features, red and thick, into its own row.
fn walk_attributes(w: &mut Walk, cx: &WalkCx, e: &Element, bounds: &mut AABB) {
    for feature in e.features() {
        if !ATTRIBUTE_FEATURES.contains(&feature.feature_type.as_str()) {
            continue;
        }

        for outline in &feature.outlines {
            let r = if let (1, Some(p)) = (outline.point_count(), outline.get_point(0)) {
                walk_point(w.glyph, &p, cx.row)
            } else {
                walk_polyline(w.seg, outline, cx.row)
            };
            bounds.union_with(&r.bounds);
        // --8<-- [end:step-7c]
        }
    }
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
        Geometry::Line(l) => walk_line(w.seg, l, cx.row),
        Geometry::Polyline(pl) => walk_polyline(w.seg, pl, cx.row),
        Geometry::NurbsCurve(c) => walk_nurbscurve(w.seg, c, cx.row),
        Geometry::Plane(p) => walk_plane(w.seg, p, cx.row),
        Geometry::OBB(b) => walk_obb(w.seg, b, cx.row),
        Geometry::Point(p) => walk_point(w.glyph, p, cx.row),
        Geometry::PointCloud(pc) => walk_cloud(w.cloud, pc, cx),
        // --8<-- [start:step-7d]
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

            if cx.attributes {
                walk_attributes(w, cx, e, &mut row.bounds);
            }

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

    /// A box element with an axis, a section dot and a non-attribute cut.
    fn walk_element(attributes: bool) -> (Upload, Row) {
        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let axis = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(100.0, 0.0, 0.0)]);
        element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
        let centroid = Polyline::new(vec![Point::new(5.0, 5.0, 5.0)]);
        element.add_feature(ElementFeature::new(
            "centroid",
            -1,
            vec![centroid],
            "centroid",
        ));
        let cut = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(0.0, 200.0, 0.0)]);
        element.add_feature(ElementFeature::new("cut", 0, vec![cut], "cut"));
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

    /// Attributes add one ribbon and one dot to the element's own row.
    #[test]
    fn attributes_join_the_element_row() {
        let (off, row_off) = walk_element(false);
        let (on, row_on) = walk_element(true);
        assert_eq!(on.seg.ribbons.len(), off.seg.ribbons.len() + 1);
        assert_eq!(on.glyph.dots.len(), off.glyph.dots.len() + 1);
        assert!(on.seg.ribbons.iter().all(|r| r.instance_id == 4));
        assert_eq!(on.glyph.dots.last().map(|d| d.instance_id), Some(4));
        assert_eq!(row_off.bounds.max_point()[0], 5.0);
        assert_eq!(row_on.bounds.max_point()[0], 100.0);
        assert_eq!(row_on.bounds.max_point()[1], 5.0);
    }
}
        // --8<-- [end:step-7d]
