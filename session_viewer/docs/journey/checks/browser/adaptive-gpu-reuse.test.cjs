const assert=require('node:assert/strict');
const vm=require('node:vm');
const {test}=require('node:test');
const {snapshot}=require('./adaptive-gpu-reuse.cjs');

function fixture(lane='mesh'){
    const curve=lane==='curve',frame={quality:{pending:false,limited:null,bytes:1000,generations:1,qualities:[[7,.25,.2,false]]},
        stats:curve?[0,0,0,0]:[10,12,30,4000],usage:curve?[0,0,0,0]:[1,2**35,1,256],path:curve?[32,2304,10,11016]:[0,0,0,0],
        surfaces:[],cads:curve?[]:[{id:7,source:'original-cad'}],curveCpu:curve?[1,1,1,100]:[0,0,0,0]};
    const state={frame,flushes:0,rafs:0,waits:0,onFlush:null,onWait:null,arguments:[]};
    const attributes={'data-math-display':'quality','data-gpu-stats':'stats','data-gpu-usage':'usage','data-path-gpu':'path',
        'data-surfaces':'surfaces','data-cads':'cads','data-curve-cpu':'curveCpu'};
    const sandbox=vm.createContext({input:null,document:{querySelector:()=>({getAttribute:name=>{
        const value=state.frame[attributes[name]];return value===null||value===undefined?null:JSON.stringify(value);
    }})},Event:class{constructor(type){this.type=type;}},requestAnimationFrame:callback=>{state.rafs++;queueMicrotask(()=>callback());},
        window:{dispatchEvent:event=>{assert.equal(event.type,'resize');state.flushes++;if(state.onFlush)state.onFlush(state);}}});
    const execute=async(fn,arg)=>{sandbox.input=arg;return structuredClone(await vm.runInContext('('+fn.toString()+')(input)',sandbox));};
    const page={evaluate:execute,waitForFunction:async(fn,arg,options)=>{
        state.arguments.push({arg,options});
        for(let attempt=0;attempt<5;attempt++){state.waits++;if(state.onWait)state.onWait(state);if(await execute(fn,arg))return;}
        throw Error('Fake wait timeout: mathematical qualities did not qualify');
    }};
    const options={lane,source:curve?'curve':'cad',expectedRows:curve?0:1,expectedQualities:1};
    return{page,state,options,run:()=>snapshot(page,options)};
}

test('accepted CAD snapshots flush stale GPU attributes and prove complete same-camera stats and usage reuse',async()=>{
    const f=fixture();f.state.onFlush=state=>{if(state.flushes===1)state.frame.stats=[14,16,33,4400];};
    const result=await f.run();assert.equal(result.adaptive,true);assert.deepEqual(result.stats,[14,16,33,4400]);
    assert.equal(result.usage[1],2**35,'Arena capacity is not compared to a guessed payload cap');
    assert.equal(f.state.flushes,2);assert.equal(f.state.rafs,6);assert(f.state.arguments.every(row=>row.arg===1&&row.options.timeout>0));
});

test('quality/generation changes during publication restart settlement before reading fresh GPU tuples',async()=>{
    const f=fixture('curve');
    f.state.frame.path=[40,2880,13,19080];
    f.state.onFlush=state=>{if(state.flushes===1){state.frame.quality.generations=2;state.frame.quality.qualities=[[7,.2,.18,false]];state.frame.path=[36,2592,15,23000];}};
    const result=await f.run();assert.equal(f.state.flushes,3);assert.equal(result.quality.generations,2);assert.deepEqual(result.path,[36,2592,15,23000]);
});

test('pending work is waited for and errors/limited/empty/unaccepted mathematical fixtures never qualify vacuously',async()=>{
    const pending=fixture();pending.state.frame.quality.pending=true;pending.state.onWait=state=>{if(state.waits===2)state.frame.quality.pending=false;};
    assert.equal((await pending.run()).adaptive,true);assert.equal(pending.state.waits,2);
    for(const change of [quality=>{quality.limited='budget';},quality=>{quality.qualities=[];},quality=>{quality.qualities[0][3]=true;},
        quality=>{quality.qualities[0][1]=0;},quality=>{quality.qualities[0][2]=Infinity;},quality=>{quality.generations=-1;}]){
        const f=fixture();change(f.state.frame.quality);await assert.rejects(f.run(),/did not qualify/);assert.equal(f.state.flushes,0);
    }
});

test('exact caller quality count is required, including shared sources and hidden CAD rows',async()=>{
    const f=fixture();f.state.frame.cads.push({id:8,source:'original-cad'});f.state.frame.quality.qualities.push([8,.25,.2,false]);
    await assert.rejects(f.run(),/did not qualify/);
    f.options.expectedQualities=2;assert.equal((await f.run()).adaptive,true);assert.equal(f.state.frame.usage[2],1,'One hidden row does not remove its mathematical source quality');
});

test('no cohort fixture can qualify with an empty expected quality set, including zero live rows',async()=>{
    const f=fixture();f.state.frame.quality.qualities=[];f.state.frame.stats=[0,0,0,0];f.state.frame.usage=[0,0,0,0];
    f.options.expectedQualities=0;await assert.rejects(f.run(),/cannot qualify with empty/);
    f.options.expectedRows=0;await assert.rejects(f.run(),/cannot qualify with empty/);
});

test('legacy absence and non-mathematical mesh fixtures retain an explicit false adaptive branch without resize',async()=>{
    const legacy=fixture();legacy.state.frame.quality=null;const before=structuredClone(legacy.state.frame.stats);
    const old=await legacy.run();assert.equal(old.adaptive,false);assert.deepEqual(old.stats,before);assert.equal(legacy.state.flushes,0);
    const curves=fixture();curves.state.frame.cads=[];curves.state.frame.curveCpu=[1,1,1,24];
    assert.equal((await curves.run()).adaptive,false,'A mathematical curve does not waive mesh-only legacy counters');assert.equal(curves.state.flushes,0);
});

test('bad live row ownership, geometry count and noninteger diagnostics reject mathematical mesh snapshots',async()=>{
    for(const change of [frame=>{frame.usage[2]=2;},frame=>{frame.usage[0]=2;},frame=>{frame.usage[0]=0;},frame=>{frame.usage[1]=-1;},frame=>{frame.stats[1]=1.5;},frame=>{frame.stats=[1,2,3];}]){
        const f=fixture();change(f.state.frame);await assert.rejects(f.run());
    }
});

test('curve snapshots require positive bounded live segments, the actual stride and all exact static counters',async()=>{
    const valid=fixture('curve');assert.equal((await valid.run()).adaptive,true);
    for(const change of [frame=>{frame.path[0]=0;frame.path[1]=0;},frame=>{frame.path[0]=1025;frame.path[1]=1025*72;},
        frame=>{frame.path[1]++;},frame=>{frame.path[2]=-1;}]){
        const f=fixture('curve');change(f.state.frame);await assert.rejects(f.run());
    }
    for(const lane of ['mesh','curve']){
        const f=fixture(lane);f.state.onFlush=state=>{if(state.flushes===2)state.frame.stats[0]++;};
        await assert.rejects(f.run(),/every mesh counter/);
    }
    const leak=fixture('curve');leak.state.onFlush=state=>{if(state.flushes===2)state.frame.path[2]++;};
    await assert.rejects(leak.run(),/complete curve tuple/);
});

test('complete usage changes during a same-camera redraw reject even if live row count is unchanged',async()=>{
    const f=fixture();f.state.onFlush=state=>{if(state.flushes===2)state.frame.usage[1]++;};
    await assert.rejects(f.run(),/complete GPU usage/);
});

test('surface fixtures require actual current surface metadata and preserve explicit fixture ownership',async()=>{
    const f=fixture();f.options.source='surface';assert.equal((await f.run()).adaptive,false);
    f.state.frame.cads=[];f.state.frame.surfaces=[{id:7,source:'original-surface'}];
    const source=structuredClone(f.state.frame.surfaces);assert.equal((await f.run()).adaptive,true);assert.deepEqual(f.state.frame.surfaces,source);
});
