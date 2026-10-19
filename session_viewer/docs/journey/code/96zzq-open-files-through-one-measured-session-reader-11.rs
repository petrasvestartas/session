let mut candidate=scene.clone();
    let key=crate::load_measure::run_result("display walk",bytes.len()as u64,&mut crate::load_measure::now,&mut crate::document_source::observe,||->Result<u32,String>{
    let forest=