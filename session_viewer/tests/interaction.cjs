/** Real canvas gestures over deterministic source geometry. See tests/README.md. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const {chromium}=require('playwright');
/** Load the generated source fixture and verify both device scales. */
async function main(){
  const fixture=process.env.VIEWER_INTERACTION_FIXTURE || '/tmp/viewer-interaction.pb';
  const bytes=await fs.readFile(fixture),cases=JSON.parse(await fs.readFile(fixture+'.json','utf8'));
  const output=process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-interaction';await fs.mkdir(output,{recursive:true});
  const browser=await chromium.launch({executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]')});
  try{for(const dpr of [1,2])await run(browser,dpr,bytes,cases,output);}finally{await browser.close();}
}
/** Exercise real object, edge and control gestures over every required family. */
async function run(browser,dpr,bytes,cases,output){
  const context=await browser.newContext({viewport:{width:1400,height:900},deviceScaleFactor:dpr});
  const page=await context.newPage(),errors=[],history=[];
  page.on('pageerror',capturePageError);
  page.on('console',captureConsoleError);
  await page.route('**/view_local.yaml',serveManifest);
  await page.route('**/pb/interaction-fixture.pb',serveFixture);
  /** Retain page failures alongside the failing selection snapshot. */
  function capturePageError(error){errors.push(String(error));}
  /** Ignore only the unresolved development reload endpoint in static builds. */
  function captureConsoleError(message){if(message.type()==='error' && !message.text().includes('{{__trunk_address__}}'))errors.push(message.text());}
  /** Supply the deterministic local scene without modifying production assets. */
  function serveManifest(route){return route.fulfill({status:200,contentType:'application/yaml',body:'name: Interaction regression\nitems:\n  - file: "pb/interaction-fixture.pb"\n'});}
  /** Serve the generated source bytes through the real loader path. */
  function serveFixture(route){return route.fulfill({status:200,contentType:'application/octet-stream',body:bytes});}
  try{
    // The yellow-coverage oracles (three pure-yellow pixels) were measured with the 1.5 px pen;
    // the viewer's 1 px default leaves a Ctrl-selected solid edge almost entirely under the
    // black silhouette. Pin the oracle's pen through the same knob the native fixtures use.
    await page.goto(new URL('?data=off&inspect=1&thickness=1.5',process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href,{waitUntil:'networkidle'});
    await page.waitForFunction(sceneReady);
    await page.bringToFront();
    await page.locator('canvas').focus();
    await page.waitForFunction(function focusedCanvas(){return document.hasFocus() && document.activeElement?.id==='canvas';});
    await key(page,'5');
    assert.equal((await snapshot(page)).outlines,false,'surface silhouettes are off by default');
    await key(page,'o');
    assert.equal((await snapshot(page)).outlines,true,'O shows surface silhouettes');
    await key(page,'o');
    assert.equal((await snapshot(page)).outlines,false,'O hides them again');
    // Hide centered names to inspect short strokes; nameplate-scene.cjs checks their default display.
    await key(page,'t');
    await page.screenshot({path:path.join(output,`scene-dpr-${dpr}.png`)});
    const parents=new Set();
    for(const spec of cases){
      await key(page,'Escape');
      let state=await snapshot(page);
      await click(page,project(state,spec.pick));
      state=await snapshot(page);
      assert.notEqual(state.selected,null,`${spec.kind}: ordinary object click`);
      const parent=state.selected;parents.add(parent);
      assert.equal(state.identity[1],spec.guid,'source GUID survives object picking');
      assert.equal(state.selection,'Object');
      await visibleYellow(page,output,`${spec.kind}-object-dpr-${dpr}`);
      if(['mesh','surface','brep'].includes(spec.kind)){
        await page.keyboard.down('Shift');
        await click(page,project(state,spec.pick),true);
        await page.keyboard.up('Shift');
        state=await snapshot(page);
        assert.equal(state.selection.Face?.parent,parent,`${spec.kind}: Ctrl+Shift selects a source face`);
        const face=state.selection.Face.face;
        assert(Number.isInteger(face));
        await visibleYellow(page,output,`${spec.kind}-face-dpr-${dpr}`);
        await page.keyboard.down('Shift');
        await click(page,project(state,spec.pick),true);
        await page.keyboard.up('Shift');
        assert.equal((await snapshot(page)).selection.Face?.face,face,'source face identity remains stable');
        await page.keyboard.down('Shift');
        await click(page,project(state,spec.edge),true);
        await page.keyboard.up('Shift');
        assert.equal((await snapshot(page)).selection.Edge?.parent,parent,'Ctrl+Shift keeps edge precedence');
        await key(page,'Escape');
      }
      if(spec.edge && ['mesh','surface','brep'].includes(spec.kind)){
        await click(page,project(state,spec.edge),true);
        state=await snapshot(page);
        assert.equal(state.selection.Edge?.parent,parent,`${spec.kind}: Ctrl must select a source edge`);
        assert(Number.isInteger(state.selection.Edge.edge));
        const identity=state.selection.Edge.edge;
        await click(page,project(state,spec.edge),true);
        assert.equal((await snapshot(page)).selection.Edge?.edge,identity,'same source edge must retain identity');
        await visibleYellow(page,output,`${spec.kind}-edge-dpr-${dpr}`);
      }
      if(spec.edge && ['line','polyline','curve'].includes(spec.kind)){
        await click(page,project(state,spec.edge),true);
        assert.equal((await snapshot(page)).selection,'Object','standalone curves expose source controls, not fabricated tessellation edges');
      }
      await key(page,'F10');state=await snapshot(page);
      assert.equal(state.selection.Controls?.parent,parent,`${spec.kind}: F10 enters controls for one parent`);
      assert.equal(state.selection.Controls.cloud,spec.kind==='cloud');
      assert(state.controls.length>=spec.minimum_controls,`${spec.kind}: original source controls must be available`);
      if(spec.kind!=='cloud')assert.equal(state.markers,state.controls.length);
      const initial=state;
      await key(page,'F10');state=await snapshot(page);
      assert.deepEqual(state.controls,initial.controls,'repeated F10 must not duplicate controls');
      assert.equal(state.markers,initial.markers);assert.equal(state.control_segments,initial.control_segments);
      let target;
      if(spec.kind==='cloud')target=project(state,spec.pick);
      else {
        let candidate=state.controls[0];
        for(const control of state.controls){if(control.position[2]>candidate.position[2])candidate=control;}
        target=project(state,candidate.position,true);
      }
      await click(page,target);state=await snapshot(page);
      assert.notEqual(state.selection.Controls?.selected,null,`${spec.kind}: visible source control must pick`);
      assert.equal(state.selection.Controls.parent,parent);
      if(spec.kind==='cloud')assert.equal(state.selection.Controls.selected.Point,12,'resident source point ID must survive the pick');
      await visibleYellow(page,output,`${spec.kind}-control-dpr-${dpr}`);
      history.push({kind:spec.kind,state});
      await key(page,'Escape');state=await snapshot(page);
      assert.equal(state.selection,'Object');assert.equal(state.selected,parent);
      assert.equal(state.markers,0);assert.equal(state.control_segments,0);
      assert.equal(state.pick_busy,false);
    }
    assert.equal(parents.size,cases.length,'each disjoint specimen must resolve its own parent');
    // A thin source mesh border: CSS tolerance must behave the same at both device scales.
    const line=findKind(cases,'brep');
    await key(page,'Escape');let state=await snapshot(page),at=project(state,line.edge);
    await click(page,[at[0],at[1]+5],true);state=await snapshot(page);
    assert(state.selection.Edge,'five CSS pixels from source edge must hit');
    await key(page,'Escape');await click(page,[at[0],at[1]+10],true);
    assert.equal((await snapshot(page)).selection,'Object','ten CSS pixels from source edge must miss');
    // Ctrl on a new source edge while controls are active clears the old parent's controls.
    const mesh=findKind(cases,'mesh');
    state=await snapshot(page);await click(page,project(state,mesh.edge),true);await key(page,'F10');
    const oldParent=(await snapshot(page)).selected;
    await click(page,at,true);state=await snapshot(page);
    assert(state.selection.Edge && state.selected!==oldParent,'Ctrl on another object replaces the specialized parent');
    assert.equal(state.markers,0);assert.equal(state.control_segments,0);
    await key(page,'Escape');
    assert.deepEqual(errors,[]);
    await fs.writeFile(path.join(output,`interaction-dpr-${dpr}.json`),JSON.stringify(history,null,2));
    console.log(`PASS interaction DPR ${dpr}: seven source kinds, object/edge/controls, identity, repeated F10, Esc, parent replacement and CSS tolerance`);
  }catch(error){await page.screenshot({path:path.join(output,`failure-dpr-${dpr}.png`)});await fs.writeFile(path.join(output,`failure-dpr-${dpr}.json`),JSON.stringify({error:String(error),errors,state:await snapshot(page)},null,2));throw error;}finally{await context.close();}
}
/** Retrieve the last submitted read-only state for assertions. */
async function snapshot(page){return page.evaluate(readSnapshot);}
/** Wait for a new submitted observation and completed picking, never a stale idle snapshot. */
async function settled(page,after){await page.waitForFunction(pickFinished,after);}
/** Send a canvas-focused keyboard action and await its resulting state. */
async function key(page,value){const before=(await snapshot(page)).submitted_at_ms;await page.keyboard.press(value);await settled(page,before);}
/** Send one ordinary or Ctrl click without duplicating application selection logic. */
async function click(page,at,ctrl=false){const before=(await snapshot(page)).submitted_at_ms;if(ctrl)await page.keyboard.down('Control');await page.mouse.click(at[0],at[1]);if(ctrl)await page.keyboard.up('Control');await settled(page,before);}
/** Apply a column-major homogeneous transform to a source point. */
function multiply(matrix,point){
  const result=[];
  for(let row=0;row<4;row++)result.push(matrix[row]*point[0]+matrix[4+row]*point[1]+matrix[8+row]*point[2]+matrix[12+row]*point[3]);
  return result;
}
/** Project original source coordinates into the fixed CSS test viewport. */
function project(state,point,local=false){
  const origin=state.origin || [0,0,0];
  const anchored=local?multiply(state.model,[...point,1]):[point[0]-origin[0],point[1]-origin[1],point[2]-origin[2],1];
  const clip=multiply(state.mvp,anchored);
  return [(clip[0]/clip[3]*0.5+0.5)*1400,(0.5-clip[1]/clip[3]*0.5)*900];
}
/** Capture selection output and require yellow coverage, including antialiased one-pixel strokes. */
async function visibleYellow(page,output,name){
  const png=await page.locator('canvas').screenshot({path:path.join(output,name+'.png')});
  const count=await page.evaluate(countYellow,png.toString('base64'));
  assert(count>=3,'selected object/edge/control must draw actual yellow coverage');
}
/** Count actual selection pixels in the browser-decoded canvas screenshot. */
async function countYellow(base64){
  const image=await createImageBitmap(await(await fetch('data:image/png;base64,'+base64)).blob());
  const canvas=document.createElement('canvas');canvas.width=image.width;canvas.height=image.height;
  const ctx=canvas.getContext('2d');ctx.drawImage(image,0,0);image.close();
  const data=ctx.getImageData(0,0,canvas.width,canvas.height).data;let yellow=0;
  for(let i=0;i<data.length;i+=4)if(data[i]>240 && data[i+1]>240 && data[i+2]<220)yellow++;
  return yellow;
}
/** Read only the production opt-in inspection snapshot inside the page. */
function readSnapshot(){return JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection'));}
/** Wait for all deterministic source families to pass the loader. */
function sceneReady(){const raw=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');return raw && JSON.parse(raw).objects>=7;}
/** Wait until the submitted selection readback has completed. */
function pickFinished(after){const raw=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');if(!raw)return false;const state=JSON.parse(raw);return state.submitted_at_ms>after && !state.pick_busy;}
/** Find a fixture family without relying on serialization row order. */
function findKind(cases,kind){for(const item of cases){if(item.kind===kind)return item;}throw new Error('Missing fixture family '+kind);}
/** Fail the process while preserving the test's diagnostic output. */
function fail(error){console.error(error);process.exitCode=1;}
main().catch(fail);
