const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
module.exports=async(original,helpers)=>{
    await require('./display-defaults.cjs')(original,helpers);
    const context=await original.context().browser().newContext({viewport:{width:320,height:240},deviceScaleFactor:3});
    try{
        const page=await context.newPage(),errors=[];page.on('pageerror',e=>errors.push(e.message));
        await page.addInitScript(()=>{
            window.__scaleTextures=[];const create=GPUDevice.prototype.createTexture;
            GPUDevice.prototype.createTexture=function(descriptor){
                const texture=create.call(this,descriptor);
                window.__scaleTextures.push({label:descriptor.label,width:texture.width,height:texture.height});
                return texture;
            };
        });
        const base=new URL(original.url());base.search='';base.hash='';
        for(const [query,scale] of [['',2],['dpr=1',1],['dpr=3',3],['dpr=0.75',0.75],['dpr=0.4',2],['dpr=NaN',2]]){
            const url=new URL(base);url.search=query;await page.goto(url.href);
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()
                &&document.querySelector('canvas')?.hasAttribute('data-grid-enabled'));
            await page.waitForFunction(scale=>document.querySelector('canvas').width===320*scale,scale);
            await page.waitForTimeout(120);
            const image=PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(page.locator('canvas'))).split(',')[1],'base64'));
            assert.deepEqual(await page.locator('canvas').evaluate(c=>[c.width,c.height]),[320*scale,240*scale]);
            const primitive=await page.evaluate(()=>window.__scaleTextures.filter(t=>t.label==='physical triangle addresses').at(-1));
            assert.ok(primitive,'The renderer created its actual primitive image');
            assert.deepEqual([primitive.width,primitive.height],[320*scale,240*scale]);
            assert.deepEqual([image.width,image.height],[960,720],'The composited screenshot retains browser display density three');
            let painted=0;for(let i=0;i<image.data.length;i+=4)if(image.data[i]<240||image.data[i+1]<240||image.data[i+2]<240)painted++;
            assert.ok(painted>10,'The scaled image contains actual scene and grid pixels');
        }
        assert.deepEqual(errors,[]);
    }finally{await context.close();await original.bringToFront();}
    console.log(`${helpers.step.id}: actual browser density three, capped GPU images and finite dpr overrides passed`);
};
