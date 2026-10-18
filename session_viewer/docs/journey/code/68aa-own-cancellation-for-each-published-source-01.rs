
thread_local! {
    static REQUESTS:RefCell<BTreeMap<u64,(String,web_sys::AbortController)>>=const {RefCell::new(BTreeMap::new())};
    static NEXT:Cell<u64>=const {Cell::new(0)};
}

pub fn cancel() {
    for (_,controller) in REQUESTS.with(|requests|std::mem::take(&mut *requests.borrow_mut())).into_values() {controller.abort();}
}

pub fn cancel_url(url:&str) {REQUESTS.with(|requests|requests.borrow_mut().retain(|_,(source,controller)| {if source==url {controller.abort();false} else {true}}));}

struct Read {
