use crate::editor::Action;

pub fn wheel(delta:f64,mode:u32,dpr:f64)->Option<Action> {
    let amount=match mode {
        0 if dpr.is_finite()&&dpr>0.=>(-delta*dpr)as f32/100.,
        1=>(-delta)as f32,
        _=>return None,
    };
    if !amount.is_finite()||amount==0. {return None;}
    Some(Action::Zoom(1./0.9_f64.powf(amount.clamp(-10.,10.)as f64)))
}
