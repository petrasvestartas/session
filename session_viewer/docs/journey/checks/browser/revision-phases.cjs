const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const crypto = require('node:crypto');

module.exports = async (original, helpers) => {
    await require('./network-phases.cjs')(original, helpers);
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, acceptDownloads: true});
    try {
        const page = await context.newPage();
        await page.goto(original.url());
        await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const specimen = await fs.readFile(path.resolve('target', `course-${helpers.step.id}`, 'sample.pb'));
        const version = crypto.createHash('sha256').update(specimen).digest('hex');
        const revisions = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()).events
            .filter(event => event.kind === 'revision').map(event => event.message));
        const open = async buffer => {
            const picker = page.waitForEvent('filechooser'); await helpers.command(page, 'Open Replace');
            await (await picker).setFiles({name: 'sample.pb', mimeType: 'application/octet-stream', buffer});
        };
        assert.deepEqual(await revisions(), []);
        await open(specimen);
        await page.waitForFunction(() => document.getElementById('status').textContent.startsWith('Document replaced'));
        assert.deepEqual(await revisions(), [version]);
        await helpers.command(page, 'Undo'); await helpers.command(page, 'Redo');
        assert.deepEqual(await revisions(), [version], 'History does not invent a new file revision');
        await open(Buffer.from([0xff]));
        await page.waitForFunction(() => document.getElementById('status').textContent.includes('Invalid session protobuf'));
        assert.deepEqual(await revisions(), [version]);
        await helpers.command(page, 'Unload Sources');
        const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
        const field = ui.controls.find(control => control.key === 'command/input');
        const box = await page.locator('canvas').boundingBox(), [left, top, right, bottom] = field.rect;
        await page.mouse.click(box.x + (left + right) / 2, box.y + (top + bottom) / 2);
        await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.type('Reload Sources'); await page.keyboard.press('Enter');
        await page.waitForFunction(() => document.getElementById('status').textContent.includes('sources restored'));
        assert.deepEqual(await revisions(), [version, version]);
        const download = page.waitForEvent('download'); await helpers.command(page, 'Report');
        const report = JSON.parse(await fs.readFile(await (await download).path(), 'utf8'));
        assert.deepEqual(report.events.filter(event => event.kind === 'revision').map(event => event.message), [version, version]);
        await open(specimen);
        await page.waitForFunction(() => document.getElementById('status').textContent.startsWith('Document replaced'));
        assert.equal((await revisions()).at(-1), version);
        console.log(`${helpers.step.id}: accepted content hashes, failure and source restoration passed`);
    } finally { await context.close(); await original.bringToFront(); }
};
