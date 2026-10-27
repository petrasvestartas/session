    let message=run_result("decode",size,clock,observe,||proto::Session::decode(bytes).map_err(|error|error.to_string()))?;
