const assert=require('node:assert/strict');

async function observe(context){
 await context.addInitScript(()=>{
  if(typeof GPUAdapter==='undefined')return;
  const state=window.__presentationProbe={first:null,last:null,errors:[],pending:{armed:false,held:0,released:0}};
  const request=GPUAdapter.prototype.requestDevice;
  GPUAdapter.prototype.requestDevice=async function(...args){
   const device=await request.apply(this,args);window.__presentationDevice=device;
   device.addEventListener('uncapturederror',e=>state.errors.push(e.error.message));
   const uniforms=new WeakMap(),groups=new WeakMap(),shaders=new WeakMap(),pipelines=new WeakMap(),commands=new WeakMap();
   const buffer=device.createBuffer.bind(device);
   device.createBuffer=function(d){const b=buffer(d);if(d.label==='view transform and normal diagnostic'){if(d.size!==112)throw Error('Unexpected actual view layout');uniforms.set(b,null);}if(d.label==='bounded pick readback'){const map=b.mapAsync.bind(b);b.mapAsync=function(...args){const real=map(...args);if(!state.pending.armed)return real;state.pending.armed=false;return real.then(async value=>{state.pending.held++;await new Promise(resolve=>window.__presentationReleasePick=resolve);state.pending.released++;window.__presentationReleasePick=null;return value;});};}return b;};
   const write=device.queue.writeBuffer.bind(device.queue);
   device.queue.writeBuffer=function(b,offset,data,start=0,size){
    if(uniforms.has(b)&&offset===0){
     const unit=ArrayBuffer.isView(data)?data.BYTES_PER_ELEMENT||1:1;
     const bytes=ArrayBuffer.isView(data)?new Uint8Array(data.buffer,data.byteOffset,data.byteLength):new Uint8Array(data);
     const first=start*unit,length=size===undefined?bytes.length-first:size*unit;
     if(length!==112)throw Error('Actual view upload must contain112bytes');
     const copy=bytes.slice(first,first+length),view=new DataView(copy.buffer,copy.byteOffset,copy.byteLength);
     uniforms.set(b,Array.from({length:28},(_,i)=>view.getFloat32(i*4,true)));
    }
    return write(b,offset,data,start,size);
   };
   const group=device.createBindGroup.bind(device);
   device.createBindGroup=function(d){const g=group(d);for(const e of d.entries){const b=e.resource?.buffer;if(e.binding===0&&uniforms.has(b))groups.set(g,b);}return g;};
   const shader=device.createShaderModule.bind(device);
   device.createShaderModule=function(d){const s=shader(d);shaders.set(s,d.code);return s;};
   const tag=d=>{const source=shaders.get(d.vertex.module)||'',fragment=shaders.get(d.fragment?.module)||'';return{grid:d.primitive?.topology==='line-list'&&d.vertex.buffers?.[0]?.arrayStride===44&&['rg16uint','rg32uint'].includes(d.fragment?.targets?.[1]?.format),marker:d.vertex.buffers?.[0]?.arrayStride===32&&source.includes('marker_radius'),outline:fragment.includes('global_alpha')&&fragment.includes('selected_alpha'),label:d.label||''};};
   const pipeline=device.createRenderPipeline.bind(device);
   device.createRenderPipeline=function(d){const p=pipeline(d);pipelines.set(p,tag(d));return p;};
   const asyncPipeline=device.createRenderPipelineAsync.bind(device);
   device.createRenderPipelineAsync=async function(d){const p=await asyncPipeline(d);pipelines.set(p,tag(d));return p;};
   const encoder=device.createCommandEncoder.bind(device);
   device.createCommandEncoder=function(...args){
    const e=encoder(...args),draws=[],begin=e.beginRenderPass.bind(e),finish=e.finish.bind(e);
    e.beginRenderPass=function(d){
     const p=begin(d),setPipeline=p.setPipeline.bind(p),setGroup=p.setBindGroup.bind(p);let selected=null,view=null;
     p.setPipeline=function(value){selected=pipelines.get(value)||null;return setPipeline(value);};
     p.setBindGroup=function(slot,value,...rest){if(slot===0)view=groups.get(value)||null;return setGroup(slot,value,...rest);};
     for(const name of ['draw','drawIndexed','drawIndirect','drawIndexedIndirect']){
      const run=p[name].bind(p);
      p[name]=function(...values){
       if(name.endsWith('Indirect')||values[0]>0&&(values[1]===undefined||values[1]>0))draws.push({pass:d.label||'',pipeline:selected,view:view&&uniforms.get(view)?.slice(),kind:name,instances:name.endsWith('Indirect')?null:values[1]??1,firstInstance:name==='draw'?values[3]??0:name==='drawIndexed'?values[4]??0:null});
       return run(...values);
      };
     }
     return p;
    };
    e.finish=function(...args){const result=finish(...args);commands.set(result,draws);return result;};return e;
   };
   const submit=device.queue.submit.bind(device.queue);
   device.queue.submit=function(values){
    const list=Array.from(values),draws=list.flatMap(v=>commands.get(v)||[]);
    if(draws.some(d=>d.pass==='canvas'&&d.view)){state.last={draws,view:draws.find(d=>d.pass==='canvas'&&d.view).view};if(!state.first)state.first=state.last;}
    return submit(list);
   };
   return device;
  };
 });
}

async function first(page,expected,which='first'){
 await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready'&&window.__presentationProbe?.first);
 await page.evaluate(async()=>{await new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r)));await window.__presentationDevice.queue.onSubmittedWorkDone();});
 const actual=await page.evaluate(()=>{const c=document.querySelector('canvas'),r=c.getBoundingClientRect();return{...window.__presentationProbe,cssSize:[r.width,r.height],size:[c.width,c.height],viewport:[innerWidth,innerHeight]};});
 assert.deepEqual(actual.errors,[]);assert.deepEqual(actual.viewport,expected.viewport);
 assert.deepEqual(actual.size,actual.cssSize.map(v=>Math.max(1,Math.round(v*expected.density))));
 const frame=actual[which];assert(frame);const view=frame.view;assert.equal(view.length,28);assert.deepEqual(view.slice(16,18),actual.size);
 for(const [index,value]of [[18,expected.density],[19,expected.opacity],[20,0],[21,expected.opacity],[22,expected.lit?1:0],[23,expected.backfaces?1:0]])assert(Math.abs(view[index]-value)<1e-6,'Actual first submitted view field'+index);
 const draws=frame.draws;
 assert.equal(draws.some(d=>d.pipeline?.grid),expected.grid,'Actual first submitted grid draw');
 assert.equal(draws.some(d=>d.pass==='finite scene ink'&&d.pipeline?.marker),expected.vertices,'Actual first submitted mesh-marker draw');
 assert.equal(draws.some(d=>d.pipeline?.outline),expected.outlines,'Actual first submitted outline composite');
 return actual;
}

module.exports={observe,first};
