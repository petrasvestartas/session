pub fn placed(place:&session_rust::Xform,anchor:[f64;3])->session_rust::Xform {
 let mut local=place.clone();for i in 0..3 {local.m[12+i]-=anchor[i];}local
}
#[derive(Default)]