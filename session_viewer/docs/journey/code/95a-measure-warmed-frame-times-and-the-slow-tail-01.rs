#[derive(Debug,PartialEq,serde::Serialize)]
pub struct Summary{
    pub samples:usize,
    pub median_ms:f64,
    pub p95_ms:f64,
    pub max_ms:f64,
}

pub fn summarize(samples:&[f64],warmup:usize)->Result<Summary,&'static str>{
    if samples.iter().any(|v|!v.is_finite()||*v<=0.){return Err("Frame durations must be positive and finite");}
    let measured=samples.get(warmup..).filter(|v|!v.is_empty()).ok_or("No measured frames after warmup")?;
    let mut sorted=measured.to_vec();sorted.sort_by(f64::total_cmp);
    let count=sorted.len();let middle=count/2;
    let median=if count%2==0{sorted[middle-1]*0.5+sorted[middle]*0.5}else{sorted[middle]};
    let rank=(count*95).div_ceil(100)-1;
    Ok(Summary{samples:count,median_ms:median,p95_ms:sorted[rank],max_ms:sorted[count-1]})
}
