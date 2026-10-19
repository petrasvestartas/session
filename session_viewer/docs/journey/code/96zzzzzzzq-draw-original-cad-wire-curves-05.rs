pub fn wires(row:&crate::scene::Object,anchor:[f64;3],colour:Option<[f32;4]>,extra:f32)->Vec<crate::chain::Segment>{
    let Some(cad)=&row.cad else{return Vec::new()};let model=crate::render_space::placed(&row.model,anchor);
    cad.wires.iter().flat_map(|wire|{let mut points=Vec::new();for p in &wire.points{let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32();if points.last()!=Some(&p){points.push(p);}}
        crate::chain::segments(&points,colour.unwrap_or([0.,0.,0.,1.]),crate::mesh_pens::width(cad.source.width)+extra,0)
    }).collect()
}

pub fn example