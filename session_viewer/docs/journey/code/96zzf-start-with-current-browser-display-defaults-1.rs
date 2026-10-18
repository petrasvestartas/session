pub struct Settings { pub opacity:f32, pub grid:bool }

impl Settings {
    pub fn new(coarse:bool,opacity:Option<&str>,nogrid:Option<&str>)->Self {
        let default=if coarse {1.} else {0.95};
        let opacity=opacity.and_then(|s|s.parse::<f32>().ok())
            .filter(|v|v.is_finite()).unwrap_or(default).clamp(0.,1.);
        Self {opacity,grid:nogrid.is_none()}
    }

    pub fn apply(&self,editor:&mut crate::editor::Editor) {
        editor.opacity=self.opacity;
        editor.camera.grid=self.grid.then(Default::default);
    }
}

#[cfg(target_arch="wasm32")]
pub fn initialize(editor:&mut crate::editor::Editor) {
    let Some(window)=web_sys::window() else {return};
    let coarse=window.match_media("(pointer: coarse)").ok().flatten()
        .is_some_and(|media|media.matches());
    let search=window.location().search().unwrap_or_default();
    let query=web_sys::UrlSearchParams::new_with_str(&search).ok();
    let opacity=query.as_ref().and_then(|q|q.get("opacity"));
    let nogrid=query.as_ref().and_then(|q|q.get("nogrid"));
    Settings::new(coarse,opacity.as_deref(),nogrid.as_deref()).apply(editor);
}
