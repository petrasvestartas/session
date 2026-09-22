/** Focus, pointer cancellation, hidden canvas and unchanged-framebuffer DPR regression. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const {chromium}=require('playwright');
function snapshot(){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return raw?JSON.parse(raw):null;}
function pointFixture(){const xyz=Buffer.alloc(27);xyz[0]=25;xyz.writeDoubleLE(0,1);xyz[9]=33;xyz.writeDoubleLE(0,10);xyz[18]=41;xyz.writeDoubleLE(0,19);const p=Buffer.concat([Buffer.from([10,1,112]),xyz,Buffer.from([49,0,0,0,0,0,0,32,64])]);const objects=Buffer.concat([Buffer.from([26,p.length]),p]);return Buffer.concat([Buffer.from([10,1,115,26,objects.length]),objects]);}
function cancelPointer(){document.querySelector('#canvas').dispatchEvent(new PointerEvent('pointercancel',{pointerType:'mouse',pointerId:1,bubbles:true}));}
function focusInput(){const input=document.createElement('input');input.id='focus-probe';input.style='position:fixed;top:0;left:0';document.body.append(input);input.focus();}
function canvasVisible(visible){document.querySelector('#canvas').style.display=visible?'block':'none';}
async function main(){
 const output=process.env.VIEWER_TEST_OUTPUT||'/tmp/session-viewer-lifecycle';await fs.mkdir(output,{recursive:true});
 const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:process.env.VIEWER_CHROME_ARGS?JSON.parse(process.env.VIEWER_CHROME_ARGS):[]});
 const context=await browser.newContext({viewport:{width:1000,height:700},deviceScaleFactor:1});const page=await context.newPage();const errors=[];
 function onError(error){errors.push(String(error));}
 async function manifest(route){await route.fulfill({status:200,contentType:'application/toml',body:'name="lifecycle"\n[[items]]\nfile="pb/lifecycle.pb"\ndisplay_only=true\n'});}
 async function geometry(route){await route.fulfill({status:200,contentType:'application/octet-stream',body:pointFixture()});}
 page.on('pageerror',onError);await page.route('**/scenes/lifecycle.toml',manifest);await page.route('**/pb/lifecycle.pb',geometry);
 try{
  await page.goto((process.env.VIEWER_URL||'http://localhost:8770/')+'?scene=lifecycle.toml&data=off&inspect=1');await page.bringToFront();
  await page.waitForFunction(function ready(){return JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection')||'{}').objects===1;},null,{timeout:60000});
  const initial=await page.evaluate(snapshot);await page.evaluate(focusInput);await page.keyboard.press('7');await page.waitForTimeout(100);
  assert.deepEqual((await page.evaluate(snapshot)).mvp,initial.mvp,'typing focus must preserve camera');
  await page.locator('#canvas').focus();await page.mouse.move(500,350);await page.mouse.down({button:'right'});await page.mouse.move(560,370);await page.waitForTimeout(100);
  await page.evaluate(cancelPointer);await page.waitForTimeout(100);const canceled=await page.evaluate(snapshot);
  await page.mouse.move(700,500);await page.waitForTimeout(100);await page.mouse.up({button:'right'});
  assert.deepEqual((await page.evaluate(snapshot)).mvp,canceled.mvp,'canceled pointer must stop orbit');
  await page.mouse.move(500,350);await page.mouse.down();await page.evaluate(cancelPointer);await page.mouse.up();await page.waitForTimeout(100);
  assert.equal((await page.evaluate(snapshot)).selected,null,'canceled click must not select');
  await page.evaluate(canvasVisible,false);await page.waitForTimeout(150);const hidden=await page.evaluate(snapshot);await page.waitForTimeout(300);
  assert.equal((await page.evaluate(snapshot)).frames,hidden.frames,'hidden canvas must stop frame submission');
  await page.evaluate(canvasVisible,true);await page.waitForTimeout(150);
  const cdp=await context.newCDPSession(page);await cdp.send('Emulation.setDeviceMetricsOverride',{width:500,height:350,deviceScaleFactor:2,mobile:false});
  await page.waitForFunction(function scaled(){const state=JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection')||'{}');return state.logical_canvas?.[0]===500;});
  const scaled=await page.evaluate(snapshot);assert.deepEqual(scaled.canvas,[1000,700]);assert.deepEqual(scaled.logical_canvas,[500,350]);
  await page.screenshot({path:output+'/lifecycle.png'});assert.deepEqual(errors,[]);
  await fs.writeFile(output+'/lifecycle.json',JSON.stringify({initial,canceled,hidden,scaled,errors},null,2));console.log('lifecycle: input focus, pointer cancel, hidden canvas and same-framebuffer DPR passed');
 }finally{await browser.close();}
}
main().catch(function failed(error){console.error(error);process.exitCode=1;});
