pub fn density(ratio:f64,override_value:Option<&str>)->f64 {
    let ratio=if ratio>0. {ratio} else {1.};
    let cap=override_value.and_then(|s|s.parse::<f32>().ok())
        .filter(|v|v.is_finite()&&*v>=0.5).map(f64::from).unwrap_or(2.);
    ratio.min(cap)
}

#[cfg(target_arch="wasm32")]
fn query()->Option<web_sys::UrlSearchParams>{
    web_sys::UrlSearchParams::new_with_str(&web_sys::window()?.location().search().ok()?).ok()
}

#[cfg(target_arch="wasm32")]
pub fn scale(ratio:f64)->f64 {
    let value=query().and_then(|q|q.get("dpr"));
    density(ratio,value.as_deref())
}

#[cfg(target_arch="wasm32")]
pub fn initialize