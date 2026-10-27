const assert = require('node:assert/strict');
const fs = require('node:fs');
module.exports = async (page, helpers) => {
    await require('./instance-features.cjs')(page, helpers);
    const canvas = page.locator('canvas');
    const value = name => canvas.getAttribute(name).then(JSON.parse);
    const open = async file => {
        const choosing = page.waitForEvent('filechooser');
        await helpers.command(page, 'Open'); await (await choosing).setFiles(file);
        await page.waitForFunction(() => document.querySelector('canvas').getAttribute('data-object-count') === '2');
    };
    for (const command of ['Close', 'Select Object', 'Element Attributes Off', 'Element Interactions On']) await helpers.command(page, command);
    await open({name:'feature-pick.pb',mimeType:'application/octet-stream',buffer:fs.readFileSync('assets/pb/course-feature-pick.pb')});
    for (const command of ['Layers All', 'View Grid Off', 'View Front', 'View Orthographic', 'Fit', 'Select None']) await helpers.command(page, command);
    const rows = await value('data-tree');
    const first = rows.find(row => row.label === 'Pick first'), locked = rows.find(row => row.label === 'Pick locked');
    assert(first && locked); assert.equal((await value('data-path-gpu'))[0],2);
    const click = async point => {
        await page.keyboard.press('Escape');
        const matrix = await value('data-camera-matrix'), box = await canvas.boundingBox();
        const clip = [0,1,2,3].map(row=>point.reduce((sum,n,axis)=>sum+matrix[axis*4+row]*n,matrix[12+row]));
        assert(Math.abs(clip[0]/clip[3])<1 && Math.abs(clip[1]/clip[3])<1);
        const before = (await value('data-pick-gpu'))[3];
        await page.mouse.click(box.x+(clip[0]/clip[3]+1)/2*box.width,box.y+(1-clip[1]/clip[3])/2*box.height);
        await page.waitForFunction(before=>{const pick=JSON.parse(document.querySelector('canvas').getAttribute('data-pick-gpu'));return pick[3]>before && pick[0]===0 && pick[1]===0;},before);
    };
    await click([2.5,.4,.2]); assert.deepEqual(await value('data-selected-ids'),[], 'A locked original cannot be selected through its generated line');
    await click([.5,.4,.2]); assert.deepEqual(await value('data-selected-ids'),first.ids, 'The feature pixel selects the original Element');
    assert(Number(await canvas.getAttribute('data-highlight-bytes'))>0);
    await helpers.command(page,'Delete'); assert.equal(await value('data-object-count'),1);assert.equal((await value('data-path-gpu'))[0],1);
    await helpers.command(page,'Undo');assert.equal(await value('data-object-count'),2);assert.equal((await value('data-path-gpu'))[0],2);
    const downloading=page.waitForEvent('download');await helpers.command(page,'Save');const saved=await(await downloading).path();
    await helpers.command(page,'Close');await open(saved);
    await page.waitForFunction(()=>document.querySelector('#status').textContent==='Document replaced. Undo restores the previous document.');
    assert.equal((await value('data-path-gpu'))[0],2);assert.equal((await value('data-tree')).filter(row=>row.label.startsWith('piece ')).length,2);
    console.log(`${helpers.step.id}: actual feature pixels target original CAD owners, enforce locking and regenerate once after archive reopen`);
};
