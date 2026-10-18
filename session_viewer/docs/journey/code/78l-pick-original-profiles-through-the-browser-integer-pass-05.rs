            });
            if rename_action {panel.tree.rename_result(change.as_ref().err().map(String::as_str));}
            if tool_action && change.is_ok() {highlight_changed=true;panel.result(&editor.tool_status); }
            match change {
                Ok(Some(Change::Scene)) => {
