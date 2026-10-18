pub(crate) fn capacity(device:&wgpu::Device)->usize {(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize}
pub struct Rows {