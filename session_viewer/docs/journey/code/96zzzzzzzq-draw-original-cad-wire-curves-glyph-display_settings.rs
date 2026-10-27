#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Settings {pub thickness:f32,pub feather:f32,pub cloud_scale:f32,pub edl:f32,pub lod:f32}
impl Default for Settings {fn default()->Self {Self{thickness:1.0,feather:1.0,cloud_scale:1.0,edl:0.25,lod:0.0}}}
impl Settings {
    pub fn from_values(mut get:impl FnMut(&str)->Option<String>)->Self {
        let mut value=|name,default| get(name).and_then(|s|s.parse::<f32>().ok()).filter(|v|v.is_finite()).unwrap_or(default);
        Self{thickness:value("thickness",1.0).max(0.1),feather:value("aa",1.0).clamp(0.5,4.0),cloud_scale:value("cloud",1.0),edl:value("edl",0.25),lod:value("lod",0.0)}
    }
    pub fn point_size(&mut self,size:f32)->Result<(),&'static str>{
        if !size.is_finite()||size<=0.0{return Err("View Point Size needs a positive number");}
        self.cloud_scale=size.clamp(0.25,8.0);Ok(())
    }
    pub fn line(self,matrix:[f32;16],size:[u32;2],density:f32,opacity:f32,anchor:[f64;3])->Vec<u8>{
        let mut bytes=crate::production_ribbon::line_uniform(matrix,size,density,opacity,anchor);
        bytes[..4].copy_from_slice(&(self.thickness*density).to_ne_bytes());bytes[44..48].copy_from_slice(&self.feather.to_ne_bytes());bytes
    }
}
