//! Arctic-mode uniform: projection matrices + SSAO parameters, bind group 0 binding 0
//! of the ssao pass. Kept separate from `CameraUniform` so the shared 112-byte layout
//! used by every geometry shader never changes.

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArcticUniform {
    pub proj:      [[f32; 4]; 4],
    pub inv_proj:  [[f32; 4]; 4],
    /// Hemisphere kernel (+Z oriented, lengths clustered near the origin via
    /// `mix(0.1, 1.0, (i/n)^2)` like the learnopengl reference). Used by
    /// ao_mode 0 (classic SSAO); .w unused.
    pub kernel:    [[f32; 4]; 32],
    /// AO radius in viewer units (m). Scaled from scene bbox diagonal per frame.
    pub radius_ws: f32,
    pub bias_ws:   f32,
    pub intensity: f32,
    pub flags:     u32,
    /// 0 = SSAO (hemisphere kernel), 1 = HBAO (horizon), 2 = GTAO (ground-truth arcs).
    pub ao_mode:   u32,
    /// Outline (union-silhouette boundary) width in pixels.
    pub outline_px: f32,
    /// Viewport size in pixels (used by the analytic ground pass).
    pub screen_w:  u32,
    pub screen_h:  u32,
    /// Arctic ground plane in VIEW space (analytic, rendered by ray intersection
    /// — no giant quad, no f32 vertex precision wobble). Point on plane (.xyz).
    pub plane_p_vs: [f32; 4],
    /// Unit plane normal in view space (.xyz).
    pub plane_n_vs: [f32; 4],
}

impl Default for ArcticUniform {
    fn default() -> Self {
        let ident = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Self {
            proj:      ident,
            inv_proj:  ident,
            kernel:    build_kernel(),
            radius_ws: 0.05,
            bias_ws:   0.001,
            intensity: 0.55,
            flags:     0,
            ao_mode:   2,
            outline_px: 2.0,
            screen_w:  1,
            screen_h:  1,
            plane_p_vs: [0.0, 0.0, 0.0, 0.0],
            plane_n_vs: [0.0, 0.0, 1.0, 0.0],
        }
    }
}

/// Deterministic LCG hemisphere kernel: +Z hemisphere directions (z in [0,1] like
/// the learnopengl reference — grazing samples detect shallow creases and contact),
/// sample lengths clustered near the origin (`mix(0.1, 1.0, (i/n)^2)`).
pub fn build_kernel() -> [[f32; 4]; 32] {
    let mut state: u32 = 0x9E37_79B9;
    let mut rand01 = move || {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (state >> 8) as f32 / 16_777_216.0
    };
    let mut kernel = [[0.0f32; 4]; 32];
    for (i, k) in kernel.iter_mut().enumerate() {
        let x = rand01() * 2.0 - 1.0;
        let y = rand01() * 2.0 - 1.0;
        let z = rand01();
        let len = (x * x + y * y + z * z).sqrt().max(1e-6);
        let t = i as f32 / 32.0;
        let scale = 0.1 + 0.9 * t * t;
        *k = [x / len * scale, y / len * scale, z / len * scale, 0.0];
    }
    kernel
}

// Byte layout must match the WGSL `Arctic` struct exactly (no align(16) tail pad).
const _: () = assert!(std::mem::size_of::<ArcticUniform>() == 704);

pub fn create_arctic_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("arctic.uniform"),
        contents: bytemuck::bytes_of(&ArcticUniform::default()),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}
