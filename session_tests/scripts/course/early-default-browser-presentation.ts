import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {course,docs,hash,read,viewer,write} from './model.ts';
const self=fileURLToPath(import.meta.url),core=fileURLToPath(new URL('./historical-browser-actions-presentation.ts',import.meta.url));
const callbacks:Record<string,string>={ '03a-paint':'03a-paint','03b-memory':'03b-memory','03b-state':'03b-state' };
const sourceHashes:Record<string,string>=JSON.parse('{"03a-paint":"e4a5c9764a26a606ce0989a132d01dcc6226545d47e549081070e70231e77b12","03b-memory":"1b74211d21418ebe6dd4b15cb5b7e14a2a2130790e59f0b804e039c647c86e60","03b-state":"b7df71bba6ab74bd73b67343818616d055bc3a7c37ae3887b8631252d77e0284"}');
const sha=(f:string)=>hash(fs.readFileSync(f));
export function earlyDefaultRecipe(id:string){
 const name=callbacks[id];if(!name)return null;const step=course().steps.find(s=>s.id===id);assert(step);assert(!step.browser_actions?.length);assert.equal(step.browser_check,'journey/checks/browser/'+name+'.cjs');const file=path.join(docs,step.browser_check);assert.equal(sha(file),sourceHashes[id]);
 return{kind:'source-proven-startup-render-only',actions:[],callback:step.browser_check,declaredMetadataStatus:step.browser_status,authority:'The original callback only reads the already-rendered canvas and compares an external previous screenshot; it issues no commands/navigation/scene mutation. Capture this same startup scene at actual DPR2. Original comparator assertions and failed missing-baseline evidence remain unchanged; no comparator or behavior acceptance claimed.'};
}
async function implementation(){
 const source=read(core),start=source.indexOf('export function historicalImageRecipe('),end=source.indexOf('\nfunction check(',start);assert(start>0&&end>start);const replacement='const sourceHashes='+JSON.stringify(sourceHashes)+';\n'+earlyDefaultRecipe.toString().replace('function earlyDefaultRecipe','function historicalImageRecipe');
 // The generated recipe has the same source contracts as this public wrapper.
 let adapted=source.slice(0,start)+replacement+source.slice(end);const changes=[{before:source.slice(start,end),after:replacement}];
 const replace=(before:string,after:string)=>{assert.equal(adapted.split(before).length,2);adapted=adapted.replace(before,after);changes.push({before,after});};
 replace("const self = fileURLToPath(import.meta.url);","const self = fileURLToPath(import.meta.url);\nconst callbacks="+JSON.stringify(callbacks)+';');
 replace("new URL('./'+name, import.meta.url)","new URL('./'+name,"+JSON.stringify(pathToFileURL(core).href)+")");
 replace("'target/course-presentation/historical-action-scenes'","'target/course-presentation/early-default-scenes'");
 adapted=adapted.replace(/from '(\.\/[^']+)'/g,(before,spec)=>{const after="from '"+new URL(spec,pathToFileURL(core)).href+"'";changes.push({before,after});return after;});
 let reversed=adapted;for(const c of [...changes].reverse())reversed=reversed.replace(c.after,c.before);assert.equal(reversed,source);
 const folder=path.join(viewer,'target/course-presentation/early-default-helper',hash(JSON.stringify({self:sha(self),core:sha(core),adapted})).slice(0,20)),file=path.join(folder,'worker.ts');if(fs.existsSync(file))assert.equal(read(file),adapted);else write(file,adapted);
 const bridge={status:'SOURCE_PROVEN_STARTUP_ONLY_TRANSPORT',helper:{path:self,sha256:sha(self)},originalWorker:{path:core,sha256:sha(core)},adaptedWorker:{path:file,sha256:sha(file)},sourceHashes,changes,reverseExact:true,scope:'Only recipe selection/default-empty actions and transport paths differ; actual GPU/DPR/bundle/source/render checks remain original. No frozen comparator assertion changed.'},proof=path.join(folder,'bridge.json');if(!fs.existsSync(proof))write(proof,JSON.stringify(bridge,null,2)+'\n');return{api:await import(pathToFileURL(file).href),proof};
}
export async function prepareEarlyDefaultScene(id:string,builtFile:string){assert(earlyDefaultRecipe(id));return(await implementation()).api.prepareHistoricalScene(id,builtFile);}
export async function runEarlyDefaultScene(preparedFile:string){const{api,proof}=await implementation(),r=await api.runHistoricalScene(preparedFile),result={...r,scope:earlyDefaultRecipe(r.id)!.authority,earlyDefaultBridge:{path:proof,sha256:sha(proof)}},file=path.join(r.folder,'early-default-capture.json');if(fs.existsSync(file))assert.equal(read(file),JSON.stringify(result,null,2)+'\n');else write(file,JSON.stringify(result,null,2)+'\n');return result;}
if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,id,file]=process.argv.slice(2);const r=mode==='prepare'?await prepareEarlyDefaultScene(id,file):mode==='run'?await runEarlyDefaultScene(id):mode==='source-check'?(await implementation()).proof:null;assert(r,'prepare ID BUILT | run PREPARED | source-check');console.log(JSON.stringify(r,null,2));}
