
    pub fn update(
        &mut self,
        event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, JsValue> {
        let rect = canvas.get_bounding_client_rect();
