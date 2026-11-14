use super::{deck, font, paint, run};
use std::rc::Rc;

// --8<-- [start:stage-presentation]
pub struct Presentation {
    pub stage: run::Stage,
    pub atlas: paint::Atlas,
    pub frame: Option<paint::Frame>,
}
// --8<-- [end:stage-presentation]

impl Presentation {
    pub fn new(
        fonts: font::Fonts,
        index: deck::Index,
        chapters: Vec<deck::Chapter>,
        pictures: Vec<(String, paint::Picture)>,
        now: f64,
    ) -> Self {
        let fonts = Rc::new(fonts);
        let mut atlas = paint::Atlas::new(&fonts);
        let mut stage = run::Stage::new(fonts, index, chapters);
        for (name, picture) in pictures {
            let aspect = picture.size[0] as f32 / picture.size[1].max(1) as f32;
            stage
                .pictures
                .insert(name, (atlas.pictures.len() as u16, aspect));
            atlas.pictures.push(picture);
        }
        stage.now = now;
        Self {
            stage,
            atlas,
            frame: None,
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

    pub fn paint(&mut self, painter: &egui::Painter, screen: egui::Rect, ppp: f32) -> bool {
        let frame = paint::Frame::new(screen, ppp).stacked(screen, self.stage.split);
        self.frame = Some(frame);
        self.atlas.overflowed = false;
        paint::paint(&mut self.stage, &mut self.atlas, painter, &frame, screen);
        let more = self.stage.animating() || self.stage.changed || self.atlas.overflowed;
        self.stage.changed = false;
        more
    }
}
