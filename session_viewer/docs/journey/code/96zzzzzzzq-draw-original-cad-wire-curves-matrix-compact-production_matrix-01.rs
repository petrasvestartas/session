    pub objects: Vec<u8>,
    pub translations: Vec<u8>,
    pub starts: Vec<u32>,
    pub owners: Vec<u32>,
}
pub fn rows(scene:&crate::scene::Scene,anchor:[f64;3],selected:&[crate::scene::ObjectId],limit:u64)->Result<Rows,&'static str> {
    use crate::production_ribbon::floats;
    let visible:Vec<_>=scene.matrices().iter().filter(|r|scene.visible(r.id)).collect();
    let count=visible.len();
    if count as u64>limit/96 {return Err("Matrix object rows exceed device storage");}
    let mut out=Rows {words:Vec::new(),objects:Vec::with_capacity(count*96),translations:vec![0;count*16],starts:Vec::new(),owners:Vec::with_capacity(count)};
    for _ in 0..count {floats(&mut out.objects,session_rust::Xform::identity().to_f32());floats(&mut out.objects,[1.0;4]);out.objects.extend(2u32.to_ne_bytes());out.objects.extend([0;12]);}
    let mut packed=MatrixRows::default();
    for (index,row) in visible.into_iter().enumerate() {
        let index=index as u32;
        out.owners.push(crate::feature_owner::display_owner(scene,row.id).map_or(0,crate::scene::ObjectId::value));
        let size=HEADER_WORDS as u64+row.source.extra["rows"].as_u64().unwrap()*row.source.extra["cols"].as_u64().unwrap()*VALUE_WORDS as u64;
        if packed.words.len() as u64+size>limit/4 {return Err("Matrix cells exceed device storage");}
        out.starts.push(packed.words.len() as u32);
