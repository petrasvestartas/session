fn data_base(data:Option<&str>)->String {match data {Some("off"|"")=>String::new(),Some(value) if value.starts_with("https://")||local(value)=>{if value.ends_with('/') {value.into()} else {format!("{value}/")}},_=>DATA_BASE.into()}}
pub fn named(path:&str,data:Option<&str>)->Route {
 let base=data_base(data);let mut path=if path.contains('.') {path.to_owned()} else {format!("{path}.yaml")};
 if !path.contains('/') {path=format!("scenes/{path}");}
 Route {manifest:join(&base,&path),base,poll_ms:None,notify:None}
}

pub fn select(scene:Option<&str>