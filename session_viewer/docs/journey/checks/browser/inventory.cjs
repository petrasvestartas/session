const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./performance.cjs')(page,helpers);
    const bytes=await page.evaluate(async()=>{const response=await fetch('assets/pb/course-metadata.pb');if(!response.ok)throw Error('Missing original metadata fixture');return Array.from(new Uint8Array(await response.arrayBuffer()));});
    const open=async(buffer,name)=>{const chooser=page.waitForEvent('filechooser');await helpers.command(page,'Open');await(await chooser).setFiles({name,mimeType:'application/octet-stream',buffer});await page.waitForFunction(()=>Number(document.querySelector('canvas').getAttribute('data-object-count'))===4);};
    await helpers.command(page,'Close');await open(Buffer.from(bytes),'metadata.pb');
    const usage=JSON.parse(await page.locator('canvas').getAttribute('data-source-metadata'));assert.equal(usage[0],1);assert.ok(usage[1]>0);
    const download=page.waitForEvent('download');await helpers.command(page,'Save');const saved=await download;const stream=await saved.createReadStream();const chunks=[];for await(const chunk of stream)chunks.push(chunk);
    await helpers.command(page,'Close');assert.deepEqual(JSON.parse(await page.locator('canvas').getAttribute('data-source-metadata')),[0,0]);await open(Buffer.concat(chunks),'metadata.session');assert.equal(JSON.parse(await page.locator('canvas').getAttribute('data-source-metadata'))[0],1);
};
