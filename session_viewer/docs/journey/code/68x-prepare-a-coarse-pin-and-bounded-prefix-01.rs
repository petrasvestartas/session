use std::{cell::Cell,rc::Rc};
use crate::{page_cache::{Budget,Charge},range_cloud::Prepared,range_source::Source};

pub struct Cursor {pub prepared:Rc<Prepared>,pub from:u32,pub color_at:u64,pub loaded:u32,pub ceiling:u32}
fn reserve(prepared:&Prepared,mut count:u32)->Result<(u32,Charge),String> {
    while count>0 {
        let bytes=crate::stream_budget::temporary_page_bytes(count).ok_or("Point page size overflow")?;
        match prepared.pages.borrow_mut().reserve(bytes) {Ok(charge)=>return Ok((count,charge)),Err(_) if count>1=>count/=2,Err(error)=>return Err(error.into())}
    }
    Err("Empty streamed source page".into())
}

pub async fn start(source:Rc<Source>,budget:Rc<Budget>,share:u32)->Result<Cursor,String> {
    let prepared=Prepared::new(source,budget).map_err(str::to_owned)?;
    prepared.generation.borrow_mut().start().map_err(str::to_owned)?;
    let end=(prepared.source.lod.first[0] as u32).checked_add(prepared.source.lod.count[0] as u32).ok_or("Root range overflow")?;
    if end==0||end>crate::stream_budget::PREFIX_POINTS {return Err("Original coarse node exceeds the initial page limit".into());}
    let (root_count,charge)=reserve(&prepared,end)?;
    if root_count!=end {return Err("Original coarse node exceeds the page memory budget".into());}
    let (root,_)=prepared.source.page_attributes(0..end,prepared.source.fields.colors_at).await?;
    let root=root.slice(prepared.source.lod.first[0] as u32..end).map_err(str::to_owned)?;
    prepared.pin_root(root,charge).map_err(str::to_owned)?;
    let requested=crate::stream_budget::prefix(prepared.source.fields.count,share,share);
    let (count,charge)=reserve(&prepared,requested)?;
    let (page,color_at)=prepared.source.page_attributes(0..count,prepared.source.fields.colors_at).await?;
    drop(prepared.adopt(page,charge).map_err(str::to_owned)?);
    Ok(Cursor {prepared,from:count,color_at,loaded:count,ceiling:count})
}

