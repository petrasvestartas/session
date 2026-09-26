use super::{GpuCtx, SsaoPipelines, Target, pipelines};
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

struct Warmup {
    device: wgpu::Device,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
    ready: [Option<SsaoPipelines>; 2],
    compiled: u8,
    pending: bool,
    idle: bool,
}

thread_local! {
    static WARMUP: RefCell<Option<Warmup>> = const { RefCell::new(None) };
}

pub fn take(ctx: &GpuCtx, target: Target) -> Option<SsaoPipelines> {
    WARMUP.with_borrow_mut(|job| {
        let job = job.as_mut()?;
        if job.device != ctx.device || job.format != target.format {
            return None;
        }
        job.ready[usize::from(target.samples > 1)].take()
    })
}

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

fn request() {
    let callback = Closure::once_into_js(move |deadline: JsValue| {
        let remaining = if deadline.is_undefined() {
            1.0
        } else {
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
    if result.is_none() {
        WARMUP.with_borrow_mut(|job| {
            if let Some(job) = job {
                job.pending = false;
            }
        });
    }
}
