            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
