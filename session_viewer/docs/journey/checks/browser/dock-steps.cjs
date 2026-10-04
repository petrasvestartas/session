const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
const ids=['03c-history','03ca-typing','03cb-submit','03cc-edit','03cd-vocabulary','03ce-complete','03cf-accept','03cg-keys','03ch-popup','03ci-browse','03cj-prepare','03ck-rects','03ck-pointer','03cl-wheel','03c-layout','03cn-handoff','03d-input'];
module.exports=async(page,{step})=>{
 const stage=ids.indexOf(step.id);assert(stage>=0);
 const canvas=page.locator('canvas');
 const state=()=>canvas.evaluate(c=>JSON.parse(c.getAttribute('data-command-ui')||'{}'));
 const text=async value=>page.waitForFunction(value=>{const c=document.querySelector('canvas');return (JSON.parse(c.getAttribute('data-command-ui')||'{}').command??c.getAttribute('data-command'))===value;},value);
 const history=async()=>canvas.evaluate(c=>JSON.parse(c.getAttribute('data-command-ui')||'{}').history??JSON.parse(c.getAttribute('data-history')||'[]'));
 const start=await canvas.evaluate(c=>c.toDataURL());
 const before=PNG.sync.read(Buffer.from(start.split(',')[1],'base64'));
 if(stage===0){let ink=0;for(let y=before.height-200;y<before.height;y++)for(let x=0;x<280;x++){const i=(y*before.width+x)*4;if([...before.data.subarray(i,i+3)].every(c=>c<180))ink++;}assert(ink>300,'Retained history must render actual text');return;}
 await page.mouse.click(8,8);await page.keyboard.type('g');await text('g');await page.keyboard.press('Backspace');await text('');
 if(stage>=3){await page.keyboard.type('abc');await page.keyboard.press('ControlOrMeta+A');await page.keyboard.type('x');await text('x');await page.keyboard.press('Escape');await text('');}
 if(stage>=2){
  const previous=(await history()).length;
  await page.keyboard.type('Help');await page.keyboard.press('Enter');await text('');
  assert.equal((await history()).length,previous+1,'Enter submits once');
  if(stage>=4)assert.equal((await history()).at(-1),'> Help\nHelp');
  await page.keyboard.press('Enter');assert.equal((await history()).length,previous+1,'Empty Enter adds nothing');
 }
 if(stage>=5){
  await page.keyboard.type('He');await text('Help');await page.keyboard.press('Backspace');await text('He');
  if(stage>=6){await page.keyboard.press('Tab');await text('Help ');await page.keyboard.press('Escape');await text('');}
  else {await page.keyboard.press('ControlOrMeta+A');await page.keyboard.press('Backspace');await text('');}
 }
 if(stage>=9){await page.keyboard.type('He');await text('Help');await page.keyboard.press('ArrowDown');await text('Help');await page.keyboard.press('Escape');await text('');}
 if(stage>=12){
  await page.keyboard.type('He');await text('Help');const ui=await state(),choice=ui.controls.find(c=>c.key==='command/completion/Help');assert(choice);
  const r=choice.rect;await page.mouse.click((r[0]+r[2])/2,(r[1]+r[3])/2);await text('');
  const retained=await history(),fold=(await state()).controls.find(c=>c.key==='command/collapse');assert(fold);
  await page.mouse.click((fold.rect[0]+fold.rect[2])/2,(fold.rect[1]+fold.rect[3])/2);assert.deepEqual(await history(),retained);
  await page.mouse.click(8,8);await page.keyboard.type('g');await text('g');await page.keyboard.press('Escape');await text('');
 }
 if(stage>=12){
  const r=(await state()).controls.find(c=>c.key==='command/input').rect;
  await page.mouse.move((r[0]+r[2])/2,(r[1]+r[3])/2);await page.mouse.down();
  assert.equal((await state()).pointer_owned,true,'Field press owns its pointer');
  await canvas.evaluate(c=>c.dispatchEvent(new PointerEvent('pointercancel',{bubbles:true,pointerId:1,clientX:10,clientY:10})));
  assert.equal((await state()).pointer_owned,false,'Cancellation releases ownership');await page.mouse.up();
  if(stage===16){
   await page.mouse.down();assert.equal((await state()).pointer_owned,true);
   await canvas.evaluate(c=>c.dispatchEvent(new PointerEvent('lostpointercapture',{bubbles:true,pointerId:1,clientX:10,clientY:10})));
   assert.equal((await state()).pointer_owned,false,'Lost capture also releases ownership');await page.mouse.up();
  }
 }
 if(stage>=13){
  const field=(await state()).controls.find(c=>c.key==='command/input').rect;
  await page.mouse.move((field[0]+field[2])/2,(field[1]+field[3])/2);
  await page.mouse.wheel(0,120);await text('Help');await page.keyboard.press('Escape');await text('');
 }
 const after=PNG.sync.read(Buffer.from((await canvas.evaluate(c=>c.toDataURL())).split(',')[1],'base64'));
 assert.deepEqual(after.data.subarray(0,440*before.width*4),before.data.subarray(0,440*before.width*4),'Command input must preserve scene pixels');
 assert.equal(await page.locator('button,input:not([type=file])').count(),0);
 // Finish with the lesson's visible result, after the broader input checks.
 if(stage===5){await page.keyboard.type('He');await text('Help');}
 if(stage===6){await page.keyboard.type('He');await page.keyboard.press('Tab');await text('Help ');}
 if(stage===7){await page.keyboard.type('He');await page.keyboard.press('Tab');await page.keyboard.press('Enter');await text('');}
 if(stage>=8){await page.keyboard.type('He');await text('Help');}

};
