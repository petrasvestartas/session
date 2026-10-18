    let query=query();
    editor.lit=query.as_ref().is_some_and(|q|q.has("lit"));editor.backfaces=query.as_ref().is_some_and(|q|q.has("backface"));