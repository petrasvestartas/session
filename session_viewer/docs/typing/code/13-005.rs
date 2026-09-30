
// Control point = a point that shapes a curve, surface or mesh; moving it reshapes the geometry.
// Control net = the thin lines joining neighbouring control points.
impl State {
    /// F10: show the control points of the selected object.
    pub fn enable_controls(&mut self) {
        let Some(parent) = self.scene.selected else {
            self.status("Select one object before pressing F10");
            return;
        };

        // already on
        // `matches!` tests a pattern, here with an `if` guard, and returns a bool
        if matches!(self.selection, SelectionMode::Controls { parent: active, .. } if active == parent)
        {
            return;
        }
        // a released document comes back first
        let mut released = false;

        if released {
            return;
        }

        // the points come from the source geometry
        let controls = match self.scene.geometry(parent) {
            Some(geometry) => Controls::from_geometry(geometry),
            None => {
                self.status("Source controls are unavailable for this display-only object");
                return;
            }
        };

        if !controls.cloud && controls.points.is_empty() {
            self.status("This object has no selectable source controls");
            return;
        }

        // the gizmo would take the same clicks
        self.selection.enable_controls(Some(parent), controls.cloud);
        self.gpu.arena.source_faces.select(&self.gpu.ctx, None);
        self.gpu.segments.set_edge(&self.gpu.ctx, None);
        self.gpu.set_selected(parent, false);
        // `then_some` turns true into Some(parent) and false into None
        self.gpu
            .splat
            .set_controls(controls.cloud.then_some(parent));
        self.controls = controls;
        self.upload_controls(); // register:controls
        self.status("Control points: click to select; Esc to leave");
        self.touch();
    }

    /// Send the control dots and their links to the GPU.
    pub(crate) fn upload_controls(&mut self) {
        // start empty, so nothing doubles
        self.gpu.controls.reset();
        self.gpu.control_net.reset();
        let SelectionMode::Controls {
            parent, selected, ..
        } = self.selection
        else {
            return;
        };
        let scale = f64::from(self.gpu.config.width) / self.logical_size()[0]; // device pixels per CSS pixel
        let mut glyphs = GlyphRows::default();

        // one dot per control point
        for control in &self.controls.points {
            let color = if Some(control.id) == selected {
                [1.0, 1.0, 0.0, 1.0] // yellow: selected
            } else {
                [0.15, 0.35, 0.9, 1.0] // blue
            };
            glyphs.dots.push(GlyphPoint {
                center: render_position(control.position),
                radius: -3.5 * scale as f32, // negative: a pixel size, not a world size
                color,
                instance_id: parent,
                facing: FACING_UNKNOWN,          // no face orientation
                facing_ext: [FACING_UNKNOWN; 2], // no neighbour orientation
            });
        }

        let mut segments = SegRows::default();

        // one thin line per link between control points
        for &[start, end] in &self.controls.links {
            segments.ribbons.push(CylinderSegment {
                p0: render_position(self.controls.points[start].position),
                p1: render_position(self.controls.points[end].position),
                radius: 0.0,
                color: 0xffcc8866,
                instance_id: parent,
                facing: FACING_UNKNOWN, // no face orientation
            });
        }

        self.gpu
            .controls
            .append(&self.gpu.ctx, &self.gpu.layouts, &glyphs);
        self.gpu
            .control_net
            .append(&self.gpu.ctx, &self.gpu.layouts, &segments);
    }

    /// A control point was clicked: select it.
    fn apply_control(&mut self, pick: Pick, cloud: bool) {
        let SelectionMode::Controls { parent, .. } = self.selection else {
            return;
        };

        if pick.row != parent {
            return;
        }

        let id = if cloud {
            // a cloud point: find its source id
            let Some((owner, local)) = self.gpu.cloud.row_of(pick.sub) else {
                return;
            };

            if owner != parent {
                return;
            }

            self.gpu.splat.set_point(Some(pick.sub));
            let Some(source) = self.scene.point_at(parent, local) else {
                self.status("Source point ID unavailable; no local display ID was substituted");
                return;
            };
            ControlId::Point(source.id)
        } else {
            // a control dot: top two bits 01, index in the rest
            if pick.sub & 0xc000_0000 != 0x4000_0000 {
                return;
            }

            let Some(control) = self.controls.points.get((pick.sub & 0x3fff_ffff) as usize) else {
                return;
            };
            control.id
        };
        self.selection = SelectionMode::Controls {
            parent,
            selected: Some(id),
            cloud,
        };
        self.upload_controls(); // register:controls
        self.status(&format!("Selected {id:?}"));
        self.touch();
    }
}
