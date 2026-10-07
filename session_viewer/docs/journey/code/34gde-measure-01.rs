pub struct Completed<'a> {
    pub name: &'a str,
    pub duration_ms: f64,
    pub elapsed_ms: f64,
    pub bytes: u64,
    pub succeeded: bool,
}

pub fn run<T>(name: &str, bytes: u64, clock: &mut impl FnMut() -> f64,
    observe: &mut impl FnMut(Completed<'_>), operation: impl FnOnce() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    let started = clock();
    let result = operation();
    let elapsed_ms = clock();
    let duration_ms = elapsed_ms - started;
    if started.is_finite() && started >= 0.0 && elapsed_ms.is_finite() && duration_ms >= 0.0 {
        observe(Completed { name, duration_ms, elapsed_ms, bytes, succeeded: result.is_ok() });
    }
    result
}

pub fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    { crate::browser_phase::now() }
    #[cfg(not(target_arch = "wasm32"))]
    {
        static ORIGIN: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
        ORIGIN.get_or_init(std::time::Instant::now).elapsed().as_secs_f64() * 1000.0
    }
}
