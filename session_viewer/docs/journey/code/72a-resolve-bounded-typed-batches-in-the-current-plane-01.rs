use crate::{cplane::CPlane, drawing_coords as coords};
use session_rust::{Point,Vector};

pub fn points(text:&str, fixed:Option<CPlane>, frame:CPlane, previous:&[[f64;3]], cursor:Option<[f64;3]>) -> Result<Vec<[f64;3]>,&'static str> {
    coords::set_plane(fixed);
    let (x,y)=frame.axes();let mut all=previous.to_vec();let mut added=Vec::new();
    for word in text.split_whitespace() {
        if all.len()>=4096 {return Err("Drawing commands accept at most 4096 points");}
        if word.trim_start_matches(['w','W','@']).split([',','<']).any(|part|part.parse::<f64>().is_ok_and(|v|!v.is_finite())) {return Err("Coordinates must be finite and within ±1e12");}
        let (typed,world)=coords::parse_world(word).ok_or("Use x,y,z, @x,y,z, distance<angle, or w for world")?;
        let last=all.last().map(|p|Point::new(p[0],p[1],p[2]));
        let along=all.last().zip(cursor).and_then(|(p,c)|{
            let d=std::array::from_fn::<_,3,_>(|i|c[i]-p[i]);let n=d.iter().map(|v|v*v).sum::<f64>().sqrt();
            (n>1e-12).then(||Vector::new(d[0]/n,d[1]/n,d[2]/n))
        });
        let p=coords::place(typed,world,&x,&y,last.as_ref(),along.as_ref()).ok_or("This coordinate needs a previous point or a cursor direction")?;
        let p=crate::tool_coords::checked([p[0],p[1],p[2]])?;all.push(p);added.push(p);
    }
    if added.is_empty() {return Err("A coordinate is required");}Ok(added)
}

