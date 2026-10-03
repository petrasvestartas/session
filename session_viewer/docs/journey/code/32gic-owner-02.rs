pub struct ReloadJob<P = ()> {
    issued: u64,
    pending: Option<Pending<P>>,
}

impl<P> Default for ReloadJob<P> {
    fn default() -> Self { Self { issued: 0, pending: None } }
}

impl<P> ReloadJob<P> {
