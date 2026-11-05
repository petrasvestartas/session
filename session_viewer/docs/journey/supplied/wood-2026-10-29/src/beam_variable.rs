use crate::element::{polyline_feature, WoodElement};
use crate::geometry::{frame_along, loft_stations, mesh_brep};
use crate::proto;
use session_rust::element::ElementFeature;
use session_rust::{BRep, Line, Mesh, Plane, Point, Polyline};

/// A beam whose section changes along a straight axis, as wood's BeamVariable.
#[derive(Clone, Debug)]
pub struct BeamVariable {
    pub name: String,                                        // Element name.
    pub axis: Line, // Straight reference line from the first section to the last.
    pub sections: Vec<Polyline>, // Closed rings with one point count, one per station in axis order.
    pub cuts: Vec<Plane>,        // Planes the solid is cut by, kept by the C++ side.
    pub solid_features: Vec<proto::InteractionFeatureSolid>, // Features other elements put on it, carried untouched.
    pub plane_features: Vec<proto::InteractionFeaturePlane>, // Planes other elements cut it by, carried untouched.
}

impl BeamVariable {

    /// A variable beam through its sections along an axis.
    pub fn new(axis: &Line, sections: Vec<Polyline>, name: &str) -> Self {
        Self {
            name: name.to_string(),
            axis: axis.clone(),
            sections,
            cuts: Vec::new(),
            solid_features: Vec::new(),
            plane_features: Vec::new(),
        }
    }

    /// A variable beam through its sections, the axis from the first section's centre to the last's.
    pub fn through(sections: Vec<Polyline>, name: &str) -> Self {
        let first: Point = sections.first().map(Polyline::center).unwrap_or_default();
        let last: Point = sections.last().map(Polyline::center).unwrap_or_default();

        Self::new(&Line::from_points(&first, &last), sections, name)
    }
}

impl WoodElement for BeamVariable {
    const TYPE: &'static str = "BeamVariable";
    type Payload = proto::BeamVariable;

    fn from_payload(name: &str, payload: proto::BeamVariable) -> Option<Self> {
        Some(Self {
            name: name.to_string(),
            axis: Line::from_proto(payload.axis?),
            sections: payload
                .sections
                .into_iter()
                .map(Polyline::from_proto)
                .collect(),
            cuts: payload.cuts.into_iter().map(Plane::from_proto).collect(),
            solid_features: payload.solid_features,
            plane_features: payload.plane_features,
        })
    }

    fn payload(&self) -> proto::BeamVariable {
        proto::BeamVariable {
            axis: Some(self.axis.to_proto()),
            sections: self.sections.iter().map(Polyline::to_proto).collect(),
            cuts: self.cuts.iter().map(Plane::to_proto).collect(),
            solid_features: self.solid_features.clone(),
            plane_features: self.plane_features.clone(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn solid(&self) -> Mesh {
        loft_stations(&self.sections)
    }

    fn brep(&self) -> BRep {
        mesh_brep(&self.solid())
    }

    /// Its axis through the stations, its two end sections and its top and bottom outlines.
    fn features(&self) -> Vec<ElementFeature> {
        let stations: Vec<Point> = self
            .sections
            .iter()
            .map(|section| self.axis.closest_point(&section.center(), false).1)
            .collect();
        let axis = match stations.len() > 1 {
            true => Polyline::new(stations),
            false => Polyline::new(vec![self.axis.start(), self.axis.end()]),
        };
        let mut features = vec![polyline_feature("axis", &axis, -1)];
        let ends: Vec<&Polyline> = self
            .sections
            .iter()
            .filter(|s| s.point_count() > 0)
            .collect();

        // the end sections only: the inner ones would draw lines across the merged faces
        if let Some(first) = ends.first() {
            features.push(polyline_feature("section", first, -1));
        }

        if ends.len() > 1 {
            features.push(polyline_feature("section", ends[ends.len() - 1], -1));
        }

        let top = side_outline(&self.sections, top_side(&self.sections));

        if top.point_count() > 0 {
            features.push(polyline_feature("top", &top, -1));
            let bottom = side_outline(&self.sections, (top_side(&self.sections) + 2) % 4);
            features.push(polyline_feature("bottom", &bottom, -1));
        }

        features
    }

    fn base_plane(&self) -> Option<Plane> {
        let first = self.sections.first()?;

        frame_along(
            self.axis.start(),
            first.get_point(1)? - first.get_point(0)?,
            self.axis.to_vector(),
        )
    }
}

/// The face of side k of four-cornered sections.
fn side_outline(sections: &[Polyline], k: usize) -> Polyline {
    if sections.len() < 2 || sections.iter().any(|section| section.point_count() != 5) {
        return Polyline::new(Vec::new());
    }

    let mut ring: Vec<Point> = sections
        .iter()
        .filter_map(|section| section.get_point(k))
        .collect();
    let back: Vec<Point> = sections
        .iter()
        .filter_map(|section| section.get_point((k + 1) % 4))
        .collect();
    ring.extend(back.into_iter().rev());

    match ring.len() < 3 {
        true => Polyline::new(Vec::new()),
        false => Polyline::new(ring).closed(),
    }
}

/// The height of side k of a four-cornered section: the sum of its two corners' z.
fn side_height(section: &Polyline, k: usize) -> f64 {
    let z = |i: usize| section.get_point(i).map_or(0.0, |p| p[2]);
    z(k) + z((k + 1) % 4)
}

/// The side of the first section highest in z, by the midpoint of its two corners.
fn top_side(sections: &[Polyline]) -> usize {
    match sections.first() {
        Some(first) if first.point_count() == 5 => (1..4).fold(0, |top, k| {
            if side_height(first, k) > side_height(first, top) {
                k
            } else {
                top
            }
        }),
        _ => 0,
    }
}
