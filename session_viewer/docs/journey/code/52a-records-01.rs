    xforms.get(guid).cloned().unwrap_or_else(Xform::identity)
}

pub struct Node { pub source:proto::TreeNode,pub object:Option<ObjectId>,pub label:String,pub parent:Option<usize>,pub depth:usize,pub end:usize,pub readonly:bool }
pub struct Document {pub key:u32,pub label:String,pub nodes:Vec<Node>}
#[derive(Clone,Default)]
pub struct Forest { pub documents:Vec<Rc<Document>> }

impl Forest {}
