/** Independent surface/edge overrides, authored reset, and portable persistence. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const {PNG}=require('pngjs');
const {chromium}=require('playwright');
const ui=require('../docs/extensions/browser.cjs');
const state=async p=>JSON.parse(await p.locator('canvas').getAttribute('data-viewer-inspection'));
async function wait(p,fn){await p.waitForFunction(src=>{const raw=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');return raw&&Function('s',`return (${src})(s)`)(JSON.parse(raw));},fn.toString());}
function pixels(bytes){const png=PNG.sync.read(bytes);let red=0,blue=0;for(let y=100;y<650;y++)for(let x=100;x<1050;x++){const i=(y*png.width+x)*4;const [r,g,b]=png.data.subarray(i,i+3);if(r>g*1.5&&r>b*1.5)red++;if(b>r*1.4&&b>g*1.15)blue++;}return {red,blue};}
(async()=>{
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:false,args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});
 const page=await browser.newPage({viewport:{width:1400,height:900},acceptDownloads:true});const errors=[];page.on('pageerror',e=>errors.push(e.message));
 try {
 await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:'name: Color tutorial\nitems:\n  - file: color-box.pb\n'}));
 await page.route('**/color-box.pb',r=>r.fulfill({path:path.resolve(__dirname,'../docs/extensions/split.pb')}));
 await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);await wait(page,s=>s.objects>=2);
 for(const label of ['Split tutorial'])await ui.button(page,'open/',label);
 await page.mouse.click(900,140);await page.keyboard.press('7');await page.waitForTimeout(180);await page.keyboard.press('f');await page.waitForTimeout(180);
 async function color(channel,value){await ui.button(page,'color/','Color Joined shell');await ui.button(page,'color-channel/',channel);await ui.button(page,'color/',value);}
 const baseline=pixels(await page.screenshot());
 await color('Faces','Red');await wait(page,s=>s.color_count===1);
 await color('Edges','Blue');await wait(page,s=>s.edge_color_count===1);
 let colored=pixels(await page.screenshot());assert(colored.red>baseline.red+1000,JSON.stringify({baseline,colored}));assert(colored.blue>baseline.blue+30,JSON.stringify({baseline,colored}));
 await ui.button(page,'color/','Color Joined shell');await ui.button(page,'color-channel/','Faces');await page.mouse.move(600,600);await page.screenshot({path:'/tmp/viewer-color-tutorial.png'});await fs.writeFile('/tmp/viewer-color-tutorial.json',JSON.stringify({state:await state(page),ui:await ui.ui(page)},null,2));await page.keyboard.press('Escape');
 const download=page.waitForEvent('download');await ui.button(page,'toolbar/Save','Save');const file='/tmp/viewer-colors.session';await (await download).saveAs(file);
 await color('Faces','Original');await wait(page,s=>s.color_count===0&&s.edge_color_count===1);let reset=pixels(await page.screenshot());assert(reset.red<colored.red/2);assert(reset.blue>baseline.blue+30);
 await color('Edges','Original');await wait(page,s=>s.color_count===0&&s.edge_color_count===0);reset=pixels(await page.screenshot());assert(Math.abs(reset.blue-baseline.blue)<30);
 const chooser=page.waitForEvent('filechooser');await ui.button(page,'toolbar/Open','Open');await (await chooser).setFiles(file);await wait(page,s=>s.color_count===1&&s.edge_color_count===1);await page.waitForTimeout(200);
 colored=pixels(await page.screenshot());assert(colored.red>baseline.red+1000);assert(colored.blue>baseline.blue+30);
 assert.deepEqual(errors,[]);console.log('PASS distinct rendered face/edge colors, per-channel Original, Save/Open, full viewer screenshot');
 }catch(e){await page.screenshot({path:'/tmp/viewer-color-failure.png'});console.log(await ui.ui(page));throw e;}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
