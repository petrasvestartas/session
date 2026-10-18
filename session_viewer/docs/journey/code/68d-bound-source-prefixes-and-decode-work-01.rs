pub const PREFIX_POINTS:u32=2_000_000;
pub const CHUNK_POINTS:u32=2_000_000;
pub const MAX_POINTS:u32=6_000_000;
pub const MIN_PREFIX:u32=250_000;
pub const MIN_BYTES:u64=64*1024*1024;
pub const PAGE_BYTES:usize=256*1024*1024;

pub fn prefix(count:u32,share:u32,remaining:u32)->u32 {count.min(PREFIX_POINTS).min(share.min(remaining.max(MIN_PREFIX)))}
pub fn next(from:u32,total:u32,remaining:u32)->Option<std::ops::Range<u32>> {
    if from>=total||remaining==0 {return None;}let count=(total-from).min(CHUNK_POINTS).min(remaining);Some(from..from+count)
}
pub fn temporary_page_bytes(count:u32)->Option<usize> {usize::try_from(count).ok()?.checked_mul(160)}
