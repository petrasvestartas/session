
/// Vertex slot 0: the arena's packed vertex (position, normal, color).
pub fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
    crate::engine::gpu::arena::GpuVertex::layout()
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod meshes_tests {
    use super::*;
    use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
    use session_rust::{RenderVertex, Xform};

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// An empty frame compiles the backdrop only, an MSAA flip back compiles nothing, all compile.
    fn pipelines_compile_on_first_use_and_once_per_target() {
        let mut gpu = pollster::block_on(Gpu::new_headless(96, 96)).unwrap();
        let input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let boot = created().0;
        gpu.render_offscreen(&input);
        assert!(
            created().0 - boot <= 2,
            "an empty frame compiles the backdrop only"
        );
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), 0));

        for position in [
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
        ] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [0.0, 0.0, 1.0],
                color: [0.3, 0.5, 0.7, 1.0],
            });
            upload.arena.vids.push(0);
        }

        upload.arena.idx = vec![0, 1, 2, 0, 2, 3];
        gpu.set_scene(&upload);
        let mut compiled = Vec::new();

        for samples in [4, 1, 4, 1] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(96, 96);
            gpu.render_offscreen(&input);
            compiled.push(created().0);
        }

        assert!(
            compiled[0] > boot + 2,
            "the solid frame compiled its pipelines"
        );
        assert_eq!(
            compiled[2], compiled[1],
            "back at 4x nothing compiles again"
        );
        assert_eq!(
            compiled[3], compiled[1],
            "back at 1x nothing compiles again"
        );
        // the ones no frame used yet compile without a validation error too
        assert!(gpu.ctx.cache.compile_all() as u32 >= compiled[3] - boot);
    }
}
