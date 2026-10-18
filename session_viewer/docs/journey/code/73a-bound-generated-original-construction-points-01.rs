}

pub fn points(kind:&str,plane:CPlane,sides:usize,points:&[[f64;3]])->Result<Vec<[f64;3]>,&'static str> {
    if kind=="polygon" && !(3..crate::tool::MAX_POINTS).contains(&sides) {return Err("Polygon sides must be between 3 and 4095");}
    let points:Vec<_>=points.iter().map(|p|Point::new(p[0],p[1],p[2])).collect();
    let result=construction_points(kind,plane,sides,&points).map_err(|_|if kind=="rectangle" {"Rectangle corners must define a nonzero width and height"}else{"Polygon radius must be above zero"})?;
    result.iter().map(|p|crate::tool_coords::checked([p[0],p[1],p[2]])).collect()
}
