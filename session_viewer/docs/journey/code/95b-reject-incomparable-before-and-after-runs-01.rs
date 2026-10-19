use crate::{bench_config::Conditions,bench_samples::{Summary,summarize}};

#[derive(serde::Serialize,serde::Deserialize)]
pub struct Run{
    pub conditions:Conditions,
    pub revision:String,
    pub warmup:usize,
    pub frame_ms:Vec<f64>,
    pub gpu_buffers:u64,
    pub gpu_textures:u64,
    pub original_bytes:u64,
}

pub struct Comparison{
    pub before:Summary,
    pub after:Summary,
    pub median_change_ms:f64,
    pub p95_change_ms:f64,
}

pub fn compare(before:&Run,after:&Run)->Result<Comparison,&'static str>{
    before.conditions.validate()?;after.conditions.validate()?;
    if before.conditions!=after.conditions{return Err("Runs have different hardware, scene or rendering conditions");}
    if before.revision.trim().is_empty()||after.revision.trim().is_empty(){return Err("Record both renderer revisions");}
    if before.original_bytes!=after.original_bytes{return Err("Original source memory changed; inspect retention before comparing");}
    let a=summarize(&before.frame_ms,before.warmup)?;let b=summarize(&after.frame_ms,after.warmup)?;
    Ok(Comparison{median_change_ms:b.median_ms-a.median_ms,p95_change_ms:b.p95_ms-a.p95_ms,before:a,after:b})
}
