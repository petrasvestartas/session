const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');
const root = process.cwd();
const specs = JSON.parse(fs.readFileSync('docs/practice.json', 'utf8'));
const out = path.join(root, 'docs/screenshots/practice');
fs.mkdirSync(out, {recursive:true});
const escape = text => text.replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;');
const hash = text => crypto.createHash('sha256').update(text).digest('hex');
(async () => {
 const browser = await chromium.launch({channel:'chrome',headless:true});
 const page = await browser.newPage({viewport:{width:1000,height:760},deviceScaleFactor:1});
 const records = {browser:browser.version(), captured:new Date().toISOString(), description:'Browser captures of real exercise source and recorded compiler/test output. These are source/check snapshots, not a running CAD scene.', lessons:{}};
 for (const [id,spec] of Object.entries(specs)) {
  const dir = path.join(root,'target/course-checks',id);
  const source = spec.split(':')[0];
  const prepared = fs.readFileSync(path.join(dir,'prepared.txt'),'utf8');
  const lines = prepared.split('\n');
  const at = lines.findIndex(line=>line.includes('Type the '));
  const first = Math.max(0,at-4);
  const ready = lines.slice(first, at+6).map((line,i)=>`${String(first+i+1).padStart(3)}  ${line}`).join('\n');
  const typed = fs.readFileSync(path.join(dir,'typed.txt'),'utf8');
  const section = spec.split(':')[1];
  const answer = fs.readFileSync(path.join('docs/lessons',id,source),'utf8').split('\n');
  const start = answer.findIndex(line => line.includes(`[start:${section}]`)) + 1;
  const end = answer.findIndex((line,index) => index >= start && line.includes(`[end:${section}]`));
  const current = answer.slice(start,end).filter(line => !/--8<-- \[(?:start|end):/.test(line)).join('\n')+'\n';
  if (current !== typed) throw Error(id+' source changed after its recorded checks');
  const wasm = fs.readFileSync(path.join(dir,'wasm.log'),'utf8');
  const native = fs.readFileSync(path.join(dir,'native.log'),'utf8');
  if (!wasm.includes('Finished `dev`') || !native.includes('test result: ok.')) throw Error(id+' has no successful recorded checks');
  const report = '$ cargo check --lib -j4\n'+wasm.split('\n').filter(line=>line.includes('Finished `dev`')).join('\n')+'\n\n$ cargo xtest --lib -j4\n'+native.split('\n').filter(line=>/^running \d+ tests|^test result:/.test(line)).join('\n');
  records.lessons[id] = {source, prepared:hash(prepared),typed:hash(typed),wasm:hash(wasm),native:hash(native)};
  for (const [step,title,body,caption] of [
   [1,'Your prepared exercise',ready,'The placeholder is inside the supplied file. Keep the surrounding code.'],
   [2,'The completed typing region',typed,'Source captured from the completed exercise; the lesson listing contains the same code.'],
   [3,'The recorded checks',report,'Actual compiler and native test output for this completed practice crate.']]) {
   await page.setContent(`<style>*{box-sizing:border-box}body{margin:0;padding:32px;background:#f0f1f5;color:#182033;font:17px system-ui}main{background:white;border:1px solid #dce0e8;border-radius:12px;padding:28px}small{color:#596176}h1{font-size:27px;margin:8px 0}p{line-height:1.5}pre{background:#f7f8fa;border:1px solid #dce0e8;padding:18px;font:14px/1.55 monospace;white-space:pre-wrap;overflow-wrap:anywhere}footer{color:#596176;font-size:14px}</style><main><small>SESSION VIEWER · LESSON ${id} · STEP ${step}</small><h1>${title}</h1><p>${escape(source)}</p><pre>${escape(body.trim())}</pre><footer>${caption}</footer></main>`);
   await page.locator('main').screenshot({path:path.join(out,`${id}-${step}.png`)});
  }
 }
 records.frame01 = {};
 for (const [name, colour] of [['frame','white'],['frame-grey','grey']]) {
  const ppm = fs.readFileSync(`target/course-checks/01/${name}.ppm`);
  const header = Buffer.from('P6\n640 480\n255\n');
  if (!ppm.subarray(0,header.length).equals(header)) throw Error('Unexpected course frame format');
  const png = new PNG({width:640,height:480});
  for (let i=0;i<640*480;i++) {
   ppm.copy(png.data,i*4,header.length+i*3,header.length+i*3+3);
   png.data[i*4+3]=255;
  }
  const pixels = PNG.sync.write(png).toString('base64');
  records.frame01[name] = {source:'examples/course_frame.rs',readback:hash(ppm)};
  await page.setContent(`<style>body{margin:0;padding:24px;font:18px system-ui;background:#eef0f4}main{display:inline-block;background:white;padding:24px}img{display:block;border:1px solid #bbb}p{max-width:640px}</style><main><h2>Lesson 01 · ${colour === 'white' ? 'First GPU frame' : 'Change the shader colour'}</h2><img src="data:image/png;base64,${pixels}"><p>Actual 640 × 480 pixel readback from the checkpoint renderer. The fullscreen triangle paints a ${colour} background.</p></main>`);
  await page.locator('main').screenshot({path:path.join(out,`01-${name}.png`)});
 }
 fs.writeFileSync(path.join(out,'captures.json'),JSON.stringify(records,null,2)+'\n');
 await browser.close();
})().catch(error=>{console.error(error);process.exit(1)});

// From session_viewer, after check_practice.py: NODE_PATH=target/course-tools/node_modules node docs/capture_practice.cjs.
