    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn late_entity_answers_require_current_ticket_original_owner_and_current_selection_once() {
        let mut scene = Scene::empty();
        let id = scene
            .insert_sheet(Rc::new(
                Prepared::new(Rc::new(crate::sheet_example::source())).unwrap(),
            ))
            .unwrap();
        let target = Some(Target {
            owner: id,
            part: Part::SheetEntity(1),
        });
        let mut gate = Gate::default();
        let (first, cancelled) = gate.begin(&scene.sheets()[0], 1).unwrap();
        let (second, _) = gate.begin(&scene.sheets()[0], 1).unwrap();
        assert!(cancelled.get());
        assert!(!gate.finish(first, &scene, target));
        assert!(gate.finish(second, &scene, target));
        assert!(!gate.finish(second, &scene, target));
        let (third, _) = gate.begin(&scene.sheets()[0], 1).unwrap();
        assert!(!gate.finish(third, &scene, None));
        let (fourth, _) = gate.begin(&scene.sheets()[0], 1).unwrap();
        scene.clear();
        assert!(!gate.finish(fourth, &scene, target));
    }
    #[test]
    fn entity_queries_hold_no_original_source_and_cancellation_retires_the_read_token() {
        let mut scene = Scene::empty();
        scene
            .insert_sheet(Rc::new(
                Prepared::new(Rc::new(crate::sheet_example::source())).unwrap(),
            ))
            .unwrap();
        let source = Rc::downgrade(&scene.sheets()[0].prepared);
        let mut gate = Gate::default();
        let (_, token) = gate.begin(&scene.sheets()[0], 1).unwrap();
        scene.clear();
        assert!(source.upgrade().is_none());
        gate.cancel();
        assert!(token.get());
    }
}
