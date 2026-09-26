// --8<-- [start:update-clipping]
use super::State;
use crate::app::clipping::{self, Mode};
use crate::engine::gpu::Instance;
use crate::engine::gpu::clip::{Clip, ClipPlane, MAX_PLANES};
use session_rust::element::ElementGeometry;
use session_rust::{Geometry, Plane};
use std::collections::HashMap;
use std::rc::Rc;

impl State {
    /// Cut with every clipping plane this frame, where its row is drawn; hidden planes still cut.
    pub(crate) fn update_clipping(&mut self) {
        let mut planes = [ClipPlane::default(); MAX_PLANES];
        let mut count = 0;
        let mut hidden = 0;
        let enabled = self.gpu.pass::<Clip>().enabled;

        // a clipping plane is an ordinary scene object; its row carries FLAG_CLIPPING_PLANE, so the table lists them
        for &row in self.gpu.objects.clipping_rows() {
            if !enabled || count == MAX_PLANES {
                break;
            }

            let Some(Geometry::Plane(plane)) = self.scene.geometry(row) else {
                continue;
            };
            // where the row is drawn, so a gumball drag moves the cut with it
            let Some(plane) = self
                .gpu
                .objects
                .placement(row)
                .and_then(|place| clipping::clip_plane(plane, &place))
            else {
                continue;
            };
            planes[count] = plane;
            count += 1;
            hidden += usize::from(
                self.gpu
                    .objects
                    .row(row)
                    .is_some_and(|object| object.flags & Instance::FLAG_HIDDEN != 0),
            );
        }

        // a plane was just hidden: say it still cuts
        if hidden > self.features.clip_hidden {
            self.status(
                "The clipping plane is hidden and still cuts · Clipping Plane Off shows everything · Delete removes it",
            );
        }

        self.features.clip_hidden = hidden;

        // the first cut: every mesh walked so far is checked for closedness, once
        if count > 0 && clipping::verify_solids() {
            self.verify_solids();
        }

        self.gpu.set_clip_planes(&planes[..count]);
        let scene = &self.scene;
        self.gpu.find_solids(|row| scene.solid_faces(row));
    }
    // --8<-- [end:update-clipping]

    // --8<-- [start:verify-solids]
    /// Flag every mesh walked so far as closed or not, once; open edges already say not.
    fn verify_solids(&mut self) {
        let skip = Instance::FLAG_OPEN
            | Instance::FLAG_DEAD
            | Instance::FLAG_PRINT
            | Instance::FLAG_SHEET
            | Instance::FLAG_SINGLE;
        // instances of one definition share its geometry: walk it once
        // keyed by the geometry's address: `from_ref` turns a reference into a raw pointer, used only as a number
        let mut walked: HashMap<*const Geometry, u32> = HashMap::new();

        for row in 0..self.gpu.objects.len() {
            let Some(flags) = self.gpu.objects.row(row).map(|object| object.flags) else {
                continue;
            };

            if flags & skip != 0 || flags & Instance::FLAG_HAS_FACES == 0 {
                continue;
            }

            let mut shape = self.scene.geometry(row);
            // --8<-- [start:21-instance-definition]
            shape = shape.or_else(|| self.scene.instance_definition(row)); // register:instancing
            // --8<-- [end:21-instance-definition]

            let Some(shape) = shape else {
                continue;
            };
            let bits = match walked.get(&std::ptr::from_ref(shape)) {
                Some(&bits) => bits,
                None => {
                    let solid = match shape {
                        Geometry::Mesh(mesh) => clipping::solid_orientation(mesh),
                        Geometry::Element(element) => match element.geometry() {
                            ElementGeometry::Mesh(mesh) => clipping::solid_orientation(mesh),
                            _ => continue,
                        },
                        _ => continue,
                    };
                    let bits = clipping::solid_flags(solid);
                    walked.insert(std::ptr::from_ref(shape), bits);
                    bits
                }
            };

            for bit in [Instance::FLAG_CLOSED, Instance::FLAG_INWARD] {
                self.gpu
                    .objects
                    .set_flag(&self.gpu.ctx, row, bit, bits & bit != 0);
            }
        }
    }
    // --8<-- [end:verify-solids]

    // --8<-- [start:clipping-status]
    /// The clipping planes as JSON, for the inspection tests.
    pub fn clipping_status(&self) -> serde_json::Value {
        let clip = self.gpu.pass::<Clip>();
        let planes: Vec<[f64; 4]> = clip
            .planes()
            .iter()
            .map(|p| [p.normal[0], p.normal[1], p.normal[2], p.offset])
            .collect();
        serde_json::json!({
            "enabled": clip.enabled,
            "fill": if clip.fill == 1 { "Solid" } else { "Hatch" },
            "count": planes.len(),
            "planes": planes,
            "rows": self.gpu.objects.clipping_rows(),
            "hidden": self.features.clip_hidden,
            "scene_box": [
                self.gpu.bounds.cx,
                self.gpu.bounds.cy,
                self.gpu.bounds.cz,
                self.gpu.bounds.hx,
                self.gpu.bounds.hy,
                self.gpu.bounds.hz
            ],
        })
    }
}
// --8<-- [end:clipping-status]
