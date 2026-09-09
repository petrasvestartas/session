/** Controlled object-selection latency using real input and identical serialized fixtures.
 * Submission, the following browser-frame callback and screenshot completion are separate
 * observations. None is a photon/presentation timer. Run --self-test without a browser.
 */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const crypto = require('node:crypto');
const os = require('node:os');

/** Install timestamp capture before input reaches the viewer, observing its existing snapshot. */
function installProbe() {
  const canvas = document.querySelector('canvas');
  window.pickingProbe = {pending:null};
  document.addEventListener('pointerup', recordRelease, true);
  const observer = new MutationObserver(recordSubmission);
  observer.observe(canvas, {attributes:true, attributeFilter:['data-viewer-inspection']});
  /** Capture an actual trusted left-button release in the page's performance clock. */
  function recordRelease(event) {
    const sample = window.pickingProbe.pending;
    if (!sample || sample.release_at_ms !== null || event.button !== 0 || !event.isTrusted) return;
    const raw = canvas.getAttribute('data-viewer-inspection');
    const snapshot = raw ? JSON.parse(raw) : null;
    sample.release_at_ms = performance.now();
    sample.event_timestamp_ms = event.timeStamp;
    sample.release_frames = snapshot?.frames ?? 0;
    sample.release_position = [event.clientX,event.clientY];
  }
  /** Observe the first new submitted color frame carrying the requested ordinary selection. */
  function recordSubmission() {
    const sample = window.pickingProbe.pending;
    if (!sample || sample.release_at_ms === null || sample.submitted_at_ms !== null || sample.done) return;
    const raw = canvas.getAttribute('data-viewer-inspection');
    if (!raw) return;
    const snapshot = JSON.parse(raw);
    if (snapshot.selected === null || snapshot.selected === undefined || snapshot.frames <= sample.release_frames) return;
    if (sample.expected_parent !== null && snapshot.selected !== sample.expected_parent) {
      sample.error = `wrong parent: expected ${sample.expected_parent}, got ${snapshot.selected}`;
      sample.done = true;
      return;
    }
    const submitted = snapshot.submitted_at_ms;
    if (!Number.isFinite(submitted)) {
      sample.error = 'inspection lacks a finite submission timestamp';
      sample.done = true;
      return;
    }
    if (submitted < sample.release_at_ms) return;
    sample.selected_parent = snapshot.selected;
    sample.selected_identity = snapshot.identity ?? null;
    sample.submitted_at_ms = submitted;
    sample.observed_at_ms = performance.now();
    sample.submitted_frames = snapshot.frames;
    sample.input_to_submitted_ms = submitted - sample.release_at_ms;
    sample.input_to_observed_ms = sample.observed_at_ms - sample.release_at_ms;
    requestAnimationFrame(recordNextFrame);
  }
  /** This callback marks a following browser frame opportunity, not display presentation. */
  function recordNextFrame(timestamp) {
    const sample = window.pickingProbe.pending;
    if (!sample || sample.submitted_at_ms === null || sample.done) return;
    sample.next_frame_timestamp_ms = timestamp;
    sample.input_to_next_browser_frame_ms = performance.now() - sample.release_at_ms;
    sample.done = true;
  }
}

/** Arm one measurement; no synthetic selection or viewer API call occurs here. */
function armMeasurement(expectedParent) {
  window.pickingProbe.pending = {expected_parent:expectedParent, release_at_ms:null,
    submitted_at_ms:null, done:false, error:null};
}
/** Read measurement readiness without adding a fixed sleep to its timestamps. */
function measurementReady() { return Boolean(window.pickingProbe.pending?.done); }
/** Copy the small observation record back to the runner. */
function measurementResult() { return window.pickingProbe.pending; }
/** Read the browser clock after screenshot completion, retaining RPC overhead in the bound. */
function browserNow() { return performance.now(); }
/** Read only the optional production/baseline inspection surface. */
function snapshot() {
  const raw = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
  return raw ? JSON.parse(raw) : null;
}
/** Both implementations must finish loading the exact seven-family source document. */
function sceneReady() {
  const raw = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
  if (!raw) return false;
  const state = JSON.parse(raw);
  return state.objects === 7 && state.frames > 0 && state.mvp?.length === 16;
}
/** Wait for a submitted clear frame, rather than assuming Escape took effect after a delay. */
function selectionCleared(previousFrames) {
  const raw = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
  if (!raw) return false;
  const state = JSON.parse(raw);
  return state.selected === null && state.frames > previousFrames && !state.pick_busy;
}
/** Require a frame following the top-view key before projecting source coordinates. */
function frameAdvanced(previousFrames) {
  const raw = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
  return raw && JSON.parse(raw).frames > previousFrames;
}
/** Decode actual screenshot pixels; yellow verification is outside the timed submission path. */
async function countYellow(base64) {
  const response = await fetch('data:image/png;base64,' + base64);
  const image = await createImageBitmap(await response.blob());
  const canvas = document.createElement('canvas');
  canvas.width = image.width; canvas.height = image.height;
  const context = canvas.getContext('2d');
  context.drawImage(image,0,0); image.close();
  const pixels = context.getImageData(0,0,canvas.width,canvas.height).data;
  let yellow = 0;
  for (let index=0; index<pixels.length; index+=4) {
    if (pixels[index]>240 && pixels[index+1]>240 && pixels[index+2]<32) yellow++;
  }
  return yellow;
}
/** Use the inspected camera and actual CSS canvas rectangle for both builds. */
function project(state, point, rectangle) {
  const origin = state.origin ?? [0,0,0];
  const source = [point[0]-origin[0],point[1]-origin[1],point[2]-origin[2],1];
  const clip = [0,0,0,0];
  for (let row=0; row<4; row++) {
    for (let column=0; column<4; column++) clip[row] += state.mvp[column*4+row]*source[column];
  }
  assert(Number.isFinite(clip[3]) && clip[3] !== 0,'invalid projection');
  return [rectangle.x+(clip[0]/clip[3]*.5+.5)*rectangle.width,
    rectangle.y+(.5-clip[1]/clip[3]*.5)*rectangle.height];
}
/** Return nearest-rank quantiles while preserving missing/failed workloads as null. */
function percentile(values, fraction) {
  if (!values.length) return null;
  const sorted = values.slice().sort(compareNumbers);
  return sorted[Math.max(0,Math.ceil(sorted.length*fraction)-1)];
}
/** Numeric ordering shared by median and p95 calculation. */
function compareNumbers(a,b) { return a-b; }
/** Aggregate only samples with verified yellow output; failed baseline highlights stay failures. */
function summarize(samples) {
  const metrics = ['input_to_submitted_ms','input_to_next_browser_frame_ms','input_to_capture_upper_bound_ms'];
  const result = {samples:samples.length,verified_highlights:0,failures:0};
  const passed = [];
  for (const sample of samples) {
    if (sample.passed) passed.push(sample);
    else result.failures++;
  }
  result.verified_highlights = passed.length;
  for (const metric of metrics) {
    const values = [];
    for (const sample of passed) if (Number.isFinite(sample[metric])) values.push(sample[metric]);
    result[metric] = {median:percentile(values,.5),p95:percentile(values,.95)};
  }
  return result;
}
/** Clear through real keyboard input and wait for its specific submitted result. */
async function clearSelection(page, timeout) {
  const previous = await page.evaluate(snapshot);
  await page.keyboard.press('Escape');
  await page.waitForFunction(selectionCleared,previous.frames,{timeout});
}
/** Measure a real press/release, then independently verify its screenshot highlight. */
async function measure(page, spec, parent, output, label, timeout) {
  await clearSelection(page,timeout);
  const state = await page.evaluate(snapshot);
  const rectangle = await page.locator('canvas').boundingBox();
  assert(rectangle,'missing canvas rectangle');
  const target = project(state,spec.pick,rectangle);
  await page.mouse.move(target[0],target[1]);
  await page.evaluate(armMeasurement,parent);
  await page.mouse.down({button:'left'});
  await page.mouse.up({button:'left'});
  let result;
  try {
    await page.waitForFunction(measurementReady,null,{timeout});
    result = await page.evaluate(measurementResult);
  } catch (error) {
    result = await page.evaluate(measurementResult);
    result.error = `selection observation timed out: ${String(error)}`;
  }
  result.target_css = target;
  result.kind = spec.kind;
  result.origin = state.origin ?? [0,0,0];
  result.origin_source = state.origin ? 'inspection' : 'zero fallback for this origin-centered fixture';
  result.capture_started_at_ms = await page.evaluate(browserNow);
  const image = await page.locator('canvas').screenshot({path:path.join(output,label+'.png')});
  result.capture_finished_at_ms = await page.evaluate(browserNow);
  result.input_to_capture_upper_bound_ms = result.release_at_ms === null ? null : result.capture_finished_at_ms-result.release_at_ms;
  result.yellow_pixels = await page.evaluate(countYellow,image.toString('base64'));
  if (!result.error && result.yellow_pixels<3) result.error = 'submitted selection has fewer than three opaque yellow pixels';
  if (!result.error && result.selected_identity && result.selected_identity[1] !== spec.guid) result.error = 'selection identity differs from source fixture GUID';
  result.passed = !result.error && Number.isFinite(result.input_to_submitted_ms);
  return result;
}
/** Run identical warm-up and repeated Escape/click workloads, without timed fixed waits. */
async function runVersion(browser, version, bytes, cases, output, count) {
  const directory = path.join(output,version.name); await fs.mkdir(directory,{recursive:true});
  const context = await browser.newContext({viewport:{width:1400,height:900},deviceScaleFactor:1});
  const page = await context.newPage();
  const errors = [], report = {name:version.name,url:version.url,viewport:[1400,900],dpr:1,cases:[],errors};
  /** Preserve browser exceptions without aborting the other controlled build's report. */
  function pageError(error) { errors.push(String(error)); }
  /** Keep GPU errors; ignore only the static development reload endpoint. */
  function consoleError(message) {
    if (message.type()==='error' && !message.text().includes('{{__trunk_address__}}')) errors.push(message.text());
  }
  /** Both builds receive exactly the same local manifest bytes. */
  function manifest(route) { return route.fulfill({status:200,contentType:'application/yaml',body:'name: Picking performance\nitems:\n  - file: "pb/interaction-fixture.pb"\n'}); }
  /** Serve the same Buffer through the existing ordinary file loader. */
  function fixture(route) { return route.fulfill({status:200,contentType:'application/octet-stream',body:bytes}); }
  page.on('pageerror',pageError); page.on('console',consoleError);
  await page.route('**/view_local.yaml',manifest);
  await page.route('**/pb/interaction-fixture.pb',fixture);
  const timeout = Number(process.env.VIEWER_PICK_TIMEOUT_MS || 15000);
  try {
    const url = new URL(version.url); url.searchParams.set('data','off'); url.searchParams.set('inspect','1');
    await page.goto(url.href,{waitUntil:'networkidle',timeout:60000});
    await page.bringToFront(); await page.waitForFunction(sceneReady,null,{timeout:60000});
    await page.locator('canvas').focus();
    const initial = await page.evaluate(snapshot);
    await page.keyboard.press('5'); await page.waitForFunction(frameAdvanced,initial.frames,{timeout});
    report.initial = await page.evaluate(snapshot);
    report.environment = await page.evaluate(browserEnvironment);
    await page.evaluate(installProbe);
    const parents = new Set();
    for (const spec of cases) {
      const warmup = await measure(page,spec,null,directory,spec.kind+'-warmup',timeout);
      const parent = warmup.selected_parent ?? null;
      if (parent !== null) parents.add(parent);
      const samples = [];
      for (let index=0; index<count; index++) {
        samples.push(await measure(page,spec,parent,directory,`${spec.kind}-${index+1}`,timeout));
      }
      const family = {kind:spec.kind,guid:spec.guid,parent,warmup,samples,summary:summarize(samples)};
      report.cases.push(family);
      await fs.writeFile(path.join(directory,spec.kind+'.json'),JSON.stringify(family,null,2));
      console.log(JSON.stringify({version:version.name,kind:spec.kind,...family.summary}));
    }
    report.distinct_parents = parents.size;
    if (parents.size !== cases.length) report.errors.push('source families did not resolve to seven distinct parents');
  } catch (error) {
    report.errors.push(String(error));
    await page.screenshot({path:path.join(directory,'failure.png')});
  } finally {
    await fs.writeFile(path.join(directory,'report.json'),JSON.stringify(report,null,2));
    await context.close();
  }
  return report;
}
/** Record actual browser/canvas scale without asking the baseline for new GPU APIs. */
function browserEnvironment() {
  const rectangle = document.querySelector('canvas').getBoundingClientRect();
  return {userAgent:navigator.userAgent,devicePixelRatio:devicePixelRatio,
    canvas_css:[rectangle.width,rectangle.height],visibility:document.visibilityState};
}
/** Execute the before/after comparison sequentially on one browser and host. */
async function main() {
  const {chromium} = require('playwright');
  const fixture = process.env.VIEWER_INTERACTION_FIXTURE || '/tmp/viewer-interaction.pb';
  const bytes = await fs.readFile(fixture), cases = JSON.parse(await fs.readFile(fixture+'.json','utf8'));
  assert.equal(cases.length,7,'expected the maintained seven-family interaction fixture');
  const output = process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-picking-performance';
  await fs.mkdir(output,{recursive:true});
  const count = Number(process.env.VIEWER_PICK_SAMPLES || 10);
  assert(Number.isInteger(count) && count>0,'sample count must be positive');
  const versions = [{name:'baseline',url:process.env.VIEWER_BASELINE_URL || 'http://127.0.0.1:45693/'},
    {name:'current',url:process.env.VIEWER_URL || 'http://127.0.0.1:18772/'}];
  // Both builds are prerequisites this runner cannot supply: say which one is missing before
  // a browser is launched, instead of failing inside a navigation timeout.
  for (const version of versions) {
    try { await fetch(version.url); }
    catch (error) { throw new Error(`${version.name} viewer is not reachable at ${version.url}; serve a controlled baseline build (VIEWER_BASELINE_URL) and the current build (VIEWER_URL) before running this comparison`); }
  }
  const browser = await chromium.launch({executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless:process.env.VIEWER_HEADLESS==='1',args:JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]')});
  const reports = [];
  try {
    for (const version of versions) reports.push(await runVersion(browser,version,bytes,cases,output,count));
  } finally { await browser.close(); }
  const summary = {fixture,fixture_sha256:crypto.createHash('sha256').update(bytes).digest('hex'),
    host:{platform:os.platform(),release:os.release(),cpu:os.cpus()[0]?.model},samples_per_family:count,
    timing_scope:{input:'trusted pointerup capture using performance.now()',
      submitted:'first new inspected color submission carrying the selected object',
      next_browser_frame:'requestAnimationFrame callback after that observation; presentation proxy only',
      capture_upper_bound:'screenshot completion plus browser RPC overhead; not photon timing'},versions:[]};
  let failed = false;
  for (const report of reports) {
    const families = [];
    for (const family of report.cases) { families.push({kind:family.kind,...family.summary}); failed ||= family.summary.failures>0; }
    failed ||= report.errors.length>0 || families.length!==cases.length;
    summary.versions.push({name:report.name,url:report.url,environment:report.environment,errors:report.errors,families});
  }
  await fs.writeFile(path.join(output,'summary.json'),JSON.stringify(summary,null,2));
  if (failed) process.exitCode=1;
}
/** Check timing causality, quantiles and projection with a fake DOM; this does not use a GPU. */
function selfTest() {
  const vm = require('node:vm');
  let now=0, encoded=JSON.stringify({selected:null,frames:1}), observer, frame;
  const listeners = {};
  const canvas = {getAttribute:readAttribute};
  function readAttribute() { return encoded; }
  function querySelector() { return canvas; }
  function addEventListener(name,callback) { listeners[name]=callback; }
  function nowTime() { return now; }
  function scheduleFrame(callback) { frame=callback; }
  class Observer { constructor(callback) { observer=callback; } observe() {} }
  const sandbox = {window:{},document:{querySelector,addEventListener},performance:{now:nowTime},MutationObserver:Observer,requestAnimationFrame:scheduleFrame};
  vm.runInNewContext(`(${installProbe.toString()})()`,sandbox);
  vm.runInNewContext(`(${armMeasurement.toString()})(6)`,sandbox);
  observer(); assert.equal(sandbox.window.pickingProbe.pending.submitted_at_ms,null);
  now=10; listeners.pointerup({button:0,isTrusted:true,timeStamp:9,clientX:20,clientY:30});
  encoded=JSON.stringify({selected:6,frames:2,submitted_at_ms:15}); now=16; observer();
  assert.equal(sandbox.window.pickingProbe.pending.input_to_submitted_ms,5);
  assert.equal(sandbox.window.pickingProbe.pending.done,false);
  now=20; frame(19); assert.equal(sandbox.window.pickingProbe.pending.input_to_next_browser_frame_ms,10);
  assert.equal(sandbox.window.pickingProbe.pending.done,true);
  assert.equal(percentile([1,2,3,4,5,6,7,8,9,10],.95),10);
  assert.equal(summarize([{passed:false,input_to_submitted_ms:1}]).input_to_submitted_ms.median,null);
  assert.deepEqual(project({mvp:[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]},[0,0,0],{x:2,y:3,width:100,height:50}),[52,28]);
  console.log('PASS CPU-only picking probe: trusted release, submission causality, next-frame proxy, failed-highlight exclusion and projection');
}
/** Preserve a nonzero status while leaving any completed report artifacts available. */
function fail(error) { console.error(error); process.exitCode=1; }
if (process.argv.includes('--self-test')) selfTest(); else main().catch(fail);
