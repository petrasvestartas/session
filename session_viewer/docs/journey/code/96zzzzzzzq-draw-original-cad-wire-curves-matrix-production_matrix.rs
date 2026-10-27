/// Words before a matrix's values; the shader reads the header at these offsets.
pub const HEADER_WORDS: usize = 16;
/// Words per value: its f32, its whole part, its thousandths with the sign in the top bit.
pub const VALUE_WORDS: usize = 3;
/// The sign in a value's third word.
pub const SIGN_BIT: u32 = 0x8000_0000;
pub const ROWS_AT: usize = 0; // row count
pub const COLS_AT: usize = 1; // column count
pub const INSTANCE_AT: usize = 2; // object row
pub const CELL_AT: usize = 3; // edge length of a cell, f32 bits
pub const LOW_AT: usize = 4; // smallest non-zero entry, f32 bits
pub const HIGH_AT: usize = 5; // largest non-zero entry, f32 bits
pub const LINE_AT: usize = 6; // edge colour, packed rgba
pub const MIN_AT: usize = 8; // colour of the smallest entry, packed rgba
pub const MAX_AT: usize = 9; // colour of the largest entry, packed rgba
pub const ZERO_AT: usize = 10; // colour of a zero entry, packed rgba
pub const TEXT_AT: usize = 11; // colour of the values, packed rgba

/// Matrices of one upload as words: per matrix a header, then its values row by row, VALUE_WORDS each.
#[derive(Default)]
pub struct MatrixRows {
    pub words: Vec<u32>, // block after block
}


pub const SOURCE: &str = include_str!("production_matrix.wgsl");
pub const PROVENANCE: &str = include_str!("production_matrix_provenance.json");
pub struct Walk { pub bounds: session_rust::AABB }
impl Walk { pub fn thin(bounds: session_rust::AABB) -> Self { Self { bounds } } }

#[derive(Clone)]
pub struct Row {
    pub id: crate::scene::ObjectId,
    pub guid: String,
    pub source: std::rc::Rc<session_rust::Component>,
    pub model: session_rust::Xform,
}
impl Row {
    pub fn bounds(&self) -> crate::bounds::Bounds {
        let cell=self.source.extra.get("cell").and_then(serde_json::Value::as_f64).unwrap_or(1.0);
        let rows=self.source.extra["rows"].as_u64().unwrap() as f64;
        let cols=self.source.extra["cols"].as_u64().unwrap() as f64;
        let points=[[-0.5*cell,0.5*cell+cell*0.85,0.0],[(cols-0.5)*cell,0.5*cell+cell*0.85,0.0],[-0.5*cell,(0.5-rows)*cell,0.0],[(cols-0.5)*cell,(0.5-rows)*cell,0.0]];
        let points=points.map(|p|{let p=self.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]});
        let mut bounds=crate::bounds::Bounds::point(points[0]);for p in points {bounds.include(p);}bounds
    }
}

pub fn validate(source:&session_rust::Component)->Result<(), &'static str> {
    let count=|name| source.extra.get(name).and_then(serde_json::Value::as_u64).filter(|n|*n>0&&*n<=u32::MAX as u64);
    let rows=count("rows").ok_or("Matrix needs a positive row count")?;
    let cols=count("cols").ok_or("Matrix needs a positive column count")?;
    let size=rows.checked_mul(cols).filter(|n|*n<=16_000_000).ok_or("Matrix exceeds sixteen million cells")?;
    let data=source.extra.get("data").and_then(serde_json::Value::as_array).ok_or("Matrix needs row-major values")?;
    if data.len() as u64!=size||data.iter().any(|v|v.as_f64().is_none_or(|v|!v.is_finite()||!(v as f32).is_finite())) {return Err("Matrix dimensions or finite values are invalid");}
    let cell=source.extra.get("cell").and_then(serde_json::Value::as_f64).unwrap_or(1.0);
    if !cell.is_finite()||cell<=0.0||!((rows.max(cols) as f64*cell) as f32).is_finite() {return Err("Matrix cell extent is invalid");}
    Ok(())
}

pub struct Rows {
    pub words: Vec<u8>,
    pub objects: Vec<u8>,
    pub translations: Vec<u8>,
    pub starts: Vec<u32>,
}
pub fn rows(scene:&crate::scene::Scene,anchor:[f64;3],selected:&[crate::scene::ObjectId],limit:u64)->Result<Rows,&'static str> {
    use crate::production_ribbon::floats;
    let visible:Vec<_>=scene.matrices().iter().filter(|r|scene.visible(r.id)).collect();
    let count=visible.iter().map(|r|r.id.value()).max().unwrap_or(1) as usize;
    if count as u64>limit/96 {return Err("Matrix object rows exceed device storage");}
    let mut out=Rows {words:Vec::new(),objects:Vec::with_capacity(count*96),translations:vec![0;count*16],starts:Vec::new()};
    for _ in 0..count {floats(&mut out.objects,session_rust::Xform::identity().to_f32());floats(&mut out.objects,[1.0;4]);out.objects.extend(2u32.to_ne_bytes());out.objects.extend([0;12]);}
    let mut packed=MatrixRows::default();
    for row in visible {
        let index=row.id.value()-1;
        let size=HEADER_WORDS as u64+row.source.extra["rows"].as_u64().unwrap()*row.source.extra["cols"].as_u64().unwrap()*VALUE_WORDS as u64;
        if packed.words.len() as u64+size>limit/4 {return Err("Matrix cells exceed device storage");}
        out.starts.push(packed.words.len() as u32);
        crate::production_matrix_walk::walk(&mut packed,&row.source,index);
        let mut model=row.model.to_f32();model[12..15].fill(0.0);
        let mut object=Vec::with_capacity(96);floats(&mut object,model);floats(&mut object,[1.0;4]);object.extend(u32::from(selected.contains(&row.id)).to_ne_bytes());object.extend([0;12]);
        out.objects[index as usize*96..(index as usize+1)*96].copy_from_slice(&object);
        for axis in 0..3 {let offset=index as usize*16+axis*4;out.translations[offset..offset+4].copy_from_slice(&((row.model.m[12+axis]-anchor[axis]) as f32).to_ne_bytes());}
    }
    out.words=packed.words.into_iter().flat_map(u32::to_ne_bytes).collect();Ok(out)
}

pub fn title(source:&session_rust::Component,model:&session_rust::Xform)->Option<crate::scene_label::Source> {
    let (text,corner,height,color)=crate::production_matrix_walk::title(source)?;
    let at=model.transform_point(&session_rust::Point::new(corner[0],corner[1],corner[2]));
    let right=model.transform_vector(&session_rust::Vector::new(1.0,0.0,0.0));
    let up=model.transform_vector(&session_rust::Vector::new(0.0,1.0,0.0));
    let scale=right.magnitude();let up_scale=up.magnitude();
    let mut label=crate::text_example::label();label.text=text;label.color=color;label.bare=true;
    label.placement=crate::text::TextPlacement::WorldPlane {world:[at[0],at[1],at[2]],right:[right[0]/scale,right[1]/scale,right[2]/scale],up:[up[0]/up_scale,up[1]/up_scale,up[2]/up_scale],world_height:height*scale};
    Some(crate::scene_label::Source {guid:format!("matrix-title/{}",uuid::Uuid::new_v4()),name:source.name.clone(),visible:true,locked:false,label})
}
