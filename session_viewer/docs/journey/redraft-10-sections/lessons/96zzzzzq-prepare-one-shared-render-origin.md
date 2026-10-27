# 21.3 · Browser ownership of frame matrices and wakeups

**Steps:** 3 · **Source:** [camera.rs](../../../../src/camera.rs) · [state.rs](../../../../src/state.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Large World Coordinates (f64): P_world = 10^7 m</text>
  <circle cx="64" cy="138" r="4" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="64" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(0,0,0)</text>
  <line x1="64" y1="138" x2="236" y2="74" stroke="#94a3b8" stroke-width="1.75" stroke-dasharray="5 3" marker-end="url(#arr)"/>
  <circle cx="244" cy="70" r="5" fill="#dc2626"/>
  <polygon points="244,70 278,54 272,92" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="218" y="54" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Anchor_f64</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU f64 Delta: P_rel_f32 = (P_world_f64 - Anchor_f64) as f32</text>
  <circle cx="486" cy="112" r="5" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="486" y="132" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Anchor = (0,0,0)_rel</text>
  <line x1="486" y1="112" x2="574" y2="82" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <polygon points="574,82 634,58 622,118" fill="#bbf7d0" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="606" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">||P_rel|| &lt; 10 m (Zero Jitter)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Jitter-Free Far-Origin Orbit + Crisp Depth Precision</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="866,142 934,58 1002,142" fill="#dbeafe" stroke="#111111" stroke-width="2.2"/>
  <line x1="934" y1="58" x2="934" y2="142" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-96zzzzzq-prepare-one-shared-render-origin"></a>1. Prepare one shared render origin

### 1. `src/render_space.rs`

Expose a finite bounded delay beyond the existing strict 200ms throttle.

**Locate this existing block:**

```rust
    }
    pub fn update(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn retry_after_ms(&self, now: f64) -> Option<u32> {
        now.is_finite().then(|| {
            (self.last_ms + 201. - now)
                .ceil()
                .clamp(1., 201.) as u32
        })
    }
    pub fn update(
```

### 2. `src/renderer.rs`

Adopt one origin without source cancellation; reuse existing input owners and skip unchanged data.

**Locate this existing block:**

```rust
    }

    fn set_source_ink(&mut self, scene: &Scene) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn camera_matrix(&self, camera: &crate::camera::Camera) -> [f32; 16] {
        camera
            .view_projection_anchored(self.anchor())
            .to_f32()
    }
    pub fn rebase_delay(&self, now: f64) -> Option<u32> {
        self.space.retry_after_ms(now)
    }
    pub fn prepare_origin(
        &mut self,
        camera: &crate::camera::Camera,
        scene: &Scene,
        target: Option<crate::pick_sources::Target>,
        ids: &[ObjectId],
        now: f64,
    ) -> Result<crate::render_space::Rebase, &'static str> {
        let previous = self.anchor();
        let mut frame = self
            .space
            .update(camera.target, camera.distance, now)
            .ok_or("Invalid camera origin")?;
        frame.moved &= previous.map(f64::to_bits) != frame.anchor.map(f64::to_bits);
        if frame.moved {
            self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
            for mesh in &mut self.meshes {
                let [calls, bytes] = mesh.rebase(&self.queue, frame.anchor);
                self.settings_writes += calls;
                self.settings_bytes += bytes;
            }
            self.set_source_ink(scene);
            if !self
                .highlight
                .rebase_face(&self.queue, frame.anchor, target)
            {
                self.set_highlight(scene, target)?;
            }
            self.select_objects(scene, ids)?;
            self.set_grid_on(self.grid_settings, self.grid_plane)?;
        }
        self.cloud_camera(camera);
        Ok(frame)
    }

    fn set_source_ink(&mut self, scene: &Scene) {
```

### 3. `src/render_space.rs`

Supply deterministic throttle-deadline and invalid-clock checks without requiring native tests in the learner project.

**Locate this existing block:**

```rust
        );
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        );
    }
}

#[cfg(test)]
mod delay_tests {
    use super::*;
    #[test]
    fn retry_delay_waits_past_the_strict_origin_throttle() {
        let mut origin = Origin::default();
        origin.update([0.; 3], 3., 0.).unwrap();
        assert!(
            origin
                .update([2000., 0., 0.], 3., 100.)
                .unwrap()
                .pending
        );
        assert_eq!(origin.retry_after_ms(100.), Some(101));
        assert_eq!(origin.retry_after_ms(200.), Some(1));
        assert_eq!(origin.retry_after_ms(-100.), Some(201));
        assert_eq!(origin.retry_after_ms(f64::NAN), None);
        assert_eq!(origin.retry_after_ms(f64::INFINITY), None);
        assert!(
            origin
                .update([2000., 0., 0.], 3., 201.)
                .unwrap()
                .moved
        );
    }
}
```

### 4. `src/global_outline_tests.rs`

Supply aggregate original mesh/ink/point/cloud/sheet/text/grid pixels, precise ground, unchanged records, throttled wake timing, preserved pending flight/retry and exact restoration/source ownership.

**Locate this existing block:**

```rust
            .rebase_face(&gpu.queue, gpu.anchor(), target)
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .rebase_face(&gpu.queue, gpu.anchor(), target)
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}

#[test]
fn actual_shared_origin_keeps_all_original_lanes_and_pending_picks_coherent() {
    let mut scene = Scene::empty();
    let mesh = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.25, -0.25, -0.5],
                    [0.25, -0.25, -0.5],
                    [0., 0.25, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let line = scene
        .insert_line(
            crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                -0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let path = scene
        .insert_path(
            crate::chain::PreparedChain::polyline(Rc::new(
                session_rust::Polyline::new(vec![
                    session_rust::Point::new(-0.5, -0.5, -0.5),
                    session_rust::Point::new(0., -0.25, -0.5),
                    session_rust::Point::new(0.5, -0.5, -0.5),
                ]),
            ))
            .unwrap(),
        )
        .unwrap();
    let point = scene
        .insert_point(
            crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                0.75, 0., -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let cloud = scene
        .insert_cloud(
            crate::cloud::Prepared::new(Rc::new(
                session_rust::PointCloud::from_coords(
                    vec![-0.75, 0., -0.5],
                    vec![255, 0, 0, 255],
                    vec![],
                ),
            ))
            .unwrap(),
        )
        .unwrap();
    let sheet = Rc::new(session_rust::proto::Sheet {
        guid: crate::sheet_example::source().guid,
        coords: vec![0.25, 0.25, -0.5, 0.75, 0.25, -0.5],
        widths: vec![0.0625],
        colors: vec![0xff00_0000],
        segment_count: 1,
        entity_count: 1,
        source_ids: vec![0],
        ..Default::default()
    });
    let drawing = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(sheet.clone()).unwrap(),
        ))
        .unwrap();
    let text_source = Rc::new(crate::scene_label::example(
        crate::text::TextPlacement::Anchor {
            world: [-0.5, 0.75, -0.5],
            offset: [0.; 2],
        },
    ));
    let text = scene
        .insert_text(crate::scene_label::Prepared::new(text_source.clone()).unwrap())
        .unwrap();
    let ids = [line, path, point, cloud, drawing, text];
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 3.;
    camera.aspect = 4. / 3.;
    let mut gpu = renderer(&scene);
    gpu.select_objects(&scene, &ids).unwrap();
    let settings = Some(crate::grid::Settings {
        spacing: 0.125,
        half_cells: 4,
    });
    let plane = |origin| crate::cplane::CPlane::Frame {
        origin,
        x: [1., 0., 0.],
        y: [0., 1., 0.],
    };
    gpu.set_grid_on(settings, plane([0., 0., -0.75]))
        .unwrap();
    assert!(
        !gpu.prepare_origin(&camera, &scene, None, &ids, 0.)
            .unwrap()
            .moved
    );
    let transform = gpu.camera_matrix(&camera);
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    assert!(
        baseline
            .chunks_exact(4)
            .any(|p| p[..3] != [255; 3])
    );
    let ground = gpu.ambient_rows.ground_at([0.; 3]);
    let uploads = gpu.stats()[0];
    let anchor = [2000., -2000., 2000.];
    let mut far = scene.clone();
    for id in [mesh, line, path, point, cloud, drawing, text] {
        far.translate(id, anchor).unwrap();
    }
    gpu.set_scene(&far, None);
    camera.target = anchor;
    gpu.set_grid_on(
        settings,
        plane([anchor[0], anchor[1], anchor[2] - 0.75]),
    )
    .unwrap();
    let frame = gpu
        .prepare_origin(&camera, &far, None, &ids, 300.)
        .unwrap();
    assert!(frame.moved && !frame.pending);
    assert_eq!(gpu.camera_matrix(&camera), transform);
    let actual = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    assert!(
        actual == baseline,
        "Shared original lanes must retain exact local/distant pixels"
    );
    assert_eq!(gpu.ambient_rows.ground_at(anchor), ground);
    let writes = gpu.stats();
    assert!(
        !gpu.prepare_origin(&camera, &far, None, &ids, 301.)
            .unwrap()
            .moved
    );
    assert_eq!(gpu.stats(), writes);
    gpu.request_pick([0., 0.], transform)
        .unwrap();
    camera.target[0] += 2000.;
    let pending = gpu
        .prepare_origin(&camera, &far, None, &ids, 350.)
        .unwrap();
    assert!(!pending.moved && pending.pending);
    assert_eq!(gpu.rebase_delay(350.), Some(151));
    assert_eq!(&gpu.pick_usage()[..2], &[1, 0]);
    let moved = gpu
        .prepare_origin(&camera, &far, None, &ids, 501.)
        .unwrap();
    assert!(moved.moved && !moved.pending);
    assert_eq!(gpu.camera_matrix(&camera), transform);
    assert_eq!(&gpu.pick_usage()[..2], &[1, 0]);
    wait(&gpu);
    assert!(gpu.take_pick(transform).is_none());
    assert_eq!(&gpu.pick_usage()[..2], &[1, 0]);
    wait(&gpu);
    assert!(
        gpu.take_pick(transform)
            .unwrap()
            .unwrap()
            .is_none()
    );
    camera.target = anchor;
    assert!(
        gpu.prepare_origin(&camera, &far, None, &ids, 802.)
            .unwrap()
            .moved
    );
    let restored = crate::cap_gpu_tests::pixels_at(&gpu, gpu.camera_matrix(&camera));
    assert!(
        restored == baseline,
        "Restored shared origin must restore exact pixels"
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        far.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    assert!(Rc::ptr_eq(
        &far.texts()[0].prepared.source,
        &text_source
    ));
    assert!(Rc::ptr_eq(
        &far.sheets()[0].prepared.original,
        &sheet
    ));
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

## <a id="checkpoint-96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes"></a>2. Prepare browser frame matrices and owned wakes

### 1. `src/timer.rs`

Retain the kind of owned browser timer for cancellation. Reuse the existing callback owner for both repeating reports and one-shot wakes. Schedule the requested timer kind through the same owned callback. Keep the callback alive exactly as long as its timer owner. Cancel the matching interval or timeout when its owner exits.

**Locate this existing block:**

```rust
    id: i32,
    _callback: Closure<dyn FnMut()>,
}

impl Timer {
    pub fn every(
        milliseconds: i32,
        tick: impl FnMut() + 'static,
    ) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let callback = Closure::wrap(Box::new(tick) as Box<dyn FnMut()>);
        let id = window.set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            milliseconds,
        )?;
        Ok(Self {
            window,
            id,
            _callback: callback,
        })
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.window
            .clear_interval_with_handle(self.id);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    id: i32,
    repeat: bool,
    _callback: Closure<dyn FnMut()>,
}

impl Timer {
    pub fn every(
        milliseconds: i32,
        tick: impl FnMut() + 'static,
    ) -> Result<Self, JsValue> {
        Self::start(milliseconds, tick, true)
    }
    pub fn after(
        milliseconds: i32,
        tick: impl FnMut() + 'static,
    ) -> Result<Self, JsValue> {
        Self::start(milliseconds, tick, false)
    }
    fn start(
        milliseconds: i32,
        tick: impl FnMut() + 'static,
        repeat: bool,
    ) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let callback = Closure::wrap(Box::new(tick) as Box<dyn FnMut()>);
        let id = if repeat {
            window.set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                milliseconds,
            )?
        } else {
            window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                milliseconds,
            )?
        };
        Ok(Self {
            window,
            id,
            repeat,
            _callback: callback,
        })
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if self.repeat {
            self.window
                .clear_interval_with_handle(self.id);
        } else {
            self.window
                .clear_timeout_with_handle(self.id);
        }
    }
```

### 2. `src/browser.rs`

Use the matching renderer matrix for every existing browser draw, request and readback path. Expose the actual local frame without replacing the existing precise world-camera diagnostic. Report the matrix after initial viewport layout so it matches the first presented frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &serde_json::to_string(&editor.camera.view_projection().m)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
```

Replace it with:

```rust
        &serde_json::to_string(&editor.camera.view_projection().m)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
        "data-render-origin",
        &serde_json::to_string(&renderer.anchor()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-render-matrix",
        &serde_json::to_string(&renderer.camera_matrix(&editor.camera)).unwrap(),
    )?;
```

**Locate this block:**

```rust
                    &surface,
                    &renderer,
                    &editor.background,
                    &editor.camera.uniform(),
                    editor.normal_view,
                    &mut panel,
                ) {
```

Replace it with:

```rust
                    &surface,
                    &renderer,
                    &editor.background,
                    &renderer.camera_matrix(&editor.camera),
                    editor.normal_view,
                    &mut panel,
                ) {
```

**Locate this block:**

```rust
                                    p.shift_key() && !p.ctrl_key() && !p.meta_key()
                                }),
                        );
                        if let Err(error) =
                            renderer.request_pick(screen, editor.camera.uniform())
                        {
                            report(error);
                        }
                        None
```

Replace it with:

```rust
                                    p.shift_key() && !p.ctrl_key() && !p.meta_key()
                                }),
                        );
                        if let Err(error) = renderer.request_pick(
                            screen,
                            renderer.camera_matrix(&editor.camera),
                        ) {
                            report(error);
                        }
                        None
```

**Locate this block:**

```rust
                                remove: pointer.ctrl_key() || pointer.meta_key(),
                            });
                        if !modified {
                            if let Err(error) =
                                renderer.request_pick(from, editor.camera.uniform())
                            {
                                report(error);
                                return;
                            }
```

Replace it with:

```rust
                                remove: pointer.ctrl_key() || pointer.meta_key(),
                            });
                        if !modified {
                            if let Err(error) = renderer.request_pick(
                                from,
                                renderer.camera_matrix(&editor.camera),
                            ) {
                                report(error);
                                return;
                            }
```

**Locate this block:**

```rust
                                        pick_remove.set(false);
                                        if let Err(error) = renderer.request_pick(
                                            screen,
                                            editor.camera.uniform(),
                                        ) {
                                            report(error);
                                        }
```

Replace it with:

```rust
                                        pick_remove.set(false);
                                        if let Err(error) = renderer.request_pick(
                                            screen,
                                            renderer.camera_matrix(&editor.camera),
                                        ) {
                                            report(error);
                                        }
```

**Locate this block:**

```rust
                                        pick_remove.set(false);
                                        if let Err(error) = renderer.request_pick(
                                            screen,
                                            editor.camera.uniform(),
                                        ) {
                                            report(error)
                                        }
```

Replace it with:

```rust
                                        pick_remove.set(false);
                                        if let Err(error) = renderer.request_pick(
                                            screen,
                                            renderer.camera_matrix(&editor.camera),
                                        ) {
                                            report(error)
                                        }
```

**Locate this block:**

```rust
                    {
                        pick_additive.set(false);
                        pick_remove.set(false);
                        if let Err(error) =
                            renderer.request_pick(screen, editor.camera.uniform())
                        {
                            report(error)
                        }
                        None
```

Replace it with:

```rust
                    {
                        pick_additive.set(false);
                        pick_remove.set(false);
                        if let Err(error) = renderer.request_pick(
                            screen,
                            renderer.camera_matrix(&editor.camera),
                        ) {
                            report(error)
                        }
                        None
```

**Locate this block:**

```rust
                gesture.cancel();
                Some(handle_action)
            } else if event.type_() == "viewer-pick" {
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => {
                        let target = hit
                            .and_then(|hit| renderer.resolve_pick(&editor.scene, hit));
```

Replace it with:

```rust
                gesture.cancel();
                Some(handle_action)
            } else if event.type_() == "viewer-pick" {
                match renderer.take_pick(renderer.camera_matrix(&editor.camera)) {
                    Some(Ok(hit)) => {
                        let target = hit
                            .and_then(|hit| renderer.resolve_pick(&editor.scene, hit));
```

**Locate this block:**

```rust
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            editor.normal_view,
            &mut panel,
        )?;
```

Replace it with:

```rust
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        inspect(&editor, &renderer)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &renderer.camera_matrix(&editor.camera),
            editor.normal_view,
            &mut panel,
        )?;
```

**Locate this block:**

```rust
    let limit = device.limits().max_texture_dimension_2d;
    let density = window.device_pixel_ratio();
    let density = if density.is_finite() && density > 0.0 {
        density
    } else {
        1.0
    };
    let width = ((window
        .inner_width()?
        .as_f64()
        .ok_or("No window width")?
        * density)
        .round() as u32)
        .clamp(1, limit);
    let height = ((window
        .inner_height()?
        .as_f64()
        .ok_or("No window height")?
        * density)
        .round() as u32)
        .clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
```

Replace it with:

```rust
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window
        .inner_width()?
        .as_f64()
        .ok_or("No window width")? as u32)
        .clamp(1, limit);
    let height = (window
        .inner_height()?
        .as_f64()
        .ok_or("No window height")? as u32)
        .clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
```

## <a id="checkpoint-96zzzzzs-adopt-the-shared-origin-in-the-browser"></a>3. Adopt the shared origin in the browser

### 1. `src/browser_rebase.rs`

Connect the existing origin policy and timer owner without retaining an editor or scene in a wake.

Create the file and type:

```rust
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::JsValue;

pub fn prepare(
    renderer: &mut crate::renderer::Renderer,
    editor: &crate::editor::Editor,
) -> Result<crate::render_space::Rebase, JsValue> {
    renderer
        .prepare_origin(
            &editor.camera,
            editor.display_scene(),
            editor.selection(),
            &editor.highlighted_objects(),
            crate::browser_phase::now(),
        )
        .map_err(JsValue::from_str)
}

pub struct Wake {
    timer: Option<crate::timer::Timer>,
    pending: Rc<Cell<bool>>,
    fault: crate::gpu_fault::Fault,
}
impl Wake {
    pub fn new(fault: crate::gpu_fault::Fault) -> Self {
        Self {
            timer: None,
            pending: Rc::new(Cell::new(false)),
            fault,
        }
    }
    pub fn pending(&self) -> bool {
        self.pending.get()
    }
    pub fn sync(&mut self, pending: bool, delay: u32) -> Result<(), JsValue> {
        if !pending {
            self.pending.set(false);
            self.timer = None;
            return Ok(());
        }
        if self.pending.get() {
            return Ok(());
        }
        let flag = self.pending.clone();
        let fault = self.fault.clone();
        self.timer = Some(crate::timer::Timer::after(
            delay as i32,
            move || {
                flag.set(false);
                if crate::browser_runtime::owns(&fault) && fault.message().is_none() {
                    if let (Some(window), Ok(event)) = (
                        web_sys::window(),
                        web_sys::Event::new("viewer-origin"),
                    ) {
                        let _ = window.dispatch_event(&event);
                    }
                }
            },
        )?);
        self.pending.set(true);
        Ok(())
    }
}
```

### 2. `src/lib.rs`

Compile the browser-only frame and wake adapter.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod browser_report;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_rebase;
#[cfg(target_arch = "wasm32")]
mod browser_report;
```

### 3. `src/browser.rs`

Prepare actual origin-relative inputs before the initial frame and its diagnostics. Keep the wake owner inside the runtime-owned input closure. Use the current origin before judging or queueing asynchronous picks. Adopt action changes before temporary inputs, avoid duplicate selection uploads, and schedule only a pending origin move. Deliver the owned wake through the existing runtime listener owner. Expose the actual precise camera target used by the origin policy for controlled browser frame checks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "data-render-origin",
        &serde_json::to_string(&renderer.anchor()).unwrap(),
    )?;
```

Replace it with:

```rust
        "data-render-origin",
        &serde_json::to_string(&renderer.anchor()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-camera-target",
        &serde_json::to_string(&editor.camera.target).unwrap(),
    )?;
```

**Locate this block:**

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
```

Replace it with:

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-origin")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
```

**Locate this block:**

```rust
                    Err(error) => panel.result(&error),
                }
            }
            let guides = editor.guide_overlay_at(renderer.anchor());
            if let Err(error) = renderer.set_guides(&guides) {
                panel.result(error);
```

Replace it with:

```rust
                    Err(error) => panel.result(&error),
                }
            }
            let origin_frame =
                match crate::browser_rebase::prepare(&mut renderer, &editor) {
                    Ok(frame) => frame,
                    Err(error) => {
                        report(&format!("Cannot prepare view: {error:?}"));
                        return;
                    }
                };
            if origin_frame.moved {
                highlight_changed = false;
            }
            if let Err(error) = origin_wake.sync(
                origin_frame.pending,
                renderer
                    .rebase_delay(crate::browser_phase::now())
                    .unwrap_or(1),
            ) {
                report(&format!("Cannot schedule view: {error:?}"));
                return;
            }
            let _ = pointer_canvas.set_attribute(
                "data-origin-pending",
                if origin_wake.pending() {
                    "true"
                } else {
                    "false"
                },
            );
            let guides = editor.guide_overlay_at(renderer.anchor());
            if let Err(error) = renderer.set_guides(&guides) {
                panel.result(error);
```

**Locate this block:**

```rust
                line
            };

            let mut highlight_changed = false;
            let action = if event.type_() == "cancel"
                && event
```

Replace it with:

```rust
                line
            };

            if let Err(error) = crate::browser_rebase::prepare(&mut renderer, &editor) {
                report(&format!("Cannot prepare view: {error:?}"));
                return;
            }
            let mut highlight_changed = false;
            let action = if event.type_() == "cancel"
                && event
```

**Locate this block:**

```rust
    drop(editor);
    let sheet_query = crate::browser_sheet::Control::default();
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if active_fault.message().is_some() {
```

Replace it with:

```rust
    drop(editor);
    let sheet_query = crate::browser_sheet::Control::default();
    let owned_editor = std::rc::Rc::clone(&shared);
    let mut origin_wake = crate::browser_rebase::Wake::new(fault.clone());
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if active_fault.message().is_some() {
```

**Locate this block:**

```rust
        panel.update(None, &canvas)?;
        inspect(&editor, &renderer)?;
        let frame_started = crate::browser_phase::now();
        present(
```

Replace it with:

```rust
        panel.update(None, &canvas)?;
        crate::browser_rebase::prepare(&mut renderer, &editor)?;
        canvas.set_attribute("data-origin-pending", "false")?;
        inspect(&editor, &renderer)?;
        let frame_started = crate::browser_phase::now();
        present(
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 21.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 21.3
npm --prefix ../session_tests run course -- save 21.3
```

Continue with [21.4 · Reversed depth and homogeneous near clipping](../../consolidation/96zzzzzt-share-depth-endpoints-and-comparisons.md).
