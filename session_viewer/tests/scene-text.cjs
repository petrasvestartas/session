/** Source text has the same select/hide lifecycle regardless of camera orientation. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');

/** Read the actual last submitted source labels and camera, without mutating app state. */
function snapshot() {
  return JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection') || '{}');
}

/** Project the center of a shaped camera-facing plate into canvas CSS coordinates. */
function center(state, label) {
  const p = [...label.world.map((v, i) => v - state.origin[i]), 1];
  const m = state.mvp, clip = [0,0,0,0];
  for (let r = 0; r < 4; r++) for (let c = 0; c < 4; c++) clip[r] += m[c*4+r]*p[c];
  const at = [(clip[0]/clip[3]*.5+.5)*state.logical_canvas[0], (.5-clip[1]/clip[3]*.5)*state.logical_canvas[1]];
  if (label.placement === 'world_billboard') {
    const scale = label.world_height * Math.hypot(m[1],m[5],m[9]) * state.logical_canvas[1] / (2*clip[3]*label.font_size);
    at[0] += label.line_box[0]*scale/2;
    at[1] += label.line_box[1]*scale/2;
  }
  return at;
}

/** Exercise authored world billboards and automatically supplied CAD document titles. */
async function main() {
  const output = process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-scene-text';
  await fs.mkdir(output,{recursive:true});
  const teapot = await fs.readFile(path.join(__dirname,'../assets/pb/view_mixed_teapot.pb'));
  const browser = await chromium.launch({executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless:process.env.VIEWER_HEADLESS === '1',args:JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]')});
  const reports = [];
  try {
    for (const dpr of [1,2]) for (const kind of ['world_billboard','nameplate']) {
      const context = await browser.newContext({viewport:{width:1000,height:700},deviceScaleFactor:dpr});
      const page = await context.newPage(), errors = [];
      page.on('pageerror',function failed(error) { errors.push(String(error)); });
      await page.route('**/view_local.yaml',function manifest(route) {
        const body = kind === 'world_billboard'
          ? 'name: Camera text\nitems: []\ntexts:\n  - text: selectable_camera_text\n    at: [0,0,10]\n    height: 32\n    camera_facing: true\n'
          : 'name: Camera title\nitems:\n  - file: pb/text-teapot.pb\n';
        return route.fulfill({status:200,contentType:'application/yaml',body});
      });
      await page.route('**/pb/text-teapot.pb',function fixture(route) {
        return route.fulfill({status:200,contentType:'application/octet-stream',body:teapot});
      });
      await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL || 'http://localhost:8771/').href);
      await page.bringToFront();
      await page.waitForFunction(kind => {
        const s = JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection') || '{}');
        return s.frames > 0 && s.text_labels?.some(label => label.placement === kind && label.object !== null);
      },kind,{timeout:60000});
      await page.locator('#canvas').focus();
      await page.keyboard.press('t');
      let state = await page.evaluate(snapshot);
      const label = state.text_labels.find(label => label.placement === kind && label.object !== null);
      const at = center(state,label);
      assert(at[0] > 0 && at[0] < 1000 && at[1] > 0 && at[1] < 700,'source plate must be visible');
      await page.mouse.click(...at);
      await page.waitForFunction(row => JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).selected === row,label.object);
      state = await page.evaluate(snapshot);
      assert.equal(state.selection,'Object');
      assert.deepEqual(state.text_labels.find(item => item.id === label.id).color,[0,0,0,255]);
      await page.locator('#canvas').screenshot({path:path.join(output,`${kind}-selected-dpr${dpr}.png`)});
      await page.keyboard.press('h');
      await page.waitForFunction(id => !JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).text_labels.some(label => label.id === id),label.id);
      await page.mouse.click(...at); await page.waitForTimeout(100);
      assert.notEqual((await page.evaluate(snapshot)).selected,label.object,'hidden source text cannot pick');
      await page.keyboard.press('s');
      await page.waitForFunction(id => JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).text_labels.some(label => label.id === id),label.id);
      await page.mouse.click(...at);
      await page.waitForFunction(row => JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).selected === row,label.object);
      await page.keyboard.press('f'); await page.waitForTimeout(100);
      assert.deepEqual(errors,[]);
      reports.push({kind,dpr,row:label.object,at,errors});
      await context.close();
    }
  } finally { await browser.close(); }
  await fs.writeFile(path.join(output,'results.json'),JSON.stringify(reports,null,2)+'\n');
  console.log('PASS camera-facing source text: real picks, black selected glyphs, hide/show, F and DPR1/2');
}
main().catch(function failed(error) { console.error(error); process.exitCode = 1; });
