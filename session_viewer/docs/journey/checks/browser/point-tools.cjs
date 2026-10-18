const assert = require('node:assert/strict');
module.exports = async (original, helpers) => {
    await require('./commands.cjs')(original, helpers);
    const run = async page => {
        if (helpers.snappingDefined) { await helpers.command(page, 'Snap Off'); await helpers.command(page, 'Grid Snap Off'); }
        const canvas = page.locator('canvas'), value = async name => JSON.parse(await canvas.getAttribute(name));
        const count = async () => (await value('data-stroke-usage'))[0];
        const screen = async p => {
            const m = await value('data-camera-matrix'), box = await canvas.boundingBox();
            const q = [0,1,2,3].map(row => p.reduce((sum,v,axis) => sum+m[axis*4+row]*v,m[12+row]));
            return [box.x+(q[0]/q[3]+1)/2*box.width,box.y+(1-q[1]/q[3])/2*box.height];
        };
        for (const projection of ['View Perspective', 'View Orthographic']) {
            await helpers.command(page, 'Close'); await helpers.command(page, 'View Reset'); await helpers.command(page, projection);
            await helpers.command(page, 'Line'); await helpers.command(page, '-0.5,0,0');
            const before = await value('data-gpu-stats');
            await page.mouse.move(...await screen([0.5,0,0]));
            await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-tool'))?.cursor);
            assert.equal(await count(), 0); assert.equal(await canvas.getAttribute('data-preview-bytes'), '76');
            assert.deepEqual(await value('data-gpu-stats'), before, 'Hover updates temporary display, not primary geometry');
            await helpers.command(page, 'NaN,0,0');
            assert.equal((await value('data-command-ui')).status, 'Coordinates must be finite and within ±1e12');
            assert.equal((await value('data-tool')).points.length, 1); assert.equal(await count(), 0);
            await helpers.command(page, '-0.5,0,0');
            assert.equal((await value('data-command-ui')).status, 'Line endpoints must differ');
            assert.equal((await value('data-tool')).points.length, 1); assert.equal(await count(), 0);
            const ui = await value('data-command-ui'); assert.equal(ui.completion_rect, null);
            const cancel = ui.controls.find(c => c.key === 'command/option/Cancel').rect;
            await page.mouse.click((cancel[0]+cancel[2])/2,(cancel[1]+cancel[3])/2);
            assert.equal(await value('data-tool'), null); assert.equal(await count(), 0); assert.equal(await canvas.getAttribute('data-preview-bytes'), '0');
            await helpers.command(page, 'Line'); await helpers.command(page, '-0.5,0,0');
            await page.mouse.click(...await screen([0.5,0,0]));
            assert.equal(await value('data-tool'), null); assert.equal(await count(), 1);
            await helpers.command(page, 'Undo'); assert.equal(await count(), 0);
            await helpers.command(page, 'Redo'); assert.equal(await count(), 1);
            await helpers.command(page, 'Line'); await page.mouse.click(...await screen([0,0.5,0]));
            assert.equal((await value('data-tool')).points.length, 1);
            await page.keyboard.press('Escape'); assert.equal(await value('data-tool'), null);
            await helpers.command(page, 'Undo'); assert.equal(await count(), 0, 'Cancelled drafts add no Undo entry');
            await helpers.command(page, 'Redo'); assert.equal(await count(), 1);
            await helpers.command(page, 'Point'); await helpers.command(page, '0.125,0.25,0');
            assert.equal((await value('data-point-gpu'))[0], 1); await helpers.command(page, 'Undo'); assert.equal((await value('data-point-gpu'))[0], 0);
            await helpers.command(page, 'Line');
            const field = (await value('data-command-ui')).controls.find(c => c.key === 'command/input').rect;
            await page.mouse.click((field[0]+field[2])/2,(field[1]+field[3])/2); await page.keyboard.press('Enter');
            assert.equal(await value('data-tool'), null, 'An empty Enter cancels a fixed-count unfinished tool');
            await helpers.command(page, 'Close'); assert.equal(await canvas.getAttribute('data-preview-bytes'), '0');
        }
    };
    for (const density of [1,2]) {
        const context = await original.context().browser().newContext({viewport: {width:960,height:640},deviceScaleFactor:density});
        try {
            const page = await context.newPage(), errors=[]; page.on('pageerror',e=>errors.push(e.message)); await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            await run(page); assert.deepEqual(errors,[]);
        } finally { await context.close(); }
    }
    if (helpers.snappingDefined) { await helpers.command(original, 'Snap Off'); await helpers.command(original, 'Grid Snap Off'); }
    await helpers.command(original, 'Close'); await helpers.command(original, 'View Reset'); await helpers.command(original, 'View Grid Off');
    await helpers.command(original, 'Line'); await helpers.command(original, '-0.65,-0.35,0');
    const canvas=original.locator('canvas'), m=JSON.parse(await canvas.getAttribute('data-camera-matrix')), box=await canvas.boundingBox(), p=[0.65,0.35,0];
    const q=[0,1,2,3].map(row=>p.reduce((sum,v,axis)=>sum+m[axis*4+row]*v,m[12+row]));
    await original.mouse.move(box.x+(q[0]/q[3]+1)/2*box.width,box.y+(1-q[1]/q[3])/2*box.height);
    await original.waitForFunction(()=>document.querySelector('canvas').getAttribute('data-preview-bytes')==='76');
};
