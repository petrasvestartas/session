const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./opacity.cjs')(page,helpers);
    // Open the reference source produced by the supplied native example.
    const bytes=await page.evaluate(async()=>{const response=await fetch('assets/pb/course-comparison.pb');if(!response.ok)throw Error('Missing original comparison source');return Array.from(new Uint8Array(await response.arrayBuffer()));});
    await helpers.command(page,'Close');const chooser=page.waitForEvent('filechooser');await helpers.command(page,'Open');await(await chooser).setFiles({name:'comparison.pb',mimeType:'application/octet-stream',buffer:Buffer.from(bytes)});
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cads')).length===4);
    for(const line of ['Layers Off','View Grid Off','View Isometric','Opacity 1','View Outline On','Fit'])await helpers.command(page,line);
    const originals=JSON.parse(await page.locator('canvas').getAttribute('data-cads'));
    await helpers.command(page,'Opacity 0.4');await helpers.command(page,'Opacity 1');assert.deepEqual(JSON.parse(await page.locator('canvas').getAttribute('data-cads')),originals);
};
