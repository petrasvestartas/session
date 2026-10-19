#[cfg(not(target_arch="wasm32"))]
pub fn snapshot()->serde_json::Value{
    use current_viewer::app::command;
    let outcome=|line:&str|match command::parse(line){Ok(action)=>serde_json::json!({"accepted":true,"selection":action.needs_selection(),"keeps_draft":action.keeps_draft(),"keeps_split":action.keeps_split()}),Err(error)=>serde_json::json!({"accepted":false,"error":error})};
    serde_json::Value::Array(command::REGISTRY.iter().map(|entry|{
        let spec=entry.spec();
        serde_json::json!({"names":spec.names,"aliases":spec.aliases,"hint":spec.hint,"arity":spec.arity,"wait_for_option":spec.wait_for_option,"wait_after_option":spec.wait_after_option,
            "alias_results":spec.aliases.iter().map(|line|serde_json::json!({"line":line,"canonical":command::canonical(line),"result":outcome(line)})).collect::<Vec<_>>(),
            "bare":spec.names.iter().map(|line|serde_json::json!({"line":line,"result":outcome(line)})).collect::<Vec<_>>(),
            "options":spec.options.iter().map(|line|serde_json::json!({"line":line,"canonical":command::canonical(line),"result":outcome(line)})).collect::<Vec<_>>()})
    }).collect())
}
