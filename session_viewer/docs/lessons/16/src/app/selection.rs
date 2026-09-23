//! What is selected: a whole object, or one edge inside it, which the outline pass and the commands both read.
use session_rust::element::ElementGeometry;
use session_rust::{Geometry, NurbsCurve, NurbsSurface, Point};

/// One control point of a geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub enum ControlId {
    Vertex(usize), // mesh, polyline or BRep vertex
    Curve { curve: usize, point: usize },
    Surface { surface: usize, u: usize, v: usize }, // surface control point
    Point(u32),
}

/// What is selected inside one object.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub enum SelectionMode {
    #[default]
    Object,
    Edge {
        parent: u32, // object row
        edge: u32,
    },
    Controls {
        parent: u32, // object row
        selected: Option<ControlId>,
        cloud: bool, // parent is a point cloud
    },
}

impl SelectionMode {
    /// The object row of a sub-selection.
    pub fn parent(&self) -> Option<u32> {
        match self {
            Self::Object => None,
            Self::Edge { parent, .. } | Self::Controls { parent, .. } => Some(*parent),
        }
    }

    /// Select one edge.
    pub fn select_edge(&mut self, parent: u32, edge: u32) {
        *self = Self::Edge { parent, edge };
    }

    /// Show controls of `parent`; false when nothing changed.
    pub fn enable_controls(&mut self, parent: Option<u32>, cloud: bool) -> bool {
        let Some(parent) = parent else { return false };

        if matches!(self, Self::Controls { parent: active, .. } if *active == parent) {
            return false;
        }

        *self = Self::Controls {
            parent,
            selected: None,
            cloud,
        };
        true
    }

    /// Back to object selection; returns the parent.
    pub fn escape(&mut self) -> Option<u32> {
        let parent = self.parent();
        *self = Self::Object;
        parent
    }
}

/// One control point and where it is.
#[derive(Clone, Debug)]
pub struct Control {
    pub id: ControlId,
    pub position: [f64; 3],
}

/// Every control of one object and the lines between them.
#[derive(Default)]
pub struct Controls {
    pub points: Vec<Control>,
    pub links: Vec<[usize; 2]>, // control net lines, by index
    pub cloud: bool, // a point cloud, controls stay on the GPU
}

impl Controls {
    /// Add one control; None when its position is not finite.
    fn push(&mut self, id: ControlId, point: &Point) -> Option<usize> {
        let position = [point[0], point[1], point[2]];

        if !position.into_iter().all(f64::is_finite) {
            return None;
        }

        let index = self.points.len();
        self.points.push(Control { id, position });
        Some(index)
    }

    /// The controls of one geometry.
    pub fn from_geometry(geometry: &Geometry) -> Self {
        let mut controls = Self::default();
        controls.append_geometry(geometry);
        controls
    }

    /// Add the controls of one geometry.
    fn append_geometry(&mut self, geometry: &Geometry) {
        match geometry {
            Geometry::Mesh(mesh) => self.mesh(mesh),
            Geometry::Line(line) => {
                self.push(ControlId::Vertex(0), &line.start());
                self.push(ControlId::Vertex(1), &line.end());

                if self.points.len() == 2 {
                    self.links.push([0, 1]);
                }
            }
            Geometry::Polyline(polyline) => {
                let mut previous = None;

                for (index, coords) in polyline.coords.chunks_exact(3).enumerate() {
                    let current = self.push(
                        ControlId::Vertex(index),
                        &Point::new(coords[0], coords[1], coords[2]),
                    );

                    if let (Some(start), Some(end)) = (previous, current) {
                        self.links.push([start, end]);
                    }

                    previous = current;
                }
            }
            Geometry::NurbsCurve(curve) => self.curve(curve, 0),
            Geometry::NurbsSurface(surface) => self.surface(surface, 0),
            Geometry::BRep(brep) => self.brep(brep),
            Geometry::PointCloud(_) => self.cloud = true,
            Geometry::Point(point) => {
                self.push(ControlId::Vertex(0), point);
            }
            Geometry::Element(element) => match element.geometry() {
                ElementGeometry::Mesh(mesh) => self.mesh(mesh),
                ElementGeometry::BRep(brep) => self.brep(brep),
                ElementGeometry::None => {}
            },
            Geometry::Plane(_) | Geometry::OBB(_) => {}
        }
    }

    /// Every mesh vertex.
    fn mesh(&mut self, mesh: &session_rust::Mesh) {
        for key in mesh.vertices() {
            if let Some(vertex) = mesh.vertex.get(&key) {
                self.push(
                    ControlId::Vertex(key),
                    &Point::new(vertex.x, vertex.y, vertex.z),
                );
            }
        }
    }

    /// BRep vertices, then curve and surface nets.
    fn brep(&mut self, brep: &session_rust::BRep) {
        for (index, vertex) in brep.m_vertices.iter().enumerate() {
            self.push(ControlId::Vertex(index), &vertex.point);
        }

        for (index, curve) in brep.m_curves_3d.iter().enumerate() {
            self.curve(curve, index);
        }

        for (index, surface) in brep.m_surfaces.iter().enumerate() {
            self.surface(surface, index);
        }
    }

    /// A curve's control polygon.
    fn curve(&mut self, curve: &NurbsCurve, index: usize) {
        let mut previous = None;

        for point in 0..curve.cv_count() {
            let current = match curve.get_cv(point) {
                Some(position) => self.push(
                    ControlId::Curve {
                        curve: index,
                        point,
                    },
                    &position,
                ),
                None => None,
            };

            if let (Some(start), Some(end)) = (previous, current) {
                self.links.push([start, end]);
            }

            previous = current;
        }
    }

    /// A surface's control net.
    fn surface(&mut self, surface: &NurbsSurface, index: usize) {
        let [width, height] = surface.m_cv_count;
        let mut previous = vec![None; height];

        for u in 0..width {
            let mut last = None;

            for (v, above) in previous.iter_mut().enumerate() {
                let current = match surface.get_cv(u, v) {
                    Some(position) => self.push(
                        ControlId::Surface {
                            surface: index,
                            u,
                            v,
                        },
                        &position,
                    ),
                    None => None,
                };

                if let (Some(start), Some(end)) = (*above, current) {
                    self.links.push([start, end]);
                }

                if let (Some(start), Some(end)) = (last, current) {
                    self.links.push([start, end]);
                }

                *above = current;
                last = current;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A new parent replaces the old selection; escape keeps the parent.
    #[test]
    fn switching_parent_replaces_specialized_selection_and_escape_retains_parent() {
        let mut mode = SelectionMode::default();
        mode.select_edge(4, 17);
        assert!(mode.enable_controls(Some(4), false));
        assert!(!mode.enable_controls(Some(4), false));
        mode.select_edge(9, 3);
        assert_eq!(mode, SelectionMode::Edge { parent: 9, edge: 3 });
        assert_eq!(mode.escape(), Some(9));
        assert_eq!(mode, SelectionMode::Object);
        assert!(!mode.enable_controls(None, false));
    }

    /// A line's controls are its two ends.
    #[test]
    fn line_controls_use_original_endpoints() {
        let geometry = Geometry::Line(session_rust::Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).into());
        let controls = Controls::from_geometry(&geometry);
        assert_eq!(controls.points.len(), 2);
        assert_eq!(controls.points[1].position, [4.0, 5.0, 6.0]);
        assert_eq!(controls.points[1].id, ControlId::Vertex(1));
        assert_eq!(controls.links, [[0, 1]]);
    }
}
