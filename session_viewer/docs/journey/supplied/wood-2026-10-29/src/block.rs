use crate::element::WoodElement;
use crate::geometry::{brep_between_loops, frame_along, mesh_brep};
use crate::proto;
use session_rust::{BRep, Mesh, Plane, Polyline};

/// A solid block, as wood's Block (written under "Solid").
#[derive(Clone, Debug)]
pub struct Block {
    pub name: String,                                        // Element name.
    pub loops: Vec<Polyline>, // Bottom loop, top loop, then their holes paired in order.
    pub cuts: Vec<Plane>,     // Planes the solid is cut by, kept by the C++ side.
    pub source_mesh: Option<Mesh>, // The solid when it came as a mesh.
    pub solid_features: Vec<proto::InteractionFeatureSolid>, // Features other elements put on it, carried untouched.
    pub plane_features: Vec<proto::InteractionFeaturePlane>, // Planes other elements cut it by, carried untouched.
}

impl Block {

    /// A block lofted between a bottom and a top loop, then hole pairs.
    pub fn new(loops: Vec<Polyline>, name: &str) -> Self {
        Self {
            name: name.to_string(),
            loops,
            cuts: Vec::new(),
            source_mesh: None,
            solid_features: Vec::new(),
            plane_features: Vec::new(),
        }
    }

    /// The loops as the bottom list and the top list; empty for an odd count or fewer than two.
    fn split_loops(&self) -> (Vec<Polyline>, Vec<Polyline>) {
        if self.loops.len() < 2 || !self.loops.len().is_multiple_of(2) {
            return (Vec::new(), Vec::new());
        }

        let bottom = self.loops.iter().step_by(2).cloned().collect();
        let top = self.loops.iter().skip(1).step_by(2).cloned().collect();

        (bottom, top)
    }
}

impl WoodElement for Block {
    const TYPE: &'static str = "Solid";
    const LEGACY_TYPE: &'static str = "BlockElement";
    type Payload = proto::Block;

    fn from_payload(name: &str, payload: proto::Block) -> Option<Self> {
        Some(Self {
            name: name.to_string(),
            loops: payload
                .loops
                .into_iter()
                .map(Polyline::from_proto)
                .collect(),
            cuts: payload.cuts.into_iter().map(Plane::from_proto).collect(),
            source_mesh: payload.source_mesh.map(Mesh::from_proto),
            solid_features: payload.solid_features,
            plane_features: payload.plane_features,
        })
    }

    fn payload(&self) -> proto::Block {
        proto::Block {
            source_mesh: self.source_mesh.as_ref().map(Mesh::to_proto),
            solid_features: self.solid_features.clone(),
            plane_features: self.plane_features.clone(),
            loops: self.loops.iter().map(Polyline::to_proto).collect(),
            cuts: self.cuts.iter().map(Plane::to_proto).collect(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn solid(&self) -> Mesh {
        let (bottom, top) = self.split_loops();

        match bottom.is_empty() {
            true => self.source_mesh.clone().unwrap_or_default(),
            false => Mesh::loft(&bottom, &top, true, true),
        }
    }

    fn brep(&self) -> BRep {
        let (bottom, top) = self.split_loops();

        match bottom.is_empty() {
            true => mesh_brep(&self.solid()),
            false => brep_between_loops(&bottom, &top),
        }
    }

    fn base_plane(&self) -> Option<Plane> {
        if self.loops.len() < 2 || self.loops[0].point_count() < 3 {
            return None;
        }

        let bottom = &self.loops[0];
        let mut up = bottom.clone().get_plane().z_axis();

        if up.dot(&(self.loops[1].center() - bottom.center())) < 0.0 {
            up = -up;
        }

        frame_along(
            bottom.get_point(0)?,
            bottom.get_point(1)? - bottom.get_point(0)?,
            up,
        )
    }
}
