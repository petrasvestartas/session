}
impl Control {
    pub fn observe(
        &self,
        scene: &Scene,
        selected: Option<Target>,
        fault: &crate::gpu_fault::Fault,
    ) {
        let row = selected.and_then(|target| match target.part {
            Part::SheetEntity(entity) if target.valid(scene) => scene
                .sheets()
                .iter()
                .find(|row| row.id == target.owner)
                .map(|row| (row, entity)),
            _ => None,
        });
        let key = row.map(|(row, entity)| (row.id, entity, Rc::as_ptr(&row.prepared) as usize));
        let mut owner = self.0.borrow_mut();
        if owner.selected == key {
            return;
        }
        owner.gate.cancel();
        if let Some(abort) = owner.abort.take() {
            abort.abort();
        }
        owner.reply = None;
        owner.selected = key;
        owner
            .tables
            .retain(|_, (source, _)| source.strong_count() > 0);
        let Some((row, entity)) = row else {
            return;
        };
        let original = &row.prepared.original;
        if original.meta.is_empty() {
            return;
        }
        let Some(url) = row.url.as_ref() else {
            owner.reply = Some(Err(
                "Entity metadata needs the drawing's published URL".into()
            ));
            return;
        };
        let url = match web_sys::Url::new_with_base(&original.meta, url) {
            Ok(url) if matches!(url.protocol().as_str(), "http:" | "https:") => url.href(),
            _ => {
                owner.reply = Some(Err("Entity metadata needs an HTTP drawing URL".into()));
                return;
            }
        };
        let Ok(abort) = web_sys::AbortController::new() else {
            return;
        };
        let Ok((ticket, cancelled)) = owner.gate.begin(row, entity) else {
            return;
        };
        let cached = owner
            .tables
            .get(&key.unwrap().2)
            .map(|(_, table)| table.clone());
        let count = original.entity_count;
        let source = Rc::downgrade(&row.prepared);
        let source_key = key.unwrap().2;
        let signal = abort.signal();
        owner.abort = Some(abort);
        let weak = Rc::downgrade(&self.0);
        let fault = fault.clone();
        drop(owner);
        wasm_bindgen_futures::spawn_local(async move {
            let result =
                crate::sheet_entity_fetch::read(&url, entity, count, cached, &cancelled, &signal)
                    .await;
            complete(weak, fault, ticket, source, source_key, result);
        });
    }
    pub fn take(&self) -> Option<Result<EntityMeta, String>> {
        self.0.borrow_mut().reply.take()
