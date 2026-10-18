use std::sync::{Arc,atomic::{AtomicU8,Ordering}};
/// Reads back how many words the last scan needed.
pub struct PoolReport {
    buffer: wgpu::Buffer, // 16-byte CPU-readable copy
    ready: Arc<AtomicU8>, // 0 waiting, 1 mapped, 2 failed
    copied: bool,         // a copy was encoded this frame
    inflight: bool,       // the copy is being mapped
}

impl PoolReport {
    /// Create the readback buffer.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("triangle.tiles.report"),
                size: 16,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            ready: Arc::new(AtomicU8::new(0)),
            copied: false,
            inflight: false,
        }
    }

    /// Copy the first tile record into the readback buffer.
    pub fn copy(&mut self, encoder: &mut wgpu::CommandEncoder, tiles: &wgpu::Buffer) {
        if self.inflight {
            return;
        }

        encoder.copy_buffer_to_buffer(tiles, 0, &self.buffer, 0, 16);
        self.copied = true;
    }

    /// Start mapping the copy; call once after submit.
    pub fn map(&mut self) {
        if !self.copied || self.inflight {
            return;
        }

        self.copied = false;
        self.inflight = true;
        let flag = self.ready.clone();
        self.buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                flag.store(if result.is_ok() { 1 } else { 2 }, Ordering::Release);
                #[cfg(target_arch="wasm32")] crate::browser_tiles::wake();
            });
    }

    /// Words the last scan needed, once the copy is readable.
    pub fn poll(&mut self) -> Option<u64> {
        if !self.inflight {
            return None;
        }

        let status = self.ready.load(Ordering::Acquire);

        if status == 0 {
            return None;
        }

        self.ready.store(0, Ordering::Release);
        self.inflight = false;

        if status != 1 {
            return None;
        }

        let words = {
            let bytes = self.buffer.slice(..).get_mapped_range();
            u32::from_le_bytes(bytes[4..8].try_into().expect("report words"))
        };
        self.buffer.unmap();
        Some(u64::from(words))
    }
}

