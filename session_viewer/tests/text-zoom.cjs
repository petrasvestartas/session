/** Actual Chrome page zoom through its settings UI, in a disposable browser profile. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const os=require('node:os');
const path=require('node:path');
const {chromium}=require('playwright');
/** Exercise actual Chrome page zoom in a disposable profile. */
async function main(){
  const profile=await fs.mkdtemp(path.join(os.tmpdir(),'viewer-text-zoom-'));
  const output=process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-text-zoom';await fs.mkdir(output,{recursive:true});
  const context=await chromium.launchPersistentContext(profile,{viewport:null,executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]')});
  try{
    const page=await context.newPage(),settings=await context.newPage();
    await settings.goto('chrome://settings/appearance');
    await settings.locator('#zoomLevel').selectOption('1');
    await page.goto(new URL('text-quality.html',process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
    await page.waitForFunction(fixtureReady);
    const baseline=await page.evaluate(deviceScale),reports=[];
    for(const zoom of [1,1.25,1.5,2]){
      await settings.bringToFront();await settings.locator('#zoomLevel').selectOption(String(zoom));
      await page.bringToFront();
      await page.waitForFunction(zoomApplied,baseline*zoom);
      const report=await page.evaluate(readReport);
      assert.equal(report.stats.shape_count,5,'real page zoom must keep logical shaping unchanged');
      assert.equal(report.stats.missing_glyphs,0);
      assert(report.lineMetrics.every(compatibleWidth),'same-font metrics remain stable at page zoom');
      const png=await page.locator('#text-quality-canvas').screenshot({path:path.join(output,`text-zoom-${zoom}.png`)});
      const white=await page.evaluate(countWhite,png.toString('base64'));
      assert(white>100,'zoomed text must draw actual opaque white pixels');
      reports.push({pageZoom:zoom,baseDevicePixelRatio:baseline,report,whitePixels:white});
    }
    await fs.writeFile(path.join(output,'zoom.json'),JSON.stringify(reports,null,2));
    console.log('PASS actual Chrome page zoom 100%,125%,150%,200%: framebuffer follows actual DPR, logical shaping and same-font metrics stable');
  }finally{await context.close();await fs.rm(profile,{recursive:true,force:true});}
}
/** Count rendered white interiors inside a real page-zoom screenshot. */
async function countWhite(base64){
  const image=await createImageBitmap(await(await fetch('data:image/png;base64,'+base64)).blob());
  const canvas=document.createElement('canvas');canvas.width=image.width;canvas.height=image.height;
  const ctx=canvas.getContext('2d');ctx.drawImage(image,0,0);image.close();const pixels=ctx.getImageData(0,0,canvas.width,canvas.height).data;
  let count=0;for(let i=0;i<pixels.length;i+=4)if(pixels[i]>240 && pixels[i+1]>240 && pixels[i+2]>240)count++;return count;
}
/** Confirm the GPU fixture finished explicit font loading and initial preparation. */
function fixtureReady(){return window.textQuality?.report;}
/** Read the actual browser device scale without emulation overrides. */
function deviceScale(){return devicePixelRatio;}
/** Wait until both Chrome and the fixture adopt the requested page zoom. */
function zoomApplied(expected){return Math.abs(devicePixelRatio-expected)<0.01 && Math.abs(window.textQuality.report.effectiveScale-devicePixelRatio)<0.01;}
/** Retrieve the fixture's logical metrics and resource counters. */
function readReport(){return window.textQuality.report;}
/** Limit only logical metric drift, not differences between rasterizers. */
function compatibleWidth(line){return Math.abs(line.browser-line.shaper)<=0.2;}
/** Preserve command failure status after the disposable profile is released. */
function fail(error){console.error(error);process.exitCode=1;}
main().catch(fail);
