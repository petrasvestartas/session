    let coarse=window.match_media("(pointer: coarse)").ok().flatten()
        .is_some_and(|media|media.matches());
    let query=query();
    editor.display=crate::display_settings::Settings::from_values(|name|query.as_ref().and_then(|q|q.get(name)));
    editor.ink_visibility.vertices=!query.as_ref().is_some_and(|q|q.has("nomarkers"));
    editor.outlines=query.as_ref().is_some_and(|q|q.has("outlines"));
    editor.lit=query.as_ref().is_some_and(|q|q.has("lit"));editor.backfaces=query.as_ref().is_some_and(|q|q.has("backface"));
