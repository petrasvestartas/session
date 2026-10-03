struct Pending<P> {
    ticket: u64,
    keys: Vec<ReloadKey>,
    payload: P,
}