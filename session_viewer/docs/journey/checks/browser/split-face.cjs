const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./split.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const click=async point=>{const m=await value('data-camera-matrix'),v=[...point,1],q=[0,1,2,3].map(i=>v.reduce((sum,x,j)=>sum+x*m[j*4+i],0)),box=await page.locator('canvas').boundingBox();await page.mouse.click(box.x+(q[0]/q[3]+1)*box.width/2,box.y+(1-q[1]/q[3])*box.height/2);};
    for(const line of ['Close','Nurbs Surface 4 Points w-2,-2,0 w2,-2,0 w2,2,0 w-2,2,0','Line w0,-3,0 w0,3,0','View Isometric','Select None','Fit'])await helpers.command(page,line);
    await click([-1,0,0]);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids')).length===1);await helpers.command(page,'Split');assert.equal((await value('data-split')).face,null);await click([0,2.5,0]);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-split')).cutters.length===1);await helpers.command(page,'Split');assert.equal((await value('data-surfaces')).length,0);assert.equal((await value('data-cads'))[0].faces.length,2);await helpers.command(page,'Undo');assert.equal((await value('data-surfaces')).length,1);await helpers.command(page,'Redo');assert.equal((await value('data-cads'))[0].faces.length,2);await helpers.command(page,'Select None');await helpers.command(page,'Fit');
};
