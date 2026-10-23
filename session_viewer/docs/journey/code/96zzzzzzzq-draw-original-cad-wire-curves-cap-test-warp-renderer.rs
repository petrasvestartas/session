pub(crate) fn renderer(scene: &Scene) -> Renderer {
    let backends = if cfg!(windows) { wgpu::Backends::DX12 } else { wgpu::Backends::VULKAN };
    let mut backend_options = wgpu::BackendOptions::default();
    if cfg!(windows) {
        let executable = std::env::current_exe().expect("Locate the task-local WARP test executable");
        let directory = executable.parent().expect("WARP test executable directory");
        let compiler = directory.join("dxcompiler.dll");
        assert!(compiler.is_file() && directory.join("dxil.dll").is_file(),
            "Copy this test executable beside the task-local DXC 1.9 dxcompiler.dll and dxil.dll; no global DLL installation is used");
        backend_options.dx12.shader_compiler = wgpu::Dx12Compiler::DynamicDxc {
            dxc_path: compiler.to_string_lossy().into_owned(),
        };
        eprintln!("actual original cap test DXC: {}", compiler.display());
    }
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends, flags: Default::default(), memory_budget_thresholds: Default::default(),
        backend_options, display: None,
    });
    let adapter = if cfg!(windows) {
        pollster::block_on(instance.enumerate_adapters(backends)).into_iter().find(|adapter| {
            let info = adapter.get_info();
            info.backend == wgpu::Backend::Dx12
                && info.device_type == wgpu::DeviceType::Cpu
                && info.vendor == 0x1414 && info.device == 0x8c
                && info.name.eq_ignore_ascii_case("Microsoft Basic Render Driver")
        }).expect("Native Windows GPU tests require the Microsoft Basic Render Driver DX12 CPU adapter (1414:008c)")
    } else {
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap()
    };
    let info = adapter.get_info();
    if cfg!(windows) {
        assert_eq!(info.backend, wgpu::Backend::Dx12);
        assert_eq!(info.device_type, wgpu::DeviceType::Cpu);
        assert_eq!((info.vendor, info.device), (0x1414, 0x8c));
        assert!(info.name.eq_ignore_ascii_case("Microsoft Basic Render Driver"));
        eprintln!("actual original cap test software adapter: {info:?}");
    }
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e| panic!("actual original cap: {e}")));
    let mut renderer = Renderer::new(device, queue, wgpu::TextureFormat::Rgba8Unorm, scene);
    renderer.resize(Viewport { width: 320, height: 240 });
    renderer
}
