import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {assetsFor, expected, fingerprint, hash, lockFor, materialize, read, viewer, write} from './model.ts';
import {nativeTarget, runCommand, signature} from './verify.ts';

const checkpoint = '59f-cad';
const fixtures = fileURLToPath(new URL('./fixtures/', import.meta.url));
const runner = fileURLToPath(import.meta.url);
const digest = (file: string) => hash(fs.readFileSync(file));
const stamp = () => new Date().toISOString().replace(/[^0-9TZ]/g, '');
const save = (file: string, value: unknown) => {
    assert(!fs.existsSync(file), `Preserve existing evidence: ${file}`);
    write(file, JSON.stringify(value, null, 2) + '\n');
};

function inputs() {
    const files = expected(checkpoint);
    const assets = assetsFor(checkpoint);
    return {
        files, assets,
        record: {
            checkpoint, sourceFingerprint: fingerprint(files), sourceFiles: Object.keys(files).length,
            signature: signature(checkpoint), runnerSha256: digest(runner),
            lockSha256: digest(lockFor(checkpoint)),
            fixtures: Object.fromEntries(['early-boundary-regression.rs', 'early-surface-regression.rs']
                .map(name => [name, digest(path.join(fixtures, name))])),
            assets: Object.fromEntries(assets.map(name => [name, digest(path.join(viewer, name))])),
            depthExampleSha256: hash(depthExample),
        },
    };
}

function prepare() {
    const source = inputs();
    const sourceKey = hash(JSON.stringify(source.record));
    const directory = path.join(viewer, 'target', `course-early-quality-${sourceKey.slice(0, 20)}`);
    assert(!fs.existsSync(directory), `Use the existing prepared receipt: ${directory}`);
    fs.mkdirSync(directory);
    const stage = path.join(directory, 'source');
    materialize(source.files, stage, lockFor(checkpoint), source.assets);
    const alias = `viewer_journey_early_quality_${sourceKey.slice(0, 20)}`;
    const manifestPath = path.join(stage, 'Cargo.toml');
    const oldManifest = read(manifestPath);
    assert.equal(oldManifest.split('name = "viewer_journey"').length, 2);
    assert.equal(oldManifest.split('crate-type = ["cdylib", "rlib"]').length, 2);
    const manifest = oldManifest.replace('name = "viewer_journey"', `name = "${alias}"`)
        .replace('crate-type = ["cdylib", "rlib"]', `name = "${alias}"\ncrate-type = ["rlib"]`);
    write(manifestPath, manifest);
    const lockPath = path.join(stage, 'Cargo.lock');
    const lock = read(lockPath);
    assert.equal(lock.split('name = "viewer_journey"').length, 2);
    write(lockPath, lock.replace('name = "viewer_journey"', `name = "${alias}"`));
    const examples = {
        early_boundary_regression: read(path.join(fixtures, 'early-boundary-regression.rs')),
        early_surface_regression: read(path.join(fixtures, 'early-surface-regression.rs')),
        early_surface_ink_depth: depthExample,
    };
    const examplePins: Record<string, string> = {};
    for (const [name, text] of Object.entries(examples)) {
        const file = path.join(stage, 'examples', `${name}.rs`);
        // An import alias keeps every original fixture assertion and body byte intact.
        write(file, `use ${alias} as viewer_journey;\n${text}`);
        examplePins[file] = digest(file);
    }
    const pins: Record<string, string> = {
        ...examplePins, [manifestPath]: digest(manifestPath), [lockPath]: digest(lockPath),
        ...Object.fromEntries(Object.keys(source.files).filter(name => name !== 'Cargo.toml')
            .map(name => [path.join(stage, name), digest(path.join(stage, name))])),
        ...Object.fromEntries(source.assets.map(name => [path.join(stage, name), digest(path.join(stage, name))])),
    };
    const receipt = path.join(directory, 'prepared.json');
    save(receipt, {
        status: 'PREPARED focused original59f source; no execution claimed', sourceKey, alias, stage,
        inputs: source.record, pins, examples: Object.keys(examples),
        overlays: {manifestBeforeSha256: hash(oldManifest), manifestAfterSha256: hash(manifest),
            scope: 'Private unique native package/rlib plus kernel path; seed lock root package rename only'},
    });
    console.log(receipt);
}

function run(receipt: string, dxc?: string) {
    const prepared = JSON.parse(read(receipt));
    assert.deepEqual(inputs().record, prepared.inputs, 'Authored source/dependency/fixture inputs changed');
    for (const [file, wanted] of Object.entries(prepared.pins)) assert.equal(digest(file), wanted, file);
    const directory = path.dirname(receipt);
    const prefix = `early-quality-${prepared.sourceKey.slice(0, 12)}-${stamp()}`;
    const env: Record<string, string> = {COURSE_MEMORY_MIB: '4096', COURSE_TIMEOUT_SECONDS: '1800', CARGO_INCREMENTAL: '0'};
    let compiler: {path: string; sha256: string; dxilSha256: string} | null = null;
    if (process.platform === 'win32') {
        assert(dxc && path.isAbsolute(dxc), 'Pass --dxc with the task-local absolute dxcompiler.dll path');
        compiler = {path: dxc, sha256: digest(dxc), dxilSha256: digest(path.join(path.dirname(dxc), 'dxil.dll'))};
        env.VIEWER_DXC_PATH = dxc;
    }
    const commands: unknown[] = [];
    commands.push(runCommand(`${prefix}-build`, ['cargo', 'build', ...prepared.examples.flatMap((name: string) => ['--example', name]), '--locked', '--offline',
        '--target', nativeTarget(), '-j1', '-vv', '--message-format=json-render-diagnostics'], prepared.stage, env));
    const evidence = path.join(viewer, 'target/course-checks');
    const log = read(path.join(evidence, `${prefix}-build.log`));
    const artifacts = log.split(/\r?\n/).flatMap(line => {try {return [JSON.parse(line)];} catch {return [];}});
    const library = artifacts.find(row => row.reason === 'compiler-artifact' && row.target.name === prepared.alias
        && row.filenames.some((file: string) => file.endsWith('.rlib')));
    assert(library, 'Actual unique native rlib artifact required');
    assert.equal(path.resolve(library.target.src_path), path.join(prepared.stage, 'src/lib.rs'));
    const results = [];
    const retained = path.join(directory, prefix);
    fs.mkdirSync(retained);
    const commandEvidence = (name: string) => {
        const logPath = path.join(evidence, `${name}.log`);
        const resourcesPath = path.join(evidence, `${name}.resources.json`);
        const resources = process.platform === 'win32' ? JSON.parse(read(resourcesPath)) : null;
        if (resources) {
            assert.equal(resources.exitCode, 0);
            assert.equal(resources.timedOut, false);
            assert.equal(resources.processors, 2);
            assert.equal(resources.jobCommitLimitBytes, 4294967296);
        }
        return {log: {path: logPath, sha256: digest(logPath)},
            resources: resources ? {path: resourcesPath, sha256: digest(resourcesPath), measured: resources} : null};
    };
    const build = commandEvidence(`${prefix}-build`);
    for (const name of prepared.examples) {
        const artifact = artifacts.find(row => row.reason === 'compiler-artifact' && row.target.name === name && row.executable);
        assert(artifact, `Actual example executable required: ${name}`);
        assert.equal(path.resolve(artifact.target.src_path), path.join(prepared.stage, 'examples', `${name}.rs`));
        const executable = path.join(retained, path.basename(artifact.executable));
        fs.copyFileSync(artifact.executable, executable, fs.constants.COPYFILE_EXCL);
        const executableSha256 = digest(executable);
        const label = `${prefix}-${name.replaceAll('_', '-')}`;
        commands.push(runCommand(label, [executable], viewer, env));
        assert.equal(digest(executable), executableSha256);
        const output = read(path.join(evidence, `${label}.log`));
        const success = name === 'early_boundary_regression' ? 'PASS exact authored cylinder, closed seam, chain spans, and original CAD owner addresses'
            : name === 'early_surface_regression' ? 'PASS exact rational surface borders, unchanged face topology, joined source edge IDs and source archive'
                : 'PASS original eight depth-occlusion cases';
        assert(output.includes(success), `Actual fixture completion required: ${name}`);
        results.push({name, executable, executableSha256, artifact, ...commandEvidence(label)});
    }
    assert.deepEqual(inputs().record, prepared.inputs, 'Source/dependency postflight');
    for (const [file, wanted] of Object.entries(prepared.pins)) assert.equal(digest(file), wanted, file);
    if (compiler) {
        assert.equal(digest(compiler.path), compiler.sha256);
        assert.equal(digest(path.join(path.dirname(compiler.path), 'dxil.dll')), compiler.dxilSha256);
    }
    save(path.join(retained, 'result.json'), {
        status: 'PASS focused early boundary/surface and original eight depth-occlusion cases only',
        prepared: {path: receipt, sha256: digest(receipt)}, inputs: prepared.inputs,
        compiler, library, build, commands, results,
        scope: 'No whole native suite, historical browser callback, or image publication acceptance implied',
    });
    console.log(path.join(retained, 'result.json'));
}

// Original proven eight-case body and its native setup; no screenshot/frame publication.
const depthExample = String.raw`use std::io::Write;
use viewer_journey::renderer::Renderer;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 960;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        eprintln!("{}", record.args());
    }

    fn flush(&self) {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Warn);
    pollster::block_on(render())
}

async fn render() -> Result<(), Box<dyn std::error::Error>> {
    let backends = if cfg!(windows) { wgpu::Backends::DX12 }
        else if cfg!(target_os = "macos") { wgpu::Backends::METAL }
        else { wgpu::Backends::VULKAN };
    let mut backend_options = wgpu::BackendOptions::default();
    if cfg!(windows) {
        let compiler = std::path::PathBuf::from(std::env::var("VIEWER_DXC_PATH")
            .map_err(|_| "Set VIEWER_DXC_PATH to the task-local DXC 1.9 dxcompiler.dll")?);
        if !compiler.is_absolute() || !compiler.is_file()
            || !compiler.with_file_name("dxil.dll").is_file() {
            return Err("Task-local DXC and dxil DLLs are required".into());
        }
        backend_options.dx12.shader_compiler = wgpu::Dx12Compiler::DynamicDxc {
            dxc_path: compiler.to_string_lossy().into_owned(),
        };
    }
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options,
        display: None,
    });
    let adapter = if cfg!(windows) {
        instance.enumerate_adapters(backends).await.into_iter().find(|adapter| {
            let info = adapter.get_info();
            info.backend == wgpu::Backend::Dx12 && info.device_type == wgpu::DeviceType::Cpu
                && info.vendor == 0x1414 && info.device == 0x8c
        }).ok_or("Windows frame evidence requires the Microsoft WARP DX12 adapter")?
    } else { instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..Default::default()
    }).await? };
    eprintln!("Native frame adapter: {:?}", adapter.get_info());
    let (device, queue) = adapter.request_device(&Default::default()).await?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| panic!("GPU validation: {error}")));
    let format=wgpu::TextureFormat::Rgba8UnormSrgb;
    prove_surface_ink_depth(&device,&queue,format);
    println!("PASS original eight depth-occlusion cases");
    Ok(())
}

fn scene_pixels(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat,
    size: [u32; 2], renderer: &mut Renderer, camera: &viewer_journey::camera::Camera,
) -> Vec<u8> {
    scene_pixels_mode(device, queue, format, size, renderer, camera, false)
}

fn scene_pixels_mode(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat,
    size: [u32; 2], renderer: &mut Renderer, camera: &viewer_journey::camera::Camera, normals: bool,
) -> Vec<u8> {
    let texture = device.create_texture(&wgpu::TextureDescriptor { label: Some("stroke width check"),
        size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    let view = texture.create_view(&Default::default());
    renderer.resize(viewer_journey::viewport::Viewport { width: size[0], height: size[1] }); renderer.draw_mode(&view, &viewer_journey::background::Background::default(), &camera.uniform(), normals);
    let mut encoder = device.create_command_encoder(&Default::default());
    let row = size[0] * 4;
    let readback = device.create_buffer(&wgpu::BufferDescriptor { label: Some("stroke pixels"), size: (row * size[1]) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &readback,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(size[1]) } }, texture.size());
    queue.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::channel();
    readback.slice(..).map_async(wgpu::MapMode::Read, move |result| send.send(result).unwrap());
    device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).unwrap(); receive.recv().unwrap().unwrap();
    readback.slice(..).get_mapped_range().to_vec()
}

fn prove_surface_ink_depth(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
    use viewer_journey::{scene::Scene, prepared::PreparedMesh, stroke::PreparedLine, chain::PreparedChain, marker::PreparedPoint, camera::{Camera, Projection}};
    for projection in [Projection::Perspective, Projection::Orthographic] {
        for zoom in [1.0, 2.0] {
            for density in [1, 2] {
                let size = [640 * density, 480 * density]; let mut camera = Camera::default(); camera.projection = projection; camera.zoom(zoom);
                let mut scene = Scene::demo(); scene.clear();
                let mut mesh = session_rust::Mesh::from_vertices_and_faces(vec![session_rust::Point::new(-0.8, -0.8, 0.25), session_rust::Point::new(0.8, -0.8, 0.25), session_rust::Point::new(0.8, 0.8, 0.25), session_rust::Point::new(-0.8, 0.8, 0.25)], vec![vec![0, 1, 2, 3]]);
                mesh.set_objectcolor(session_rust::Color::new(0.1, 0.5, 0.1, 1.0)); scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();
                let mut renderer = Renderer::new(device.clone(), queue.clone(), format, &scene); renderer.set_density(density as f32);
                let baseline = scene_pixels(device, queue, format, size, &mut renderer, &camera);
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0); line.width = 9.0;
                scene.insert_line(PreparedLine::new(std::rc::Rc::new(line)).unwrap()).unwrap();
                let curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0), session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                scene.insert_path(PreparedChain::curve(std::rc::Rc::new(curve)).unwrap()).unwrap();
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0); point.width = 12.0;
                scene.insert_point(PreparedPoint::new(std::rc::Rc::new(point)).unwrap()).unwrap();
                renderer.set_scene(&scene, None); assert_eq!(scene_pixels(device, queue, format, size, &mut renderer, &camera), baseline,
                    "Surface must occlude behind-surface line, curve and point: {projection:?}, {zoom}, {density}");
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.5, 0.6, 0.0, 0.5); line.width = 5.0;
                scene.insert_line(PreparedLine::new(std::rc::Rc::new(line)).unwrap()).unwrap();
                let mut point = session_rust::Point::new(0.3, 0.3, 0.5); point.width = 12.0; point.pointcolor = session_rust::Color::new(0.1, 0.1, 0.8, 1.0);
                scene.insert_point(PreparedPoint::new(std::rc::Rc::new(point)).unwrap()).unwrap();
                let mut equal = session_rust::Line::new(-0.6, 0.55, 0.25, 0.6, 0.55, 0.25); equal.width = 5.0;
                scene.insert_line(PreparedLine::new(std::rc::Rc::new(equal)).unwrap()).unwrap(); renderer.set_scene(&scene, None);
                let pixels = scene_pixels(device, queue, format, size, &mut renderer, &camera);
                let pixel = |p: [f64; 3]| {
                    let p = camera.view_projection().transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                    let x = ((p[0] + 1.0) * size[0] as f64 * 0.5) as usize; let y = ((1.0 - p[1]) * size[1] as f64 * 0.5) as usize;
                    &pixels[(y * size[0] as usize + x) * 4..][..3]
                };
                assert!(pixel([-0.4, 0.0, 0.5]).iter().all(|&v| v < 30), "Front line must draw");
                let blue = pixel([0.3, 0.3, 0.5]); assert!(blue[2] > blue[0] + 50, "Front original point must draw");
                assert!(pixel([0.0, 0.55, 0.25]).iter().all(|&v| v < 30), "Flat coplanar edge must draw at equal depth: {projection:?}, {zoom}, {density}");
            }
        }
    }
}
`;

const args = process.argv.slice(2);
if (args[0] === '--prepare') prepare();
else if (args[0] === '--run' && args[1]) run(path.resolve(args[1]), args.includes('--dxc') ? path.resolve(args[args.indexOf('--dxc') + 1]) : undefined);
else throw Error('Use --prepare, then --run PREPARED_JSON [--dxc ABSOLUTE_DLL]');
