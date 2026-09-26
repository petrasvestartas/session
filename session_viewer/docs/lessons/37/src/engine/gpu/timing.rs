use super::buffers::GpuCtx;

/// Most marks one frame may place.
const MARKS: u32 = 32;

/// GPU milliseconds between named marks of a frame, from timestamp queries.
pub struct PassTimer {
    set: wgpu::QuerySet,                      // one timestamp per mark
    resolved: wgpu::Buffer,                   // ticks, resolved on the GPU
    readback: wgpu::Buffer,                   // ticks, readable by the CPU
    marks: Vec<&'static str>,                 // this frame's marks, in order
    period: f64,                              // nanoseconds per tick
    pub spans: Vec<(&'static str, Vec<f64>)>, // ms since the previous mark, one entry per frame
}

impl PassTimer {
    /// A timer, when the device writes timestamps between passes.
    pub fn new(ctx: &GpuCtx) -> Option<Self> {
        let wanted =
            wgpu::Features::TIMESTAMP_QUERY | wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS;

        if !ctx.device.features().contains(wanted) {
            return None;
        }

        let bytes = u64::from(MARKS) * wgpu::QUERY_SIZE as u64;
        let buffer = |label, usage| {
            ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: bytes,
                usage,
                mapped_at_creation: false,
            })
        };
        Some(Self {
            set: ctx.device.create_query_set(&wgpu::QuerySetDescriptor {
                label: Some("pass timer"),
                ty: wgpu::QueryType::Timestamp,
                count: MARKS,
            }),
            resolved: buffer(
                "pass timer.resolved",
                wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
            ),
            readback: buffer(
                "pass timer.readback",
                wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            ),
            marks: Vec::new(),
            period: f64::from(ctx.queue.get_timestamp_period()),
            spans: Vec::new(),
        })
    }

    /// Timestamp the GPU here; the span ending at this mark is named `label`.
    pub fn mark(&mut self, encoder: &mut wgpu::CommandEncoder, label: &'static str) {
        if self.marks.len() as u32 >= MARKS {
            return;
        }

        encoder.write_timestamp(&self.set, self.marks.len() as u32);
        self.marks.push(label);
    }

    /// Copy this frame's timestamps out; call before submit.
    pub fn resolve(&self, encoder: &mut wgpu::CommandEncoder) {
        let count = self.marks.len() as u32;

        if count < 2 {
            return;
        }

        encoder.resolve_query_set(&self.set, 0..count, &self.resolved, 0);
        encoder.copy_buffer_to_buffer(
            &self.resolved,
            0,
            &self.readback,
            0,
            u64::from(count) * wgpu::QUERY_SIZE as u64,
        );
    }

    /// Wait for the submitted frame and add each span to its series.
    pub fn collect(&mut self, ctx: &GpuCtx) {
        let marks = std::mem::take(&mut self.marks);

        if marks.len() < 2 {
            return;
        }

        let slice = self
            .readback
            .slice(..marks.len() as u64 * wgpu::QUERY_SIZE as u64);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = ctx.device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
        let ticks: Vec<u64> = slice
            .get_mapped_range()
            .chunks_exact(8)
            .map(|bytes| u64::from_le_bytes(bytes.try_into().expect("eight bytes")))
            .collect();
        self.readback.unmap();

        for (at, label) in marks.iter().enumerate().skip(1) {
            let ms = ticks[at].saturating_sub(ticks[at - 1]) as f64 * self.period / 1e6;

            match self.spans.iter_mut().find(|span| span.0 == *label) {
                Some(span) => span.1.push(ms),
                None => self.spans.push((label, vec![ms])),
            }
        }
    }

    /// Median ms of every span, in mark order.
    pub fn medians(&self) -> Vec<(&'static str, f64)> {
        self.spans
            .iter()
            .map(|(label, series)| {
                let mut sorted = series.clone();
                sorted.sort_by(f64::total_cmp);
                (*label, sorted[sorted.len() / 2])
            })
            .collect()
    }
}
