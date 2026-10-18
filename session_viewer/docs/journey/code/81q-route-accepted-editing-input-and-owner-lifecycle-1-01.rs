        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if let Some(choice)=crate::drawing_input::choice(text){return Some(Ok(choice));}
        if draft.extend.is_some()&&draft.plane_points.is_none(){let word=text.trim();if word.is_empty()||word.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Distance")||w.parse::<f64>().is_ok()){return Some(Ok(Action::ToolText(text.into())))}return None}
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none(){if let Some(gather)=&draft.gather{let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.starts_with('w')||word.starts_with('W')||gather.recipe.option_in(&[word]).is_some()||gather.recipe.choices().any(|choice|crate::primitive_options::compact(choice).starts_with(&crate::primitive_options::compact(word)))||!gather.typed.is_empty()){return Some(Ok(Action::ToolText(text.into())))}return None}}
