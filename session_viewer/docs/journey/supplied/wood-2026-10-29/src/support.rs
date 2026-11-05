use crate::element::WoodElement;
use crate::geometry::{
    append_brep, append_mesh, brep_between_loops, circle_segments, drill_brep, drill_mesh,
    drilled_plate_brep,
};
use crate::proto;
use session_rust::{BRep, Line, Mesh, Plane, Point, Polyline, Xform};
use std::f64::consts::PI;

/// An adjustable column base, as wood's Support, with the manufacturer's dimensions by default.
#[derive(Clone, Debug)]
pub struct Support {
    pub name: String,                     // Element name.
    pub plane: Plane, // Base plate underside centre, z up the column, x along a base plate side.
    pub height: f64,  // Base plate underside to head plate top.
    pub head_plate_diameter: f64, // Head plate disc.
    pub head_plate_thickness: f64, // Head plate disc.
    pub head_plate_recess: f64, // Depth the head plate is let into the column end.
    pub base_plate_size: f64, // Square base plate side.
    pub base_plate_thickness: f64, // Square base plate.
    pub base_plate_hole_diameter: f64, // The four anchor drillings.
    pub base_plate_hole_spacing: f64, // Centre distance of the drillings along each side.
    pub adjustment_nut_across_flats: f64, // Hexagon on the base plate.
    pub adjustment_nut_top: f64, // Top of that hexagon above the plate underside.
    pub rod_diameter: f64, // Threaded rod from the adjustment nut to the coupling nut.
    pub coupling_nut_across_flats: f64, // Hexagon under the head plate.
    pub coupling_nut_height: f64, // Hexagon under the head plate.
    pub pin_count: i32, // Pins from the head plate up into the column end.
    pub pin_diameter: f64, // Column pins.
    pub pin_length: f64, // Column pins.
    pub pin_angle: f64, // Degrees between a pair of opposed pins.
    pub pin_circle_diameter: f64, // Circle the pins start on.
    pub anchor_diameter: f64, // Anchors through the drillings into the slab.
    pub anchor_embedment: f64, // Anchor depth below the plate underside.
    pub chord_tolerance: f64, // Largest deviation of a round part's facets.
}

impl Support {

    /// A support on a plane, its origin the base plate underside centre, with the default dimensions.
    pub fn new(plane: &Plane, name: &str) -> Self {
        Self {
            name: name.to_string(),
            plane: plane.clone(),
            height: 150.0,
            head_plate_diameter: 106.0,
            head_plate_thickness: 12.0,
            head_plate_recess: 12.0,
            base_plate_size: 140.0,
            base_plate_thickness: 12.0,
            base_plate_hole_diameter: 15.0,
            base_plate_hole_spacing: 104.0,
            adjustment_nut_across_flats: 36.0,
            adjustment_nut_top: 64.0,
            rod_diameter: 30.0,
            coupling_nut_across_flats: 55.0,
            coupling_nut_height: 30.0,
            pin_count: 3,
            pin_diameter: 8.0,
            pin_length: 180.0,
            pin_angle: 25.0,
            pin_circle_diameter: 50.0,
            anchor_diameter: 12.0,
            anchor_embedment: 100.0,
            chord_tolerance: 0.05,
        }
    }

    /// The point at x, y, z in the support's frame.
    fn local(&self, x: f64, y: f64, z: f64) -> Point {
        self.plane.origin()
            + self.plane.x_axis() * x
            + self.plane.y_axis() * y
            + self.plane.z_axis() * z
    }

    /// The point on the axis at a level above the plate underside.
    pub fn at(&self, level: f64) -> Point {
        self.local(0.0, 0.0, level)
    }

    /// The closed ring of corners at radius about (cx, cy) at level z, the first at angle start.
    fn ring(&self, cx: f64, cy: f64, radius: f64, corners: usize, start: f64, z: f64) -> Polyline {
        let mut points: Vec<Point> = (0..corners)
            .map(|k| start + 2.0 * PI * k as f64 / corners as f64)
            .map(|angle| self.local(cx + radius * angle.cos(), cy + radius * angle.sin(), z))
            .collect();
        points.push(points[0].clone());

        Polyline::new(points)
    }

    /// The hexagon of a nut given across the flats, from level bottom to level top.
    fn hexagon(&self, across_flats: f64, bottom: f64, top: f64) -> [Polyline; 2] {
        let radius = across_flats / (2.0 * (PI / 6.0).cos());

        [
            self.ring(0.0, 0.0, radius, 6, 0.0, bottom),
            self.ring(0.0, 0.0, radius, 6, 0.0, top),
        ]
    }

    /// The four anchors from the plate top down to their embedment.
    pub fn anchors(&self) -> Vec<Line> {
        let half = self.base_plate_hole_spacing * 0.5;
        let corners = [(-half, -half), (half, -half), (half, half), (-half, half)];

        corners
            .iter()
            .map(|&(a, b)| {
                Line::from_points(
                    &self.local(a, b, self.base_plate_thickness),
                    &self.local(a, b, -self.anchor_embedment),
                )
            })
            .collect()
    }

    /// The base plate square and its four drillings at its bottom and top, outer loop first.
    fn base_plate(&self) -> [Vec<Polyline>; 2] {
        let half = self.base_plate_size * 0.5;
        let radius = self.base_plate_hole_diameter * 0.5;
        let corners = circle_segments(radius, self.chord_tolerance);
        let level = |z: f64| {
            let square = [
                (-half, -half),
                (half, -half),
                (half, half),
                (-half, half),
                (-half, -half),
            ];
            let mut loops = vec![Polyline::new(
                square.iter().map(|&(a, b)| self.local(a, b, z)).collect(),
            )];

            for anchor in self.anchors() {
                let offset = anchor.start() - self.plane.origin();
                loops.push(self.ring(
                    offset.dot(&self.plane.x_axis()),
                    offset.dot(&self.plane.y_axis()),
                    radius,
                    corners,
                    0.0,
                    z,
                ));
            }

            loops
        };

        [level(0.0), level(self.base_plate_thickness)]
    }

    /// The base plate drilled by its anchors, exact, in place.
    fn base_plate_brep(&self) -> BRep {
        let half = self.base_plate_hole_spacing * 0.5;
        let centres = [[-half, -half], [half, -half], [half, half], [-half, half]];
        let plane = &self.plane;
        let place = Xform::frame_to_world(
            &plane.origin(),
            &plane.x_axis(),
            &plane.y_axis(),
            &plane.z_axis(),
        );

        drilled_plate_brep(
            self.base_plate_size * 0.5,
            self.base_plate_thickness,
            &centres,
            self.base_plate_hole_diameter * 0.5,
        )
        .transformed(&place)
    }

    /// The level the coupling nut starts at: under the head plate by the nut's own height.
    fn coupling_level(&self) -> f64 {
        self.height - self.head_plate_thickness - self.coupling_nut_height
    }
}

impl WoodElement for Support {
    const TYPE: &'static str = "Support";
    type Payload = proto::Support;

    fn from_payload(name: &str, p: proto::Support) -> Option<Self> {
        Some(Self {
            name: name.to_string(),
            plane: Plane::from_proto(p.plane?),
            height: p.height,
            head_plate_diameter: p.head_plate_diameter,
            head_plate_thickness: p.head_plate_thickness,
            head_plate_recess: p.head_plate_recess,
            base_plate_size: p.base_plate_size,
            base_plate_thickness: p.base_plate_thickness,
            base_plate_hole_diameter: p.base_plate_hole_diameter,
            base_plate_hole_spacing: p.base_plate_hole_spacing,
            adjustment_nut_across_flats: p.adjustment_nut_across_flats,
            adjustment_nut_top: p.adjustment_nut_top,
            rod_diameter: p.rod_diameter,
            coupling_nut_across_flats: p.coupling_nut_across_flats,
            coupling_nut_height: p.coupling_nut_height,
            pin_count: p.pin_count,
            pin_diameter: p.pin_diameter,
            pin_length: p.pin_length,
            pin_angle: p.pin_angle,
            pin_circle_diameter: p.pin_circle_diameter,
            anchor_diameter: p.anchor_diameter,
            anchor_embedment: p.anchor_embedment,
            chord_tolerance: p.chord_tolerance,
        })
    }

    fn payload(&self) -> proto::Support {
        proto::Support {
            plane: Some(self.plane.to_proto()),
            height: self.height,
            head_plate_diameter: self.head_plate_diameter,
            head_plate_thickness: self.head_plate_thickness,
            head_plate_recess: self.head_plate_recess,
            base_plate_size: self.base_plate_size,
            base_plate_thickness: self.base_plate_thickness,
            base_plate_hole_diameter: self.base_plate_hole_diameter,
            base_plate_hole_spacing: self.base_plate_hole_spacing,
            adjustment_nut_across_flats: self.adjustment_nut_across_flats,
            adjustment_nut_top: self.adjustment_nut_top,
            rod_diameter: self.rod_diameter,
            coupling_nut_across_flats: self.coupling_nut_across_flats,
            coupling_nut_height: self.coupling_nut_height,
            pin_count: self.pin_count,
            pin_diameter: self.pin_diameter,
            pin_length: self.pin_length,
            pin_angle: self.pin_angle,
            pin_circle_diameter: self.pin_circle_diameter,
            anchor_diameter: self.anchor_diameter,
            anchor_embedment: self.anchor_embedment,
            chord_tolerance: self.chord_tolerance,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn solid(&self) -> Mesh {
        let [bottom, top] = self.base_plate();
        let adjustment = self.hexagon(
            self.adjustment_nut_across_flats,
            self.base_plate_thickness,
            self.adjustment_nut_top,
        );
        let coupling = self.hexagon(
            self.coupling_nut_across_flats,
            self.coupling_level(),
            self.coupling_level() + self.coupling_nut_height,
        );
        let rod = Line::from_points(
            &self.at(self.adjustment_nut_top),
            &self.at(self.coupling_level()),
        );
        let head = Line::from_points(
            &self.at(self.height - self.head_plate_thickness),
            &self.at(self.height),
        );

        let mut mesh = Mesh::loft(&bottom, &top, true, true);
        append_mesh(
            &mut mesh,
            &Mesh::loft(&adjustment[..1], &adjustment[1..], true, true),
        );
        append_mesh(
            &mut mesh,
            &drill_mesh(&rod, self.rod_diameter * 0.5, self.chord_tolerance),
        );
        append_mesh(
            &mut mesh,
            &Mesh::loft(&coupling[..1], &coupling[1..], true, true),
        );
        append_mesh(
            &mut mesh,
            &drill_mesh(&head, self.head_plate_diameter * 0.5, self.chord_tolerance),
        );
        mesh
    }

    /// All exact.
    fn brep(&self) -> BRep {
        let adjustment = self.hexagon(
            self.adjustment_nut_across_flats,
            self.base_plate_thickness,
            self.adjustment_nut_top,
        );
        let coupling = self.hexagon(
            self.coupling_nut_across_flats,
            self.coupling_level(),
            self.coupling_level() + self.coupling_nut_height,
        );
        let rod = Line::from_points(
            &self.at(self.adjustment_nut_top),
            &self.at(self.coupling_level()),
        );
        let head = Line::from_points(
            &self.at(self.height - self.head_plate_thickness),
            &self.at(self.height),
        );

        let mut brep = self.base_plate_brep();
        append_brep(
            &mut brep,
            brep_between_loops(&adjustment[..1], &adjustment[1..]),
        );
        append_brep(&mut brep, drill_brep(&rod, self.rod_diameter * 0.5));
        append_brep(
            &mut brep,
            brep_between_loops(&coupling[..1], &coupling[1..]),
        );
        append_brep(&mut brep, drill_brep(&head, self.head_plate_diameter * 0.5));
        brep
    }

    fn base_plane(&self) -> Option<Plane> {
        Some(self.plane.clone())
    }
}
