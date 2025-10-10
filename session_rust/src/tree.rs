use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::fs;
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
#[derive(Debug, Clone)]
pub struct TreeNode {
    inner: NodeRef,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename = "TreeNode")]
struct TreeNodeSerde {
    guid: String,
    name: String,
    children: Vec<TreeNodeSerde>,
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

    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let serde_node = self.to_serde();
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(&serde_node, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let serde_node: TreeNodeSerde = serde_json::from_str(json_data)?;
        Ok(Self::from_serde(serde_node))
    }

    fn to_serde(&self) -> TreeNodeSerde {
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

    fn from_serde(serde_node: TreeNodeSerde) -> Self {
        let node = TreeNode::new(&serde_node.name);
        node.inner.borrow_mut().guid = serde_node.guid;

        for child_serde in serde_node.children {
            let child = Self::from_serde(child_serde);
            node.add(&child);
        }

        node
    }

    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        fs::write(filepath, json)?;
        Ok(())
    }

    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
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

#[derive(Debug, Clone)]
pub struct Tree {
    pub guid: String,
    pub name: String,
    root_node: Option<TreeNode>,
}

impl Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serde_tree = TreeSerde {
            guid: self.guid.clone(),
            name: self.name.clone(),
            root: self.root_node.as_ref().map(|r| r.to_serde()),
        };
        serde_tree.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Tree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serde_tree = TreeSerde::deserialize(deserializer)?;
        let mut tree = Tree {
            guid: serde_tree.guid,
            name: serde_tree.name,
            root_node: None,
        };
        if let Some(root_serde) = serde_tree.root {
            tree.root_node = Some(TreeNode::from_serde(root_serde));
        }
        Ok(tree)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename = "Tree")]
struct TreeSerde {
    guid: String,
    name: String,
    root: Option<TreeNodeSerde>,
}

impl Tree {
    pub fn new(name: &str) -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: name.to_string(),
            root_node: None,
        }
    }

    pub fn root(&self) -> Option<TreeNode> {
        self.root_node.clone()
    }

    pub fn add(&mut self, node: &TreeNode, parent: Option<&TreeNode>) {
        if parent.is_none() {
            self.root_node = Some(node.clone());
        } else if let Some(parent_node) = parent {
            parent_node.add(node);
        }
    }

    pub fn nodes(&self) -> Vec<TreeNode> {
        if let Some(root) = &self.root_node {
            root.nodes()
        } else {
            vec![]
        }
    }

    pub fn remove(&mut self, node: &TreeNode) -> bool {
        if let Some(root) = &self.root_node {
            let node_guid = node.guid();
            if root.guid() == node_guid {
                self.root_node = None;
                true
            } else {
                // Find parent and remove from there
                if let Some(parent) = self.find_parent_of_node(&node_guid) {
                    parent.remove(node)
                } else {
                    false
                }
            }
        } else {
            false
        }
    }

    fn find_parent_of_node(&self, node_guid: &String) -> Option<TreeNode> {
        if let Some(root) = &self.root_node {
            Self::find_parent_recursive(root, node_guid)
        } else {
            None
        }
    }

    fn find_parent_recursive(node: &TreeNode, target_guid: &String) -> Option<TreeNode> {
        for child in node.children() {
            if child.guid() == *target_guid {
                return Some(node.clone());
            }
            if let Some(found) = Self::find_parent_recursive(&child, target_guid) {
                return Some(found);
            }
        }
        None
    }

    pub fn leaves(&self) -> Vec<TreeNode> {
        self.nodes().into_iter().filter(|n| n.is_leaf()).collect()
    }

    pub fn traverse(&self, strategy: &str, order: &str) -> Vec<TreeNode> {
        if let Some(root) = &self.root_node {
            root.traverse(strategy, order)
        } else {
            vec![]
        }
    }

    pub fn get_node_by_name(&self, node_name: &str) -> Option<TreeNode> {
        self.nodes().into_iter().find(|n| n.name() == node_name)
    }

    pub fn get_nodes_by_name(&self, node_name: &str) -> Vec<TreeNode> {
        self.nodes()
            .into_iter()
            .filter(|n| n.name() == node_name)
            .collect()
    }

    pub fn find_node_by_guid(&self, node_guid: &String) -> Option<TreeNode> {
        self.nodes().into_iter().find(|n| n.guid() == *node_guid)
    }

    pub fn add_child_by_guid(&mut self, parent_guid: &String, child_guid: &String) -> bool {
        let parent_node = self.find_node_by_guid(parent_guid);
        let child_node = self.find_node_by_guid(child_guid);

        if let (Some(parent), Some(child)) = (parent_node, child_node) {
            // Remove child from its current parent if it has one
            if let Some(current_parent) = child.parent() {
                current_parent.remove(&child);
            }

            // Add to new parent
            parent.add(&child);
            true
        } else {
            false
        }
    }

    pub fn get_children_guids(&self, node_guid: &String) -> Vec<String> {
        if let Some(node) = self.find_node_by_guid(node_guid) {
            node.children().iter().map(|c| c.guid()).collect()
        } else {
            vec![]
        }
    }

    /// Get children GUIDs by string GUID (API compatibility method).
    pub fn get_children(&self, node_guid: &str) -> Vec<String> {
        self.get_children_guids(&node_guid.to_string())
    }

    pub fn print_hierarchy(&self) {
        if let Some(root) = &self.root_node {
            Self::print_node(root, 0);
        }
    }

    fn print_node(node: &TreeNode, level: usize) {
        let indent = "  ".repeat(level);
        println!("{}├── {} ({})", indent, node.name(), node.guid());

        for child in node.children() {
            Self::print_node(&child, level + 1);
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let serde_tree = TreeSerde {
            guid: self.guid.clone(),
            name: self.name.clone(),
            root: self.root_node.as_ref().map(|r| r.to_serde()),
        };
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(&serde_tree, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let serde_tree: TreeSerde = serde_json::from_str(json_data)?;
        let mut tree = Tree::new(&serde_tree.name);
        tree.guid = serde_tree.guid;

        if let Some(root_serde) = serde_tree.root {
            let root = TreeNode::from_serde(root_serde);
            tree.root_node = Some(root);
        }

        Ok(tree)
    }

    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        fs::write(filepath, json)?;
        Ok(())
    }

    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tree({}, {})", self.name, self.guid)
    }
}

impl Default for TreeNode {
    fn default() -> Self {
        Self::new("my_node")
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new("my_tree")
    }
}

#[cfg(test)]
#[path = "tree_test.rs"]
mod tree_test;
