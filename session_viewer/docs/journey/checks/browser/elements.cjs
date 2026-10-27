const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./outlines.cjs')(page,helpers);
    for(const line of ['Close','Line w0,0,0 w1000,0,0','Select All','Element Beam','Width=100 Height=200',''])await helpers.command(page,line);
    await helpers.command(page,'View Isometric');await helpers.command(page,'Fit');
    for(const line of ['Element Attributes On','Element Attributes Off','Length BoundingBox'])await helpers.command(page,line);
    assert.match(await page.locator('canvas').evaluate(c=>c.getAttribute('data-controls')??JSON.parse(c.getAttribute('data-measurement')).label),/X 1000/);
    await helpers.command(page,'Undo');await helpers.command(page,'Redo');await helpers.command(page,'Delete');await helpers.command(page,'Undo');
};
