const assert=require('node:assert/strict');
const fs=require('node:fs');const path=require('node:path');const {chromium}=require('playwright');
const read=()=>JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection'));
(async()=>{
const out=process.env.VIEWER_TEST_OUTPUT;fs.mkdirSync(out,{recursive:true});
const browser=await chromium.launch({executablePath:process.env.CHROME_BIN,headless:true,args:JSON.parse(process.env.VIEWER_CHROME_ARGS)});
const results=[];
try{for(const width of [1280,390]){
const page=await browser.newPage({viewport:{width,height:width===390?844:800}});const errors=[],requests=[];page.on('pageerror',e=>errors.push(String(e)));
let release;const gate=new Promise(r=>release=r);let requested;const seen=new Promise(r=>requested=r);
await page.route('**/text/*.ttf',async route=>{const name=new URL(route.request().url()).pathname.split('/').pop();requests.push(name);requested();await gate;await route.fulfill({contentType:'font/ttf',body:fs.readFileSync(path.join('assets/text',name))});});
await page.route('**/scenes/font-test.yaml',route=>route.fulfill({contentType:'application/yaml',body:'name: font-test\nitems:\n  - file: pb/course-comparison.pb\ntexts:\n  - text: "Fixed \u03a9 cube"\n    at: [0,0,1000]\n    height: 400\n    camera_facing: true\n'}));
await page.route('**/pb/course-comparison.pb',route=>route.fulfill({contentType:'application/octet-stream',body:fs.readFileSync(process.env.VIEWER_COMPARISON_FIXTURE || 'target/application-check02/project/dist/pb/course-comparison.pb')}));
await page.goto(process.env.VIEWER_URL+'?scene=font-test.yaml&data=off&inspect=1');
await Promise.race([seen,new Promise((_,reject)=>{const timer=setTimeout(()=>reject(new Error('No full-font request within30s')),30000);timer.unref();})]);await page.waitForFunction(()=>window.viewerDiagnostics?.read().outcome==='ready').catch(async e=>{fs.writeFileSync(path.join(out,'startup-failure.json'),JSON.stringify({diagnostics:await page.evaluate(()=>window.viewerDiagnostics?.read()),inspection:await page.evaluate(read),requests},null,2));throw e;});
await page.waitForFunction(()=>JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).text_labels.some(l=>l.text==='Fixed \u03a9 cube'));
const before=await page.evaluate(read);await page.screenshot({path:path.join(out,'subset-'+width+'.png')});release();
await page.waitForFunction(old=>{const s=JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection'));return s.text_labels.find(l=>l.text==='Fixed \u03a9 cube')?.line_box[0]!==old;},before.text_labels.find(l=>l.text==='Fixed \u03a9 cube').line_box[0]);
await page.waitForTimeout(250);const after=await page.evaluate(read);await page.screenshot({path:path.join(out,'whole-'+width+'.png')});
assert.deepEqual(requests,['NotoSans-Regular.ttf','NotoSansSymbols2-Regular.ttf','NotoSansSymbols-Regular.ttf']);assert.equal(after.objects,before.objects);assert.deepEqual(after.camera_pose,before.camera_pose);assert.deepEqual(errors,[]);
results.push({width,requests,before,after,errors});await page.close();
}
fs.writeFileSync(path.join(out,'fonts.json'),JSON.stringify({passed:true,scope:'Local HTTP font delivery and changed text shaping; original subset native glyph tests remain separate',results},null,2));
}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1});