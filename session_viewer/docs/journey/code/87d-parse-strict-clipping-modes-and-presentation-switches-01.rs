use crate::clipping_geometry::Mode;
#[derive(Clone,Debug)]
pub enum Request{Begin(Mode),Create(Mode,Vec<[f64;3]>),Switch(bool),Flip,Fill(Option<bool>)}
pub fn parse(words:&[&str])->Result<Request,String>{
    let word=words.first().map(|w|w.to_ascii_lowercase());
    match (word.as_deref(),words.len()){
        (None,_)=>Ok(Request::Begin(Mode::Normal)),
        (Some("on"),1)=>Ok(Request::Switch(true)),(Some("off"),1)=>Ok(Request::Switch(false)),(Some("flip"),1)=>Ok(Request::Flip),
        (Some("hatch"),1)=>Ok(Request::Fill(Some(false))),(Some("solid"),1)=>Ok(Request::Fill(Some(true))),(Some("fill"),1)=>Ok(Request::Fill(None)),
        (Some("fill"),2)=>match words[1].to_ascii_lowercase().as_str(){"hatch"=>Ok(Request::Fill(Some(false))),"solid"=>Ok(Request::Fill(Some(true))),_=>Err("Clipping Plane Fill takes Hatch or Solid".into())},
        _=>{
            let mode=Mode::parse(words[0]).ok_or("Clipping Plane (Normal 3Point XY YZ ZX On Off Flip Fill)")?;
            if words.len()==1{return Ok(Request::Begin(mode))}
            let points=words[1..].iter().map(|word|crate::drawing_coords::absolute(word).filter(|p|p.iter().all(|v|v.is_finite()&&v.abs()<=1e12)).ok_or_else(||format!("`{word}` is not a finite world point within ±1e12"))).collect::<Result<Vec<_>,_>>()?;
            if points.len()!=mode.points(){return Err(format!("Clipping Plane {} takes {} points",mode.word(),mode.points()))}Ok(Request::Create(mode,points))
        }
    }
}
