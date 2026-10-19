    pub fn every(milliseconds:i32,tick:impl FnMut()+'static)->Result<Self,JsValue>{Self::start(milliseconds,tick,true)}
    pub fn after(milliseconds:i32,tick:impl FnMut()+'static)->Result<Self,JsValue>{Self::start(milliseconds,tick,false)}
    fn start(milliseconds:i32,tick:impl FnMut()+'static,repeat:bool)->Result<Self,JsValue>{