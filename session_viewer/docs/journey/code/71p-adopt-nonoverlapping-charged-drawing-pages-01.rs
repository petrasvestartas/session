use crate::{
    page_cache::{Budget, Cache, Charge, Held},
    sheet_page::Page,
    sheet_range::Source,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
pub const PREFIX: u32 = 25_000;
pub const CHUNK: u32 = 500_000;
pub const CEILING: u32 = 3_000_000;
pub struct Stream {
    pub source: Rc<Source>,
    pub pages: RefCell<Cache<u32, Page>>,
    pub generation: RefCell<crate::stream_generation::Generation>,
    pub loaded: Cell<u32>,
}
impl Stream {
    pub fn new(source: Rc<Source>, budget: Rc<Budget>) -> Rc<Self> {
        Rc::new(Self {
            source,
            pages: RefCell::new(Cache::new(budget)),
            generation: Default::default(),
            loaded: Cell::new(0),
        })
    }
    pub fn adopt(&self, page: Page, charge: Charge) -> Result<Rc<Held<Page>>, &'static str> {
        if page.rows.start >= page.rows.end || page.rows.end > self.source.fields.count {
            return Err("Invalid original streamed drawing page");
        }
        let mut cache = self.pages.borrow_mut();
        for key in cache.keys() {
            let old = cache.get(&key).unwrap();
            if old.value.rows.start < page.rows.end && page.rows.start < old.value.rows.end {
                return Err("Original drawing pages overlap");
            }
        }
        let end = page.rows.end;
        let held = cache.insert(page.rows.start, page, charge)?;
        self.loaded.set(self.loaded.get().max(end));
        Ok(held)
    }
    pub fn pages(&self) -> Vec<Rc<Held<Page>>> {
        let mut cache = self.pages.borrow_mut();
        cache
            .keys()
            .into_iter()
            .filter_map(|key| cache.get(&key))
            .collect()
    }
}
#[derive(Clone)]
pub enum DisplayPage {
