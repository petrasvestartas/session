// --8<-- [start:warm-job]
// Idle callback = a function the browser runs when it has spare time between frames (requestIdleCallback).
use super::{GpuCtx, SsaoPipelines, Target, pipelines};
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

/// Compiling the AO pipelines can drop frames, so the browser build compiles one sample count per idle callback.
struct Warmup {
    device: wgpu::Device,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
    ready: [Option<SsaoPipelines>; 2], // compiled, waiting for `take`
    compiled: u8,                      // bit 0 = 1x done, bit 1 = 4x done
    pending: bool,                     // a callback is already requested
    idle: bool,                        // false while the user drags
}

thread_local! {
    static WARMUP: RefCell<Option<Warmup>> = const { RefCell::new(None) };
}

/// The pipelines compiled for `target`, once the idle callback has made them.
pub fn take(ctx: &GpuCtx, target: Target) -> Option<SsaoPipelines> {
    WARMUP.with_borrow_mut(|job| {
        let job = job.as_mut()?;
        // a new device or colour format makes old pipelines useless
        if job.device != ctx.device || job.format != target.format {
            return None;
        }
        job.ready[usize::from(target.samples > 1)].take()
    })
}

/// Request an idle callback, unless one is pending, both are compiled, or the user is dragging.
pub fn schedule(ctx: &GpuCtx, target: Target, idle: bool) {
    let schedule = WARMUP.with_borrow_mut(|job| {
        if job
            .as_ref()
            .is_none_or(|job| job.device != ctx.device || job.format != target.format)
        {
            *job = Some(Warmup {
                device: ctx.device.clone(),
                queue: ctx.queue.clone(),
                format: target.format,
                ready: [None, None],
                compiled: 0,
                pending: false,
                idle,
            });
        }
        let job = job.as_mut().unwrap();
        job.idle = idle;
        if !idle || job.pending || job.compiled == 3 {
            return false;
        }
        job.pending = true;
        true
    });
    if schedule {
        request();
    }
}
// --8<-- [end:warm-job]

// --8<-- [start:warm-idle]
/// One idle callback compiles one sample count, then asks again until both are done.
fn request() {
    // `Closure::once_into_js` turns a Rust closure into a JS function the browser may call once; `move` hands it what it uses.
    let callback = Closure::once_into_js(move |deadline: JsValue| {
        // setTimeout passes no deadline, so assume there is time
        let remaining = if deadline.is_undefined() {
            1.0
        } else {
            // Reflect::get reads a JS property by name, here the deadline's timeRemaining function
            js_sys::Reflect::get(&deadline, &"timeRemaining".into())
                .ok()
                .and_then(|function| function.dyn_into::<js_sys::Function>().ok())
                .and_then(|function| function.call0(&deadline).ok())
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0)
        };
        let again = WARMUP.with_borrow_mut(|job| {
            let Some(job) = job else {
                return false;
            };
            job.pending = false;
            if !job.idle || job.compiled == 3 {
                return false;
            }
            if remaining > 0.0 {
                let slot = usize::from(job.compiled & 1 != 0);
                let ctx = GpuCtx::new(job.device.clone(), job.queue.clone());
                job.ready[slot] = Some(pipelines(
                    &ctx,
                    Target {
                        format: job.format,
                        samples: if slot == 0 { 1 } else { 4 },
                    },
                ));
                job.compiled |= 1 << slot;
                log::info!("Arctic {}x pipelines ready", if slot == 0 { 1 } else { 4 });
            }
            job.pending = job.compiled != 3;
            job.pending
        });
        if again {
            request();
        }
    });
    let result = web_sys::window().and_then(|window| {
        // some browsers lack requestIdleCallback: fall back to a 50 ms timeout
        if let Ok(function) = js_sys::Reflect::get(&window, &"requestIdleCallback".into())
            .and_then(|value| value.dyn_into::<js_sys::Function>())
        {
            function.call1(&window, &callback).ok()
        } else {
            let function = js_sys::Reflect::get(&window, &"setTimeout".into())
                .ok()?
                .dyn_into::<js_sys::Function>()
                .ok()?;
            function.call2(&window, &callback, &50.into()).ok()
        }
    });
    // nothing was scheduled: clear `pending` so a later frame tries again
    if result.is_none() {
        WARMUP.with_borrow_mut(|job| {
            if let Some(job) = job {
                job.pending = false;
            }
        });
    }
}
// --8<-- [end:warm-idle]
