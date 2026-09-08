/** Repeatable browser scene inventory and navigation measurements; see tests/README.md. */
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');

function instrumentation() {
  window.viewerProbe = {longTasks:[],frames:[],previous:0,phase:'loading',scenePosted:false,firstGeometrySubmissionMs:null};
  for(const method of ['log','info','debug']) {
    const original=console[method];
    function observeConsole(...values) {
      if(values.join(' ').includes('scene posted')) window.viewerProbe.scenePosted=true;
      return Reflect.apply(original,console,values);
    }
    console[method]=observeConsole;
  }
  const observer = new PerformanceObserver(recordTasks);
  observer.observe({type:'longtask',buffered:true});
  function recordTasks(list) { for(const entry of list.getEntries()) window.viewerProbe.longTasks.push({start:entry.startTime,duration:entry.duration}); }
  function frame(now) {
    const probe=window.viewerProbe;
    const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');
    if(probe.firstGeometrySubmissionMs===null && raw && JSON.parse(raw).objects>0) probe.firstGeometrySubmissionMs=now;
    if(probe.previous && probe.phase==='navigation') probe.frames.push(now-probe.previous);
    probe.previous=now;
    requestAnimationFrame(frame);
  }
  requestAnimationFrame(frame);
}
function snapshot() {
  const encoded=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');
  return encoded?JSON.parse(encoded):null;
}
function startNavigation(){window.viewerProbe.phase='navigation';window.viewerProbe.frames=[];}
function observations(){
  window.viewerProbe.phase='idle';
  return {probe:window.viewerProbe,resources:performance.getEntriesByType('resource').map(resourceRecord),jsHeap:performance.memory?{used:performance.memory.usedJSHeapSize,total:performance.memory.totalJSHeapSize}:null};
  function resourceRecord(entry){return {url:entry.name,start:entry.startTime,duration:entry.duration,transferBytes:entry.transferSize,encodedBytes:entry.encodedBodySize,decodedBytes:entry.decodedBodySize};}
}
function percentile(values,fraction){if(!values.length)return null;const sorted=values.toSorted((a,b)=>a-b);return sorted[Math.min(sorted.length-1,Math.floor(sorted.length*fraction))];}

async function main(){
  const output=process.env.VIEWER_TEST_OUTPUT||'/tmp/session-viewer-scenes';await fs.mkdir(output,{recursive:true});
  const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:process.env.VIEWER_CHROME_ARGS?JSON.parse(process.env.VIEWER_CHROME_ARGS):[]});
  const scenes=(process.env.VIEWER_SCENES||'view_lines,view_lines_rotated,view_meshes,view_mixed,view_mixed_solids,view_pointclouds,view_live').split(',');
  const summary=[];
  try{
    for(const scene of scenes){
      const context=await browser.newContext({viewport:{width:1400,height:900},deviceScaleFactor:1});
      await context.addInitScript(instrumentation);
      const page=await context.newPage();
      for(const cache of ['cold','warm']){
        const messages=[];const errors=[];let ready=false;
        function capture(message){const entry={at:Date.now(),type:message.type(),text:message.text()};messages.push(entry);if(entry.type==='error'&&!entry.text.includes('WebSocket')&&!entry.text.includes('404'))errors.push(entry.text);}
        function pageError(error){errors.push(String(error));}
        page.on('console',capture);page.on('pageerror',pageError);
        const started=Date.now();
        await page.goto((process.env.VIEWER_URL||'http://localhost:8770/')+'?scene='+scene+'.'+(process.env.VIEWER_SCENE_EXTENSION||'toml')+'&inspect=1'+(process.env.VIEWER_EXTRA_QUERY||''),{waitUntil:'domcontentloaded',timeout:90000});
        await page.bringToFront();
        const deadline=Date.now()+120000;
        while(!ready && Date.now()<deadline && !errors.some(gpuError)) {
          await page.waitForTimeout(200);
          ready=await page.evaluate(sceneReady);
        }
        await page.waitForTimeout(700);
        const first=await page.evaluate(snapshot);
        const loadMs=Date.now()-started;
        if(first?.objects){
          await page.locator('#canvas').focus();
          await page.keyboard.press('7');
          await page.mouse.move(700,450);
          await page.evaluate(startNavigation);
          await page.mouse.down({button:'right'});
          for(let step=0;step<80;step++){await page.mouse.move(700+120*Math.sin(step/15),450+70*Math.cos(step/15));await page.waitForTimeout(16);}
          await page.mouse.up({button:'right'});
          await page.mouse.down({button:'middle'});
          for(let step=0;step<30;step++){await page.mouse.move(700+step*2,450+step);await page.waitForTimeout(16);}
          await page.mouse.up({button:'middle'});
          for(let step=0;step<10;step++){await page.mouse.wheel(0,step<5?-100:100);await page.waitForTimeout(32);}
        }
        const data=await page.evaluate(observations);
        const active=await page.evaluate(snapshot);
        await page.waitForTimeout(1200);
        const idle=await page.evaluate(snapshot);
        await page.screenshot({path:path.join(output,scene+'-'+cache+'.png')});
        const report={scene,cache,ready,loadToSettledMs:loadMs,first,active,idle,idleExtraFrames:idle&&active?idle.frames-active.frames:null,frameMedianMs:percentile(data.probe.frames,.5),frameP95Ms:percentile(data.probe.frames,.95),...data,messages,errors};
        await fs.writeFile(path.join(output,scene+'-'+cache+'.json'),JSON.stringify(report,null,2));
        summary.push({scene,cache,ready,objects:first?.objects,loadToSettledMs:loadMs,frameMedianMs:report.frameMedianMs,frameP95Ms:report.frameP95Ms,idleExtraFrames:report.idleExtraFrames,errors});
        console.log(JSON.stringify(summary.at(-1)));
        page.off('console',capture);page.off('pageerror',pageError);
      }
      await context.close();
    }
  }finally{await browser.close();await fs.writeFile(path.join(output,'summary.json'),JSON.stringify(summary,null,2));}
}
function gpuError(error){return /WebGPU error|device lost|panicked|OutOfMemory/.test(error);}
function sceneReady(){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return Boolean(window.viewerProbe.scenePosted && raw && JSON.parse(raw).objects>0);}
main().catch(error=>{console.error(error);process.exitCode=1;});
// Viewer: NODE_PATH=/tmp/viewer-browser-test/node_modules node tests/scenes.cjs
// Browser rAF intervals describe main-thread/navigation cadence, not GPU timestamp-query duration.
