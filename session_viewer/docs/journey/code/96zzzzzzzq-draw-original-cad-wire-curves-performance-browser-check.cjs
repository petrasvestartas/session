const assert = require('node:assert/strict');

module.exports = async original => {
    const page = await original.context().newPage();
    try {
        await page.addInitScript(() => {
            const queues = new Set(), submit = GPUQueue.prototype.submit;
            GPUQueue.prototype.submit = function (commands) {
                queues.add(this);
                return submit.call(this, commands);
            };
            window.__performanceQueues = queues;
        });
        await page.goto(original.url());
        await page.bringToFront();
        await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && document.querySelector('canvas')?.hasAttribute('data-command-ui'), null, {polling: 50});
        await page.evaluate(async () => {
            const fail = message => { throw Error(message); };
            const check = (value, message) => { if (!value) fail(message); };
            const canvas = document.querySelector('canvas');
            const realRaf = window.requestAnimationFrame.bind(window);
            const realCancel = window.cancelAnimationFrame.bind(window);
            const realSubmit = GPUQueue.prototype.submit;
            const realTexture = GPUCanvasContext.prototype.getCurrentTexture;
            const pending = new Map(), queues = window.__performanceQueues;
            let next = 1000000000, frames = 0, submissions = 0;
            const nativeFrame = () => new Promise(resolve => realRaf(resolve));
            const bounded = async promise => {
                let timer;
                try {
                    return await Promise.race([promise, new Promise((_, reject) => {
                        timer = setTimeout(() => reject(Error('Presentation probe timed out')), 15000);
                    })]);
                } finally { clearTimeout(timer); }
            };
            const settled = () => bounded(Promise.all([...queues].map(queue => queue.onSubmittedWorkDone())));
            const flush = async () => {
                const time = await bounded(nativeFrame());
                const callbacks = [...pending.values()]; pending.clear();
                check(callbacks.length <= 1, 'At most one owned animation frame is pending');
                const before = frames;
                for (const callback of callbacks) callback(time);
                check(frames - before <= 1, 'At most one presentation runs in an animation frame');
                await settled();
            };
            const drain = async () => {
                for (let i = 0; i < 8; i++) {
                    await flush();
                    await bounded(nativeFrame());
                    if (!pending.size) return;
                }
                fail('Presentation did not settle');
            };
            const matrix = () => JSON.parse(canvas.getAttribute('data-camera-matrix'));
            const uploads = () => ['data-gpu-stats', 'data-point-gpu', 'data-path-gpu', 'data-stroke-usage']
                .map(name => canvas.getAttribute(name));
            window.requestAnimationFrame = callback => { const id = next++; pending.set(id, callback); return id; };
            window.cancelAnimationFrame = id => { if (!pending.delete(id)) realCancel(id); };
            GPUQueue.prototype.submit = function (commands) { queues.add(this); submissions++; return realSubmit.call(this, commands); };
            GPUCanvasContext.prototype.getCurrentTexture = function () { frames++; return realTexture.call(this); };
            try {
                check(queues.size > 0, 'Startup submitted an actual GPU frame');
                await settled();
                window.dispatchEvent(new Event('resize'));
                await drain();
                const geometry = uploads();
                frames = submissions = 0;
                for (let i = 0; i < 48; i++) window.dispatchEvent(new Event('resize'));
                check(frames === 0 && submissions === 0, 'Input burst does not submit before its animation frame');
                check(pending.size === 1, 'Resize burst retains exactly one animation frame');
                await flush();
                check(frames === 1 && submissions === 2, 'One presentation retains the existing scene and panel submissions');
                check(JSON.stringify(uploads()) === JSON.stringify(geometry), 'Resize burst performs no geometry uploads');
                await drain();

                frames = submissions = 0;
                const before = matrix(), rect = canvas.getBoundingClientRect();
                let previous = JSON.stringify(before);
                for (const delta of [50, -20, 40, -40, 20, -50]) {
                    canvas.dispatchEvent(new WheelEvent('wheel', {deltaY: delta, deltaMode: 0,
                        clientX: rect.left + rect.width / 2, clientY: rect.top + rect.height / 2,
                        bubbles: true, cancelable: true}));
                    const current = JSON.stringify(matrix());
                    check(current !== previous, 'Every ordered wheel mutation is applied before presentation');
                    previous = current;
                }
                const after = matrix();
                check(after.every((value, i) => Math.abs(value - before[i]) <= 1e-9 * Math.max(1, Math.abs(before[i]))),
                    'Balanced ordered zooms restore the camera');
                check(frames === 0 && submissions === 0 && pending.size === 1, 'Ordered zoom burst coalesces only presentation');
                await flush();
                check(frames === 1 && submissions === 2, 'Zoom burst presents once');
                check(JSON.stringify(matrix()) === JSON.stringify(after), 'Presentation keeps the latest camera');
                check(JSON.stringify(uploads()) === JSON.stringify(geometry), 'Zoom burst reuses source geometry');
                await drain();
                const idle = [frames, submissions];
                for (let i = 0; i < 3; i++) await bounded(nativeFrame());
                check(!pending.size && frames === idle[0] && submissions === idle[1], 'Idle runtime schedules and submits no frames');

                window.dispatchEvent(new Event('resize'));
                const stale = [...pending.values()];
                check(stale.length === 1, 'Disposal case starts with a queued frame');
                window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false}));
                await Promise.resolve(); await Promise.resolve();
                check(!window.wasmBindings.runtime_running(), 'Final pagehide disposes the runtime');
                check(pending.size === 0, 'Disposal cancels the queued animation frame');
                await bounded(nativeFrame());
                window.dispatchEvent(new Event('resize'));
                window.dispatchEvent(new Event('viewer-tiles'));
                await settled();
                check(frames === idle[0] && submissions === idle[1], 'Late callbacks and disposed input never present');
                window.__coalescingProof = {resizeEvents: 48, wheelEvents: 6, idleFrames: 3,
                    presentationsPerBurst: 1, submissionsPerPresentation: 2, latePresentations: 0};
            } finally {
                for (const id of pending.keys()) realCancel(id);
                window.requestAnimationFrame = realRaf;
                window.cancelAnimationFrame = realCancel;
                GPUQueue.prototype.submit = realSubmit;
                GPUCanvasContext.prototype.getCurrentTexture = realTexture;
            }
        });
        assert.deepEqual(await page.evaluate(() => window.__coalescingProof), {resizeEvents: 48, wheelEvents: 6,
            idleFrames: 3, presentationsPerBurst: 1, submissionsPerPresentation: 2, latePresentations: 0});
    } finally {
        await page.close();
        await original.bringToFront();
    }
};
