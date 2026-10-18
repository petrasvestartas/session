use crate::{editor::{Action, Editor}, rehydrate::ReloadKey, scene::ObjectId};

#[derive(Clone, Debug, PartialEq)]
pub enum Intent {
    Command(crate::command_request::Pending),
    Move { id: ObjectId, offset: [f64; 3] },
    Delete { id: ObjectId },
