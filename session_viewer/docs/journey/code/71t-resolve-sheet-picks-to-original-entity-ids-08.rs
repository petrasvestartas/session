use crate::{
    pick_sources::{Part, Target},
    scene::{ObjectId, Scene},
    sheet_source::Prepared,
};
use std::rc::{Rc, Weak};
pub struct Request {
    pub query: crate::sheet_entity::Query,
    pub source: Weak<Prepared>,
    pub owner: ObjectId,
}
#[derive(Default)]
pub struct Gate {
    next: u64,
    pending: Option<Request>,
}
impl Gate {
    pub fn begin(
        &mut self,
        row: &crate::sheet_source::Row,
        entity: u32,
    ) -> Result<(u64, Rc<std::cell::Cell<bool>>), &'static str> {
        if !row.has_entity(entity) {
            return Err("Selected original sheet entity is gone");
        }
        let id = self
            .next
            .checked_add(1)
            .ok_or("Entity query IDs exhausted")?;
        self.next = id;
        let query = crate::sheet_entity::Query::new(id, row.id.value(), entity);
        let token = query.cancelled.clone();
        self.pending = Some(Request {
            query,
            source: Rc::downgrade(&row.prepared),
            owner: row.id,
        });
        Ok((id, token))
    }
    pub fn cancel(&mut self) {
        self.pending = None;
    }
    pub fn finish(&mut self, id: u64, scene: &Scene, selected: Option<Target>) -> bool {
        if self
            .pending
            .as_ref()
            .is_none_or(|request| request.query.id != id)
        {
            return false;
        }
        let request = self.pending.take().unwrap();
        let target = Target {
            owner: request.owner,
            part: Part::SheetEntity(request.query.entity),
        };
        selected == Some(target)
            && target.valid(scene)
            && request.source.upgrade().is_some_and(|source| {
                scene
                    .sheets()
                    .iter()
                    .any(|row| row.id == request.owner && Rc::ptr_eq(&row.prepared, &source))
            })
    }
}
