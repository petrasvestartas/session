const path=require('node:path'),reuse=require('./target-resize.cjs'),quality=require('./quality-settings.cjs'),contacts=require('./ambient-contacts.cjs');
module.exports=async(page,h)=>{
    const url=new URL(page.url());url.searchParams.set('msaa','1');
    await page.goto(url.href);
    await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
    await reuse(page,h);
    const browser=page.context().browser();await page.goto('about:blank');
    await quality(browser,url.href,h,path.resolve(__dirname,'../../../screenshots/journey/review',h.step.id+'-quality'));
    await contacts.start(page,url.href);await contacts.open(page,'ordinary',h);await page.bringToFront();
};
