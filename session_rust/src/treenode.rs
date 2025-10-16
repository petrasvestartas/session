use crate::tree::Tree;
use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};
use uuid::Uuid;

// Internal type alias to hide complexity
type NodeRef = Rc<RefCell<TreeNodeInner>>;
type WeakNodeRef = Weak<RefCell<TreeNodeInner>>;

#[derive(Debug, Clone)]
struct TreeNodeInner {
    pub guid: String,
    pub name: String,
    children: Vec<NodeRef>,
    parent: Option<WeakNodeRef>,
    tree: Option<Weak<RefCell<Tree>>>,
}

/// TreeNode with a clean, simple API
///
/// TreeNodes can represent either:
/// - Geometry nodes: name is set to the geometry's GUID for lookup
/// - Organizational nodes: name is a descriptive string (e.g., "folder", "group")
///
/// When adding geometry to a Session, the TreeNode.name is automatically set to
/// the geometry.guid, allowing the tree hierarchy to reference geometry objects.
///
/// Note: TreeNode has its own guid (for identifying the node itself) and a name
/// (which for geometry nodes, stores the geometry's GUID).
#[derive(Debug, Clone)]
pub struct TreeNode {
    inner: NodeRef,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Serialize for TreeNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_serde().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TreeNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serde_repr = TreeNodeSerde::deserialize(deserializer)?;
        Ok(Self::from_serde(serde_repr))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename = "TreeNode")]
pub(crate) struct TreeNodeSerde {
    pub guid: String,
    pub name: String,
    pub children: Vec<TreeNodeSerde>,
}

impl TreeNode {
    pub fn new(name: &str) -> Self {
        Self {
            inner: Rc::new(RefCell::new(TreeNodeInner {
                guid: Uuid::new_v4().to_string(),
                name: name.to_string(),
                children: Vec::new(),
                parent: None,
                tree: None,
            })),
        }
    }

    pub fn name(&self) -> String {
        self.inner.borrow().name.clone()
    }

    pub fn guid(&self) -> String {
        self.inner.borrow().guid.clone()
    }

    pub fn add(&self, child: &TreeNode) {
        child.inner.borrow_mut().parent = Some(Rc::downgrade(&self.inner));
        child.inner.borrow_mut().tree = self.inner.borrow().tree.clone();
        self.inner.borrow_mut().children.push(child.inner.clone());
    }

    pub fn remove(&self, child: &TreeNode) -> bool {
        let child_guid = child.guid();
        let mut inner = self.inner.borrow_mut();
        if let Some(pos) = inner
            .children
            .iter()
            .position(|c| c.borrow().guid == child_guid)
        {
            let removed = inner.children.remove(pos);
            removed.borrow_mut().parent = None;
            true
        } else {
            false
        }
    }

    pub fn parent(&self) -> Option<TreeNode> {
        self.inner
            .borrow()
            .parent
            .as_ref()?
            .upgrade()
            .map(|inner| TreeNode { inner })
    }

    pub fn children(&self) -> Vec<TreeNode> {
        self.inner
            .borrow()
            .children
            .iter()
            .map(|child| TreeNode {
                inner: Rc::clone(child),
            })
            .collect()
    }

    pub fn is_root(&self) -> bool {
        self.inner.borrow().parent.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.inner.borrow().children.is_empty()
    }

    pub fn ancestors(&self) -> Vec<TreeNode> {
        let mut result = Vec::new();
        let mut current = self.parent();

        while let Some(node) = current {
            result.push(node.clone());
            current = node.parent();
        }

        result
    }

    pub fn descendants(&self) -> Vec<TreeNode> {
        let mut result = Vec::new();
        for child in self.children() {
            result.push(child.clone());
            result.extend(child.descendants());
        }
        result
    }

    pub fn nodes(&self) -> Vec<TreeNode> {
        let mut result = vec![self.clone()];
        for child in self.children() {
            result.extend(child.nodes());
        }
        result
    }

    pub fn root(&self) -> TreeNode {
        if let Some(parent) = self.parent() {
            parent.root()
        } else {
            self.clone()
        }
    }

    pub fn traverse(&self, strategy: &str, order: &str) -> Vec<TreeNode> {
        match strategy {
            "depthfirst" => self.depth_first_traverse(order),
            "breadthfirst" => self.breadth_first_traverse(),
            _ => vec![],
        }
    }

    fn depth_first_traverse(&self, order: &str) -> Vec<TreeNode> {
        match order {
            "preorder" => self.preorder_traverse(),
            "postorder" => self.postorder_traverse(),
            _ => vec![],
        }
    }

    fn preorder_traverse(&self) -> Vec<TreeNode> {
        let mut result = vec![self.clone()];
        for child in self.children() {
            result.extend(child.preorder_traverse());
        }
        result
    }

    fn postorder_traverse(&self) -> Vec<TreeNode> {
        let mut result = Vec::new();
        for child in self.children() {
            result.extend(child.postorder_traverse());
        }
        result.push(self.clone());
        result
    }

    fn breadth_first_traverse(&self) -> Vec<TreeNode> {
        let mut result = Vec::new();
        let mut queue = Vec::new();

        queue.push(self.clone());

        while let Some(node) = queue.pop() {
            result.push(node.clone());
            for child in node.children() {
                queue.insert(0, child);
            }
        }

        result
    }

    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let serde_node = self.to_serde();
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(&serde_node, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let serde_node: TreeNodeSerde = serde_json::from_str(json_data)?;
        Ok(Self::from_serde(serde_node))
    }

    pub(crate) fn to_serde(&self) -> TreeNodeSerde {
        let inner = self.inner.borrow();
        TreeNodeSerde {
            guid: inner.guid.clone(),
            name: inner.name.clone(),
            children: inner
                .children
                .iter()
                .map(|child| {
                    TreeNode {
                        inner: Rc::clone(child),
                    }
                    .to_serde()
                })
                .collect(),
        }
    }

    pub(crate) fn from_serde(serde_node: TreeNodeSerde) -> Self {
        let node = TreeNode::new(&serde_node.name);
        node.inner.borrow_mut().guid = serde_node.guid;

        for child_serde in serde_node.children {
            let child = Self::from_serde(child_serde);
            node.add(&child);
        }

        node
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.borrow();
        write!(
            f,
            "TreeNode({}, {}, {} children)",
            inner.name,
            inner.guid,
            inner.children.len()
        )
    }
}

#[cfg(test)]
#[path = "treenode_test.rs"]
mod treenode_test;
