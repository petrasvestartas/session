const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
    await require('./direct-edit.cjs')(original,{...helpers,gizmoDefined:true});
    const start=async page=>{for(const command of ['Close','View Reset','Select Object','View Grid Off','Snap Off','Grid Snap Off','Example Box','Select All','Fit'])await helpers.command(page,command);};
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const anchor=async(page,kind)=>{
        const canvas=page.locator('canvas'),g=await value(page,'data-gizmo'),m=await value(page,'data-camera-matrix'),b=await canvas.boundingBox(),o=g.origin;
        assert(o);const w=o.reduce((sum,v,i)=>sum+m[i*4+3]*v,m[15]),vertical=Math.hypot(m[1],m[5],m[9]),s=2*w/(b.height*vertical);
        const p=o.slice();if(kind==='Move X')p[0]+=86.4*s;else if(kind==='Scale X')p[0]+=48*s;else if(kind==='Rotate Z'){p[0]-=96*s/Math.SQRT2;p[1]-=96*s/Math.SQRT2;}
        const q=[0,1,2,3].map(row=>p.reduce((sum,v,i)=>sum+m[i*4+row]*v,m[12+row]));return [b.x+(q[0]/q[3]+1)*b.width/2,b.y+(1-q[1]/q[3])*b.height/2];
    };
    const click=async(page,kind)=>{const at=await anchor(page,kind);await page.mouse.click(...at);await page.waitForFunction(kind=>JSON.parse(document.querySelector('canvas').getAttribute('data-gizmo')).typing===kind,kind);};
    const number=async(page,text)=>{
        await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).controls?.some(c=>c.key==='number/input'));
        await page.keyboard.type(text);await page.keyboard.press('Enter');
    };
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:960,height:640},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',e=>errors.push(e.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            for(const projection of ['View Perspective','View Orthographic']){
                await start(page);await helpers.command(page,projection);await page.mouse.move(10,10);
                const before=await value(page,'data-row-placements'),gpu=(await value(page,'data-gpu-stats')).slice(0,2),image=await helpers.drawing(page),pick=await value(page,'data-pick-gpu');
                const at=await anchor(page,'Move X');await page.mouse.move(...at);
                await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-gizmo')).active===0);
                assert.notDeepEqual(await helpers.drawing(page),image,'Idle hovering changes the rendered handle highlight');
                assert.deepEqual(await value(page,'data-row-placements'),before);assert.equal((await value(page,'data-pick-gpu'))[3],pick[3]);
                await page.mouse.down();await page.mouse.move(at[0]+40,at[1],{steps:8});
                await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-gizmo')).matrix?.[12]>0.01);
                assert.deepEqual(await value(page,'data-row-placements'),before);assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),gpu);
                assert.notDeepEqual(await helpers.drawing(page),image);assert.equal((await value(page,'data-pick-gpu'))[3],pick[3],'Handle press owns input before asynchronous source picking');
                await page.mouse.up();const after=await value(page,'data-row-placements');assert.notDeepEqual(after,before);await helpers.command(page,'Undo');assert.deepEqual(await value(page,'data-row-placements'),before);
                await click(page,'Move X');await number(page,'0.123456789012345');const precise=await value(page,'data-row-placements');assert.equal(precise[0][1][12],before[0][1][12]+0.123456789012345);
                await helpers.command(page,'Undo');await click(page,'Scale X');await number(page,'0');assert.equal((await value(page,'data-gizmo')).typing,'Scale X');assert.deepEqual(await value(page,'data-row-placements'),before);
                const ui=await value(page,'data-command-ui');assert.equal(ui.number,'0');assert(ui.number_error);await page.keyboard.press('Escape');await helpers.command(page,'Redo');assert.deepEqual(await value(page,'data-row-placements'),precise,'Invalid and cancelled numbers preserve the earlier redo branch');
                await helpers.command(page,'Undo');const drag=await anchor(page,'Move X');await page.mouse.move(...drag);await page.mouse.down();await page.mouse.move(drag[0]+45,drag[1],{steps:8});await page.keyboard.press('Escape');await page.mouse.up();
                assert.equal((await value(page,'data-gizmo')).matrix,null);assert.deepEqual(await value(page,'data-row-placements'),before);
                const blur=await anchor(page,'Move X');await page.mouse.move(...blur);await page.mouse.down();await page.mouse.move(blur[0]+35,blur[1],{steps:8});await page.evaluate(()=>window.dispatchEvent(new Event('blur')));await page.mouse.up();
                assert.equal((await value(page,'data-gizmo')).matrix,null);assert.deepEqual(await value(page,'data-row-placements'),before);
                await click(page,'Move X');await number(page,'');assert.equal((await value(page,'data-gizmo')).typing,null);await helpers.command(page,'Redo');assert.deepEqual(await value(page,'data-row-placements'),precise,'Neutral Enter creates no history');
            }
            // Top view makes the Z rotation arc visible; release must honor Shift even when the preview began free.
            await start(page);await helpers.command(page,'View Orthographic');const arc=await anchor(page,'Rotate Z'),canvas=await page.locator('canvas').boundingBox();
            const origin=await value(page,'data-gizmo'),matrix=await value(page,'data-camera-matrix');
            const o=origin.origin,q=[0,1,2,3].map(row=>o.reduce((sum,v,i)=>sum+matrix[i*4+row]*v,matrix[12+row]));
            const center=[canvas.x+(q[0]/q[3]+1)*canvas.width/2,canvas.y+(1-q[1]/q[3])*canvas.height/2];
            const turn=-Math.PI/3,dx=arc[0]-center[0],dy=arc[1]-center[1],target=[center[0]+dx*Math.cos(turn)-dy*Math.sin(turn),center[1]+dx*Math.sin(turn)+dy*Math.cos(turn)];
            await page.mouse.move(...arc);await page.mouse.down();await page.mouse.move(...target,{steps:12});await page.keyboard.down('Shift');await page.mouse.up();await page.keyboard.up('Shift');
            const rotated=await value(page,'data-row-placements');assert(Math.abs(rotated[0][1][0])<1e-9&&Math.abs(Math.abs(rotated[0][1][1])-1)<1e-9,'Actual Shift release commits a quarter turn');
            assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    await start(original);for(const command of ['Move -1.8,0,0','Example Box','Select All','Fit','View Isometric'])await helpers.command(original,command);
};
