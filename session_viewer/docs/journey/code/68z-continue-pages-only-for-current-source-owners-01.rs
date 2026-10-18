    Ok(Cursor {prepared,from:count,color_at,loaded:count,ceiling:count})
}

pub fn continue_read(cursor:Cursor,current:Rc<dyn Fn(&Rc<Prepared>)->bool>,changed:Rc<dyn Fn()>,error:Rc<dyn Fn(String)>,cancelled:Rc<Cell<bool>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let Cursor {prepared,mut from,mut color_at,mut loaded,ceiling}=cursor;
        while current(&prepared)&&!cancelled.get() {
            let left=ceiling.saturating_sub(loaded);
            let Some(rows)=crate::stream_budget::next(from,prepared.source.fields.count,left) else {return;};
            let (count,charge)=match reserve_visible(&prepared,rows.end-rows.start,&*changed) {Ok(value)=>value,Err(message)=>{error(message);return;}};
            let ticket=match prepared.generation.borrow_mut().request() {Ok(ticket)=>ticket,Err(_)=>return,};
            let result=prepared.source.page_attributes(from..from+count,color_at).await;
            if !current(&prepared)||cancelled.get()||!prepared.generation.borrow_mut().finish(ticket) {return;}
            match result {
                Ok((page,next))=>{match prepared.adopt(page,charge) {Ok(page)=>drop(page),Err(message)=>{error(message.into());return;}}color_at=next;from+=count;loaded+=count;changed();},
                Err(message)=>{error(message);return;}
            }
        }
    });
}
