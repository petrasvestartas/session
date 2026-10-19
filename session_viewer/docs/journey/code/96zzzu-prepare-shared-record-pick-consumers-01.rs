pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared object records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]})
}
pub fn source(shader:&str)->String {
 shader.replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;").replace("return object_settings;","return objects[index];")
}
pub struct Rows {