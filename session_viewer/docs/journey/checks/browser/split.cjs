const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./editing.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const click=async point=>{const m=await value('data-camera-matrix'),v=[...point,1],q=[0,1,2,3].map(i=>v.reduce((sum,x,j)=>sum+x*m[j*4+i],0)),box=await page.locator('canvas').boundingBox();await page.mouse.click(box.x+(q[0]/q[3]+1)*box.width/2,box.y+(1-q[1]/q[3])*box.height/2);};
    for(const line of ['Close','Line w-2,0,0 w2,0,0','Line w0,-2,0 w0,2,0','View Isometric','Select None','Fit'])await helpers.command(page,line);
    await click([-1,0,0]);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids')).length===1);await helpers.command(page,'Split');const initial=await value('data-split');assert.equal(initial.cutters.length,0);await helpers.command(page,'');assert.equal((await value('data-split')).target,initial.target);
    await click([0,1,0]);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-split')).cutters.length===1);await helpers.command(page,'Split');assert.equal(await value('data-split'),null);assert.equal((await value('data-drawing-originals')).lines.length,3);await helpers.command(page,'Undo');assert.equal((await value('data-drawing-originals')).lines.length,2);await helpers.command(page,'Redo');assert.equal((await value('data-drawing-originals')).lines.length,3);
    await helpers.command(page,'Split');await helpers.command(page,'Escape');assert.equal(await value('data-split'),null);await helpers.command(page,'Select None');await helpers.command(page,'Fit');await click([-1,0,0]);
};
