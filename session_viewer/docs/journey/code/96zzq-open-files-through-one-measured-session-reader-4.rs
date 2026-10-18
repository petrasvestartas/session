    let (tree,sheets)=run_result("validation",size,clock,observe,||->Result<_,String>{
    crate::source_validate::session(&message)?;