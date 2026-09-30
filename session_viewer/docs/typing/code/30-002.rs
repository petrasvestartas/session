
/// Add Group takes an optional name; Add Edge takes nothing.
#[test]
fn session_structure_verbs_parse() {
    assert_eq!(parsed("Add Group"), Ok("AddGroup(\"\")".into()));
    assert_eq!(
        parsed("addgroup North wall"),
        Ok("AddGroup(\"North wall\")".into())
    );
    assert!(parsed("Add Group a/b").is_err());
    assert_eq!(parsed("add edge"), Ok("AddEdge".into()));
    assert!(parsed("Add Edge x").is_err());
    assert_eq!(accept("addg"), ("Add Group".into(), true));
    assert_eq!(completions("add"), vec!["Add Edge", "Add Group"]);
    assert_eq!(canonical("addedge"), "Add Edge");
}
