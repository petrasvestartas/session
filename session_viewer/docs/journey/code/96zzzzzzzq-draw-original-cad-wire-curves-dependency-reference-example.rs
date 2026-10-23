#[cfg(not(target_arch="wasm32"))]
#[path="reference_frame.rs"]
mod reference_frame;

#[cfg(not(target_arch="wasm32"))]
struct Logger;
#[cfg(not(target_arch="wasm32"))]
impl log::Log for Logger{
    fn enabled(&self,_:&log::Metadata)->bool{true}
    fn log(&self,message:&log::Record){eprintln!("[{}] {}",message.level(),message.args());}
    fn flush(&self){}
}
#[cfg(not(target_arch="wasm32"))]
fn main(){
    let _=log::set_logger(&Logger);log::set_max_level(log::LevelFilter::Info);
    let output=std::env::args().nth(1).unwrap_or_else(||"current.ppm".into());
    let source=std::env::args().nth(2).unwrap_or_else(||"comparison.pb".into());
    if !std::path::Path::new(&source).exists(){std::fs::write(&source,viewer_journey::comparison::specimen()).unwrap();}
    print!("{}",reference_frame::render(std::path::Path::new(&source),std::path::Path::new(&output),[640,480]));
}
#[cfg(target_arch="wasm32")]
fn main(){}
