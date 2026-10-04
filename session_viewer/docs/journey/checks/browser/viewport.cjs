const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (page, {command}) => {
    const size = async () => page.locator('canvas').evaluate(canvas => {
        const rect = canvas.getBoundingClientRect();
        return {width: canvas.width, height: canvas.height, cssWidth: rect.width, cssHeight: rect.height, ratio: devicePixelRatio};
    });
    const matches = async () => {
        await page.waitForFunction(() => {
            const canvas = document.querySelector('canvas');
            const rect = canvas.getBoundingClientRect();
            return canvas.width === Math.round(rect.width * devicePixelRatio)
                && canvas.height === Math.round(rect.height * devicePixelRatio);
        });
        await page.waitForTimeout(150);
        const measured = await size();
        assert.equal(measured.width, Math.round(measured.cssWidth * measured.ratio));
        assert.equal(measured.height, Math.round(measured.cssHeight * measured.ratio));
    };
    for (const viewport of [{width: 1000, height: 800}, {width: 480, height: 900}, {width: 900, height: 760}]) {
        await page.setViewportSize(viewport);
        await matches();
        await command(page, 'View Reset');
        await command(page, 'View Isometric');
        await matches();
    }
    const ready = async () => {
        await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Canvas pixels, depth and camera resize together.');
        await matches();
    };
    const bounds = async () => {
        const measured = await size();
        const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
        const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
        let left = image.width, top = image.height, right = -1, bottom = -1, count = 0;
        for (let y = 0; y < image.height - Math.ceil(34 * measured.ratio); y++) for (let x = 0; x < image.width; x++) {
            const i = (y * image.width + x) * 4;
            if (image.data[i] > 250 && image.data[i + 1] > 250 && image.data[i + 2] > 250) continue;
            left = Math.min(left, x); top = Math.min(top, y);
            right = Math.max(right, x); bottom = Math.max(bottom, y); count++;
        }
        assert(count > 1000, 'Resizing must retain visible geometry');
        return [left, top, right, bottom].map(value => value / measured.ratio);
    };
    await page.reload();
    await ready();
    const normal = await bounds();
    const session = await page.context().newCDPSession(page);
    try {
        await session.send('Emulation.setDeviceMetricsOverride', {width: 900, height: 760, deviceScaleFactor: 2, mobile: false});
        await page.reload();
        await ready();
        assert.equal((await size()).ratio, 2, 'The dense-display check must actually use density two');
        const dense = await bounds();
        assert(dense.every((value, i) => Math.abs(value - normal[i]) <= 1),
            `Display density must preserve CSS geometry bounds: ${normal} → ${dense}`);
    } finally {
        await session.send('Emulation.clearDeviceMetricsOverride');
        await session.detach();
        await page.reload();
        await ready();
    }
    assert.equal((await size()).ratio, 1, 'Restore the screenshot display density');
};
