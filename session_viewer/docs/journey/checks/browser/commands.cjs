const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./multi-selection.cjs')(page, helpers);
    const canvas = page.locator('canvas');
    const value = async name => JSON.parse(await canvas.getAttribute(name));
    const submit = async (line, canonical = line) => {
        await helpers.command(page, line, {canonical, allowUnknown: line === 'Unknown'});
        const after = await value('data-command-ui'); assert.equal(after.command, '');
        assert(after.history.at(-1).startsWith('> '+canonical+'\n'), 'Canonical names retain the intended operation and one history answer');
    };
    const status = async expected => page.waitForFunction(expected => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).status === expected, expected);
    await helpers.command(page, 'Close'); await submit('ExampleBox', 'Example Box');
    await submit('SelectNext', 'Select Next');
    const before = await value('data-row-placements');
    await helpers.command(page, 'Move 1 -2 .03');
    const moved = await value('data-row-placements');
    assert.equal(moved[0][1][12], before[0][1][12] + 1); assert.equal(moved[0][1][13], before[0][1][13] - 2);
    await helpers.command(page, 'Undo'); assert.deepEqual(await value('data-row-placements'), before);
    const rows = await canvas.getAttribute('data-selected-ids'), gpu = await canvas.getAttribute('data-gpu-stats');
    for (const [line, error] of [['Delete all', 'Delete takes no arguments'], ['Move NaN,0,0', 'Move coordinates must be finite'], ['Unknown', 'Unknown command. Type Help.']]) {
        await submit(line); await status(error);
        assert.deepEqual(await value('data-row-placements'), before); assert.equal(await canvas.getAttribute('data-selected-ids'), rows);
        assert.equal(await canvas.getAttribute('data-gpu-stats'), gpu, 'Invalid commands preserve source geometry and uploads');
    }
    await helpers.command(page, 'Redo'); assert.deepEqual(await value('data-row-placements'), moved);
    await submit('ViewPerspective', 'View Perspective'); await helpers.command(page, 'View Orthographic');
    await helpers.command(page, 'Close');
};
