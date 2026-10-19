}

pub async fn get(url:&str,opts:&GetOpts)->Result<Reply,String> {get_with_signal(url,opts,None).await}
pub async fn get_with_signal(url:&str,opts:&GetOpts,signal:Option<&web_sys::AbortSignal>)->Result<Reply,String> {
    let (start,length)=opts.range.ok_or("This reader requires a byte range")?;
    if length==0 {return Ok(Reply {status:206,etag:None,total:None,bytes:vec![]});}
    let range=crate::range_rules::header(start,length)?;
    let mut read=Read::new(url,signal)?;let init=web_sys::RequestInit::new();init.set_signal(Some(&read.controller.signal()));
    init.set_mode(web_sys::RequestMode::Cors);
    if opts.revalidate {init.set_cache(web_sys::RequestCache::NoCache);}
