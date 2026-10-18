pub const MAX_RANGE:u64=64*1024*1024;
pub const PROBE_BYTES:u64=8192;

pub fn header(start:u64,length:u64)->Result<String,String> {
    if length==0||length>MAX_RANGE {return Err("Invalid range length".into());}
    let end=start.checked_add(length-1).ok_or("Range end overflow")?;
    Ok(format!("bytes={start}-{end}"))
}

pub fn content_range(value:&str)->Option<(u64,u64,u64)> {
    let value=value.strip_prefix("bytes ")?;let (interval,total)=value.split_once('/')?;
    let (start,end)=interval.split_once('-')?;let (start,end,total)=(start.parse().ok()?,end.parse().ok()?,total.parse().ok()?);
    if start>end||end>=total {return None;}Some((start,end,total))
}

pub fn accept(start:u64,length:u64,status:u16,range:Option<&str>,received:u64)->Result<u64,String> {
    header(start,length)?;
    if status!=206 {return Err(format!("Range response requires HTTP206, received {status}"));}
    let (first,last,total)=content_range(range.ok_or("Range response lacks Content-Range")?).ok_or("Invalid Content-Range")?;
    if first!=start||last-first+1!=received||received>length {return Err("Range response does not match its requested bytes".into());}
    let end=start.checked_add(length).ok_or("Range end overflow")?;
    if received!=length&&!(start==0&&length==PROBE_BYTES&&last+1==total&&end>total) {return Err("Range response is incomplete".into());}
    Ok(total)
}
