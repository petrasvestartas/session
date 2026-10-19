impl Prepared {
    pub fn new(source:Rc<BRep>)->Result<Self,&'static str> {
        validate(&source)?;
        let _=source.guid();let faces=source.face_meshes_q(Some(QUALITY));
        if faces.len()!=source.m_faces.len()||faces.iter().any(|face|face.face.is_empty()) {return Err("BRep face could not be tessellated");}
