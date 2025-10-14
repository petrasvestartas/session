use crate::treenode::{TreeNode, TreeNodeSerde};
use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

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
            } else if let Some(parent) = self.find_parent_of_node(&node_guid) {
                parent.remove(node)
            } else {
                false
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
            if let Some(current_parent) = child.parent() {
                current_parent.remove(&child);
            }
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

    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
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

    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let serde_tree: TreeSerde = serde_json::from_str(json_data)?;
        let mut tree = Tree::new(&serde_tree.name);
        tree.guid = serde_tree.guid;

        if let Some(root_serde) = serde_tree.root {
            let root = TreeNode::from_serde(root_serde);
            tree.root_node = Some(root);
        }

        Ok(tree)
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tree({}, {})", self.name, self.guid)
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
