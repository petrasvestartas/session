pub const BYTES:u64=96;
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {crate::gpu_layout::uniform(device,BYTES)}
pub fn bytes(matrix:[f32;16],display:[f32;4])->[u8;96] {
    let frame=session_rust::Xform::from_matrix(matrix.map(f64::from));let eye=frame.eye();
    let mut values=[0.;24];values[..16].copy_from_slice(&matrix);values[16..20].copy_from_slice(&display);
    values[20..].copy_from_slice(&[eye[0]as f32,eye[1]as f32,eye[2]as f32,frame.ortho_half_height()as f32]);
    let mut bytes=[0;96];for (out,value) in bytes.chunks_exact_mut(4).zip(values) {out.copy_from_slice(&value.to_ne_bytes());}bytes
}
