//! Stage: YAML-authored mathematical slides. The left of the slide is an orthographic 3D view,
//! the right a column of TeX formulas and rounded-cell matrices; steps animate between them.
pub mod deck;
pub mod eval;
pub mod font;
pub mod paint;
pub mod run;
pub mod spatial;
pub mod tex;
#[cfg(test)]
mod tests;

use std::rc::Rc;

/// Fonts and parsed deck, handed from the loader to the state.
pub struct Loaded {
    pub fonts: font::Fonts,
    pub index: deck::Index,
    pub chapters: Vec<deck::Chapter>,
}

/// The running deck with its painter and pointer state.
pub struct StageApp {
    pub stage: run::Stage,
    pub atlas: paint::Atlas,
    pub frame: Option<paint::Frame>,
    pub tabs: Vec<(egui::Rect, usize)>,
    pub scene3: spatial::Scene3,
    press: Option<([f32; 2], [f32; 2], bool)>,
    cursor: [f32; 2],
    last_click: f64,
    published: String,
}

impl StageApp {
    pub fn new(loaded: Loaded, now: f64) -> Self {
        let fonts = Rc::new(loaded.fonts);
        let atlas = paint::Atlas::new(&fonts);
        let mut stage = run::Stage::new(fonts, loaded.index, loaded.chapters);
        stage.now = now;
        #[cfg(target_arch = "wasm32")]
        let query = |k: &str| crate::app::route::query(k).and_then(|v| v.parse::<usize>().ok());
        #[cfg(not(target_arch = "wasm32"))]
        let query = |_: &str| None::<usize>;
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
            tabs: Vec::new(),
            press: None,
            cursor: [0.0; 2],
            last_click: 0.0,
            published: String::new(),
        }
    }

    /// Keys: arrows and page keys step, up/down jump slides, digits open chapters.
    pub fn key(&mut self, key: winit::keyboard::Key<&str>, now: f64) -> bool {
        use winit::keyboard::{Key, NamedKey};
        let s = &mut self.stage;
        match key {
            Key::Named(
                NamedKey::ArrowRight | NamedKey::PageDown | NamedKey::Space | NamedKey::Enter,
            ) => s.next(now),
            Key::Named(NamedKey::ArrowLeft | NamedKey::PageUp | NamedKey::Backspace) => s.prev(now),
            Key::Named(NamedKey::ArrowDown) => s.slide(1, now),
            Key::Named(NamedKey::ArrowUp) => s.slide(-1, now),
            Key::Named(NamedKey::Home) => s.chapter_go(s.chapter, now),
            Key::Character(c) if c == " " => s.next(now),
            Key::Character(c) if c == "r" || c == "R" => s.cam_serial += 1,
            Key::Character(c) => match c.parse::<usize>() {
                Ok(d) if d >= 1 && d <= s.chapters.len() => s.chapter_go(d - 1, now),
                _ => return false,
            },
            _ => return false,
        }
        s.changed = true;
        true
    }

    /// Pointer input in CSS pixels: drag the 3D side to orbit, wheel to zoom, click to advance.
    pub fn pointer(&mut self, event: &winit::event::WindowEvent, scale: f64, now: f64, camera: &mut crate::camera::Camera) -> bool {
        use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
        let Some(f) = self.frame else { return false };
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
                    let s = f.slide(egui::pos2(self.cursor[0], self.cursor[1]));
                    self.press = Some((self.cursor, s, s[0] < self.stage.split * run::W));
                    false
                }
                ElementState::Released => {
                    let Some((start, s, orbit)) = self.press.take() else {
                        return false;
                    };
                    if (self.cursor[0] - start[0]).hypot(self.cursor[1] - start[1]) > 4.0 {
                        return false;
                    }
                    let p = egui::pos2(self.cursor[0], self.cursor[1]);
                    if let Some(&(_, c)) = self.tabs.iter().find(|(r, _)| r.contains(p)) {
                        self.stage.chapter_go(c, now);
                    } else if orbit {
                        if now - self.last_click < 350.0 {
                            self.stage.cam_serial += 1;
                        }
                        self.last_click = now;
                    } else if s[1] > 90.0 {
                        self.stage.next(now);
                    }
                    self.stage.changed = true;
                    true
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                let s = f.slide(egui::pos2(self.cursor[0], self.cursor[1]));
                if s[0] >= self.stage.split * run::W {
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

    /// Paint one frame into the egui painter; true while more frames are needed.
    pub fn paint(
        &mut self,
        painter: &egui::Painter,
        screen: egui::Rect,
        ppp: f32,
        now: f64,
    ) -> bool {
        let _ = now;
        let f = paint::Frame::new(screen, ppp);
        self.frame = Some(f);
        self.atlas.overflowed = false;
        self.tabs = paint::paint(&mut self.stage, &mut self.atlas, painter, &f, screen);
        let more = self.stage.animating() || self.stage.changed || self.atlas.overflowed;
        self.stage.changed = false;
        #[cfg(target_arch = "wasm32")]
        self.publish();
        more
    }

    /// Step, animation and overlap state on the canvas for browser checks.
    #[cfg(target_arch = "wasm32")]
    fn publish(&mut self) {
        let s = &mut self.stage;
        let (slide, step) = s.position();
        let overlaps = if s.animating() {
            Vec::new()
        } else {
            s.overlaps()
        };
        let boxes: Vec<serde_json::Value> = s
            .item_boxes()
            .into_iter()
            .map(|(id, b)| serde_json::json!({"id": id, "box": [b[0], b[1], b[2], b[3]]}))
            .collect();
        let json = serde_json::json!({
            "chapter": s.chapter,
            "chapters": s.chapters.len(),
            "step": s.step,
            "steps": s.steps(),
            "slide": slide,
            "slide_step": step,
            "title": s.title.0,
            "say": s.say.0,
            "animating": s.animating(),
            "overlaps": overlaps,
            "items": boxes,
            "errors": s.errors,
        })
        .to_string();
        if json == self.published {
            return;
        }
        self.published = json.clone();
        if let Some(canvas) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("canvas"))
        {
            let _ = canvas.set_attribute("data-stage", &json);
            let _ = canvas.set_attribute("data-stage-ready", "true");
        }
    }
}

#[cfg(target_arch = "wasm32")]
/// `?stage=name` reads `stage/name/index.yaml` from the data bucket; a path ending in `.yaml` is used as given.
pub fn route() -> Option<String> {
    let path = crate::app::route::query("stage")?;
    let ok =
        !path.is_empty() && !path.contains("..") && !path.contains("//") && !path.contains(':');
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
        let index = deck::index(&text(fetch_bytes(&url).await?)?)?;
        let base = url
            .rsplit_once('/')
            .map_or(String::new(), |(b, _)| format!("{b}/"));
        let mut chapters = Vec::new();
        for c in &index.chapters {
            let src = text(
                fetch_bytes(&format!("{base}{c}"))
                    .await
                    .map_err(|e| format!("{c}: {e}"))?,
            )?;
            chapters.push(deck::chapter(&src, c)?);
        }
        Ok(Loaded {
            fonts,
            index,
            chapters,
        })
    }
    .await;
    match result {
        Ok(loaded) => crate::app::loader::post_stage(loaded),
        Err(e) => crate::app::feedback::error(&format!("stage: {e}")),
    }
}
