#[derive(Clone)]
pub struct LineObject {
    pub guid:String,
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
