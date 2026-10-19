pub struct Draft{pub words:String,pub height:f64,pub asking:bool}
impl Draft{
    pub fn new(words:String,distance:f64,last:f64)->Result<Self,&'static str>{if !(1..=80).contains(&words.chars().count()){return Err("Text takes 1 to 80 characters")}
        Ok(Self{words,height:if last>0.{last}else{crate::text_geometry::nice(distance/25.)},asking:false})
    }
    pub fn prompt(&self)->String{if self.asking{"Type the letter height".into()}else{format!("Lower-left point of \"{}\" · Height {} (type Height N)",self.words,self.height)}}
    pub fn height(&mut self,text:&str)->Result<(),&'static str>{let words=text.split_whitespace().collect::<Vec<_>>();let value=match words.as_slice(){[word]if word.eq_ignore_ascii_case("Height")=>{self.asking=true;return Ok(())},[name,value]if name.eq_ignore_ascii_case("Height")=>*value,[value]if self.asking=>*value,_=>return Err("Height wants a number above zero")};let height=value.parse::<f64>().ok().filter(|v|v.is_finite()&&*v>0.&&*v<=1e9).ok_or("Height must be a number above zero")?;self.height=height;self.asking=false;Ok(())}
    pub fn source(&self,point:[f64;3],frame:crate::cplane::CPlane,camera:&crate::camera::Camera)->crate::scene_label::Source{let(x,y)=frame.axes();let(right,up)=crate::text_geometry::reading_axes(&x,&y,&camera.orientation.rotate_vector(session_rust::Vector::x_axis()),&camera.orientation.rotate_vector(session_rust::Vector::z_axis()));let label=crate::text_geometry::label(&self.words,&session_rust::Point::new(point[0],point[1],point[2]),right,up,self.height);crate::scene_label::Source{guid:uuid::Uuid::new_v4().to_string(),name:self.words.clone(),visible:true,locked:false,label}}
}
