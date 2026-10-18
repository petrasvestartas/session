const assert=require('node:assert/strict'),crypto=require('node:crypto'),{PNG}=require('pngjs'),canvas=require('./canvas-image.cjs');
module.exports=async(p,{world,cssRadius=6})=>{
 const value=k=>p.locator('canvas').getAttribute(k).then(JSON.parse),m=await value('data-render-matrix'),o=await value('data-render-origin'),q=world.map((v,i)=>v-o[i]),c=[0,1,2,3].map(i=>q.reduce((s,v,j)=>s+m[j*4+i]*v,m[12+i]));assert(c[3]>0);
 const image=PNG.sync.read(Buffer.from((await canvas(p.locator('canvas'))).split(',')[1],'base64')),box=await p.locator('canvas').boundingBox(),scale=image.width/box.width,x=(c[0]/c[3]+1)*image.width/2,y=(1-c[1]/c[3])*image.height/2,r=Math.ceil(cssRadius*scale),bounds=[Math.round(x)-r,Math.round(y)-r,2*r+1,2*r+1];assert(bounds[0]>=0&&bounds[1]>=0&&bounds[0]+bounds[2]<=image.width&&bounds[1]+bounds[3]<image.height-34*scale);
 let black=0,yellow=0,nonwhite=0;const bytes=[];for(let j=bounds[1];j<bounds[1]+bounds[3];j++)for(let i=bounds[0];i<bounds[0]+bounds[2];i++){const at=(j*image.width+i)*4,[r,g,b]=image.data.subarray(at,at+3);bytes.push(...image.data.subarray(at,at+4));if(Math.max(r,g,b)<100)black++;if(r>180&&g>130&&b<110)yellow++;if(Math.min(r,g,b)<245)nonwhite++;}
 return{hash:crypto.createHash('sha256').update(Buffer.from(bytes)).digest('hex'),black,yellow,nonwhite,bounds,world};
};
