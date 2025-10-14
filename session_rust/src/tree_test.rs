#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::point::Point;
    use crate::tree::{Tree, TreeNode};

    #[test]
    fn test_treenode_constructor() {
        // Test TreeNode constructor
        let node = TreeNode::new("root");
        assert_eq!(node.name(), "root");
        assert!(node.is_root());
    }

    #[test]
    fn test_treenode_to_json_data() {
        // Test TreeNode to_json_data method - NOW MUCH SIMPLER!
        let root = TreeNode::new("project_root");
        let folder1 = TreeNode::new("src");
        let folder2 = TreeNode::new("docs");
        let file1 = TreeNode::new("main.py");
        let file2 = TreeNode::new("README.md");

        root.add(&folder1);
        root.add(&folder2);
        folder1.add(&file1);
        folder2.add(&file2);

        let data = root.jsondump().unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&data).unwrap();

        assert_eq!(json_value["name"], "project_root");
        assert_eq!(json_value["type"], "TreeNode");
        assert_eq!(json_value["children"].as_array().unwrap().len(), 2);
        assert_eq!(json_value["children"][0]["name"], "src");
        assert_eq!(
            json_value["children"][0]["children"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn test_treenode_from_json_data() {
        // Test TreeNode from_json_data method
        let original_root = TreeNode::new("filesystem_root");
        let bin_folder = TreeNode::new("bin");
        let lib_folder = TreeNode::new("lib");
        let app_file = TreeNode::new("app.exe");
        let config_file = TreeNode::new("config.dll");

        original_root.add(&bin_folder);
        original_root.add(&lib_folder);
        bin_folder.add(&app_file);
        lib_folder.add(&config_file);

        let data = original_root.jsondump().unwrap();
        let restored_root = TreeNode::jsonload(&data).unwrap();

        assert_eq!(restored_root.name(), "filesystem_root");
        assert_eq!(restored_root.children().len(), 2);
        assert_eq!(restored_root.children()[0].name(), "bin");
        assert_eq!(restored_root.children()[0].children().len(), 1);
    }

    #[test]
    fn test_treenode_add() {
        // Test TreeNode add method.
        let parent = TreeNode::new("parent");
        let child = TreeNode::new("child");
        parent.add(&child);
        assert_eq!(parent.children().len(), 1);
        assert_eq!(child.parent().unwrap(), parent);
    }

    #[test]
    fn test_treenode_remove() {
        // Test TreeNode remove method
        let parent = TreeNode::new("parent");
        let child = TreeNode::new("child");

        parent.add(&child);
        parent.remove(&child);

        assert_eq!(parent.children().len(), 0);
        assert!(child.parent().is_none());
    }

    #[test]
    fn test_treenode_traverse() {
        // Test TreeNode traverse method
        let root = TreeNode::new("root");
        let child = TreeNode::new("child");
        root.add(&child);
        let nodes = root.traverse("depthfirst", "preorder");
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0], root);
    }

    #[test]
    fn test_tree_constructor() {
        // Test Tree constructor
        let tree = Tree::new("my_tree");
        assert_eq!(tree.name, "my_tree");
        assert!(tree.root().is_none());
    }

    #[test]
    fn test_tree_to_json_data() {
        // Test Tree to_json_data method
        let mut tree = Tree::new("object_hierarchy");
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point3 = Point::new(7.0, 8.0, 9.0);
        let point4 = Point::new(10.0, 11.0, 12.0);

        let root_node = TreeNode::new(&point1.guid.to_string());
        let child1 = TreeNode::new(&point2.guid.to_string());
        let child2 = TreeNode::new(&point3.guid.to_string());
        let grandchild = TreeNode::new(&point4.guid.to_string());

        tree.add(&root_node, None);
        tree.add(&child1, Some(&root_node));
        tree.add(&child2, Some(&root_node));
        tree.add(&grandchild, Some(&child1));

        let data = tree.jsondump().unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&data).unwrap();

        assert_eq!(json_value["name"], "object_hierarchy");
        assert_eq!(json_value["type"], "Tree");
        assert_eq!(json_value["root"]["name"], point1.guid.to_string());
        assert_eq!(json_value["root"]["children"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_tree_from_json_data() {
        // Test Tree from_json_data method
        let mut original_tree = Tree::new("spatial_hierarchy");
        let point1 = Point::new(100.0, 200.0, 300.0);
        let point2 = Point::new(400.0, 500.0, 600.0);
        let point3 = Point::new(700.0, 800.0, 900.0);

        let root = TreeNode::new(&point1.guid.to_string());
        let child1 = TreeNode::new(&point2.guid.to_string());
        let child2 = TreeNode::new(&point3.guid.to_string());

        original_tree.add(&root, None);
        original_tree.add(&child1, Some(&root));
        original_tree.add(&child2, Some(&root));

        let data = original_tree.jsondump().unwrap();
        let restored_tree = Tree::jsonload(&data).unwrap();

        assert_eq!(restored_tree.name, "spatial_hierarchy");
        assert_eq!(
            restored_tree.root().unwrap().name(),
            point1.guid.to_string()
        );
        assert_eq!(restored_tree.nodes().len(), 3);
    }

    #[test]
    fn test_tree_to_json_from_json() {
        // Test Tree file I/O with to_json and from_json
        let mut tree = Tree::new("my_tree");
        let point1 = Point::new(0.0, 0.0, 0.0);
        let point2 = Point::new(1.0, 1.0, 1.0);
        let point3 = Point::new(2.0, 2.0, 2.0);
        let point4 = Point::new(3.0, 3.0, 3.0);

        let root = TreeNode::new(&point1.guid.to_string());
        let branch1 = TreeNode::new(&point2.guid.to_string());
        let branch2 = TreeNode::new(&point3.guid.to_string());
        let leaf = TreeNode::new(&point4.guid.to_string());

        tree.add(&root, None);
        tree.add(&branch1, Some(&root));
        tree.add(&branch2, Some(&root));
        tree.add(&leaf, Some(&branch1));

        let filename = "test_tree.json";

        json_dump(&tree, filename, true).unwrap();
        let loaded_tree = json_load::<Tree>(filename).unwrap();

        assert_eq!(loaded_tree.name, tree.name);
        assert_eq!(
            loaded_tree.root().unwrap().name(),
            tree.root().unwrap().name()
        );
        assert_eq!(loaded_tree.nodes().len(), tree.nodes().len());
    }

    #[test]
    fn test_tree_add() {
        // Test Tree add method
        let mut tree = Tree::new("my_tree");
        let root = TreeNode::new("root");
        tree.add(&root, None);
        assert_eq!(tree.root().unwrap(), root);
        assert_eq!(tree.nodes().len(), 1);
    }

    #[test]
    fn test_tree_remove() {
        // Test Tree remove method
        let mut tree = Tree::new("my_tree");
        let root = TreeNode::new("root");

        tree.add(&root, None);
        tree.remove(&root);

        assert!(tree.root().is_none());
    }

    #[test]
    fn test_tree_get_node_by_name() {
        // Test Tree get_node_by_name method
        let mut tree = Tree::new("my_tree");
        let root = TreeNode::new("root");
        tree.add(&root, None);
        let found = tree.get_node_by_name("root");
        assert_eq!(found.unwrap(), root);
    }
}
