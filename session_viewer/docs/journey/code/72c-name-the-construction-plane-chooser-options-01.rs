    if distance>ray.max_distance {return None;}crate::tool_coords::checked([hit[0],hit[1],hit[2]]).ok()
}

pub fn buttons() -> Vec<(String,String)> {
    ["XY","XZ","YZ","3 Point","View","Cancel"].into_iter().map(|s|(s.into(),format!("CPlane {s}"))).collect()
}
