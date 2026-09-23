use super::State;
use crate::app::clipping::{self, Mode};
use crate::engine::gpu::Instance;
use crate::engine::gpu::clip::{ClipPlane, MAX_PLANES};
use session_rust::element::ElementGeometry;
use session_rust::{Geometry, Plane};
use std::rc::Rc;

impl State {
    /// Cut with every clipping plane this frame, where its row is drawn; hidden planes still cut.
    pub(crate) fn update_clipping(&mut self) {
        let mut planes = [ClipPlane::default(); MAX_PLANES];
        let mut count = 0;
        let mut hidden = 0;

        for &row in self.gpu.objects.clipping_rows() {
            if !self.gpu.clip.enabled || count == MAX_PLANES {
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
        if hidden > self.clip_hidden {
            self.status(
                "The clipping plane is hidden and still cuts · clipping_plane Off shows everything · Delete removes it",
            );
        }

        self.clip_hidden = hidden;

        // the first cut: every mesh walked so far is checked for closedness, once
        if count > 0 && clipping::verify_solids() {
            self.verify_solids();
        }

        self.gpu.set_clip_planes(&planes[..count]);
        let scene = &self.scene;
        self.gpu
            .clip
            .find_solids(&self.gpu.objects, |row| scene.face_range(row));
    }

    /// Flag every mesh walked so far as closed or not, once; open edges already say not.
    fn verify_solids(&mut self) {
        let skip = Instance::FLAG_OPEN
            | Instance::FLAG_DEAD
            | Instance::FLAG_PRINT
            | Instance::FLAG_SHEET
            | Instance::FLAG_SINGLE;

        for row in 0..self.gpu.objects.len() {
            let Some(flags) = self.gpu.objects.row(row).map(|object| object.flags) else {
                continue;
            };

            if flags & skip != 0 || flags & Instance::FLAG_HAS_FACES == 0 {
                continue;
            }

            let solid = match self.scene.geometry(row) {
                Some(Geometry::Mesh(mesh)) => clipping::solid_orientation(mesh),
                Some(Geometry::Element(element)) => match element.geometry() {
                    ElementGeometry::Mesh(mesh) => clipping::solid_orientation(mesh),
                    _ => continue,
                },
                _ => continue,
            };
            let bits = clipping::solid_flags(solid);

            for bit in [Instance::FLAG_CLOSED, Instance::FLAG_INWARD] {
                self.gpu
                    .objects
                    .set_flag(&self.gpu.ctx, row, bit, bits & bit != 0);
            }
        }
    }

    /// Half the rectangle of a new clipping plane: a little more than the scene's widest half.
    pub(crate) fn clipping_half_size(&mut self) -> f64 {
        self.refresh_bounds();
        let b = &self.gpu.bounds;
        let half = b.hx.max(b.hy).max(b.hz);

        if b.is_valid() && half.is_finite() && half > 0.0 {
            half * 1.1
        } else {
            500.0
        }
    }

    /// Start picking the points of a clipping plane in `mode`.
    pub(crate) fn pick_clipping_plane(&mut self, mode: Mode) -> String {
        let prefix = format!("{} {}", clipping::NAME, mode.word());
        self.ask_points(clipping::NAME, &prefix, mode.prompts())
    }

    /// Add a clipping plane as one undo step, then select it.
    pub(crate) fn create_clipping_plane(&mut self, plane: Plane) -> Result<String, String> {
        if self.gpu.objects.clipping_rows().len() >= MAX_PLANES {
            return Err(format!("at most {MAX_PLANES} clipping planes"));
        }

        let (doc, guid) = self
            .scene
            .create_geometry(Geometry::Plane(Rc::new(plane)))?;
        self.after_history();
        let row = self.scene.row_of(doc, &guid);
        self.select(row);
        Ok(
            "Created clipping_plane · the gumball moves the cut · clipping_plane Off shows everything · Undo removes it"
                .into(),
        )
    }

    /// Swap the kept and cut sides of every selected clipping plane.
    pub(crate) fn flip_clipping_planes(&mut self) -> Result<String, String> {
        let rows: Vec<u32> = self
            .selected_rows()
            .into_iter()
            .filter(|row| self.gpu.objects.clipping_rows().binary_search(row).is_ok())
            .collect();

        if rows.is_empty() {
            return Err("select a clipping plane to flip".into());
        }

        for &row in &rows {
            let flipped = match self.scene.geometry(row) {
                Some(Geometry::Plane(plane)) => clipping::flipped(plane),
                _ => continue,
            };
            self.scene.commit_geometry(
                row,
                Geometry::Plane(Rc::new(flipped)),
                "flip clipping plane",
            )?;
        }

        self.commit_rows();
        self.select_rows(rows, false);
        Ok("Flipped: the other side is cut away · Undo flips it back".into())
    }

    /// Turn every cut on or off; the planes stay either way.
    pub(crate) fn set_clipping(&mut self, on: bool) -> String {
        self.gpu.clip.enabled = on;
        self.touch();

        if on {
            "clipping_plane On: the planes cut again".into()
        } else {
            "clipping_plane Off: everything shows, the planes stay".into()
        }
    }

    /// Fill the section caps with black hatch or solid dark grey; `None` switches.
    pub(crate) fn set_clipping_fill(&mut self, solid: Option<bool>) -> String {
        let solid = solid.unwrap_or(self.gpu.clip.fill == 0);
        self.gpu.clip.fill = u32::from(solid);
        self.touch();
        format!(
            "clipping_plane Fill {}",
            if solid { "Solid" } else { "Hatch" }
        )
    }

    /// The clipping planes as JSON, for the inspection tests.
    pub fn clipping_status(&self) -> serde_json::Value {
        let planes: Vec<[f64; 4]> = self
            .gpu
            .clip
            .planes()
            .iter()
            .map(|p| [p.normal[0], p.normal[1], p.normal[2], p.offset])
            .collect();
        serde_json::json!({
            "enabled": self.gpu.clip.enabled,
            "fill": if self.gpu.clip.fill == 1 { "Solid" } else { "Hatch" },
            "count": planes.len(),
            "planes": planes,
            "rows": self.gpu.objects.clipping_rows(),
            "hidden": self.clip_hidden,
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
