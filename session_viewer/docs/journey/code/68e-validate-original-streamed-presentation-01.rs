#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Properties {pub point_px:f32,pub visible:bool,pub locked:bool,seen:u8}
impl Default for Properties {
    fn default()->Self {Self {point_px:1.,visible:true,locked:false,seen:0}}
}
impl Properties {
    pub fn field(&mut self,field:&crate::stream::Field,bytes:&[u8])->bool {
        let bit=match field.field {6=>1,16=>2,17=>4,_=>return true};
        if self.seen&bit!=0 {return false;}
        match (field.field,field.wire) {
            (6,1)=>{let Ok(raw)=bytes.try_into() else {return false;};let px=f64::from_le_bytes(raw);if !px.is_finite()||px<=0.||!(px as f32).is_finite() {return false;}self.point_px=px as f32;},
            (16,0) if field.value<=1=>self.visible=field.value==1,
            (17,0) if field.value<=1=>self.locked=field.value==1,
            _=>return false,
        }
        self.seen|=bit;true
    }
}
