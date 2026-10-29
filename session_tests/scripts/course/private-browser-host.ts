import {adapterMetadataPath, adapterMetadata, observeExecution, finishExecution, bundleInputs} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {hash, json, viewer, write} from './model.ts';

export async function privateBrowserHost(workspace: string, folder: string, worker: string, config: any) {
    const base = path.join(workspace, 'target/course-checks'), served: Record<string, string> = {};
    const types: Record<string, string> = {'.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.json': 'application/json', '.ttf': 'font/ttf'};
    const server = http.createServer((request, response) => {
        try {
            const url = new URL(request.url!, 'http://localhost');
            if (url.pathname === '/favicon.ico') { response.writeHead(204).end(); return; }
            const file = path.resolve(base, '.' + decodeURIComponent(url.pathname));
            assert(file.startsWith(base + path.sep));
            const actual = fs.statSync(file).isDirectory() ? path.join(file, 'index.html') : file;
            const bytes = fs.readFileSync(actual); served[path.relative(base, actual).split(path.sep).join('/')] = hash(bytes);
            response.writeHead(200, {'Content-Type': types[path.extname(actual)] || 'application/octet-stream', 'Cache-Control': 'no-store'}).end(bytes);
        } catch { response.writeHead(404).end('Not found'); }
    });
    await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve));
    const port = (server.address() as {port: number}).port;
    const metadata = adapterMetadata();
    const chrome = process.env.VIEWER_BROWSER_EXECUTABLE || path.join(viewer, 'target/course-browsers/chromium_headless_shell-1208/chrome-headless-shell-win64/chrome-headless-shell.exe');
    assert(fs.existsSync(chrome));
    const configFile = path.join(folder, 'config.json'); write(configFile, JSON.stringify(config, null, 2) + '\n');
    const log = path.join(folder, 'browser.log'), descriptor = fs.openSync(log, 'wx');
    const loader = fileURLToPath(new URL('../typescript.mjs', import.meta.url));
    const beforeExecution = observeExecution([...bundleInputs(workspace, config.id), process.execPath, loader, worker, chrome, adapterMetadataPath(), configFile]);
    try {
        const args = [path.resolve(viewer, '../session_tests/scripts/run-guarded.cjs'), '-MemoryLimitMiB', '6144', '-TimeoutSeconds', '2400',
            '-WorkingDirectory', workspace, '-ManifestPath', path.join(folder, 'resources.json'), process.execPath,
            '--require', fileURLToPath(new URL('warp-browser-timeout.cjs', import.meta.url)), '--import', pathToFileURL(loader).href, worker, '--worker', configFile];
        const code = await new Promise((resolve, reject) => {
            const child = spawn(process.execPath, args, {cwd: workspace, windowsHide: true, stdio: ['ignore', descriptor, descriptor], env: {...process.env,
                NODE_PATH: path.join(viewer, 'target/course-tools/node_modules'), COURSE_SAMPLES_PREPARED: '1', VIEWER_HEADLESS: '1',
                VIEWER_WARP_ACTION_TIMEOUT_MS: '120000', VIEWER_BROWSER_EXECUTABLE: chrome,
                JOURNEY_URL: `http://127.0.0.1:${port}/`, VIEWER_URL: `http://127.0.0.1:${port}/${config.id}/dist/`,
                VIEWER_DIST: path.join(base, config.id, 'dist'),
                VIEWER_CHROME_ARGS: JSON.stringify(['--enable-unsafe-webgpu', '--ignore-gpu-blocklist', '--use-gl=angle', '--use-angle=d3d11',
                    '--use-adapter-luid=' + metadata.selected.luid, '--enable-dawn-features=d3d_skip_shader_optimizations'])}});
            child.once('error', reject); child.once('exit', resolve);
        });
        assert.equal(code, 0, `Private browser failed; inspect ${log}`);
        const resources = json(path.join(folder, 'resources.json'));
        assert.equal(resources.exitCode, 0); assert.equal(resources.processors, 2); assert.equal(resources.jobCommitLimitBytes, 6442450944);
        return {executionIntegrity: finishExecution(beforeExecution), served, resources, log, logSha256: hash(fs.readFileSync(log))};
    } finally { fs.closeSync(descriptor); await new Promise<void>(resolve => server.close(() => resolve())); }
}

export const observeGpu = `await page.context().addInitScript(() => {
    window.__presentationAdapters=[]; window.__presentationDevices=[]; window.__presentationErrors=[];
    if (!globalThis.GPUAdapter) return;
    const original=GPUAdapter.prototype.requestDevice;
    GPUAdapter.prototype.requestDevice=async function(...args) {
        const device=await original.apply(this,args), info=this.info;
        window.__presentationAdapters.push({vendor:info?.vendor,fallback:info?.isFallbackAdapter}); window.__presentationDevices.push(device);
        device.addEventListener('uncapturederror',e=>window.__presentationErrors.push(e.error.message));
        return device;
    };
});`;

export const validateGpu = `const actualGpu=await page.evaluate(async()=>{
    await Promise.all(window.__presentationDevices.map(device=>device.queue.onSubmittedWorkDone()));
    return {adapters:window.__presentationAdapters,errors:window.__presentationErrors};
});
assert(actualGpu.adapters.length>0); for(const adapter of actualGpu.adapters){assert.match(adapter.vendor,/nvidia/i);assert.equal(adapter.fallback,false);}
assert.deepEqual(actualGpu.errors,[]);`;
