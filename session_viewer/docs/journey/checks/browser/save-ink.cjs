const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

module.exports = async (page, helpers, refusal) => {
    const source = fs.readFileSync(path.resolve('target', `course-${helpers.step.id}`, 'src/browser.rs'), 'utf8');
    const archives = source.includes('crate::scene_archive::save(');
    const downloads = [];
    const record = download => downloads.push(download);
    page.on('download', record);
    try {
        const pending = archives ? page.waitForEvent('download') : null;
        await helpers.command(page, 'Save');
        const latest = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')).history.at(-1);
        if (!archives) {
            assert.match(latest, refusal);
            assert.equal(downloads.length, 0);
            return;
        }
        const download = await pending;
        assert.equal(download.suggestedFilename(), 'viewer.session');
        const stream = await download.createReadStream();
        assert(stream);
        const chunks = [];
        for await (const chunk of stream) chunks.push(chunk);
        const bytes = Buffer.concat(chunks);
        assert(bytes.length > 100);
        const magic = Buffer.from('SESSION-VIEWER\x01\n');
        assert(bytes.subarray(0, magic.length).equals(magic));
        assert.match(latest, new RegExp(`Saved editable document \\(${bytes.length} bytes\\)`));
        assert.equal(downloads.length, 1);
    } finally { page.off('download', record); }
};
