    }
    Err("Empty streamed source page".into())
}

fn reserve_visible(prepared:&Prepared,mut count:u32,changed:&dyn Fn())->Result<(u32,Charge),String> {
    while crate::stream_budget::temporary_page_bytes(count).ok_or("Point page size overflow")?>crate::stream_budget::PAGE_BYTES {count/=2;}
    loop {
        let bytes=crate::stream_budget::temporary_page_bytes(count).ok_or("Point page size overflow")?;
        let result=prepared.pages.borrow_mut().reserve(bytes);
        if let Ok(charge)=result {return Ok((count,charge));}
        let oldest=prepared.pages.borrow().oldest();
        if let Some(key)=oldest {
            prepared.pages.borrow_mut().remove(&key);
            // Redraw drops the evicted GPU page owner and cancels picks before its compact addresses can be reused.
            changed();
        } else if count>1 {count/=2;} else {return Err("Pending readers still hold the point page budget".into());}
    }
}

