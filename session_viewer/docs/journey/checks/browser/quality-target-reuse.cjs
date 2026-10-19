const reuse=require('./target-resize.cjs');
module.exports=async(page,h)=>{
    const url=new URL(page.url());url.searchParams.set('msaa','1');
    await page.goto(url.href);
    await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
    await reuse(page,h);
};
