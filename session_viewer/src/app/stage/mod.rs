mod actions;
mod check;
pub mod deck;
mod draw;
pub mod eval;
pub mod font;
mod geometry;
mod layout;
mod matrix;
mod morph;
pub mod net;
mod object;
pub mod paint;
pub mod run;
pub mod spatial;
#[cfg(test)]
mod tests;
pub mod tex;
mod tween;
mod view;

use std::rc::Rc;

/// Fonts and parsed deck, handed from the loader to the state.
pub struct Loaded {
    pub fonts: font::Fonts,
    pub index: deck::Index,
    pub chapters: Vec<deck::Chapter>,
    pub pictures: Vec<(String, paint::Picture)>,
}

/// One finger on the screen: where it went down, where it is, and whether it orbits the 3D side.
struct Touch {
    id: u64,
    start: [f32; 2],
    at: [f32; 2],
    orbit: bool,
    down: f64, // When it went down, ms.
}

/// The running deck with its painter and pointer state.
pub struct StageApp {
    pub stage: run::Stage,
    pub atlas: paint::Atlas,
    pub frame: Option<paint::Frame>,
    pub scene3: spatial::Scene3,
    press: Option<([f32; 2], [f32; 2], bool)>, // Press on screen and slide, and whether it orbits.
    cursor: [f32; 2],
    last_click: f64,
    touch: Option<Touch>,
    #[cfg(target_arch = "wasm32")]
    published: String, // The state last written to the canvas.
}

impl StageApp {
    /// Open the deck at the chapter and step named in the page address.
    pub fn new(loaded: Loaded, now: f64) -> Self {
        let fonts = Rc::new(loaded.fonts);
        let mut atlas = paint::Atlas::new(&fonts);
        let mut stage = run::Stage::new(fonts, loaded.index, loaded.chapters);
        for (name, picture) in loaded.pictures {
            let aspect = picture.size[0] as f32 / picture.size[1].max(1) as f32;
            stage
                .pictures
                .insert(name, (atlas.pictures.len() as u16, aspect));
            atlas.pictures.push(picture);
        }
        stage.now = now;
        #[cfg(target_arch = "wasm32")]
        let query = |key: &str| crate::app::route::query(key).and_then(|v| v.parse::<usize>().ok());
        #[cfg(not(target_arch = "wasm32"))]
        let query = |_: &str| None::<usize>;
        // `?theme=dark` or `?theme=light` overrides the deck's own `theme:`
        #[cfg(target_arch = "wasm32")]
        if let Some(t) = crate::app::route::query("theme").and_then(|v| deck::Theme::parse(&v)) {
            stage.theme = t;
        }
        stage.open_at(
            query("chapter").unwrap_or(1),
            query("step").unwrap_or(0),
            now,
        );
        StageApp {
            scene3: spatial::Scene3::default(),
            stage,
            atlas,
            frame: None,
            press: None,
            cursor: [0.0; 2],
            last_click: 0.0,
            touch: None,
            #[cfg(target_arch = "wasm32")]
            published: String::new(),
        }
    }

    /// Keys: arrows and page keys step, up/down jump slides, digits open chapters, T swaps white and black.
    pub fn key(&mut self, key: winit::keyboard::Key<&str>, now: f64) -> bool {
        use winit::keyboard::{Key, NamedKey};
        let stage = &mut self.stage;
        match key {
            Key::Named(
                NamedKey::ArrowRight | NamedKey::PageDown | NamedKey::Space | NamedKey::Enter,
            ) => stage.next(now),
            Key::Named(NamedKey::ArrowLeft | NamedKey::PageUp | NamedKey::Backspace) => {
                stage.prev(now)
            }
            Key::Named(NamedKey::ArrowDown) => stage.slide(1, now),
            Key::Named(NamedKey::ArrowUp) => stage.slide(-1, now),
            Key::Named(NamedKey::Home) => stage.chapter_go(stage.chapter, now),
            Key::Character(" ") => stage.next(now),
            Key::Character(c) if c == "r" || c == "R" => stage.cam_serial += 1,
            Key::Character(c) if c == "t" || c == "T" => {
                stage.set_theme(stage.theme.toggled(), now)
            }
            Key::Character(c) => match c.parse::<usize>() {
                Ok(n) if n >= 1 && n <= stage.chapters.len() => stage.chapter_go(n - 1, now),
                _ => return false,
            },
            _ => return false,
        }
        stage.changed = true;
        true
    }

    /// Pointer input in CSS pixels: drag the 3D side to orbit, wheel to zoom, click to advance.
    pub fn pointer(
        &mut self,
        event: &winit::event::WindowEvent,
        scale: f64,
        now: f64,
        camera: &mut crate::camera::Camera,
    ) -> bool {
        use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
        let Some(frame) = self.frame else {
            return false;
        };
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let p = [(position.x / scale) as f32, (position.y / scale) as f32];
                let dx = p[0] - self.cursor[0];
                let dy = p[1] - self.cursor[1];
                self.cursor = p;
                if let Some((start, _, orbit)) = &mut self.press {
                    let moved = (p[0] - start[0]).hypot(p[1] - start[1]);
                    if *orbit && moved > 3.0 {
                        camera.orbit(dx, dy);
                        self.stage.changed = true;
                        return true;
                    }
                }
                false
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => match state {
                ElementState::Pressed => {
                    let at = frame.slide(egui::pos2(self.cursor[0], self.cursor[1]));
                    self.press = Some((self.cursor, at, at[0] < self.stage.split * run::W));
                    false
                }
                ElementState::Released => {
                    let Some((start, _, orbit)) = self.press.take() else {
                        return false;
                    };
                    if (self.cursor[0] - start[0]).hypot(self.cursor[1] - start[1]) > 4.0 {
                        return false;
                    }
                    self.tap(orbit, now);
                    true
                }
            },
            WindowEvent::Touch(touch) => self.finger(touch, scale, now, camera),
            WindowEvent::MouseWheel { delta, .. } => {
                let at = frame.slide(egui::pos2(self.cursor[0], self.cursor[1]));
                if at[0] >= self.stage.split * run::W {
                    return false;
                }
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y as f64,
                    MouseScrollDelta::PixelDelta(p) => p.y / 60.0,
                };
                camera.zoom(dy as f32);
                self.stage.changed = true;
                true
            }
            _ => false,
        }
    }

    /// A click or tap: a double tap on the 3D side to reset the view, else the next step.
    fn tap(&mut self, orbit: bool, now: f64) {
        if orbit {
            if now - self.last_click < 350.0 {
                self.stage.cam_serial += 1;
            }
            self.last_click = now;
        } else {
            self.stage.next(now);
        }
        self.stage.changed = true;
    }

    // --8<-- [start:stage-touch]
    /// One finger: swipe left for the next step and right for the one before; a slow drag on the 3D side orbits.
    fn finger(
        &mut self,
        touch: &winit::event::Touch,
        scale: f64,
        now: f64,
        camera: &mut crate::camera::Camera,
    ) -> bool {
        use winit::event::TouchPhase;
        let Some(frame) = self.frame else {
            return false;
        };
        let p = [
            (touch.location.x / scale) as f32,
            (touch.location.y / scale) as f32,
        ];
        match touch.phase {
            TouchPhase::Started => {
                let slide = frame.slide(egui::pos2(p[0], p[1]));
                let inside = (0.0..run::H).contains(&slide[1]);
                let orbit = inside && slide[0] < self.stage.split * run::W;
                self.touch = Some(Touch {
                    id: touch.id,
                    start: p,
                    at: p,
                    orbit,
                    down: now,
                });
                false
            }
            TouchPhase::Moved => {
                let Some(finger) = self.touch.as_mut().filter(|f| f.id == touch.id) else {
                    return false;
                };
                let (dx, dy) = (p[0] - finger.at[0], p[1] - finger.at[1]);
                finger.at = p;
                // a quick flick steps the slides, so the view only turns once the finger has stayed
                if !finger.orbit || now - finger.down < 220.0 {
                    return false;
                }
                camera.orbit(dx, dy);
                self.stage.changed = true;
                true
            }
            TouchPhase::Ended => {
                let Some(finger) = self.touch.take().filter(|f| f.id == touch.id) else {
                    return false;
                };
                let (dx, dy) = (p[0] - finger.start[0], p[1] - finger.start[1]);
                if dx.hypot(dy) < 8.0 {
                    self.tap(finger.orbit, now);
                } else if (!finger.orbit || now - finger.down < 350.0)
                    && dx.abs() > 40.0
                    && dx.abs() > 1.5 * dy.abs()
                {
                    if dx < 0.0 {
                        self.stage.next(now);
                    } else {
                        self.stage.prev(now);
                    }
                    self.stage.changed = true;
                }
                true
            }
            TouchPhase::Cancelled => {
                self.touch = None;
                false
            }
        }
    }

    // --8<-- [end:stage-touch]
    /// Paint one frame into the egui painter; true while more frames are needed.
    pub fn paint(
        &mut self,
        painter: &egui::Painter,
        screen: egui::Rect,
        ppp: f32,
        _now: f64,
    ) -> bool {
        let frame = paint::Frame::new(screen, ppp).stacked(screen, self.stage.split);
        self.frame = Some(frame);
        self.atlas.overflowed = false;
        paint::paint(&mut self.stage, &mut self.atlas, painter, &frame, screen);
        let more = self.stage.animating() || self.stage.changed || self.atlas.overflowed;
        self.stage.changed = false;
        #[cfg(target_arch = "wasm32")]
        self.publish();
        more
    }

    /// Step, animation and overlap state on the canvas for browser checks.
    #[cfg(target_arch = "wasm32")]
    fn publish(&mut self) {
        let stage = &mut self.stage;
        let (slide, step) = stage.position();
        let overlaps = if stage.animating() {
            Vec::new()
        } else {
            stage.overlaps()
        };
        let boxes: Vec<serde_json::Value> = stage
            .item_boxes()
            .into_iter()
            .map(|(id, b)| serde_json::json!({"id": id, "box": [b[0], b[1], b[2], b[3]]}))
            .collect();
        let json = serde_json::json!({
            "chapter": stage.chapter,
            "chapters": stage.chapters.len(),
            "step": stage.step,
            "steps": stage.steps(),
            "slide": slide,
            "slide_step": step,
            "title": stage.title.0,
            "say": stage.say.0,
            "animating": stage.animating(),
            "overlaps": overlaps,
            "items": boxes,
            "errors": stage.errors,
        })
        .to_string();
        if json == self.published {
            return;
        }
        if let Some(canvas) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("canvas"))
        {
            let _ = canvas.set_attribute("data-stage", &json);
            let _ = canvas.set_attribute("data-stage-ready", "true");
        }
        self.published = json;
    }
}

/// The deck URL of `?stage=name`: `stage/name/index.yaml` in the data bucket, or a `.yaml` path.
#[cfg(target_arch = "wasm32")]
pub fn route() -> Option<String> {
    let path = crate::app::route::query("stage")?;
    let unsafe_path = path.contains("..") || path.contains("//") || path.contains(':');
    let ok = !path.is_empty() && !unsafe_path;
    if !ok {
        log::warn!("stage: refusing {path}");
        return None;
    }
    let path = if path.ends_with(".yaml") {
        path
    } else {
        format!("stage/{}/index.yaml", path.trim_matches('/'))
    };
    Some(format!("{}{}", crate::app::route::data_base(), path))
}

/// Fetch fonts, index and chapters, then hand them to the state.
#[cfg(target_arch = "wasm32")]
pub async fn open(url: String) {
    use crate::app::fetch::fetch_bytes;
    let result: Result<Loaded, String> = async {
        let mut files = Vec::new();
        for name in font::FILES {
            files.push(
                fetch_bytes(&format!("stage/{name}"))
                    .await
                    .map_err(|e| format!("font {name}: {e}"))?,
            );
        }
        let fonts = font::Fonts::new(files)?;
        let text = |b: Vec<u8>| String::from_utf8(b).map_err(|e| e.to_string());
        let index_bytes = fetch_bytes(&url).await?;
        let index = deck::index(&text(index_bytes)?)?;
        let base = url
            .rsplit_once('/')
            .map_or(String::new(), |(b, _)| format!("{b}/"));
        let mut chapters = Vec::new();
        let mut files = Vec::new();
        for name in &index.chapters {
            let bytes = fetch_bytes(&format!("{base}{name}"))
                .await
                .map_err(|e| format!("{name}: {e}"))?;
            let src = text(bytes)?;
            for file in deck::pictures(&src) {
                if !files.contains(&file) {
                    files.push(file);
                }
            }
            chapters.push(deck::chapter(&src, name)?);
        }
        let mut pictures = Vec::new();
        for file in files {
            let bytes = fetch_bytes(&format!("{base}{file}"))
                .await
                .map_err(|e| format!("{file}: {e}"))?;
            let picture = paint::picture(&bytes).map_err(|e| format!("{file}: {e}"))?;
            pictures.push((file, picture));
        }
        Ok(Loaded {
            fonts,
            index,
            chapters,
            pictures,
        })
    }
    .await;
    match result {
        Ok(loaded) => crate::app::loader::post_stage(loaded),
        Err(e) => crate::app::feedback::error(&format!("stage: {e}")),
    }
}
