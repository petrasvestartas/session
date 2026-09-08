/** Exact-source PDF coverage regression; no replacement fonts or reconstructed strings. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const source = 'pb/view_local_sheet_querschnitt.pb';
const expectedHash = 'e3c8ba85db2d908fc5c767ac0db27628b0121e77c1540c9232290c02969ef799';
/** Compare one source and camera under forced and automatic coverage. */
async function main() {
  const origin = process.env.VIEWER_URL || 'http://127.0.0.1:8770/';
  const output = process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-pdf-text-quality';
  await fs.mkdir(output,{recursive:true});
  const bytes = Buffer.from(await (await fetch(new URL(source,origin))).arrayBuffer());
  assert.equal(crypto.createHash('sha256').update(bytes).digest('hex'),expectedHash,'update the fixture/crops deliberately if its source changes');
  const browser = await chromium.launch({executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]')});
  const results = {};
  try {
    for (const mode of ['1','auto']) {
      const page = await browser.newPage({viewport:{width:1600,height:1000},deviceScaleFactor:1});
      const errors=[],logs=[];
      page.on('pageerror',capturePageError);
      page.on('console',captureConsoleMessage);
      await page.route('**/view_local.yaml',serveManifest);
      /** Record page failures without losing the same-source comparison log. */
      function capturePageError(error){errors.push(String(error));}
      /** Retain adapter/coverage messages and reject actual application errors. */
      function captureConsoleMessage(message){logs.push(message.text());if(message.type()==='error' && !message.text().includes('{{__trunk_address__}}'))errors.push(message.text());}
      await page.goto(new URL('?data=off'+(mode==='1'?'&msaa=1':''),origin).href,{waitUntil:'networkidle'});
      await page.waitForFunction(canvasReady);
      await page.waitForTimeout(500);
      await page.locator('canvas').focus();
      await page.keyboard.press('5');
      await page.mouse.move(800,500);
      for(let step=0;step<16;step++){await page.mouse.wheel(0,-100);await page.waitForTimeout(30);}
      await page.waitForTimeout(300);
      const png = await page.screenshot({path:path.join(output,`pdf-${mode}.png`)});
      const crops=[{x:466,y:59,width:98,height:24},{x:841,y:112,width:54,height:24},{x:502,y:337,width:40,height:22}];
      results[mode] = await page.evaluate(measure,{base64:png.toString('base64'),crops});
      for(let i=0;i<crops.length;i++)await page.screenshot({path:path.join(output,`pdf-${mode}-text-${i}.png`),clip:crops[i]});
      assert.deepEqual(errors,[]);
      if(mode==='auto')assert(logs.some(coverageEnabled),'sheet-only automatic policy must enable coverage');
      await fs.writeFile(path.join(output,`pdf-${mode}.log`),logs.join('\n'));
      await page.close();
    }
    for(let i=0;i<results.auto.length;i++) {
      const before=results['1'][i],after=results.auto[i];
      assert(before.ink>20 && after.ink>20,'normal-size text must really render');
      assert(after.partial>before.partial+10,'four samples must restore partial glyph coverage');
      assert(Math.abs(after.mass-before.mass)/before.mass<0.2,'coverage must not thicken or thin the whole label excessively');
      assert(Math.abs(after.center-before.center)<1,'original word placement must remain within one raster pixel');
      for(let bound=0;bound<after.bounds.length;bound++)assert(Math.abs(after.bounds[bound]-before.bounds[bound])<=1,'original glyph extent must remain within one coverage pixel');
    }
    await fs.writeFile(path.join(output,'metrics.json'),JSON.stringify({source,sha256:expectedHash,viewport:[1600,1000],dpr:1,results},null,2));
    console.log('PASS exact PDF outlines: partial coverage restored; normal-size extents, mass and word placement retained');
  } finally {await browser.close();}
}
/** Measure glyph mass, extent and position from actual browser screenshot pixels. */
async function measure({base64,crops}) {
  const blob=await (await fetch('data:image/png;base64,'+base64)).blob();
  const image=await createImageBitmap(blob);
  const canvas=document.createElement('canvas');canvas.width=image.width;canvas.height=image.height;
  const ctx=canvas.getContext('2d');ctx.drawImage(image,0,0);image.close();
  const measurements=[];
  for(const {x,y,width,height} of crops){
    const pixels=ctx.getImageData(x,y,width,height).data;
    let ink=0,partial=0,mass=0,moment=0,left=width,right=0,top=height,bottom=0;
    for(let row=0;row<height;row++)for(let col=0;col<width;col++) {
      const index=4*(row*width+col),r=pixels[index],g=pixels[index+1],b=pixels[index+2];
      if(r!==g || g!==b || r===255)continue;
      const weight=1-r/255;ink++;mass+=weight;moment+=col*weight;
      if(r>0)partial++;
      left=Math.min(left,col);right=Math.max(right,col);top=Math.min(top,row);bottom=Math.max(bottom,row);
    }
    measurements.push({ink,partial,mass,center:moment/mass,bounds:[left,top,right,bottom]});
  }
  return measurements;
}
/** Serve the exact PDF source through the normal manifest loader. */
function serveManifest(route){return route.fulfill({status:200,contentType:'application/yaml',body:`name: PDF outline regression\nitems:\n  - file: ${source}\n    display_only: true\n`});}
/** Wait for actual canvas allocation before sending camera input. */
function canvasReady(){return document.querySelector('canvas')?.width>100;}
/** Recognize the renderer's automatic coverage policy diagnostic. */
function coverageEnabled(message){return message.includes('msaa: 4x');}
/** Report a failed source/coverage assertion to the command runner. */
function fail(error){console.error(error);process.exitCode=1;}
main().catch(fail);
