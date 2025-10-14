#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::treenode::TreeNode;

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

        // File I/O test
        json_dump(&original_root, "test_treenode.json", true).unwrap();
        let from_file: TreeNode = json_load("test_treenode.json").unwrap();
        assert_eq!(from_file.name(), original_root.name());
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
}
