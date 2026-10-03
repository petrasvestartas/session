    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::{Action, Editor}, reload_url::ReloadUrl};
    use std::{cell::RefCell, rc::Rc};

    fn cold() -> (Editor, Vec<ReloadKey>) {
        let mut editor = Editor::default();
        let location = Rc::new(ReloadUrl::new("test:request".into()));
        editor.apply(Action::ReplaceAt(crate::specimen::bytes(), location)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        let keys = editor.reload_keys(); (editor, keys)
    }

    #[test]
    fn only_current_work_can_finish_and_it_finishes_once() {
        let (_, keys) = cold(); let mut job = ReloadJob::default();
        let first = job.begin(keys.clone()).unwrap();
        let second = job.begin(keys.clone()).unwrap();
        assert!(second.ticket > first.ticket);
        assert!(job.finish(first.ticket).is_none()); assert_eq!(job.pending(), Some(second.ticket));
        let accepted = job.finish(second.ticket).unwrap(); assert_eq!(accepted.len(), 1);
        assert!(Rc::ptr_eq(&accepted[0].origin, &keys[0].origin));
        assert!(job.finish(second.ticket).is_none());
        let third = job.begin(keys.clone()).unwrap(); assert!(job.cancel());
        assert!(!job.cancel()); assert!(job.finish(third.ticket).is_none());
        assert!(job.begin(vec![keys[0].clone(), keys[0].clone()]).is_err());
        assert!(job.begin(Vec::new()).is_err()); assert_eq!(job.pending(), None);
    }

    #[test]
    fn cancellation_drops_keys_even_when_abandoned_work_holds_the_job() {
        let (mut editor, keys) = cold();
        let origin = Rc::downgrade(&keys[0].origin);
        let location = Rc::downgrade(keys[0].origin.location.as_ref().unwrap());
        let job = Rc::new(RefCell::new(ReloadJob::default()));
        let request = job.borrow_mut().begin(keys).unwrap();
        let abandoned = Rc::clone(&job);
        editor.apply(Action::Close).unwrap(); assert!(origin.upgrade().is_some());
        assert!(job.borrow_mut().cancel());
        assert!(origin.upgrade().is_none() && location.upgrade().is_none());
        assert_eq!(request.urls, vec!["test:request".to_owned()]);
        assert!(abandoned.borrow_mut().finish(request.ticket).is_none());
    }

    #[test]
    fn exhaustion_cancels_old_keys_and_never_wraps() {
        let (_, keys) = cold(); let mut job = ReloadJob::default();
        job.begin(keys.clone()).unwrap(); job.issued = u64::MAX;
        assert!(job.begin(keys).is_err()); assert_eq!(job.pending(), None);
        assert_eq!(job.issued, u64::MAX);
    }
}
