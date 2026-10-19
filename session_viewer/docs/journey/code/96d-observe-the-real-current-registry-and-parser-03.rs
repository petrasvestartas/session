#[cfg(not(target_arch="wasm32"))]
fn main(){println!("{}",serde_json::to_string_pretty(&viewer_journey::registry_audit::snapshot()).unwrap());}
#[cfg(target_arch="wasm32")]
fn main(){}
