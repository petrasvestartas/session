#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize)]
pub struct Conditions{
    pub machine:String,
    pub browser:String,
    pub adapter:String,
    pub scene_sha256:String,
    pub viewport:[u32;2],
    pub dpr:f64,
    pub camera:String,
    pub settings:String,
    pub measurement:String,
}

impl Conditions{
    pub fn validate(&self)->Result<(),&'static str>{
        if [&self.machine,&self.browser,&self.adapter,&self.camera,&self.settings,&self.measurement].iter().any(|v|v.trim().is_empty()){return Err("Record every measurement condition");}
        if self.scene_sha256.len()!=64||!self.scene_sha256.bytes().all(|v|v.is_ascii_hexdigit()){return Err("Record the original scene SHA-256");}
        if self.viewport.contains(&0)||!self.dpr.is_finite()||self.dpr<=0.{return Err("Invalid viewport or pixel density");}
        Ok(())
    }
}
