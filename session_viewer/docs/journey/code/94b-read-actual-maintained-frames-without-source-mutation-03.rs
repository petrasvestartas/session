use current_viewer::selftest::{SceneFile,render_scene};
use session_rust::Xform;
use std::path::Path;

pub fn render(source:&Path,output:&Path,size:[u32;2])->String{
    render_scene(&[SceneFile{path:source.to_string_lossy().into_owned(),place:Xform::identity(),point_px:0.,display_only:false}],size[0],size[1],output.to_str().expect("UTF-8 output path"))
}

pub fn rgb(bytes:&[u8],size:[u32;2])->Result<&[u8],&'static str>{
    let header=format!("P6\n{} {}\n255\n",size[0],size[1]);
    let body=bytes.strip_prefix(header.as_bytes()).ok_or("Unexpected PPM header")?;
    if body.len()!=size[0]as usize*size[1]as usize*3{return Err("Incomplete frame");}
    Ok(body)
}
