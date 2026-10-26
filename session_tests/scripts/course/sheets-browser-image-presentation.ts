import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {course,docs,hash,read,viewer,write} from './model.ts';

const self=fileURLToPath(import.meta.url),core=fileURLToPath(new URL('./exported-browser-image-recipes.ts',import.meta.url));
const sha=(f:string)=>hash(fs.readFileSync(f));
const id='71aj-exercise-complete-and-ranged-original-drawings';
const prefix="    await require('./nested-placements.cjs')(original,helpers);";
const inherited="    for(const line of ['Example Nested Instances','Fit','View Isometric','Select Next','Select Next'])await helpers.command(page,line);";

export function sheetsImageRecipe(value=id){
    assert.equal(value,id);const step=course().steps.find(s=>s.id===id);assert(step?.browser_check?.endsWith('/sheets.cjs'));
    const file=path.join(docs,step.browser_check),parent=path.join(docs,'journey/checks/browser/nested-placements.cjs');
    assert.equal(read(file).split(prefix).length,2);assert.equal(read(parent).split(inherited).length,2);
    const replacement=inherited.replaceAll('helpers.command(page,','helpers.command(original,');
    return {id,file,sha256:sha(file),parent:{path:parent,sha256:sha(parent)},prefix,replacement,
        scope:'Entire original sheets body and every original sheet assertion, including its ranged child-context body. Only the inherited nested-placement callback is replaced by its exact final isometric setup commands. No inherited callback acceptance.'};
}

async function implementation(){
    const recipe=sheetsImageRecipe(),source=read(core),changes:{before:string;after:string}[]=[];let adapted=source;
    const replace=(before:string,after:string)=>{assert.equal(adapted.split(before).length,2);adapted=adapted.replace(before,after);changes.push({before,after});};
    const start=source.indexOf('const exportsByModule:'),end=source.indexOf('\n};',start)+3;assert(start>0&&end>start);
    replace(source.slice(start,end),"const exportsByModule:Record<string,[string,number,number]>={'sheets':['__presentation',900,760]};\nconst sheetsRecipe="+JSON.stringify(recipe)+';');
    replace("assert(text.includes('module.exports.'+exportName+'=' )||text.includes('module.exports.'+exportName+' ='));","assert.equal(text.split(sheetsRecipe.prefix).length,2);assert.equal(sha(file),sheetsRecipe.sha256);");
    replace("return {module,file,sha256:sha(file),kind:'unchanged-export',exportName,viewport:{width,height},","return {module,file,sha256:sha(file),kind:'unchanged-export',exportName,inheritedSetup:sheetsRecipe,viewport:{width,height},");
    replace('loaded._compile(read(privateFile),privateFile);',"const original=read(privateFile);assert.equal(original.split(sheetsRecipe.prefix).length,2);loaded._compile(original.replace(sheetsRecipe.prefix,sheetsRecipe.replacement)+'\\nmodule.exports.__presentation=async(page,address,density,h)=>module.exports(page,h);\\n',privateFile);");
    replace("new URL('./'+f,import.meta.url)","new URL('./'+f,"+JSON.stringify(pathToFileURL(core).href)+")");
    adapted=adapted.replace(/from '(\.\/[^']+)'/g,(before,spec)=>{const after="from '"+new URL(spec,pathToFileURL(core)).href+"'";changes.push({before,after});return after;});
    let reversed=adapted;for(const c of [...changes].reverse())reversed=reversed.replace(c.after,c.before);assert.equal(reversed,source);
    const folder=path.join(viewer,'target/course-presentation/sheets-image-helper',hash(JSON.stringify({self:sha(self),core:sha(core),recipe,adapted})).slice(0,20));
    const worker=path.join(folder,'worker.ts');if(fs.existsSync(worker))assert.equal(read(worker),adapted);else write(worker,adapted);
    const bridge=path.join(folder,'bridge.json'),proof={status:'SOURCE_ONLY_NOT_EXECUTED',helper:{path:self,sha256:sha(self)},core:{path:core,sha256:sha(core)},worker:{path:worker,sha256:sha(worker)},recipe,changes,reverseExact:true};
    if(!fs.existsSync(bridge))write(bridge,JSON.stringify(proof,null,2)+'\n');
    return {api:await import(pathToFileURL(worker).href),bridge};
}
export async function prepareSheetsScene(built:string){const{api}=await implementation();return api.prepareExportedScene(id,built);}
export async function runSheetsScene(prepared:string){const{api,bridge}=await implementation(),r=await api.runExportedScene(prepared);const record={...r,sheetsSourceBridge:{path:bridge,sha256:sha(bridge)},scope:sheetsImageRecipe().scope};const file=path.join(r.folder,'sheets-capture.json');assert(!fs.existsSync(file));write(file,JSON.stringify(record,null,2)+'\n');return {...record,record:file};}
if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,file]=process.argv.slice(2);const result=mode==='source-check'?(await implementation()).bridge:mode==='prepare'?await prepareSheetsScene(file):mode==='run'?await runSheetsScene(file):null;assert(result);console.log(JSON.stringify(result,null,2));}
