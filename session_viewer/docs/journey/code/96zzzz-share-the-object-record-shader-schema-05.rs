pub type Record = [u8; BYTES];
pub fn shader()->&'static str {include_str!("object_settings.wgsl")}