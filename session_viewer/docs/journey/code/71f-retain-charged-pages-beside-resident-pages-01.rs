use std::rc::Rc;
use crate::{page_cache::Held,sheet_page::Page};
pub const PREFIX:u32=25000;
pub const CHUNK:u32=500000;
pub const CEILING:u32=3000000;
#[derive(Clone)]
pub enum DisplayPage {
    Resident(Rc<Page>),
    Streamed(Rc<Held<Page>>),
}
impl std::ops::Deref for DisplayPage {
    type Target = Page;
    fn deref(&self) -> &Page {
        match self {
            Self::Resident(page) => page,
            Self::Streamed(page) => &page.value,
        }
    }
}
impl DisplayPage {
    pub fn key(&self) -> usize {
        &**self as *const Page as usize
    }
}
pub fn temporary_bytes(count: u32) -> Option<usize> {
    usize::try_from(count).ok()?.checked_mul(192)
}
pub fn next(from: u32, total: u32, left: u32) -> Option<std::ops::Range<u32>> {
    let count = total.checked_sub(from)?.min(CHUNK).min(left);
    (count > 0).then(|| from..from + count)
}
