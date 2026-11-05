use crate::element::{polyline_feature, WoodElement};
use crate::geometry::{brep_between_loops, compute_newell, frame_along};
use crate::proto;
use session_rust::element::ElementFeature;
use session_rust::{BRep, Mesh, Plane, Polyline};

/// A timber plate: a bottom and a top outline of one point count, as wood's Plate.
#[derive(Clone, Debug)]
pub struct Plate {
    pub name: String,                                        // Element name.
    pub bottom: Polyline, // Bottom outline, its Newell normal pointing away from the top.
    pub top: Polyline,    // Top outline.
    pub reversed: bool,   // True when the constructor reversed both outlines.
    pub solid_features: Vec<proto::InteractionFeatureSolid>, // Features other elements put on it, carried untouched.
}

impl Plate {

    /// A plate between two outlines, both reversed when the bottom normal points toward the top, as the C++ constructor does.
    pub fn new(bottom: &Polyline, top: &Polyline, name: &str) -> Self {
        let mut bottom = bottom.clone();
        let mut top = top.clone();
        let normal = compute_newell(&bottom.get_points());
        let last = top.get_point(top.point_count().saturating_sub(1));
        let reversed = last.is_some_and(|last| (last - bottom.center()).dot(&normal) > 0.0);

        if reversed {
            bottom.reverse();
            top.reverse();
        }

        Self {
            name: name.to_string(),
            bottom,
            top,
            reversed,
            solid_features: Vec::new(),
        }
    }

    /// A plate of thickness on a closed outline, the top moved along the outline's normal side away from its winding.
    pub fn from_outline(outline: &Polyline, thickness: f64, name: &str) -> Self {
        let normal = compute_newell(&outline.get_points()).normalized();

        Self::new(outline, &outline.translated(&(normal * thickness)), name)
    }
}

impl WoodElement for Plate {
    const TYPE: &'static str = "Plate";
    const LEGACY_TYPE: &'static str = "WoodElement";
    type Payload = proto::Plate;

    fn from_payload(name: &str, payload: proto::Plate) -> Option<Self> {
        Some(Self {
            name: name.to_string(),
            bottom: Polyline::from_proto(payload.bottom?),
            top: Polyline::from_proto(payload.top?),
            reversed: payload.reversed,
            solid_features: payload.solid_features,
        })
    }

    fn payload(&self) -> proto::Plate {
        proto::Plate {
            solid_features: self.solid_features.clone(),
            bottom: Some(self.bottom.to_proto()),
            top: Some(self.top.to_proto()),
            reversed: self.reversed,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn solid(&self) -> Mesh {
        if self.bottom.point_count() < 3 || self.top.point_count() < self.bottom.point_count() {
            return Mesh::new();
        }

        Mesh::loft(
            std::slice::from_ref(&self.bottom),
            std::slice::from_ref(&self.top),
            true,
            true,
        )
    }

    fn brep(&self) -> BRep {
        if self.bottom.point_count() < 3 || self.top.point_count() < self.bottom.point_count() {
            return BRep::new();
        }

        brep_between_loops(
            std::slice::from_ref(&self.bottom),
            std::slice::from_ref(&self.top),
        )
    }

    /// Its bottom and top outlines, faces 0 and 1.
    fn features(&self) -> Vec<ElementFeature> {
        vec![
            polyline_feature("outline", &self.bottom, 0),
            polyline_feature("outline", &self.top, 1),
        ]
    }

    fn base_plane(&self) -> Option<Plane> {
        let along = self.bottom.get_point(1)? - self.bottom.get_point(0)?;

        frame_along(
            self.bottom.get_point(0)?,
            along,
            -compute_newell(&self.bottom.get_points()),
        )
    }
}
