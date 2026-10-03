const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

const image = async page => PNG.sync.read(Buffer.from((await page.locator('canvas').evaluate(c => c.toDataURL())).split(',')[1], 'base64'));
const latest = async page => JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')).history.at(-1);

module.exports = async (page, {command, drawing, step}) => {
    const baseline = await drawing(page);
    if (step.id !== '27b-model') {
        await command(page, 'Move 0.25,0,0');
        const moved = await drawing(page);
        assert.notEqual(moved, baseline, 'Typed Move changes the placed object');
        await command(page, 'Undo');
        assert.equal(await drawing(page), baseline, 'Undo restores every scene pixel');
        await command(page, 'Redo');
        assert.equal(await drawing(page), moved, 'Redo restores the moved placement');
        for (const [text, message] of [['Move 1,2', /three coordinates/], ['Move NaN,0,0', /finite/], ['Move 1 2,3,4', /must be numbers/]]) {
            await command(page, text);
            assert.match(await latest(page), message, 'The parser error appears in command history');
            assert.equal(await drawing(page), moved, 'Invalid input leaves geometry unchanged');
        }
        await command(page, 'Move 0,0,0');
        assert.equal(await drawing(page), moved, 'A zero offset leaves geometry unchanged');
        await command(page, 'Undo');
        assert.equal(await drawing(page), baseline, 'A zero offset does not consume Undo');
        await command(page, 'Redo');
        assert.equal(await drawing(page), moved, 'Invalid and zero offsets preserve history');
        await command(page, 'Undo');
        assert.equal(await drawing(page), baseline);
    }

    const {data, width, height} = await image(page);
    const gold = [];
    let empty;
    const white = (x, y) => {
        const at = (y * width + x) * 4;
        return data[at] === 255 && data[at + 1] === 255 && data[at + 2] === 255;
    };
    const selected = (x, y) => {
        const at = (y * width + x) * 4;
        return data[at] > data[at + 1] + 10 && data[at + 1] > data[at + 2] + 50;
    };
    for (let y = 10; y < height - 50; y++) for (let x = 10; x < width - 10; x++) {
        const at = (y * width + x) * 4;
        const [r, g, b] = data.subarray(at, at + 3);
        // Exact raster-edge ownership is covered when the course introduces GPU picking.
        if (r > g + 10 && g > b + 50 && selected(x - 5, y) && selected(x + 5, y)
            && selected(x, y - 5) && selected(x, y + 5)) gold.push([x, y]);
        if (!empty && white(x, y) && white(x - 5, y) && white(x + 5, y) && white(x, y - 5) && white(x, y + 5)) empty = [x, y];
    }
    assert(gold.length > 1500, 'The selected placed object is visible');
    assert(empty, 'An empty scene pixel is available');
    const box = await page.locator('canvas').boundingBox();
    const click = async ([x, y]) => {
        await page.mouse.click(box.x + (x + .5) * box.width / width, box.y + (y + .5) * box.height / height);
        await page.waitForTimeout(120);
    };
    await click(empty);
    const unselected = await drawing(page);
    assert.notEqual(unselected, baseline, 'An empty click clears selection');
    if (step.id === '27d-history') {
        await command(page, 'Move 1,0,0');
        assert.match(await latest(page), /Select an object before Move/);
        assert.equal(await drawing(page), unselected, 'Missing selection cannot move arbitrary geometry');
    }
    const target = gold[Math.floor(gold.length / 2)];
    await click(target);
    if (await drawing(page) !== baseline) {
        const fs = require('node:fs');
        fs.writeFileSync('target/course-checks/placement-pick-before.png', PNG.sync.write({data, width, height}));
        fs.writeFileSync('target/course-checks/placement-pick-after.png', PNG.sync.write(await image(page)));
        fs.writeFileSync('target/course-checks/placement-pick.json', JSON.stringify({step: step.id, target, gold: gold.length, latest: await latest(page)}));
    }
    assert.equal(await drawing(page), baseline, 'Picking a placed GPU pixel selects the same world-space object');
    if (step.id === '27d-history') {
        await command(page, 'Move 0,0,0');
        assert.equal(await drawing(page), baseline, 'A valid command after the error preserves the recovered picture');
    }
};
