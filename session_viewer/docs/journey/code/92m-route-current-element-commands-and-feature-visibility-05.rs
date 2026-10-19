    if let Some(action)=crate::gather_registry::command(name,&words[count..].join(" ")){return action.map(Command::Editor);}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if let Some(kind)=[("Element Beam",crate::element_input::Kind::Beam),("Element Column",crate::element_input::Kind::Column),("Element Plate",crate::element_input::Kind::Plate),("Element Support",crate::element_input::Kind::Support),("Element Beam Variable",crate::element_input::Kind::Variable),("Element Block",crate::element_input::Kind::Block)].into_iter().find_map(|(n,k)|(n==name).then_some(k)){let text=words[count..].join(" ");kind.read(&text)?;return Ok(Command::Editor(Action::ElementBegin{kind,text}));}
    if matches!(name,"Element Attributes"|"Element Interactions") {
        let on=crate::element_input::visibility(&words[count..])?;
        return Ok(Command::Editor(if name=="Element Attributes"{Action::ElementAttributes(on)}else{Action::ElementInteractions(on)}));
    }
    if name=="View Outline"{let on=match &words[count..]{[]=>None,[word]if word.eq_ignore_ascii_case("On")=>Some(true),[word]if word.eq_ignore_ascii_case("Off")=>Some(false),_=>return Err("View Outline (On Off)".into())};return Ok(Command::Editor(Action::ViewOutline(on)));}
    if name=="Arctic"{let on=match &words[count..]{[]=>None,[word]if word.eq_ignore_ascii_case("On")=>Some(true),[word]if word.eq_ignore_ascii_case("Off")=>Some(false),_=>return Err("Arctic (On Off)".into())};return Ok(Command::Editor(Action::Arctic(on)));}
