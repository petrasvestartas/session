use super::*;
use session_rust::Mesh;
use session_rust::Point;
use session_rust::Polyline;
use session_rust::element::ElementFeature;

/// A box element with an axis, a section dot, a joint and a hidden joint.
fn walk_element(interactions: bool) -> (Upload, Row) {
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
        quality: Default::default(),
        vert_base: 0,
        cloud_px: 0.0,
        row: 4,
        interactions,
        attributes: interactions,
        arrowhead_pens: 0.0,
    };
    let row = walk_geometry(
        &mut Walk::of(&mut up),
        &cx,
        &Geometry::Element(std::rc::Rc::new(element)),
    );
    (up, row)
}

/// Visible features add two ribbons and two dots to the element's own row, the contact dot among them, only with interactions on; the hidden one adds nothing.
#[test]
fn interactions_join_the_element_row() {
    let (off, row_off) = walk_element(false);
    let (on, row_on) = walk_element(true);
    assert_eq!(on.seg.ribbons.len(), off.seg.ribbons.len() + 2);
    assert_eq!(on.glyph.dots.len(), off.glyph.dots.len() + 2);
    assert!(on.seg.ribbons.iter().all(|r| r.instance_id == 4));
    assert_eq!(on.glyph.dots.last().map(|d| d.instance_id), Some(4));
    assert_eq!(row_off.bounds.max_point()[0], 5.0);
    assert_eq!(row_on.bounds.max_point()[0], 100.0);
    assert_eq!(row_on.bounds.max_point()[1], 5.0);
}

/// A contact polygon fills red triangles on its face, in the element's own row, instead of a ribbon.
#[test]
fn contact_polygons_fill_red() {
    let mut element = Element::new("plate");
    element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
    let square = Polyline::new(vec![
        Point::new(0.0, 0.0, 5.0),
        Point::new(4.0, 0.0, 5.0),
        Point::new(4.0, 4.0, 5.0),
        Point::new(0.0, 4.0, 5.0),
        Point::new(0.0, 0.0, 5.0),
    ]);
    element.add_feature(ElementFeature::new("contact", 0, vec![square], "side_side"));
    let mut up = Upload::default();
    let cx = WalkCx {
        quality: Default::default(),
        vert_base: 0,
        cloud_px: 0.0,
        row: 7,
        interactions: true,
        attributes: false,
        arrowhead_pens: 0.0,
    };
    let before = {
        let mut bare = Upload::default();
        let mut plain = element.clone();
        plain.set_features(vec![]);
        walk_geometry(
            &mut Walk::of(&mut bare),
            &cx,
            &Geometry::Element(std::rc::Rc::new(plain)),
        );
        bare.arena.idx.len()
    };
    walk_geometry(
        &mut Walk::of(&mut up),
        &cx,
        &Geometry::Element(std::rc::Rc::new(element)),
    );
    assert_eq!(
        up.arena.idx.len(),
        before + 6,
        "two triangles, one copy on the face"
    );
    assert!(
        up.arena
            .verts
            .iter()
            .rev()
            .take(4)
            .all(|v| v.color == CONTACT_COLOR)
    );
    assert!(up.arena.vids.iter().rev().take(4).all(|&row| row == 7));
    assert_eq!(
        up.seg.ribbons.len(),
        4,
        "a black outline, one ribbon per side"
    );
    assert!(
        up.seg
            .ribbons
            .iter()
            .all(|edge| edge.color == 0x0100_0000 && edge.facing != encode::FACING_UNKNOWN)
    );
}
