use crate::element::{polyline_feature, WoodElement};
use crate::geometry::brep_between_loops;
use crate::proto;
use session_rust::element::ElementFeature;
use session_rust::{BRep, Line, Mesh, Plane, Point, Polyline, Tolerance, Vector, Xform};

/// A timber column: a closed section at the axis base, or a profile placed there, as wood's Column.
#[derive(Clone, Debug)]
pub struct Column {
    pub name: String,                                        // Element name.
    pub axis: Line,                                          // Centreline, base to head.
    pub section: Polyline,      // Closed cross-section about the axis base.
    pub cuts: Vec<Plane>,       // Planes the solid is cut by, kept by the C++ side.
    pub profile: Vec<Polyline>, // Section loops in the profile frame; empty when the section was given.
    pub rotation: f64,          // Degrees the profile x turns from world x about the axis.
    pub solid_features: Vec<proto::InteractionFeatureSolid>, // Features other elements put on it, carried untouched.
    pub plane_features: Vec<proto::InteractionFeaturePlane>, // Planes other elements cut it by, carried untouched.
}

/// The profile x axis in world space: world x across the axis (world y along x), turned by rotation degrees.
fn profile_x(axis: &Line, rotation: f64) -> Vector {
    let along = axis.to_vector().normalized();
    let mut x = Vector::x_axis() - along.clone() * along.dot(&Vector::x_axis());

    if x.magnitude() < Tolerance::RELATIVE {
        x = Vector::y_axis() - along.clone() * along.dot(&Vector::y_axis());
    }

    x.normalized()
        .transformed(&Xform::rotation(&along, rotation, true))
}

/// Every profile loop placed at the axis base, x along profile_x and y along the axis cross it.
fn placed_profile(axis: &Line, profile: &[Polyline], rotation: f64) -> Vec<Polyline> {
    let along = axis.to_vector().normalized();
    let x = profile_x(axis, rotation);
    let y = along.cross(&x);
    let place = |point: &Point| axis.start() + x.clone() * point[0] + y.clone() * point[1];

    profile
        .iter()
        .map(|ring| Polyline::new(ring.get_points().iter().map(place).collect()))
        .collect()
}

impl Column {

    /// A column of a given section along its axis.
    pub fn new(axis: &Line, section: &Polyline, name: &str) -> Self {
        Self {
            name: name.to_string(),
            axis: axis.clone(),
            section: section.clone(),
            cuts: Vec::new(),
            profile: Vec::new(),
            rotation: 0.0,
            solid_features: Vec::new(),
            plane_features: Vec::new(),
        }
    }

    /// A column of a profile placed at the axis base, its x turned by rotation degrees from world x.
    pub fn from_profile(axis: &Line, profile: Vec<Polyline>, rotation: f64, name: &str) -> Self {
        let section = match profile.is_empty() || axis.length() <= 0.0 {
            true => Polyline::new(Vec::new()),
            false => placed_profile(axis, &profile, rotation).remove(0),
        };

        Self {
            profile,
            rotation,
            ..Self::new(axis, &section, name)
        }
    }

    /// The bottom loops and the same moved along the axis; None without a section or an axis.
    fn loops(&self) -> Option<(Vec<Polyline>, Vec<Polyline>)> {
        if self.section.point_count() < 3 || self.axis.length() <= 0.0 {
            return None;
        }

        let bottom = self.bottom_loops();
        let top = bottom
            .iter()
            .map(|ring| ring.translated(&self.axis.to_vector()))
            .collect();

        Some((bottom, top))
    }

    /// The loops the solid starts from: the placed profile when it has holes, else the section.
    fn bottom_loops(&self) -> Vec<Polyline> {
        match self.profile.len() > 1 {
            true => placed_profile(&self.axis, &self.profile, self.rotation),
            false => vec![self.section.clone()],
        }
    }
}

impl WoodElement for Column {
    const TYPE: &'static str = "Column";
    type Payload = proto::Column;

    fn from_payload(name: &str, payload: proto::Column) -> Option<Self> {
        Some(Self {
            name: name.to_string(),
            axis: Line::from_proto(payload.axis?),
            section: payload
                .section
                .map(Polyline::from_proto)
                .unwrap_or_else(|| Polyline::new(Vec::new())),
            cuts: payload.cuts.into_iter().map(Plane::from_proto).collect(),
            profile: payload
                .profile
                .into_iter()
                .map(Polyline::from_proto)
                .collect(),
            rotation: payload.rotation,
            solid_features: payload.solid_features,
            plane_features: payload.plane_features,
        })
    }

    fn payload(&self) -> proto::Column {
        proto::Column {
            solid_features: self.solid_features.clone(),
            plane_features: self.plane_features.clone(),
            axis: Some(self.axis.to_proto()),
            section: Some(self.section.to_proto()),
            cuts: self.cuts.iter().map(Plane::to_proto).collect(),
            profile: self.profile.iter().map(Polyline::to_proto).collect(),
            rotation: self.rotation,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn solid(&self) -> Mesh {
        match self.loops() {
            Some((bottom, top)) => Mesh::loft(&bottom, &top, true, true),
            None => Mesh::new(),
        }
    }

    fn brep(&self) -> BRep {
        match self.loops() {
            Some((bottom, top)) => brep_between_loops(&bottom, &top),
            None => BRep::new(),
        }
    }

    /// Its axis and its base section.
    fn features(&self) -> Vec<ElementFeature> {
        let axis = Polyline::new(vec![self.axis.start(), self.axis.end()]);
        let mut features = vec![polyline_feature("axis", &axis, -1)];

        if self.section.point_count() > 0 {
            features.push(polyline_feature("section", &self.section, -1));
        }

        features
    }

    fn base_plane(&self) -> Option<Plane> {
        if self.section.point_count() < 3 || self.axis.length() <= 0.0 {
            return None;
        }

        let z = self.axis.to_vector().normalized();
        let origin = self.section.get_point(0)?;
        let x = (self.section.get_point(1)? - origin.clone()).normalized();

        Some(Plane::from_frame(origin, x.clone(), z.cross(&x), z))
    }
}
