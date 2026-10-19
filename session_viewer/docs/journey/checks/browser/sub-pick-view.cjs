const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

module.exports = async (original, helpers) => {
    const editor = fs.readFileSync(path.resolve('target', `course-${helpers.step.id}`, 'src/editor.rs'), 'utf8');
    const retainsControlOwner = editor.includes('self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);');
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message)); await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const canvas = page.locator('canvas');
            const value = async name => JSON.parse(await canvas.getAttribute(name));
            const part = () => canvas.getAttribute('data-selection-part');
            let active = '';
            const click = async p => {
                await page.keyboard.press('Escape');
                const matrix = await value('data-camera-matrix'), box = await canvas.boundingBox();
                const clip = [0, 1, 2, 3].map(row => p.reduce((sum, n, axis) => sum + matrix[axis * 4 + row] * n, matrix[12 + row]));
                assert(Math.abs(clip[0] / clip[3]) < 1 && Math.abs(clip[1] / clip[3]) < 1, `Pick target must stay inside the canvas: ${active}`);
                const before = (await value('data-pick-gpu'))[3];
                await page.mouse.click(box.x + (clip[0] / clip[3] + 1) / 2 * box.width, box.y + (1 - clip[1] / clip[3]) / 2 * box.height);
                await page.waitForFunction(before => { const pick = JSON.parse(document.querySelector('canvas').getAttribute('data-pick-gpu')); return pick[3] > before && pick[0] === 0 && pick[1] === 0; }, before).catch(async error => { throw new Error(`Part pick ${active}, density ${density}, point ${JSON.stringify(p)}, clip ${JSON.stringify(clip)}, GPU ${JSON.stringify(await value('data-pick-gpu'))}, drag ${await canvas.getAttribute('data-canvas-drag')}`, {cause: error}); });
            };
            for (const projection of ['View Perspective', 'View Orthographic']) {
                for (const [example, mode, p, expected] of [
                    ['Example Box', 'Face', [.9, 0, .8], /source face \d+$/],
                    ['Example Box', 'Edge', [1.3, 0, .8], /source edge \d+–\d+$/],
                    ['Example Polyline', 'Edge', [-.3, -.2, 0], /source segment 0$/],
                    ['Example Curve', 'Edge', [0, .1, 0], /original curve$/],
                    ['Example Curve', 'Control', [0, .6, 0], /(?:source|original) control 1$/],
                ]) {
                    active = `${projection}: ${example} / ${mode}`;
                    await helpers.command(page, 'Close'); await helpers.command(page, 'Select Object');
                    await helpers.command(page, 'View Grid Off'); await helpers.command(page, 'View Reset');
                    await helpers.command(page, projection); await helpers.command(page, example); await helpers.command(page, 'Fit');
                    if (mode === 'Control') await helpers.command(page, 'Controls On');
                    await helpers.command(page, `Select ${mode}`); const geometry = await value('data-gpu-stats');
                    await click(p); assert.match(await part(), expected); assert(Number(await canvas.getAttribute('data-highlight-bytes')) > 0);
                    await helpers.command(page, 'Selection'); assert.match((await value('data-command-ui')).status, expected);
                    const selected = await part(); await helpers.command(page, 'Delete'); assert.equal(await part(), selected, 'Delete must refuse to delete the parent of a selected part');
                    await helpers.command(page, 'Zoom Out'); if (helpers.gizmoDefined || await canvas.getAttribute('data-gizmo') !== null) await helpers.command(page, 'Select None'); await click(p); assert.equal(await part(), selected);
                    assert.deepEqual(await value('data-gpu-stats'), geometry, 'Part highlights do not upload primary geometry');
                    if (mode === 'Control') {
                        const owner = await canvas.getAttribute('data-selected-id');
                        await helpers.command(page, 'Controls Off');
                        if (retainsControlOwner) {
                            assert.match(owner, /^ObjectId\(\d+\)$/);
                            assert.equal(await canvas.getAttribute('data-selected-id'), owner);
                            assert.equal(await part(), `Object ${owner.match(/\d+/)[0]}`);
                        } else { assert.equal(await part(), ''); }
                    }
                    await helpers.command(page, 'Close'); assert.equal(await part(), ''); assert.equal(await canvas.getAttribute('data-highlight-bytes'), '0');
                    assert.deepEqual((await value('data-pick-gpu')).slice(0, 3), [0, 0, 160]);
                }
            }
            assert.deepEqual(errors, []);
        } finally { await context.close(); }
    }
};
