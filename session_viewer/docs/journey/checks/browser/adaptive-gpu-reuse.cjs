const assert=require('node:assert/strict');

function readFrame(){
    const canvas=document.querySelector('canvas');
    const value=name=>{const text=canvas.getAttribute(name);return text===null?null:JSON.parse(text);};
    return{quality:value('data-math-display'),stats:value('data-gpu-stats'),usage:value('data-gpu-usage'),path:value('data-path-gpu'),
        surfaces:value('data-surfaces'),cads:value('data-cads'),curveCpu:value('data-curve-cpu')};
}
function drain(){return new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve)));}
function flush(){window.dispatchEvent(new Event('resize'));return new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve)));}
function ready(quality,count){
    return quality&&quality.pending===false&&quality.limited===null&&Array.isArray(quality.qualities)&&quality.qualities.length===count
        &&Number.isSafeInteger(quality.generations)&&quality.generations>=0&&Number.isSafeInteger(quality.bytes)&&quality.bytes>=0
        &&quality.qualities.every(row=>Array.isArray(row)&&row.length===4&&Number.isSafeInteger(row[0])&&row[0]>=0
            &&Number.isFinite(row[1])&&row[1]>0&&Number.isFinite(row[2])&&row[2]>0&&row[3]===false);
}
function sourcePresent(frame,source){
    const surfaces=frame.surfaces?.length||0,cads=frame.cads?.length||0;
    if(source==='surface')return surfaces>0;
    if(source==='cad')return cads>0;
    if(source==='surface-cad')return surfaces+cads>0;
    return !surfaces&&!cads&&Array.isArray(frame.curveCpu)&&frame.curveCpu[0]>0;
}
function validate(frame,options){
    const tuple=(value,name)=>assert(Array.isArray(value)&&value.length===4&&value.every(n=>Number.isSafeInteger(n)&&n>=0),name+' requires four nonnegative integer counters');
    tuple(frame.stats,'Mesh stats');tuple(frame.usage,'Mesh usage');
    assert.equal(frame.usage[2],options.expectedRows,'Live GPU mesh rows retain the fixture owners');
    assert.equal(frame.usage[0]===0,options.expectedRows===0,'Live geometry exists exactly when the fixture has live mesh rows');
    assert(frame.usage[0]<=frame.usage[2],'Unique live geometry cannot exceed live row ownership');
    if(options.lane==='curve'){
        tuple(frame.path,'Curve path');assert(frame.path[0]>0&&frame.path[0]<=1024,'The one original curve retains its bounded live segments');
        assert.equal(frame.path[1],frame.path[0]*72,'Original curve segments use the actual72-byte stride');
    }
}

async function snapshot(page,options){
    assert(['curve','mesh'].includes(options.lane));assert(['curve','surface','cad','surface-cad'].includes(options.source));
    assert(options.lane==='curve'?options.source==='curve':options.source!=='curve','Declare the actual fixture lane/source');
    assert(Number.isSafeInteger(options.expectedRows)&&options.expectedRows>=0);
    assert(Number.isSafeInteger(options.expectedQualities)&&options.expectedQualities>0,'An explicit mathematical fixture cannot qualify with empty qualities');
    const timeout=options.timeoutMs??30000;assert(Number.isSafeInteger(timeout)&&timeout>0);const deadline=Date.now()+timeout;
    await page.evaluate(drain);
    const initial=await page.evaluate(readFrame);
    const result=(frame,adaptive)=>({adaptive,lane:options.lane,source:options.source,expectedRows:options.expectedRows,
        path:frame.path,stats:frame.stats,usage:frame.usage,quality:frame.quality});
    if(initial.quality===null||!sourcePresent(initial,options.source))return result(initial,false);
    while(true){
        const remaining=deadline-Date.now();assert(remaining>0,'Mathematical GPU diagnostics did not settle within the existing timeout');
        await page.waitForFunction(count=>{
            const text=document.querySelector('canvas').getAttribute('data-math-display');if(text===null)return false;const quality=JSON.parse(text);
            return quality.pending===false&&quality.limited===null&&Array.isArray(quality.qualities)&&quality.qualities.length===count
                &&Number.isSafeInteger(quality.generations)&&quality.generations>=0&&Number.isSafeInteger(quality.bytes)&&quality.bytes>=0
                &&quality.qualities.every(row=>Array.isArray(row)&&row.length===4&&Number.isSafeInteger(row[0])&&row[0]>=0
                    &&Number.isFinite(row[1])&&row[1]>0&&Number.isFinite(row[2])&&row[2]>0&&row[3]===false);
        },options.expectedQualities,{timeout:remaining});
        const before=await page.evaluate(readFrame);await page.evaluate(flush);const fresh=await page.evaluate(readFrame);
        if(!ready(fresh.quality,options.expectedQualities)||JSON.stringify(before.quality)!==JSON.stringify(fresh.quality))continue;
        assert(sourcePresent(fresh,options.source),'The expected mathematical source disappeared during settling');validate(fresh,options);
        await page.evaluate(flush);const repeated=await page.evaluate(readFrame);
        if(!ready(repeated.quality,options.expectedQualities)||JSON.stringify(fresh.quality)!==JSON.stringify(repeated.quality))continue;
        assert(sourcePresent(repeated,options.source));validate(repeated,options);
        assert.deepEqual(repeated.stats,fresh.stats,'A settled same-camera redraw reuses every mesh counter');
        assert.deepEqual(repeated.usage,fresh.usage,'A settled same-camera redraw reuses complete GPU usage');
        if(options.lane==='curve')assert.deepEqual(repeated.path,fresh.path,'A settled same-camera redraw reuses the complete curve tuple');
        return result(repeated,true);
    }
}
module.exports={snapshot};
