import fs from 'node:fs';import path from 'node:path';import assert from 'node:assert/strict';import {fileURLToPath,pathToFileURL} from 'node:url';
import {hash,json,read,viewer,write} from './model.ts';
const self=fileURLToPath(import.meta.url),original=fileURLToPath(new URL('./browser-image-fallback.ts',import.meta.url)),transport=fileURLToPath(new URL('./owned-deep-file-buffer-transport.cjs',import.meta.url));
const sha=(f:string)=>hash(fs.readFileSync(f)),ref=(f:string)=>({path:path.resolve(f),sha256:sha(f)});
export async function captureOwnedFileFallback(id:string,builtFile:string){
 builtFile=path.resolve(builtFile);const built=json(builtFile);assert.equal(built.id,id);
 const identity={helper:ref(self),fallback:ref(original),transport:ref(transport),actualBuild:ref(builtFile),originalWorker:ref(built.worker)},key=hash(JSON.stringify(identity)).slice(0,20),folder=path.join(viewer,'target/course-presentation/owned-file-fallback',id+'-'+key);
 const worker=path.join(folder,'worker.cjs'),receipt=path.join(folder,'transport.jsonl');
 const workerSource="const fs=require('node:fs'),assert=require('node:assert/strict'),crypto=require('node:crypto');const sha=f=>crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');\n"+
 'assert.equal(sha('+JSON.stringify(transport)+'),'+JSON.stringify(sha(transport))+');assert.equal(sha('+JSON.stringify(built.worker)+'),'+JSON.stringify(sha(built.worker))+');\n'+
 'require('+JSON.stringify(transport)+').install('+JSON.stringify({workspace:built.workspace,receipt})+');\nrequire('+JSON.stringify(built.worker)+');\n';
 if(fs.existsSync(worker))assert.equal(read(worker),workerSource);else write(worker,workerSource);
 const facade={...built,folder,preparedFile:path.join(folder,'prepared.json'),worker,workerSha256:sha(worker),pins:{...built.pins,[self]:sha(self),[transport]:sha(transport),[built.worker]:sha(built.worker)},originalActualBuild:ref(builtFile),ownedFileTransport:identity};
 if(fs.existsSync(facade.preparedFile))assert.deepEqual(json(facade.preparedFile),facade);else write(facade.preparedFile,JSON.stringify(facade,null,2)+'\n');
 const source=read(original),before='const built = withExpectedSequence(() => buildImageBrowser(id,2)); check(built);',after=`const built = json(${JSON.stringify(facade.preparedFile)}); assert.equal(built.id,id); check(built);`;
 assert.equal(source.split(before).length,2);const changes:any[]=[];let adapted=source.replace(before,after);
 adapted=adapted.replace(/from '(\.\/[^']+)'/g,(match,spec)=>{const replacement="from '"+new URL(spec,pathToFileURL(original)).href+"'";changes.push({match,replacement});return replacement;});
 adapted=adapted.replace(/new URL\('([^']+)',\s*import\.meta\.url\)/g,(match,spec)=>{const replacement='new URL('+JSON.stringify(new URL(spec,pathToFileURL(original)).href)+')';changes.push({match,replacement});return replacement;});
 let reversed=adapted;for(const p of changes)reversed=reversed.replace(p.replacement,p.match);assert.equal(reversed.replace(after,before),source);
 const adapter=path.join(folder,'adapter.ts');if(fs.existsSync(adapter))assert.equal(read(adapter),adapted);else write(adapter,adapted);
 const bridge={status:'SOURCE_BOUND_OWNED_FILE_TRANSPORT_ADAPTER',...identity,facade:ref(facade.preparedFile),worker:ref(worker),adapter:ref(adapter),before,after,changes,reverseExact:true,scope:'Original fallback/canonical callback source and every assertion unchanged. Only source-bound actual build handoff and owned Windows-deep-file path to exact named byte payload transport differ. External/short paths unchanged; each conversion recorded with exact file/buffer SHA.'};
 const bridgeFile=path.join(folder,'bridge.json');if(fs.existsSync(bridgeFile))assert.deepEqual(json(bridgeFile),bridge);else write(bridgeFile,JSON.stringify(bridge,null,2)+'\n');
 const api=await import(pathToFileURL(adapter).href),result=await api.captureFallback(id);
 const record={...result,actualIncrementalBuild:ref(builtFile),ownedFileTransport:{bridge:ref(bridgeFile),receipt:fs.existsSync(receipt)?ref(receipt):null},scope:bridge.scope};
 const file=path.join(folder,'capture.json');if(fs.existsSync(file))assert.deepEqual(json(file),record);else write(file,JSON.stringify(record,null,2)+'\n');return{...record,record:file};
}
if(process.argv[1]&&path.resolve(process.argv[1])===self){const[id,file]=process.argv.slice(2);assert(id&&file);console.log(JSON.stringify(await captureOwnedFileFallback(id,file),null,2));}
