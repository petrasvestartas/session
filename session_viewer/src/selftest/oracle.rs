//! A CPU hidden-line oracle for the headless renderer: it casts a ray from the eye to points
//! along every drawn stroke through every placed triangle, and asks the rendered frame to ink
//! each point the ray reaches. It judges the GPU's visibility tests, never their tolerances:
//! a triangle hides a point only when it is clearly nearer.

use super::frame_input;
use crate::app::scene::{FileDoc, Scene};
use crate::camera::Camera;
use crate::engine::gpu::Gpu;
use crate::engine::gpu::instance::Instance;
use session_rust::{BRep, Point, Session, Xform};
use std::rc::Rc;

/// Cells across and down the screen the triangles are binned into.
const CELLS: usize = 48;

/// How much nearer than a stroke point a triangle must be to hide it, scene mm.
const CLEARANCE: f64 = 0.05;

/// Spacing of the sampled points along a stroke, px.
const STEP_PX: f64 = 3.0;

/// A pixel this dark, or a neighbour, counts as inked: a hairline far away is a light grey.
const INK_LEVEL: u8 = 190;

/// The scene as the CPU sees it: placed triangles, placed strokes and the camera.
pub(super) struct Oracle {
    eye: [f64; 3],                // scene units
    orthographic: Option<[f64; 3]>, // parallel direction; perspective rays start at eye
    ray_span: f64,               // enough world distance to start before the scene
    origin: [f64; 3],             // the point the matrix is relative to
    matrix: [f64; 16],            // view-projection, column-major, scene units in
    size: (u32, u32),             // frame size, px
    tris: Vec<[[f64; 3]; 3]>,     // placed triangles
    cells: Vec<Vec<u32>>,         // triangle indices per screen cell
    strokes: Vec<[[f64; 3]; 2]>,  // placed stroke segments
    stroke_rows: Vec<u32>,        // object row of each stroke
}

/// What the frame showed of the points the oracle found visible.
#[derive(Debug, Default)]
pub(super) struct Verdict {
    pub visible: usize,          // stroke points the ray reaches
    pub inked: usize,            // of those, drawn in the frame
    pub hidden: usize,           // samples at least 1 mm behind a face
    pub leaked: usize,           // hidden samples drawn as full-strength ink
    pub misses: Vec<(u32, u32)>, // pixels of the points not drawn
    pub covers: Vec<f64>,        // per miss: the nearest cover in front of it, mm; 0 for none
    pub rows: Vec<u32>,          // per miss: the object row of the stroke
}

impl Verdict {
    /// Share of the visible points the frame drew.
    pub fn inked_share(&self) -> f64 {
        if self.visible == 0 {
            return 1.0;
        }

        self.inked as f64 / self.visible as f64
    }
}

/// `place` applied to an f32 point.
fn placed(place: &Xform, p: [f32; 3]) -> [f64; 3] {
    let q = place.transform_point(&Point::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2])));
    [q[0], q[1], q[2]]
}

fn sub(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Where the ray `o + t d` meets the triangle, as `t`, if it does.
fn hit(o: &[f64; 3], d: &[f64; 3], tri: &[[f64; 3]; 3]) -> Option<f64> {
    let e1 = sub(&tri[1], &tri[0]);
    let e2 = sub(&tri[2], &tri[0]);
    let p = cross(d, &e2);
    let det = dot(&e1, &p);

    if det.abs() < 1e-18 {
        return None;
    }

    let inv = 1.0 / det;
    let s = sub(o, &tri[0]);
    let u = dot(&s, &p) * inv;

    if !(-1e-9..=1.0 + 1e-9).contains(&u) {
        return None;
    }

    let q = cross(&s, &e1);
    let v = dot(d, &q) * inv;

    if v < -1e-9 || u + v > 1.0 + 1e-9 {
        return None;
    }

    Some(dot(&e2, &q) * inv)
}

impl Oracle {
    /// Take the scene's walked rows before they are uploaded, under `camera`.
    pub fn new(scene: &Scene, camera: &Camera, size: (u32, u32)) -> Self {
        let tables = &scene.tables;
        let rows = &tables.obj.rows;
        // a hidden object neither draws nor occludes
        let shown = |row: u32| rows[row as usize].flags & Instance::FLAG_HIDDEN == 0;
        let mut tris = Vec::with_capacity(tables.arena.idx.len() / 3);

        for corner in tables.arena.idx.chunks_exact(3) {
            if !shown(tables.arena.vids[corner[0] as usize]) {
                continue;
            }

            let place = |index: u32| {
                let vertex = &tables.arena.verts[index as usize];
                placed(&rows[tables.arena.vids[index as usize] as usize].place, vertex.position)
            };
            tris.push([place(corner[0]), place(corner[1]), place(corner[2])]);
        }

        let shown_pipes: Vec<_> = tables
            .seg
            .pipes
            .iter()
            .filter(|pipe| shown(pipe.instance_id))
            .collect();
        let strokes = shown_pipes
            .iter()
            .map(|pipe| {
                let place = &rows[pipe.instance_id as usize].place;
                [placed(place, pipe.p0), placed(place, pipe.p1)]
            })
            .collect();
        let stroke_rows = shown_pipes.iter().map(|pipe| pipe.instance_id).collect();
        let s = camera.unit.to_meters();
        let origin = camera.origin();
        let aspect = f64::from(size.0) / f64::from(size.1);
        let mut oracle = Self {
            eye: [camera.position[0] / s, camera.position[1] / s, camera.position[2] / s],
            orthographic: (!camera.perspective).then(|| std::array::from_fn(|i|
                (camera.target[i] - camera.position[i]) / camera.distance)),
            ray_span: 2.0 * (camera.distance + 2.0 * camera.scene_extent) / s,
            origin: [origin[0], origin[1], origin[2]],
            matrix: camera.view_proj(aspect).m,
            size,
            tris,
            cells: vec![Vec::new(); CELLS * CELLS],
            strokes,
            stroke_rows,
        };
        oracle.bin();
        oracle
    }

    /// The screen pixel of a scene point, or None behind the eye.
    fn screen(&self, p: &[f64; 3]) -> Option<(f64, f64)> {
        let q = sub(p, &self.origin);
        let m = &self.matrix;
        let x = m[0] * q[0] + m[4] * q[1] + m[8] * q[2] + m[12];
        let y = m[1] * q[0] + m[5] * q[1] + m[9] * q[2] + m[13];
        let w = m[3] * q[0] + m[7] * q[1] + m[11] * q[2] + m[15];

        if w <= 1e-9 {
            return None;
        }

        Some((
            (x / w * 0.5 + 0.5) * f64::from(self.size.0),
            (0.5 - y / w * 0.5) * f64::from(self.size.1),
        ))
    }

    /// Clip w of a scene point: its distance ahead of the eye plane, scaled.
    fn clip_w(&self, p: &[f64; 3]) -> f64 {
        let q = sub(p, &self.origin);
        let m = &self.matrix;
        m[3] * q[0] + m[7] * q[1] + m[11] * q[2] + m[15]
    }

    /// Screen cell of a pixel, clamped to the grid.
    fn cell(&self, x: f64, y: f64) -> (usize, usize) {
        let fx = (x / f64::from(self.size.0) * CELLS as f64).floor();
        let fy = (y / f64::from(self.size.1) * CELLS as f64).floor();
        (
            fx.clamp(0.0, (CELLS - 1) as f64) as usize,
            fy.clamp(0.0, (CELLS - 1) as f64) as usize,
        )
    }

    /// Bin every triangle into the cells its screen box touches; one that crosses the eye
    /// plane goes everywhere.
    fn bin(&mut self) {
        let spans: Vec<((usize, usize), (usize, usize))> = self
            .tris
            .iter()
            .map(|tri| {
                let corners: Vec<Option<(f64, f64)>> = tri.iter().map(|p| self.screen(p)).collect();

                if corners.iter().any(|c| c.is_none()) {
                    return ((0, 0), (CELLS - 1, CELLS - 1));
                }

                let xs = corners.iter().map(|c| c.unwrap().0);
                let ys = corners.iter().map(|c| c.unwrap().1);
                let (x0, x1) = (xs.clone().fold(f64::MAX, f64::min), xs.fold(f64::MIN, f64::max));
                let (y0, y1) = (ys.clone().fold(f64::MAX, f64::min), ys.fold(f64::MIN, f64::max));
                (self.cell(x0, y0), self.cell(x1, y1))
            })
            .collect();

        for (index, (lo, hi)) in spans.into_iter().enumerate() {
            for cy in lo.1..=hi.1 {
                for cx in lo.0..=hi.0 {
                    self.cells[cy * CELLS + cx].push(index as u32);
                }
            }
        }
    }

    /// How far in front of `p` the nearest triangle on the ray to it lies, mm; 0 for none.
    pub fn cover(&self, p: &[f64; 3], pixel: (f64, f64)) -> f64 {
        let origin = self.orthographic.map_or(self.eye, |forward|
            std::array::from_fn(|i| p[i] - forward[i] * self.ray_span));
        let d = sub(p, &origin);
        let length = dot(&d, &d).sqrt();
        let (cx, cy) = self.cell(pixel.0, pixel.1);
        let mut most: f64 = 0.0;

        for &index in &self.cells[cy * CELLS + cx] {
            if let Some(t) = hit(&origin, &d, &self.tris[index as usize])
                && t > 0.0
            {
                most = most.max((1.0 - t) * length);
            }
        }

        most
    }

    /// Sample every stroke inside `region` (left, top, right, bottom, px) and judge the frame.
    pub fn judge(&self, rgba: &[u8], region: (f64, f64, f64, f64)) -> Verdict {
        let mut verdict = Verdict::default();
        let mut visible_pixels = std::collections::HashSet::new();
        let mut hidden_samples = Vec::new();
        let inside = |x: f64, y: f64| x >= region.0 && y >= region.1 && x < region.2 && y < region.3;

        for (stroke, &row) in self.strokes.iter().zip(&self.stroke_rows) {
            // the part in front of the eye: clip w is affine along the stroke
            let (w0, w1) = (self.clip_w(&stroke[0]), self.clip_w(&stroke[1]));

            if w0 <= 0.0 && w1 <= 0.0 {
                continue;
            }

            let crossing = |w: f64, other: f64| (1e-3 - w) / (other - w);
            let h0 = if w0 <= 0.0 { crossing(w0, w1) } else { 0.0 };
            let h1 = if w1 <= 0.0 { 1.0 - crossing(w1, w0) } else { 1.0 };
            let at = |h: f64| -> [f64; 3] {
                std::array::from_fn(|i| stroke[0][i] + (stroke[1][i] - stroke[0][i]) * h)
            };
            let (Some(a), Some(b)) = (self.screen(&at(h0)), self.screen(&at(h1))) else {
                continue;
            };
            let length = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
            let steps = (length / STEP_PX).ceil().clamp(1.0, 4096.0) as usize;

            // the ends stay off: a neighbouring face meets the stroke there
            for step in 1..steps {
                let p = at(h0 + (h1 - h0) * step as f64 / steps as f64);
                let Some(pixel) = self.screen(&p) else {
                    continue;
                };

                if !inside(pixel.0, pixel.1) {
                    continue;
                }

                let cover = self.cover(&p, pixel);

                if cover > CLEARANCE {
                    // Contact rounding is irrelevant this far behind a face. The
                    // 3x3 ink neighbourhood can still overlap a visible end joint.
                    if cover > 1.0 {
                        hidden_samples.push((pixel, self.inked(rgba, pixel)));
                    }
                    continue;
                }

                verdict.visible += 1;
                visible_pixels.insert((pixel.0.floor() as i64, pixel.1.floor() as i64));

                if self.inked(rgba, pixel) {
                    verdict.inked += 1;
                } else {
                    verdict.misses.push((pixel.0 as u32, pixel.1 as u32));
                    verdict.covers.push(cover);
                    verdict.rows.push(row);
                }
            }
        }

        // A dark neighbour may belong to a different, visible edge. Exclude that
        // overlap before calling it a leak; this matters at orthographic joints.
        for (pixel, inked) in hidden_samples {
            let (x, y) = (pixel.0.floor() as i64, pixel.1.floor() as i64);
            if (-3..=3).any(|dy| (-3..=3).any(|dx| visible_pixels.contains(&(x + dx, y + dy)))) {
                continue;
            }
            verdict.hidden += 1;
            verdict.leaked += usize::from(inked);
        }
        verdict
    }

    /// True when the pixel or one beside it is dark.
    fn inked(&self, rgba: &[u8], pixel: (f64, f64)) -> bool {
        let (w, h) = (self.size.0 as i64, self.size.1 as i64);
        let (px, py) = (pixel.0.floor() as i64, pixel.1.floor() as i64);

        for dy in -1..=1 {
            for dx in -1..=1 {
                let (x, y) = (px + dx, py + dy);

                if x < 0 || y < 0 || x >= w || y >= h {
                    continue;
                }

                let at = ((y * w + x) * 4) as usize;

                if rgba[at] < INK_LEVEL && rgba[at + 1] < INK_LEVEL && rgba[at + 2] < INK_LEVEL {
                    return true;
                }
            }
        }

        false
    }
}

/// Load `session` headless, take the oracle's rows, render one frame under `aim`.
pub(super) fn render_with_oracle(
    session: Session,
    size: (u32, u32),
    aim: impl FnOnce(&mut Camera, &session_rust::AABB),
) -> (Oracle, Vec<u8>) {
    render_configured(session, size, aim, |_| {})
}

/// Render with explicit display settings so decorations cannot count as edge ink.
fn render_configured(
    session: Session,
    size: (u32, u32),
    aim: impl FnOnce(&mut Camera, &session_rust::AABB),
    configure: impl FnOnce(&mut Gpu),
) -> (Oracle, Vec<u8>) {
    let mut gpu = pollster::block_on(Gpu::new_headless(size.0, size.1)).expect("headless gpu");
    configure(&mut gpu);
    let mut scene = Scene::new();
    scene.add_file(FileDoc {
        name: "oracle".into(),
        session: Rc::new(session),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    let aspect = f64::from(size.0) / f64::from(size.1);
    let mut camera = Camera::new();
    camera.fit(&scene.tables.bounds, aspect);
    aim(&mut camera, &scene.tables.bounds);
    let oracle = Oracle::new(&scene, &camera, size);
    let s = camera.unit.to_meters();
    println!(
        "oracle camera: eye ({:.1}, {:.1}, {:.1}) target ({:.1}, {:.1}, {:.1}) mm, bounds centre ({:.1}, {:.1}, {:.1}) half ({:.1}, {:.1}, {:.1}), {} triangles, {} strokes",
        camera.position[0] / s, camera.position[1] / s, camera.position[2] / s,
        camera.target[0] / s, camera.target[1] / s, camera.target[2] / s,
        scene.tables.bounds.cx, scene.tables.bounds.cy, scene.tables.bounds.cz,
        scene.tables.bounds.hx, scene.tables.bounds.hy, scene.tables.bounds.hz,
        oracle.tris.len(), oracle.strokes.len()
    );
    scene.upload_to(&mut gpu);
    gpu.find_solids(|row| scene.solid_faces(row));
    let input = frame_input(&mut gpu, &camera, aspect);
    let rgba = gpu.render_offscreen(&input);
    (oracle, rgba)
}

/// Check the same geometry at grazing views, near the eye and in orthographic projection.
/// White faces and no grid/outlines make dark pixels evidence of the stroke pass itself.
#[test]
fn close_up_edges_preserve_visibility_across_views_and_samples() {
    if pollster::block_on(Gpu::new_headless(8, 8)).is_err() {
        eprintln!("no GPU adapter; skipped");
        return;
    }

    for (angle, height, inset, size, perspective) in [
        (0.7_f64, 3.0, 300.0, (800, 600), true),
        (3.0, 25.0, 300.0, (1536, 864), true),
        (12.0, 25.0, -200.0, (900, 700), true),
        (3.0, 25.0, 300.0, (900, 700), false),
    ] {
        for msaa in [1, 4] {
            let mut session = Session::new("plate matrix");
            session.add_brep(BRep::create_box(3000.0, 400.0, 120.0), None);
            let (oracle, rgba) = render_configured(session, size, |camera, bounds| {
                let s = camera.unit.to_meters();
                camera.orbit(60.0_f32.to_radians() / 0.005,
                    (angle as f32 - 30.0).to_radians() / 0.005);
                let eye = [bounds.cx - bounds.hx + inset, bounds.cy - 50.0,
                    bounds.cz + bounds.hz + height];
                let forward = [angle.to_radians().cos(), 0.0, -angle.to_radians().sin()];
                let distance = 2000.0;
                camera.target = std::array::from_fn(|i| (eye[i] + forward[i] * distance) * s);
                camera.distance = distance * s;
                camera.perspective = perspective;
                camera.update_position();
            }, |gpu| {
                gpu.view.show_grid = false;
                gpu.view.show_outlines = false;
                gpu.view.markers = false;
                gpu.view.lit = false;
                gpu.view.opacity = 1.0;
                gpu.view.msaa_forced = Some(msaa);
            });
            let verdict = oracle.judge(&rgba, (0.0, 0.0, f64::from(size.0), f64::from(size.1)));
            println!("angle {angle}, height {height}, perspective {perspective}, MSAA{msaa}: {} / {} visible, {} / {} hidden leaks",
                verdict.inked, verdict.visible, verdict.leaked, verdict.hidden);
            if let Ok(out) = std::env::var("VIEWER_ORACLE_OUT") {
                write_marked(&format!("{out}-{angle}-{perspective}-{msaa}.ppm"), &rgba, size, &verdict);
            }
            assert!(verdict.visible > 100, "The case must expose edges");
            assert!(verdict.inked_share() >= 0.98, "{} / {} visible samples missing", verdict.misses.len(), verdict.visible);
            assert!(verdict.leaked <= 5.max(verdict.hidden / 50), "{} / {} hidden samples leak", verdict.leaked, verdict.hidden);
        }
    }
}

/// The frame with the misses marked red, as a PPM, for a look.
pub(super) fn write_marked(path: &str, rgba: &[u8], size: (u32, u32), verdict: &Verdict) {
    let mut marked = rgba.to_vec();

    // red: nothing in front of the point; orange: a face within the clearance
    for (&(x, y), &cover) in verdict.misses.iter().zip(&verdict.covers) {
        let at = ((y * size.0 + x) * 4) as usize;
        marked[at..at + 3].copy_from_slice(&if cover > 0.0 { [255, 140, 0] } else { [255, 0, 0] });
    }

    super::write_ppm(path, &marked, size.0, size.1).expect("write marked frame");
}

/// A long plate seen from a hand's width above one end, along its length: its faces cross the
/// near plane and span a million pixels, where the old screen-space plane fit lost the depth
/// of its own crease to rounding and drew the top edges dashed.
#[test]
fn close_up_edges_of_a_near_plate_are_continuous() {
    if pollster::block_on(Gpu::new_headless(8, 8)).is_err() {
        eprintln!("no GPU adapter; skipped");
        return;
    }

    let mut session = Session::new("plate");
    session.add_brep(BRep::create_box(3000.0, 400.0, 120.0), None);
    let size = (1200, 700);
    let (oracle, rgba) = render_with_oracle(session, size, |camera, bounds| {
        let s = camera.unit.to_meters();
        // heading along +x, 3 degrees down: the fit looks 60 degrees round and 30 degrees down
        camera.orbit(60.0_f32.to_radians() / 0.005, -(27.0_f32.to_radians() / 0.005));
        // the eye 25 mm over the top face, 300 mm in from the near end, 50 mm off the middle
        let eye = [bounds.cx - bounds.hx + 300.0, bounds.cy - 50.0, bounds.cz + bounds.hz + 25.0];
        let forward = [3.0_f64.to_radians().cos(), 0.0, -(3.0_f64.to_radians().sin())];
        let distance = 2000.0;
        camera.target = std::array::from_fn(|i| (eye[i] + forward[i] * distance) * s);
        camera.distance = distance * s;
        camera.update_position();
    });
    let verdict = oracle.judge(&rgba, (0.0, 0.0, f64::from(size.0), f64::from(size.1)));
    println!(
        "plate: {} visible edge points, {} inked, {} misses",
        verdict.visible,
        verdict.inked,
        verdict.misses.len()
    );

    // VIEWER_ORACLE_OUT=path writes the frame with the misses in red
    if let Ok(out) = std::env::var("VIEWER_ORACLE_OUT") {
        write_marked(&out, &rgba, size, &verdict);
    }

    assert!(
        verdict.visible > 200,
        "the plate's edges should cross the frame: {verdict:?}"
    );
    assert!(
        verdict.inked_share() >= 0.98,
        "{} of {} visible edge points are not drawn",
        verdict.visible - verdict.inked,
        verdict.visible
    );
}
