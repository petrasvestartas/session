
#[cfg(target_arch = "wasm32")]
impl App {
    /// Keep the whole fonts for the page's life, shared by the labels and the panels.
    fn use_fonts(&mut self, faces: Vec<Vec<u8>>) {
        let Some(state) = &mut self.state else { return };
        let faces: Vec<&'static [u8]> = faces
            .into_iter()
            // `Box::leak` hands the bytes a 'static lifetime: they are never freed, which suits fonts kept for the page's life
            .map(|face| &*Box::leak(face.into_boxed_slice()))
            .collect();

        if let Ok(faces) = <[&'static [u8]; 3]>::try_from(faces) {
        }
    }
}
