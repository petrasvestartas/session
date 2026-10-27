import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {course,docs,hash,read,viewer,write} from './model.ts';

const self=fileURLToPath(import.meta.url),core=fileURLToPath(new URL('./exported-browser-image-recipes.ts',import.meta.url));
const sha=(f:string)=>hash(fs.readFileSync(f));
const ids:Record<string,string>={
 '96zm-write-triangle-addresses-beside-color':'vertex-pulling',
 '96zs-write-first-pick-primitives-from-shared-geometry':'vertex-pulling',
 '96zz-separate-hidden-ink-color-from-pick-ownership':'finite-surfaces',
 '96zza-connect-section-primitive-images':'cap-primitives',
 '96zzc-share-section-addresses-with-outlines-and-ambient':'section-resources',
 '96zzd-pack-surface-position-normal-and-color':'section-resources',
 '96zzf-start-with-current-browser-display-defaults':'display-scale',
 '96zzg-grow-buffers-without-losing-rows':'buffer-growth',
 '96zzm-connect-the-renderer-to-one-mesh-arena':'arena-cache',
 '96zzo-read-instance-properties-through-draw-slots':'slot-pipeline',
 '96zzzzzzt-keep-contact-ambient-inputs-on-originals':'ambient-contacts',
 '96zzzzzzu-keep-depth-and-address-samples-separate':'ambient-contacts',
};

function plan(module:string){
 const authorities:any[]=[];
 const source=(name:string)=>{const file=path.join(docs,'journey/checks/browser/'+name+'.cjs'),text=read(file);if(!authorities.some(r=>r.path===file))authorities.push({path:file,sha256:sha(file)});return text;};
 const fragment=(name:string,start:string,end='\n};')=>{const text=source(name),at=text.indexOf(start);assert(at>=0,name+': start');assert.equal(text.indexOf(start,at+1),-1,name+': unique start');const stop=text.indexOf(end,at);assert(stop>at,name+': end');const body=text.slice(at,stop);return {name,offset:at,length:body.length,sha256:hash(body),body};};
 const parts:any[]=[];
 const block=(body:string,argument='original')=>`await(async(${argument},helpers)=>{const assert=require('node:assert/strict');\n${body}\n})(page,h);`;
 const add=(f:any,argument='original')=>{parts.push(f);return block(f.body,argument);};
 const close=(name:string)=>add(fragment(name,"    await helpers.command(original,'Close');"));
 const caps=()=>{source('normal-view');return close('vertex-pulling')+close('finite-surfaces')+close('cap-primitives');};
 const display=()=>{source('display-scale');const text=source('display-defaults'),start=text.indexOf('    const base='),stop=text.indexOf('    await page.goto(base.href);',start);assert(start>=0&&stop>start);const declarations={name:'display-defaults',offset:start,length:stop-start,body:text.slice(start,stop),sha256:hash(text.slice(start,stop))};const tail=fragment('display-defaults','    await page.setViewportSize({width:900,height:760});');parts.push(declarations,tail);return block(declarations.body+tail.body,'page');};
 const labels=()=>{const text=source('labels'),line='    const value=async(page,name)=>JSON.parse(await page.locator(\'canvas\').getAttribute(name));';assert.equal(text.split(line).length,2);const tail=fragment('labels',"    for(const line of ['Close','Layers Off','View Grid Off','Example CAD','Example Scene Text','Fit','View Isometric','Select Next','Select Next'])");parts.push({name:'labels',offset:text.indexOf(line),length:line.length,body:line,sha256:hash(line)},tail);return block(line+'\n'+tail.body);};
 let body='';
 if(module==='vertex-pulling'){source('normal-view');body=close(module);}
 else if(module==='finite-surfaces'){source('normal-view');body=close('vertex-pulling')+close(module);}
 else if(['cap-primitives','section-resources'].includes(module)){source(module);body=caps();}
 else if(module==='display-scale')body=display();
 else if(module==='buffer-growth'){source(module);body=display()+labels();}
 else if(['arena-cache','slot-pipeline'].includes(module)){source(module);source('arena-cache');source('buffer-growth');source('section-resources');body=display()+labels()+caps();}
 else{assert.equal(module,'ambient-contacts');const text=source(module),authority="await start(page,address);await open(page,'ordinary',h);";assert(text.includes(authority));body=authority;parts.push({name:module,offset:text.lastIndexOf(authority),length:authority.length,sha256:hash(authority),body:authority});}
 return {body,authority:parts[0].body,authorities,parts,scope:'Exact literal final original-main-page writers and all assertions within those fragments. Original child/prefix matrices excluded; no whole canonical callback acceptance. Display-defaults uses its original final fresh navigation/reset and original settings/grid assertions.'};
}

export function mainPageImageRecipe(id:string){const module=ids[id];if(!module)return null;const step=course().steps.find(s=>s.id===id);assert(step?.browser_check);assert.equal(path.basename(step.browser_check,'.cjs'),module);assert(!step.browser_actions?.length);return {id,module,...plan(module)};}

async function implementation(){
 const setups=Object.fromEntries([...new Set(Object.values(ids))].map(m=>[m,plan(m)])),original=read(core),changes:{before:string;after:string}[]=[];let adapted=original;
 const replace=(before:string,after:string)=>{assert.equal(adapted.split(before).length,2);adapted=adapted.replace(before,after);changes.push({before,after});};
 const start=original.indexOf('const exportsByModule:'),end=original.indexOf('\n};',start)+3;assert(start>0&&end>start);
 replace(original.slice(start,end),'const exportsByModule:Record<string,[string,number,number]>='+JSON.stringify(Object.fromEntries(Object.keys(setups).map(m=>[m,['__presentation',900,760]])))+';\nconst mainSetups='+JSON.stringify(setups)+';\nconst exactIds='+JSON.stringify(ids)+';');
 replace("assert(text.includes('module.exports.'+exportName+'=' )||text.includes('module.exports.'+exportName+' ='));","assert.equal(exactIds[id],module);for(const r of mainSetups[module].authorities)assert.equal(sha(r.path),r.sha256);");
 replace("return {module,file,sha256:sha(file),kind:'unchanged-export',exportName,viewport:{width,height},","return {module,file,sha256:sha(file),kind:'unchanged-export',exportName,mainSetup:mainSetups[module],viewport:{width,height},");
 replace('loaded._compile(read(privateFile),privateFile);',"loaded._compile(read(privateFile)+'\\nmodule.exports.__presentation=async(page,address,density,h)=>{'+p.recipe.mainSetup.body+'};\\n',privateFile);");
 replace("new URL('./'+f,import.meta.url)","new URL('./'+f,"+JSON.stringify(pathToFileURL(core).href)+")");
 adapted=adapted.replace(/from '(\.\/[^']+)'/g,(before,spec)=>{const after="from '"+new URL(spec,pathToFileURL(core)).href+"'";changes.push({before,after});return after;});
 let reversed=adapted;for(const c of [...changes].reverse())reversed=reversed.replace(c.after,c.before);assert.equal(reversed,original);
 const folder=path.join(viewer,'target/course-presentation/main-page-image-helper',hash(JSON.stringify({self:sha(self),core:sha(core),adapted})).slice(0,20)),file=path.join(folder,'worker.ts');if(fs.existsSync(file))assert.equal(read(file),adapted);else write(file,adapted);
 const bridge=path.join(folder,'bridge.json');if(!fs.existsSync(bridge))write(bridge,JSON.stringify({status:'SOURCE_ONLY_NOT_EXECUTED',helper:{path:self,sha256:sha(self)},original:{path:core,sha256:sha(core)},adapted:{path:file,sha256:sha(file)},ids,setups,changes,reverseExact:true},null,2)+'\n');return {api:await import(pathToFileURL(file).href),bridge};
}
export async function prepareMainPageScene(id:string,built:string){assert(mainPageImageRecipe(id));const{api}=await implementation();return api.prepareExportedScene(id,built);}
export async function runMainPageScene(prepared:string){const{api,bridge}=await implementation(),r=await api.runExportedScene(prepared),record={...r,mainPageSourceBridge:{path:bridge,sha256:sha(bridge)}};const file=path.join(r.folder,'main-page-capture.json');assert(!fs.existsSync(file));write(file,JSON.stringify(record,null,2)+'\n');return {...record,record:file};}
if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,id,file]=process.argv.slice(2);const result=mode==='recipe'?mainPageImageRecipe(id):mode==='source-check'?(await implementation()).bridge:mode==='prepare'?await prepareMainPageScene(id,file):mode==='run'?await runMainPageScene(id):null;assert(result);console.log(JSON.stringify(result,null,2));}
