const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
module.exports=async(page,helpers)=>{
 await require('./dock-steps.cjs')(page,{...helpers,step:{...helpers.step,id:'03c-layout'}});
 await page.mouse.click(8,8);
 const canvas=page.locator('canvas');
 const field=await canvas.evaluate(c=>JSON.parse(c.getAttribute('data-command-ui')).controls.find(c=>c.key==='command/input').rect);
 assert(Math.abs(field[3]-field[1]-22)<2,'The field retains its 22 CSS pixel row');
 const png=PNG.sync.read(Buffer.from((await canvas.evaluate(c=>c.toDataURL())).split(',')[1],'base64'));
 const scale=png.width/(await canvas.boundingBox()).width;
 const top=(left,right)=>{
  for(let y=Math.ceil((field[1]+2)*scale);y<(field[3]-2)*scale;y++)
   for(let x=Math.ceil(left*scale);x<right*scale;x++){
    const i=(y*png.width+x)*4;
    if([...png.data.subarray(i,i+3)].every(c=>c<130))return y;
   }
  assert.fail('Actual text must render in the command row');
 };
 const label=top(Math.max(0,field[0]-85),field[0]-5);
 const text=top(field[0]+5,field[0]+35);
 assert(Math.abs(label-text)<=2*scale,'Label and typed text share the centre line');
};
