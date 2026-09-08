/** Browser regression for chapter05's local box and four-millimetre floor fixtures. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const {chromium}=require('playwright');
/** Count diagnostic colors from a real canvas capture at its original framebuffer size. */
async function colors(png){
 const bitmap=await createImageBitmap(await(await fetch('data:image/png;base64,'+png)).blob());
 const canvas=new OffscreenCanvas(bitmap.width,bitmap.height),ctx=canvas.getContext('2d');
 ctx.drawImage(bitmap,0,0);bitmap.close();const bytes=ctx.getImageData(0,0,canvas.width,canvas.height).data;
 const result={red:0,blue:0,magenta:0,black:0};
 for(let i=0;i<bytes.length;i+=4){const [r,g,b]=bytes.slice(i,i+3);
  if(r>195&&g<60&&b<60)result.red++;if(r<60&&g<60&&b>195)result.blue++;
  if(r>195&&g<60&&b>195)result.magenta++;if(r<40&&g<40&&b<40)result.black++;
 }
 return result;
}
/** Drive exact camera/quality variants without any external geometry or bucket. */
async function main(){
 const out=process.env.VIEWER_TEST_OUTPUT||'/tmp/viewer-course-visibility';await fs.mkdir(out,{recursive:true});
 const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});
 const results=[];
 try{for(const dpr of [1,2]){
  const page=await browser.newPage({viewport:{width:1400,height:900},deviceScaleFactor:dpr});const errors=[];
  page.on('pageerror',record);function record(error){errors.push(String(error));}
  for(const spec of [{name:'box',query:''},{name:'floor-top',query:'fixture=floor&top&msaa=1'},{name:'floor-far',query:'fixture=floor&perspective&distance=16&msaa=4'}]){
   await page.goto((process.env.VIEWER_URL||'http://127.0.0.1:8770/')+'?'+spec.query);await page.bringToFront();
   await page.waitForFunction(()=>JSON.parse(document.querySelector('#canvas')?.dataset.tutorialInspection||'{}').stage===5);
   const state=await page.locator('#canvas').getAttribute('data-tutorial-inspection');
   const capture=await page.locator('#canvas').screenshot();const count=await page.evaluate(colors,capture.toString('base64'));
   const result={dpr,case:spec.name,state:JSON.parse(state),colors:count};results.push(result);
   await fs.writeFile(path.join(out,spec.name+'-'+dpr+'.png'),capture);
   assert.equal(count.magenta,0,'four-millimetre hidden ink stays hidden');
   if(spec.name==='box'){assert(count.red>200,'red edges remain');assert(count.black>20,'black source markers remain');}
   else assert(count.blue>5,'visible blue floor stroke remains');
  }
  assert.deepEqual(errors,[]);await page.close();
 }}finally{await browser.close();await fs.writeFile(path.join(out,'results.json'),JSON.stringify(results,null,2));}
 console.log('PASS chapter05: local grey box and near/far floor visibility at DPR1/2');
}
main().catch(error=>{console.error(error);process.exitCode=1;});
