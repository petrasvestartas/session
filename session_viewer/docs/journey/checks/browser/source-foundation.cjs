const assert = require('node:assert/strict');
const path = require('node:path');
const ids = ['32f-metadata', '32fa-access', '32fb-boundary'];

module.exports = async (page, helpers) => {
    const {command, drawing, step} = helpers;
    const stage = ids.indexOf(step.id);
    assert(stage >= 0);
    await require('./replace.cjs')(page, helpers);
    const rows = async () => JSON.parse(await page.locator('canvas').getAttribute('data-row-metadata'));
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const baseline = await rows(), pixels = await drawing(page);
    assert.equal(baseline.length, 3);
    for (const [guid, sourceGuid, name, visible, locked] of baseline) {
        assert.equal(guid, sourceGuid);
        assert.equal(typeof name, 'string'); assert(name.length > 0);
        assert.equal(visible, true); assert.equal(locked, false);
    }
    const cpu = await data('data-cpu-usage'), gpu = await data('data-gpu-usage');
    await command(page, 'Move 0.2,0,0.1');
    assert.deepEqual(await rows(), baseline, 'Placement does not alter descriptive values');
    assert.deepEqual((await data('data-cpu-usage')).slice(1, 5), cpu.slice(1, 5));
    assert.deepEqual(await data('data-gpu-usage'), gpu);
    assert.notEqual(await drawing(page), pixels);
    await command(page, 'Undo');
    assert.deepEqual(await rows(), baseline); assert.equal(await drawing(page), pixels);
    const specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    const picker = page.waitForEvent('filechooser');
    await command(page, 'Open'); await (await picker).setFiles(specimen);
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
    const duplicate = await rows();
    assert.equal(duplicate.length, 6);
    assert.equal(new Set(duplicate.map(row => row[0])).size, 6, 'Saved identities stay distinct');
    for (let i = 0; i < 3; i++) {
        assert.notEqual(duplicate[i][0], duplicate[i + 3][0]);
        assert.deepEqual(duplicate[i].slice(1), duplicate[i + 3].slice(1), 'Original source GUID, names and flags survive duplicate import');
    }
    await command(page, 'Undo'); assert.deepEqual(await rows(), baseline);
    assert.equal(await drawing(page), pixels);
    await command(page, 'Redo'); assert.deepEqual(await rows(), duplicate);
    await command(page, 'Undo');
    // The selected imported row survives this Undo; keep that same row for the final image.
    await command(page, 'Orbit Right'); await command(page, 'Orbit Up');
    await command(page, `Move ${(0.42 + stage * 0.06).toFixed(2)},0,${(0.18 + stage * 0.03).toFixed(2)}`);
    await command(page, 'Fit');
    assert.deepEqual(await rows(), baseline);
};
