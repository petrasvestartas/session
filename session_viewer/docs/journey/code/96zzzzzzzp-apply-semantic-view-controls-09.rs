    editor.ink_visibility.vertices=!query.as_ref().is_some_and(|q|q.has("nomarkers"));
    editor.outlines=query.as_ref().is_some_and(|q|q.has("outlines"));
    editor.lit=query.as_ref()