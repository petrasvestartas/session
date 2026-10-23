        return Ok(Command::Editor(if name=="Element Attributes"{Action::ElementAttributes(on)}else{Action::ElementInteractions(on)}));
    }
    if name=="Opacity"{let word=words.get(count).ok_or("Opacity 0..1 · Example: Opacity 0.5")?;let value=word.parse::<f64>().ok().filter(|v|v.is_finite()&&(0.0..=1.0).contains(v)).ok_or("Opacity takes a value from 0 to 1")?;return Ok(Command::Editor(Action::Opacity(value as f32)));}
    if name=="View Plane Size"{if words.len()!=count+1{return Err("View Plane Size needs a positive number of mm".into())}let size=words[count].parse::<f64>().ok().filter(|v|v.is_finite()&&*v>0.0&&(*v as f32).is_finite()).ok_or("View Plane Size needs a positive number of mm")?;return Ok(Command::Editor(Action::PlaneSize(size)));}
    if name=="View Outline"{let on=match &words[count..]{[]=>None,[word]if word.eq_ignore_ascii_case("On")=>Some(true),[word]if word.eq_ignore_ascii_case("Off")=>Some(false),_=>return Err("View Outline (On Off)".into())};return Ok(Command::Editor(Action::ViewOutline(on)));}
    if name=="View Xray"{let on=crate::element_input::visibility(&words[count..])?;return Ok(Command::Editor(Action::ViewXray(on)));}
    if name=="View Lighting"||name=="View Backfaces" {let on=crate::element_input::visibility(&words[count..])?;return Ok(Command::Editor(if name=="View Lighting"{Action::Lighting(on)}else{Action::Backfaces(on)}));}
