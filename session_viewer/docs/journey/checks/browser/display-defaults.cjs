const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    const base=new URL(page.url());base.search='';base.hash='';
    const ready=p=>p.waitForFunction(()=>window.wasmBindings?.runtime_running?.()
        &&document.querySelector('canvas')?.hasAttribute('data-grid-enabled'));
    const settings=async p=>({opacity:JSON.parse(await p.locator('canvas').getAttribute('data-clipping')).opacity,
        grid:await p.locator('canvas').getAttribute('data-grid-enabled')==='true'});
    const expect=async(p,opacity,grid)=>{const actual=await settings(p);assert.ok(Math.abs(actual.opacity-opacity)<1e-6);assert.equal(actual.grid,grid);};
    await page.goto(base.href);await ready(page);await expect(page,0.95,true);
    for(const [query,opacity,grid] of [['opacity=0.5&nogrid=1',0.5,false],['opacity=NaN',0.95,true],['opacity=2&nogrid=0',1,false]]){
        const url=new URL(base);url.search=query;await page.goto(url.href);await ready(page);await expect(page,opacity,grid);
    }
    await helpers.command(page,'Opacity 0.4');await helpers.command(page,'View Grid On');await helpers.command(page,'Orbit Up');
    await page.setViewportSize({width:960,height:640});await page.waitForFunction(()=>document.querySelector('canvas').width===960);
    await expect(page,0.4,true);
    const context=await page.context().browser().newContext({viewport:{width:390,height:760},hasTouch:true,isMobile:true,deviceScaleFactor:1});
    try{
        const phone=await context.newPage(),errors=[];phone.on('pageerror',e=>errors.push(e.message));
        await phone.goto(base.href);await ready(phone);assert.equal(await phone.evaluate(()=>matchMedia('(pointer: coarse)').matches),true);
        await expect(phone,1,true);
        const override=new URL(base);override.search='opacity=0.5&nogrid=1';await phone.goto(override.href);await ready(phone);await expect(phone,0.5,false);
        assert.deepEqual(errors,[]);
    }finally{await context.close();await page.bringToFront();}
    await page.setViewportSize({width:900,height:760});await page.goto(base.href);await ready(page);await expect(page,0.95,true);
    await helpers.command(page,'Close');await helpers.command(page,'View Isometric');await helpers.command(page,'Example CAD');await helpers.command(page,'Fit');
    await page.keyboard.press('Escape');await page.waitForTimeout(150);const withGrid=await helpers.drawing(page);
    await helpers.command(page,'View Grid Off');await page.keyboard.press('Escape');await page.waitForTimeout(150);
    assert.notEqual(await helpers.drawing(page),withGrid,'The startup grid is actual drawing output');
    await helpers.command(page,'View Grid On');await helpers.command(page,'Select Next');await page.keyboard.press('Escape');await expect(page,0.95,true);
    console.log(`${helpers.step.id}: actual desktop/coarse-pointer startup, URL overrides, persistent view commands and grid pixels passed`);
};
