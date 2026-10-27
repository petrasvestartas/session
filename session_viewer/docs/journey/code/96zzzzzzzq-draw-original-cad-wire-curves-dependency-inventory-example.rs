#[cfg(not(target_arch="wasm32"))]
#[path="registry_audit.rs"]
mod registry_audit;

#[cfg(not(target_arch="wasm32"))]
fn main(){println!("{}",serde_json::to_string_pretty(&registry_audit::snapshot()).unwrap());}
#[cfg(target_arch="wasm32")]
fn main(){}
