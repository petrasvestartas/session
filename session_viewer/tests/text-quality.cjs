/** Browser regression for the maintained text fixture. See tests/README.md. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const { chromium } = require('playwright');

/** Run the matched-font fixture under each supported emulated device scale. */
async function main() {
  const output = process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-text-quality';
  await fs.mkdir(output, { recursive: true });
  const browser = await chromium.launch({
    executablePath: process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless: process.env.VIEWER_HEADLESS === '1',
    args: process.env.VIEWER_CHROME_ARGS ? JSON.parse(process.env.VIEWER_CHROME_ARGS) : [],
  });
  try {
    for (const dpr of [1, 1.25, 2]) await checkContext(browser, dpr, output);
  } finally { await browser.close(); }
}

/** Verify visible glyphs, logical metrics and resource lifecycle in one browser context. */
async function checkContext(browser, dpr, output) {
  const context = await browser.newContext({viewport:{width:1500,height:1400},deviceScaleFactor:dpr});
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', capturePageError);
  page.on('console', captureConsoleError);
  /** Retain browser exceptions for the final assertion. */
  function capturePageError(error) {errors.push(String(error));}
  /** Treat GPU and application console errors as regression failures. */
  function captureConsoleError(message) {if(message.type()==='error') errors.push(message.text());}
  await page.goto((process.env.VIEWER_URL || 'http://127.0.0.1:8770/')+'text-quality.html');
  await page.waitForFunction(fixtureReady, {timeout:30000});
  let report = await page.evaluate(readReport);
  assert.equal(report.fontsLoaded, true);
  assert.equal(report.stats.missing_glyphs, 0);
  assert.equal(report.stats.shape_count, 5);
  assert(Math.abs(report.effectiveScale-dpr)<0.002);
  assert(report.lineMetrics.every(compatibleWidth), 'same-font width drift exceeds 0.2 CSS px');
  const canvasCapture = await page.locator('#text-quality-canvas').screenshot();
  const ink = await page.evaluate(countInk,canvasCapture.toString('base64'));
  assert(ink.every(visibleInk),'each specimen must produce visible white WebGPU glyph pixels');
  await page.screenshot({path:path.join(output,`text-dpr-${dpr}.png`),fullPage:true});
  const cases=[report];
  for(const scale of ['1','1.25','1.5','2']){
    await page.selectOption('#scale',scale);
    report=await page.evaluate(readReport);
    assert.equal(report.stats.shape_count,5,'DPI change must not reshape source text');
    assert.equal(report.stats.missing_glyphs,0);
    cases.push(report);
  }
  await page.check('#selected');
  await page.selectOption('#background','8421504');
  await page.evaluate(resetFixture);
  report=await page.evaluate(readReport);
  assert.equal(report.stats.shape_count,10,'release then recreate must shape each label once');
  assert(report.stats.atlas_resets>=1);
  await page.screenshot({path:path.join(output,`text-gray-selected-dpr-${dpr}.png`),fullPage:true});
  await page.setViewportSize({width:1700,height:1400});
  await page.waitForFunction(resizedFixture);
  assert.equal(await page.getAttribute('html','data-gpu-error'),null);
  assert.equal(await page.getAttribute('html','data-error'),null);
  assert.deepEqual(errors,[]);
  await fs.writeFile(path.join(output,`metrics-dpr-${dpr}.json`),JSON.stringify(cases,null,2));
  console.log(`PASS text DPR ${dpr}: same-font metrics, scale/cache reuse, selection, release, resize`);
  await context.close();
}
/** Count opaque interiors in every physical-size specimen screenshot. */
async function countInk(base64){
  const response=await fetch('data:image/png;base64,'+base64);
  const bitmap=await createImageBitmap(await response.blob());
  const canvas=document.createElement('canvas');canvas.width=bitmap.width;canvas.height=bitmap.height;
  const context=canvas.getContext('2d');context.drawImage(bitmap,0,0);
  const data=context.getImageData(0,0,canvas.width,canvas.height).data;
  const rows=JSON.parse(window.textQuality.fixture.specimens());
  const physicalScale=canvas.height/window.textQuality.report.logical[1];
  const counts=[];
  for(const row of rows){
    let count=0;
    const top=Math.floor(row.top*physicalScale);
    const bottom=Math.ceil((row.top+row.lineHeight*row.text.split('\n').length)*physicalScale);
    for(let y=top;y<bottom;y++)for(let x=0;x<canvas.width;x++){
      const offset=4*(y*canvas.width+x);
      if(data[offset]>220&&data[offset+1]>220&&data[offset+2]>220)count++;
    }
    counts.push(count);
  }
  bitmap.close();return counts;
}
/** Require enough pixels to reject an empty or missing specimen. */
function visibleInk(count){return count>100;}
/** Stop waiting on fixture failures and accept only the explicit ready signal. */
function fixtureReady(){if(document.documentElement.dataset.error)throw new Error(document.documentElement.dataset.error);return document.documentElement.dataset.ready==='true';}
/** Retrieve the fixture metrics without changing its state. */
function readReport(){return window.textQuality.report;}
/** Exercise the public disposal and reconstruction action. */
function resetFixture(){window.textQuality.reset();}
/** Wait until the fixture adopts the wider logical canvas. */
function resizedFixture(){return window.textQuality.report.logical[0]>750;}
/** Bound logical advance drift independently of raster differences. */
function compatibleWidth(line){return Math.abs(line.browser-line.shaper)<=0.2;}
main().catch(fail);
/** Return a failed test process with the original diagnostic. */
function fail(error){console.error(error);process.exitCode=1;}
