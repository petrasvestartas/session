const assert = require('node:assert/strict');

module.exports = async (page, helpers) => {
    const {command, drawing, step} = helpers;
    await require('./replace.cjs')(page, helpers);
    const stats = async () => JSON.parse(await page.locator('canvas').getAttribute('data-gpu-stats'));
    const cached = ['31b-cache', '31c-incremental'].includes(step.id);
    const incremental = step.id === '31c-incremental';
    const baseline = await drawing(page), before = cached ? await stats() : null;
    for (let index = 0; index < 3; index++) {
        await command(page, 'Select Next');
        if (index < 2) assert.notEqual(await drawing(page), baseline, 'Each row has independent selection colour');
    }
    assert.equal(await drawing(page), baseline, 'Cycling selection restores every scene pixel');
    if (cached) {
        const selected = await stats();
        assert.equal(selected[0], before[0], 'Selection must not upload geometry');
        if (incremental) {
            assert.equal(selected[1], before[1], 'Selection must retain object uniforms');
            assert.equal(selected[2] - before[2], 6, 'Each selection transition changes two rows');
            assert.equal(selected[3] - before[3], 96, 'Six selection writes contain sixteen bytes each');
        } else assert.equal(selected[1] - before[1], 9, 'This transitional endpoint still rebuilds three uniforms per synchronization');
    }
    const moving = cached ? await stats() : null;
    await command(page, 'Move 0.125,0,0');
    assert.notEqual(await drawing(page), baseline, 'Move changes the selected placement');
    if (cached) {
        const moved = await stats();
        assert.equal(moved[0], moving[0], 'Move retains geometry');
        if (incremental) assert.deepEqual(moved, [moving[0], moving[1], moving[2] + 1, moving[3] + 64], 'Move writes only its matrix');
    }
    await command(page, 'Undo');
    assert.equal(await drawing(page), baseline, 'Uniform update and Undo restore exact scene pixels');
    if (incremental) {
        assert.deepEqual(await stats(), [moving[0], moving[1], moving[2] + 2, moving[3] + 128]);
        const unchanged = await stats();
        await command(page, 'Move 0,0,0');
        assert.deepEqual(await stats(), unchanged, 'A no-op queues no settings writes');
        const camera = await stats();
        await command(page, 'Pan Right');
        await command(page, 'Pan Left');
        assert.deepEqual(await stats(), camera, 'Camera changes do not synchronize object settings');
        assert.equal(await drawing(page), baseline);
        await command(page, 'Delete');
        assert.equal(Number(await page.locator('canvas').getAttribute('data-object-count')), 2);
        assert.deepEqual(await stats(), unchanged, 'Deleting the selected row needs no geometry upload or settings write');
        await command(page, 'Undo');
        assert.equal(Number(await page.locator('canvas').getAttribute('data-object-count')), 3);
        assert.notEqual(await drawing(page), baseline, 'Document Undo leaves selection cleared after Delete');
        assert.deepEqual(await stats(), [unchanged[0] + 1, unchanged[1] + 1, unchanged[2], unchanged[3]]);
        await command(page, 'Select Next');
        await command(page, 'Select Next');
        assert.equal(await drawing(page), baseline, 'Reselecting the restored row recovers every scene pixel');
        assert.deepEqual(await stats(), [unchanged[0] + 1, unchanged[1] + 1, unchanged[2] + 3, unchanged[3] + 48]);
    }
    await command(page, 'Orbit Right');
    if (step.id !== '31-settings') await command(page, 'Orbit Up');
    if (cached) {
        await command(page, 'Move 0.1,0,0.15');
        if (incremental) await command(page, 'Move 0.1,-0.15,0');
    }
    await command(page, 'Fit');
};
