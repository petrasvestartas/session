pub fn run<T>(name: &str, bytes: u64, clock: &mut impl FnMut() -> f64,
    observe: &mut impl FnMut(Completed<'_>), operation: impl FnOnce() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    run_result(name,bytes,clock,observe,operation)
}

pub fn run_result<T,E>(name:&str,bytes:u64,clock:&mut impl FnMut()->f64,
    observe:&mut impl FnMut(Completed<'_>),operation:impl FnOnce()->Result<T,E>)->Result<T,E>{
