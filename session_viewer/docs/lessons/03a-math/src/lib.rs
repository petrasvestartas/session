pub mod app {
    pub mod stage {
        pub mod actions;
        pub mod check;
        pub mod deck;
        pub mod draw;
        pub mod eval;
        pub mod geometry;
        pub mod layout;
        pub mod matrix;
        pub mod net;
        pub mod object;
        pub mod run;
        pub mod view;
        pub mod font;
        pub mod tex;
        pub mod paint;
        pub mod tween;
        pub mod morph;
        #[cfg(test)]
        mod motion_tests;
        #[cfg(test)]
        mod production_tests;
    }
}

pub fn fonts() -> Result<app::stage::font::Fonts, String> {
    app::stage::font::Fonts::new(vec![
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../assets/stage/NewCMMath-Book.otf")).to_vec(),
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../assets/stage/NewCM10-Book.otf")).to_vec(),
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../assets/stage/NewCM10-Bold.otf")).to_vec(),
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../assets/stage/NewCMMono10-Book.otf")).to_vec(),
    ])
}

#[cfg(target_arch = "wasm32")]
mod browser;

#[cfg(test)]
mod tests;
