    if let Some(action)=crate::gather_registry::command(name,&words[count..].join(" ")){return action.map(Command::Editor);}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="Clipping Plane"{return crate::clipping_input::parse(&words[count..]).map(|r|Command::Editor(Action::Clipping(r)));}
    if name=="Construction Plane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
    if matches!(name,"Arrow"|"Point"|"Line"|"Curve"|"Polyline"|"Polyline Points"|"Polyline Rectangle"|"Polyline Polygon") && words.len()>count {
