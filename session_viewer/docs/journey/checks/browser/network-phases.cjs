const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const http = require('node:http');

module.exports = async (original, helpers) => {
    await require('./upload-phases.cjs')(original, helpers);
    const specimen = await fs.readFile(path.resolve('target', `course-${helpers.step.id}`, 'sample.pb'));
    const server = http.createServer((request, response) => {
        response.writeHead(200, {'Content-Type': 'application/octet-stream', 'Content-Length': specimen.length,
            'Access-Control-Allow-Origin': '*', 'Timing-Allow-Origin': '*', 'Cache-Control': 'no-store'});
        response.write(specimen.subarray(0, 10));
        setTimeout(() => response.end(specimen.subarray(10)), 160);
    });
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    const url = `http://127.0.0.1:${server.address().port}/source.pb?private=secret#fragment`;
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, acceptDownloads: true});
    try {
        const page = await context.newPage(), errors = [];
        page.on('pageerror', error => errors.push(error.message));
        await page.addInitScript(({url}) => {
            window.__network = {url, held: [], mode: 'real'};
            const create = URL.createObjectURL;
            URL.createObjectURL = value => value instanceof File ? window.__network.url : Reflect.apply(create, URL, [value]);
            const fetch = window.fetch;
            window.fetch = function (input, options) {
                if (input !== window.__network.url) return Reflect.apply(fetch, this, [input, options]);
                if (window.__network.mode === 'reject') return Promise.reject(new Error('Controlled network refusal'));
                if (window.__network.mode !== 'hold') return Reflect.apply(fetch, this, [input, options]);
                const actual = Reflect.apply(fetch, this, [input]);
                return new Promise((resolve, reject) => {
                    const entry = {}; window.__network.held.push(entry);
                    actual.then(value => { entry.ready = true; entry.release = () => resolve(value); }, reject);
                });
            };
        }, {url});
        const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const submit = async line => {
            const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
            const field = ui.controls.find(control => control.key === 'command/input');
            const box = await page.locator('canvas').boundingBox(), [left, top, right, bottom] = field.rect;
            await page.mouse.click(box.x + (left + right) / 2, box.y + (top + bottom) / 2);
            await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.type(line); await page.keyboard.press('Enter');
        };
        const phases = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()).phases
            .filter(phase => phase.name.startsWith('source fetch') || phase.name === 'network response'));
        for (const scenario of ['real', 'reject', 'credentials', 'cancel', 'superseded', 'exit']) {
            await page.goto(original.url()); await ready();
            if (scenario === 'credentials') await page.evaluate(() => { const value = new URL(window.__network.url);
                value.username = 'private-user'; value.password = 'private-password'; window.__network.url = value.href; });
            const picker = page.waitForEvent('filechooser'); await helpers.command(page, 'Open Replace');
            await (await picker).setFiles({name: 'sample.pb', mimeType: 'application/octet-stream', buffer: specimen});
            await page.waitForFunction(() => document.getElementById('status').textContent.startsWith('Document replaced'));
            await helpers.command(page, 'Unload Sources'); const image = await helpers.drawing(page);
            await page.evaluate(scenario => { window.__network.mode = ['real', 'reject', 'credentials'].includes(scenario) ? scenario : 'hold'; }, scenario);
            await submit('Reload Sources');
            if (['real', 'reject', 'credentials'].includes(scenario)) {
                await page.waitForFunction(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()).phases
                    .some(phase => phase.name.startsWith('source fetch')));
                const actual = await phases();
                assert.equal(actual[0].name, scenario === 'real' ? 'source fetch' : 'source fetch failed');
                assert.equal(actual[0].bytes, scenario === 'real' ? specimen.length : 0);
                assert.equal(actual[0].source, url.split('?')[0]);
                if (scenario === 'real') {
                    await page.waitForFunction(() => document.getElementById('status').textContent.includes('sources restored'));
                    assert.equal(actual.length, 2, 'The real HTTP response has a browser network entry');
                    const entry = await page.evaluate(url => performance.getEntriesByName(url).at(-1).toJSON(), url);
                    assert.equal(actual[1].durationMs, entry.responseEnd - entry.fetchStart);
                    assert.equal(actual[1].elapsedMs, entry.responseEnd);
                    assert.equal(actual[1].bytes, entry.transferSize);
                    assert(actual[0].durationMs >= 140);
                } else assert.equal(actual.length, 1);
                const download = page.waitForEvent('download'); await helpers.command(page, 'Report');
                const report = JSON.parse(await fs.readFile(await (await download).path(), 'utf8'));
                assert(!JSON.stringify(report).includes('private=secret'));
                assert(!JSON.stringify(report).includes('private-user') && !JSON.stringify(report).includes('private-password'));
                assert.deepEqual(report.phases.filter(phase => phase.name.startsWith('source fetch') || phase.name === 'network response'), actual);
                assert.equal(await helpers.drawing(page), image);
            } else {
                await page.waitForFunction(() => window.__network.held[0]?.ready);
                assert.deepEqual(await phases(), []);
                if (scenario === 'exit') {
                    await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false})));
                    await page.waitForFunction(() => !window.wasmBindings.runtime_running());
                } else if (scenario === 'superseded') {
                    await submit('Reload Sources');
                    await page.waitForFunction(() => window.__network.held[1]?.ready);
                } else await helpers.command(page, 'Cancel Reload');
                await page.evaluate(() => window.__network.held[0].release()); await page.waitForTimeout(100);
                assert.deepEqual(await phases(), [], 'A revoked ticket cannot retain network diagnostics');
                assert.equal(await helpers.drawing(page), image);
                if (scenario === 'superseded') {
                    await page.evaluate(() => window.__network.held[1].release());
                    await page.waitForFunction(() => document.getElementById('status').textContent.includes('sources restored'));
                    assert.equal((await phases()).filter(phase => phase.name === 'source fetch').length, 1);
                }
            }
            console.log(`${helpers.step.id}: actual accepted network timing ${scenario} passed`);
        }
        assert.deepEqual(errors, []);
    } finally {
        await context.close(); server.closeAllConnections(); await new Promise(resolve => server.close(resolve));
        await original.bringToFront();
    }
};
