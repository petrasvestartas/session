/** Production canvas regression: persistent document text and centered selected source names. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');

/** Read the opt-in submitted state; this hook cannot mutate selection or rendering. */
function readState() { return JSON.parse(document.getElementById('canvas').dataset.viewerInspection); }
/** Wait until source upload and persistent annotation have both reached a rendered frame. */
function ready() {
  const raw=document.getElementById('canvas')?.dataset.viewerInspection;
  if(!raw)return false;
  const state=JSON.parse(raw);
  return state.objects===7 && state.text_labels?.length>0 && !state.pick_busy;
}
/** Wait for asynchronous picking to drain after an input event. */
function settled() { const state=JSON.parse(document.getElementById("canvas").dataset.viewerInspection);return !state.pick_busy; }
/** Project a source-world anchor using the same rebased frame as the production text lane. */
function project(state, world) {
  const point=[world[0]-state.origin[0],world[1]-state.origin[1],world[2]-state.origin[2],1];
  const clip=[];
  for(let row=0;row<4;row++)clip.push(state.mvp[row]*point[0]+state.mvp[4+row]*point[1]+state.mvp[8+row]*point[2]+state.mvp[12+row]);
  return [(clip[0]/clip[3]*0.5+0.5)*state.canvas[0],(0.5-clip[1]/clip[3]*0.5)*state.canvas[1]];
}
/** Find one source family without assuming protobuf serialization order. */
function family(cases,kind) {for(const item of cases)if(item.kind===kind)return item;throw Error('missing fixture '+kind);}
/** Find the selected source label separately from stable document annotations. */
function selectedLabel(state) {for(const label of state.text_labels)if(label.id===0)return label;return null;}
/** Count black backplate and white glyph pixels inside a predicted nameplate box. */
async function pixelCounts(request) {
  const bitmap=await createImageBitmap(await (await fetch('data:image/png;base64,'+request.png)).blob());
  const canvas=document.createElement('canvas');canvas.width=bitmap.width;canvas.height=bitmap.height;
  const context=canvas.getContext('2d');context.drawImage(bitmap,0,0);bitmap.close();
  const data=context.getImageData(0,0,canvas.width,canvas.height).data;
  let black=0,white=0,total=0;
  for(let y=Math.max(0,Math.ceil(request.box[1]));y<Math.min(canvas.height,Math.floor(request.box[3]));y++){
    for(let x=Math.max(0,Math.ceil(request.box[0]));x<Math.min(canvas.width,Math.floor(request.box[2]));x++){
      const offset=(y*canvas.width+x)*4;total++;
      if(data[offset]<8&&data[offset+1]<8&&data[offset+2]<8)black++;
      if(data[offset]>240&&data[offset+1]>240&&data[offset+2]>240)white++;
    }
  }
  return {black,white,total};
}
/** Verify source/style state and actual black/white pixels at its projected source center. */
async function captureLabel(page,state,label,dpr,output,name) {
  const scale=label.id===0?0.75:1;
  assert.equal(label.placement,'nameplate');assert.equal(label.font_size,18*scale);assert.equal(label.line_height,26*scale);
  assert.deepEqual(label.color,[255,255,255,255]);assert.deepEqual(label.padding,label.id===0?[12.75,3]:[6,4]);assert.equal(label.rounded,label.id===0);
  if(label.rounded)assert.ok(label.padding[0]>=label.line_box[1]/2+label.padding[1], 'the complete shaped line must fit between the rounded caps');
  const center=project(state,label.world);
  const width=(label.line_box[0]+2*label.padding[0])*dpr,height=(label.line_box[1]+2*label.padding[1])*dpr;
  const png=await page.locator('#canvas').screenshot({path:path.join(output,`${name}-dpr${dpr}.png`)});
  const pixels=await page.evaluate(pixelCounts,{png:png.toString('base64'),box:[center[0]-width/2+1,center[1]-height/2+1,center[0]+width/2-1,center[1]+height/2-1]});
  assert.ok(pixels.black>pixels.total*0.45,'source-centered plate must be opaque black');
  assert.ok(pixels.white>40*dpr*dpr,'source-centered name must have actual white glyph interiors');
  return {label,center,pixels};
}
/** Exercise both required source names, parent replacement, camera motion, F10 and Escape. */
async function main() {
  const fixture=process.env.VIEWER_INTERACTION_FIXTURE || '/tmp/viewer-interaction.pb';
  const bytes=await fs.readFile(fixture),cases=JSON.parse(await fs.readFile(fixture+'.json','utf8'));
  const output=process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-nameplate-scene';await fs.mkdir(output,{recursive:true});
  const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});
  const reports=[];
  try{for(const dpr of [1,2]){
    const context=await browser.newContext({viewport:{width:1400,height:900},deviceScaleFactor:dpr});const page=await context.newPage(),errors=[];
    page.on('pageerror',function failure(error){errors.push(String(error))});
    page.on('console',function failure(message){if(message.type()==='error'&&!message.text().includes('{{__trunk_address__}}'))errors.push(message.text())});
    await page.route('**/favicon.ico',function favicon(route){return route.fulfill({status:204})});
    await page.route('**/view_local.yaml',function manifest(route){return route.fulfill({status:200,contentType:'application/yaml',body:'name: Nameplate regression\nitems:\n  - file: "pb/interaction-fixture.pb"\n'})});
    await page.route('**/pb/interaction-fixture.pb',function source(route){return route.fulfill({status:200,contentType:'application/octet-stream',body:bytes})});
    await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);
    await page.bringToFront();await page.waitForFunction(ready,null,{timeout:60000});await page.locator('#canvas').focus();
    await page.keyboard.press('5');await page.waitForTimeout(100);
    let state=await page.evaluate(readState);let persistent;
    for(const label of state.text_labels)if(label.id>0){persistent=label;break;}
    assert.ok(persistent,'an explicit document name is visible before any selection');
    const history=[await captureLabel(page,state,persistent,dpr,output,'persistent')];
    await page.keyboard.press('t');await page.waitForTimeout(100);
    state=await page.evaluate(readState);
    assert.equal(selectedLabel(state),null,'T before selection changes the persistent preference');
    assert.ok(state.text_labels.some(function documentLabel(label){return label.id>0;}));
    for(const kind of ['mesh','brep']){
      const spec=family(cases,kind),at=project(state,spec.pick);
      await page.mouse.click(at[0]/dpr,at[1]/dpr);await page.waitForTimeout(100);await page.waitForFunction(settled);
      state=await page.evaluate(readState);
      assert.equal(selectedLabel(state),null,'new source selections retain the disabled name preference');
      const row=state.selected;assert.notEqual(row,null);
      await page.keyboard.press('t');await page.waitForTimeout(100);state=await page.evaluate(readState);
      assert.equal(state.selected,row,'T preserves the selected source row');
      assert.ok(state.text_labels.some(function documentLabel(label){return label.id>0;}),'T preserves persistent document labels');
      const label=selectedLabel(state);assert.ok(label);
      assert.equal(label.text,`interaction ${kind}`,'the label uses the geometry name, not its file/GUID');
      assert.ok(Math.abs(label.world[0]-spec.pick[0])<0.001);assert.ok(Math.abs(label.world[1]-spec.pick[1])<0.001);
      history.push(await captureLabel(page,state,label,dpr,output,kind));
      const shapes=state.text.shape_count;
      await page.mouse.wheel(0,-100);await page.waitForTimeout(100);state=await page.evaluate(readState);
      assert.equal(state.text.shape_count,shapes,'camera changes reuse the source-name shape');
      history.push(await captureLabel(page,state,selectedLabel(state),dpr,output,kind+'-zoom'));
      await page.keyboard.press('t');await page.waitForTimeout(100);state=await page.evaluate(readState);
      assert.equal(selectedLabel(state),null,'T hides the selected name');assert.equal(state.selected,row);
      assert.ok(state.text_labels.some(function documentLabel(label){return label.id>0;}));
      await page.keyboard.press('F10');await page.waitForTimeout(100);await page.keyboard.press('Escape');await page.waitForTimeout(100);
      state=await page.evaluate(readState);assert.equal(state.markers,0);assert.equal(state.control_segments,0);
      assert.equal(selectedLabel(state),null,'F10 and Escape preserve the disabled name preference');
      assert.equal(state.selected,row,'Escape restores the same parent');
    }
    await page.keyboard.press('t');await page.waitForTimeout(100);state=await page.evaluate(readState);
    assert.equal(selectedLabel(state).text,'interaction brep','T restores the final source name');
    history.push(await captureLabel(page,state,selectedLabel(state),dpr,output,'toggle-restored'));
    assert.deepEqual(errors,[]);reports.push({dpr,history,state});await context.close();
  }}finally{await browser.close();}
  await fs.writeFile(path.join(output,'scene-nameplates.json'),JSON.stringify(reports,null,2)+'\n');
  console.log('PASS main canvas: persistent text, actual centered source names, white/black pixels, DPR1/2, zoom/F10/Esc and persistent T preference');
}
main().catch(function failed(error){console.error(error);process.exitCode=1});
