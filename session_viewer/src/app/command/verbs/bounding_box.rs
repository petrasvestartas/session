use crate::State;
use crate::app::command::{Action, Spec};
use session_rust::{AABB, Geometry, OBB, Point, Vector, Xform};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["BoundingBox"],
        "BoundingBox · create a world axis aligned box around the selection",
        parse,
    )
};

fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(BoundingBox))
}

#[derive(Debug)]
struct BoundingBox;

impl Action for BoundingBox {
    fn run(&self, state: &mut State) -> Result<String, String> {
        super::measure::loading(state)?;
        let bounds = selected_bounds(state)?;
        super::geometry::create(
            state,
            Geometry::OBB(Rc::new(box_geometry(&bounds))),
            "bounding box",
        )
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

pub(super) fn selected_bounds(state: &State) -> Result<AABB, String> {
    let mut bounds = AABB::empty();
    for row in state.selected_rows() {
        if let Some(object) = state.gpu.objects.row_bounds(row) {
            let object = match (state.scene.geometry(row), state.scene.placement_of(row)) {
                (Some(geometry), Some(place)) => object_bounds(geometry, &place, object),
                _ => object,
            };
            bounds.union_with(&object);
        }
    }
    bounds
        .is_valid()
        .then_some(bounds)
        .ok_or_else(|| "Select an object with a bounding box".into())
}

fn object_bounds(geometry: &Geometry, place: &Xform, fallback: AABB) -> AABB {
    if let Geometry::Element(element) = geometry {
        let mut element = (**element).clone();
        let corners = element.aabb().corners().map(|p| p.transformed(place));
        AABB::from_points(&corners, 0.0)
    } else {
        fallback
    }
}

fn box_geometry(bounds: &AABB) -> OBB {
    OBB::new(
        Point::new(bounds.cx, bounds.cy, bounds.cz),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(bounds.hx, bounds.hy, bounds.hz),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Point;

    #[test]
    fn boxes_keep_world_position_and_flat_extents() {
        for end in [Point::new(17.0, 29.0, 43.0), Point::new(17.0, 29.0, 31.0)] {
            let bounds = AABB::from_points(&[Point::new(11.0, 23.0, 31.0), end], 0.0);
            let box_ = box_geometry(&bounds);
            assert_eq!(box_.center, Point::new(bounds.cx, bounds.cy, bounds.cz));
        }
        assert!(crate::app::command::parse("BoundingBox").is_ok());
        assert!(crate::app::command::parse("Bounding Box").is_ok());
        assert!(crate::app::command::parse("BoundingBox 1").is_err());
        assert!(crate::app::command::parse("Length BoundingBox").is_ok());
        assert!(crate::app::command::parse("Length BoundingBox 1").is_err());
    }

    #[test]
    fn placed_elements_measure_geometry_without_distant_features() {
        use session_rust::{Element, Mesh};
        let mut element = Element::new("block");
        element.set_geometry(Mesh::create_box(10.0, 20.0, 30.0));
        let geometry = Geometry::Element(Rc::new(element));
        let place = &Xform::translation(100.0, 200.0, 300.0) * &Xform::scale_xyz(2.0, 3.0, 4.0);
        let b = object_bounds(
            &geometry,
            &place,
            AABB::from_points(
                &[
                    Point::new(-1000.0, -1000.0, -1000.0),
                    Point::new(1000.0, 1000.0, 1000.0),
                ],
                0.0,
            ),
        );
        assert_eq!([b.cx, b.cy, b.cz], [100.0, 200.0, 300.0]);
        assert_eq!([2.0 * b.hx, 2.0 * b.hy, 2.0 * b.hz], [20.0, 60.0, 120.0]);
    }
}
