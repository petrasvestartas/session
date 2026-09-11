//! The editing gestures: what a pointer on a gizmo handle, a Delete and a Ctrl+Z do.
//!
//! State is the only place that knows the camera, the scene and the GPU at once, so the join
//! lives here and the four pure modules below it stay free of all three.
//!
//! A drag is three moments. It BEGINS by remembering the object's own transform and the
//! placement it was drawn with. It MOVES by writing a preview into the row's GPU placement -
//! two small writes, no document touched. It ENDS by writing the document once, which is what
//! makes one gesture one undo step.

use crate::app::command::Command;
use crate::app::layers::{self, Layer};
use crate::app::gizmo::{ARM, Axis, BALL_AT, Drag, Gizmo, HUB, Handle};
use crate::app::walk::encode::FACING_UNKNOWN;
use crate::state::render_position;
use crate::engine::gpu::glyphs::{GlyphPoint, GlyphRows};
use crate::engine::gpu::segments::{CylinderSegment, SegRows};
use crate::state::{SelectionMode, State};
use session_rust::{Point, Xform};

/// A drag in progress: the row, its transform when the drag started, and the handle.
pub struct GizmoDrag {
    row: u32,
    /// The object's LOCAL transform at the grab. Every frame's preview is measured from this,
    /// never from the frame before, so a dropped frame changes nothing.
    base_local: Xform,
    /// The full placement at the grab, for the same reason, on the GPU side.
    base_place: [f64; 16],
    drag: Drag,
}

impl State {
    /// Put the gizmo on the selected row's box centre, or take it away. A row with no box -
    /// a single point, a text label - gets no widget rather than one at the world origin.
    pub fn place_gizmo(&mut self, row: Option<u32>) {
        let Some(box_) = row.and_then(|r| self.gpu.objects.row_bounds(r)) else {
            self.gizmo = None;
            self.upload_gizmo();
            return;
        };
        let origin = Point::new(
            f64::from(box_.min[0] + box_.max[0]) * 0.5,
            f64::from(box_.min[1] + box_.max[1]) * 0.5,
            f64::from(box_.min[2] + box_.max[2]) * 0.5,
        );
        match self.gizmo.as_mut() {
            Some(gizmo) => gizmo.set_origin(origin),
            None => self.gizmo = Some(Gizmo::new(origin)),
        }
        self.upload_gizmo();
    }

    /// Try to grab a gizmo handle under the cursor. False means the click was not on the
    /// widget and the caller should go on to pick.
    pub fn begin_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some(row) = self.scene.selected else {
            return false;
        };
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let per_px = self.world_per_px();
        let Some(gizmo) = self.gizmo.as_mut() else {
            return false;
        };
        let Some(handle) = gizmo.hit(&from, &dir, per_px) else {
            return false;
        };
        let Some(drag) = gizmo.begin(handle, &from, &dir) else {
            return false;
        };
        let Some(base_local) = self.scene.local_xform_of(row) else {
            return false;
        };
        let Some(base_place) = self.scene.placement_of(row) else {
            return false;
        };
        self.dragging = Some(GizmoDrag {
            row,
            base_local,
            base_place,
            drag,
        });
        true
    }

    /// Move the object with the pointer. The document is not touched: this is a preview.
    pub fn drag_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some(active) = self.dragging.as_ref() else {
            return false;
        };
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let Some(gizmo) = self.gizmo.as_ref() else {
            return false;
        };
        let Some(delta) = gizmo.update(&active.drag, &from, &dir) else {
            return false;
        };
        let place = crate::math::mat_mul(&delta, &active.base_place);
        self.gpu
            .objects
            .set_placement(&self.gpu.ctx, active.row, &place);
        self.place_gizmo(Some(active.row));
        self.touch();
        true
    }

    /// Let go: the document records the whole gesture as one transaction.
    pub fn end_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some(active) = self.dragging.take() else {
            return false;
        };
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let Some(gizmo) = self.gizmo.as_mut() else {
            return false;
        };
        gizmo.drag = None;
        let Some(delta) = gizmo.update(&active.drag, &from, &dir) else {
            return false;
        };
        let local = Xform::from_matrix(crate::math::mat_mul(&delta, &active.base_local.m));
        let label = match active.drag.handle {
            Handle::Translate(_) => "move",
            Handle::Rotate(_) => "rotate",
            Handle::Scale(_) | Handle::ScaleUniform => "scale",
        };
        if let Some(place) = self.scene.set_row_xform(active.row, local, label) {
            self.gpu
                .objects
                .set_placement(&self.gpu.ctx, active.row, &place);
        }
        self.touch();
        true
    }

    /// Delete the selection. The rows change, so every document is walked again.
    pub fn delete_selected(&mut self) {
        let Some(row) = self.scene.selected else {
            return;
        };
        if !self.scene.delete_row(row) {
            return;
        }
        self.select(None);
        self.scene.rebuild(&mut self.gpu);
        self.place_gizmo(None);
        self.update_label();
        self.touch();
    }

    /// Undo the last edit in the document that was edited last - Ctrl+Z.
    pub fn undo(&mut self) {
        if self.scene.undo() {
            self.after_history();
        }
    }

    /// Redo it - Ctrl+Y, or Ctrl+Shift+Z.
    pub fn redo(&mut self) {
        if self.scene.redo() {
            self.after_history();
        }
    }

    /// An undo can bring an object back or take one away, so the rows are rebuilt rather than
    /// patched. The selection is dropped because the row it named may not exist any more.
    fn after_history(&mut self) {
        self.selection = SelectionMode::Object;
        self.select(None);
        self.scene.rebuild(&mut self.gpu);
        self.place_gizmo(None);
        self.update_label();
        self.touch();
    }

    /// World length of one screen pixel at the gizmo, for a hit test that must feel the same
    /// however far the camera is: the widget is sized in pixels, so its grab radius is too.
    fn world_per_px(&self) -> f64 {
        let (_, h) = self.viewport();
        if h <= 0.0 {
            return 1.0;
        }
        2.0 * self.camera.distance * (crate::math::FOVY_DEG * 0.5).to_radians().tan() / h
    }
}

/// The three axis colours every CAD tool agrees on, as the packed RGBA a stroke row carries.
const AXIS_COLORS: [u32; 3] = [0xff2222dd, 0xff22bb22, 0xffdd4422];

impl State {
    /// Rebuild the widget's rows. Three arms as strokes, three balls and a hub as markers,
    /// in the two lanes the control net already uses - so the widget adds no shader, no
    /// pipeline and no pass, only rows.
    ///
    /// The arms are sized in PIXELS, converted to world at the widget's own depth, so the
    /// gumball is the same size on screen wherever the camera is.
    pub fn upload_gizmo(&mut self) {
        self.gpu.gizmo_arms.reset();
        self.gpu.gizmo_dots.reset();
        let Some(gizmo) = self.gizmo.as_ref() else {
            return;
        };
        let per_px = self.world_per_px();
        let origin = gizmo.origin.clone();
        // World coordinates, so the rows draw against the identity row rather than an object's.
        let widget = self
            .gpu
            .objects
            .widget_row(&self.gpu.ctx, &self.gpu.layouts);
        let arm = ARM * per_px;
        let ball = BALL_AT * per_px;
        let mut segments = SegRows::default();
        let mut glyphs = GlyphRows::default();
        for (i, axis) in [Axis::X, Axis::Y, Axis::Z].into_iter().enumerate() {
            let u = axis.unit();
            let tip = [
                origin[0] + u[0] * arm,
                origin[1] + u[1] * arm,
                origin[2] + u[2] * arm,
            ];
            let at = [
                origin[0] + u[0] * ball,
                origin[1] + u[1] * ball,
                origin[2] + u[2] * ball,
            ];
            segments.ribbons.push(CylinderSegment {
                p0: render_position([origin[0], origin[1], origin[2]]),
                p1: render_position(tip),
                radius: 0.0,
                color: AXIS_COLORS[i],
                instance_id: widget,
                facing: FACING_UNKNOWN,
            });
            glyphs.dots.push(GlyphPoint {
                center: render_position(at),
                radius: -5.0,
                color: unpack_color(AXIS_COLORS[i]),
                instance_id: widget,
                facing: FACING_UNKNOWN,
                facing_ext: [FACING_UNKNOWN; 2],
            });
        }
        glyphs.dots.push(GlyphPoint {
            center: render_position([origin[0], origin[1], origin[2]]),
            radius: -(HUB as f32),
            color: [1.0, 1.0, 1.0, 1.0],
            instance_id: widget,
            facing: FACING_UNKNOWN,
            facing_ext: [FACING_UNKNOWN; 2],
        });
        self.gpu
            .gizmo_arms
            .append(&self.gpu.ctx, &self.gpu.layouts, &segments);
        self.gpu
            .gizmo_dots
            .append(&self.gpu.ctx, &self.gpu.layouts, &glyphs);
    }
}

/// A packed `0xAARRGGBB` row colour as the four floats a marker wants.
fn unpack_color(packed: u32) -> [f32; 4] {
    [
        ((packed >> 16) & 0xff) as f32 / 255.0,
        ((packed >> 8) & 0xff) as f32 / 255.0,
        (packed & 0xff) as f32 / 255.0,
        ((packed >> 24) & 0xff) as f32 / 255.0,
    ]
}

impl State {
    /// Run one typed line. The string that comes back is what to show the person who typed it:
    /// what happened, or why nothing did.
    ///
    /// Every arm calls an action the viewer already has, so a command cannot drift from the
    /// key that does the same thing.
    pub fn run_command(&mut self, line: &str) -> Result<String, String> {
        let command = crate::app::command::parse(line)?;
        let needs_selection = matches!(
            command,
            Command::Move(_) | Command::Rotate { .. } | Command::Scale(_) | Command::Delete
        );
        if needs_selection && self.scene.selected.is_none() {
            return Err("nothing is selected".into());
        }
        match command {
            Command::Move(d) => self.apply(Xform::translation(d[0], d[1], d[2]), "move"),
            Command::Rotate { axis, degrees } => {
                let about = self.gizmo.as_ref().map(|g| g.origin.clone());
                let turn = rotation_about(axis, degrees, about.as_ref());
                self.apply(turn, "rotate")
            }
            Command::Scale(k) => {
                let about = self.gizmo.as_ref().map(|g| g.origin.clone());
                self.apply(scaling_about(k, about.as_ref()), "scale")
            }
            Command::Delete => {
                self.delete_selected();
                Ok("deleted".into())
            }
            Command::Undo => {
                self.undo();
                Ok("undone".into())
            }
            Command::Redo => {
                self.redo();
                Ok("redone".into())
            }
            Command::Hide => {
                self.hide_selected();
                Ok("hidden".into())
            }
            Command::ShowAll => {
                self.show_all();
                Ok("everything shown".into())
            }
            Command::Fit => {
                self.fit_selected_or_all();
                Ok("fitted".into())
            }
            Command::Escape => {
                self.escape_selection();
                Ok("selection cleared".into())
            }
        }
    }

    /// One recorded transform on the selection, with the row's placement written back.
    fn apply(&mut self, delta: Xform, label: &str) -> Result<String, String> {
        let Some(row) = self.scene.selected else {
            return Err("nothing is selected".into());
        };
        let Some(place) = self.scene.transform_row(row, &delta, label) else {
            return Err("this row cannot be edited".into());
        };
        self.gpu
            .objects
            .set_placement(&self.gpu.ctx, row, &place);
        self.place_gizmo(Some(row));
        self.touch();
        Ok(label.into())
    }
}

/// A turn about a point rather than about the world origin: translate the point to the origin,
/// turn, translate back. Without the centring, `rotate z 90` would swing the object around the
/// file's origin, which is rarely where it is.
fn rotation_about(axis: Axis, degrees: f64, about: Option<&Point>) -> Xform {
    let turn = match axis {
        Axis::X => Xform::rotation_x(degrees, true),
        Axis::Y => Xform::rotation_y(degrees, true),
        Axis::Z => Xform::rotation_z(degrees, true),
    };
    centred(turn, about)
}

/// The same centring for a scale. The kernel already has the centred form, so this only has to
/// choose a centre: the widget's origin, or the world origin when there is no widget.
fn scaling_about(factor: f64, about: Option<&Point>) -> Xform {
    match about {
        Some(p) => Xform::scale_uniform(p, factor),
        None => Xform::scale_xyz(factor, factor, factor),
    }
}

fn centred(inner: Xform, about: Option<&Point>) -> Xform {
    let Some(p) = about else {
        return inner;
    };
    let to = Xform::translation(p[0], p[1], p[2]);
    let back = Xform::translation(-p[0], -p[1], -p[2]);
    &(&to * &inner) * &back
}

impl State {
    /// Hide or show everything one panel row controls, through the same hide set `H` writes.
    ///
    /// A layer with anything still visible hides; a layer wholly hidden comes back. That rule
    /// makes one click enough on a half-hidden layer, which is what a person means by it.
    pub fn toggle_layer(&mut self, layer: Layer) {
        let rows = layers::of_layer(&self.scene, layer);
        if rows.is_empty() {
            return;
        }
        let hidden: Vec<bool> = rows
            .iter()
            .map(|&row| {
                self.scene
                    .identity_of(row)
                    .is_some_and(|id| self.scene.hidden.contains(&id))
            })
            .collect();
        let hide = !hidden.iter().all(|&h| h);
        for (&row, was) in rows.iter().zip(&hidden) {
            if *was == hide {
                continue;
            }
            let Some(identity) = self.scene.identity_of(row) else {
                continue;
            };
            if hide {
                self.scene.hidden.insert(identity);
            } else {
                self.scene.hidden.remove(&identity);
            }
            self.gpu.set_hidden(row, hide);
        }
        if self.scene.selected.is_some_and(|row| rows.contains(&row)) && hide {
            self.select(None);
        }
        self.refresh_layers();
        self.update_label();
        self.touch();
    }

    /// Redraw the panel from the scene, when it is open.
    pub fn refresh_layers(&mut self) {
        if !crate::app::feedback::layers_open() {
            return;
        }
        let rows: Vec<crate::app::feedback::LayerRow> = layers::rows(&self.scene)
            .into_iter()
            .map(|row| crate::app::feedback::LayerRow {
                key: row.layer.key(),
                label: row.label,
                count: row.count,
                hidden: row.hidden,
            })
            .collect();
        crate::app::feedback::layers_panel(&rows);
    }
}

impl State {
    /// Open or close the layers panel - `L`. Opening fills it; closing leaves it empty, so a
    /// scene change while it is shut costs nothing.
    pub fn toggle_layers_panel(&mut self) {
        let open = !crate::app::feedback::layers_open();
        crate::app::feedback::layers_visible(open);
        if open {
            self.refresh_layers();
        }
        self.touch();
    }
}
