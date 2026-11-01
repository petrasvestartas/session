import fs from 'node:fs';import path from 'node:path';import assert from 'node:assert/strict';import {fileURLToPath,pathToFileURL} from 'node:url';
import {hash,json,read,viewer,write} from './model.ts';
const self=fileURLToPath(import.meta.url),original=fileURLToPath(new URL('./browser-image-fallback.ts',import.meta.url)),transport=fileURLToPath(new URL('./owned-deep-file-buffer-transport.cjs',import.meta.url));
const sha=(f:string)=>hash(fs.readFileSync(f)),ref=(f:string)=>({path:path.resolve(f),sha256:sha(f)});
export function stageDeclaredPbAssets(built:any){
 const native=path.resolve(built.native),workspace=path.resolve(built.workspace),rows:any[]=[];
 const checked=(root:string,file:string)=>{
  assert.equal(fs.realpathSync(root),root,'Asset root cannot be redirected');
  const relative=path.relative(root,file);assert(relative&&!relative.startsWith('..')&&!path.isAbsolute(relative));
  let current=root;for(const part of relative.split(path.sep)){
   current=path.join(current,part);
   if(fs.existsSync(current)){assert(!fs.lstatSync(current).isSymbolicLink());assert.equal(fs.realpathSync(current),current,'Asset path cannot be redirected');}
  }
 };
 for(const[name,digest]of Object.entries(built.source.assets)){
  if(!name.startsWith('assets/pb/'))continue;
  assert(/^assets\/pb\/(?:[A-Za-z0-9_-]+\/)*[A-Za-z0-9_-]+\.pb$/.test(name),'Invalid declared PB asset path');
  assert(typeof digest==='string'&&/^[a-f0-9]{64}$/.test(digest));
  const source=path.resolve(native,name),destination=path.resolve(workspace,name);checked(native,source);checked(workspace,destination);
  assert(fs.statSync(source).isFile());assert.equal(sha(source),digest,'Declared PB source mismatch');
  if(fs.existsSync(destination)){assert(fs.statSync(destination).isFile());assert.equal(sha(destination),digest,'Declared PB destination collision');}
  rows.push({name,source:{path:source,sha256:digest},destination:{path:destination,sha256:digest},bytes:fs.statSync(source).size});
 }
 for(const row of rows){
  const source=row.source.path,destination=row.destination.path;checked(native,source);checked(workspace,destination);
  assert.equal(sha(source),row.source.sha256);
  fs.mkdirSync(path.dirname(destination),{recursive:true});checked(workspace,destination);
  if(!fs.existsSync(destination))fs.copyFileSync(source,destination,fs.constants.COPYFILE_EXCL);
  assert.equal(sha(destination),row.destination.sha256);
 }
 return rows;
}
export async function captureOwnedFileFallback(id:string,builtFile:string){
 builtFile=path.resolve(builtFile);const built=json(builtFile);assert.equal(built.id,id);
 const declaredPbAssets=stageDeclaredPbAssets(built),assetPins=Object.fromEntries(declaredPbAssets.flatMap(row=>[row.source,row.destination]).map(item=>[item.path,item.sha256]));
 const identity={declaredPbAssets,helper:ref(self),fallback:ref(original),transport:ref(transport),actualBuild:ref(builtFile),originalWorker:ref(built.worker)},key=hash(JSON.stringify(identity)).slice(0,20),folder=path.join(viewer,'target/course-presentation/owned-file-fallback',id+'-'+key);
 const worker=path.join(folder,'worker.cjs'),receipt=path.join(folder,'transport.jsonl');
 const workerSource="const fs=require('node:fs'),assert=require('node:assert/strict'),crypto=require('node:crypto');const sha=f=>crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');\n"+
 'assert.equal(sha('+JSON.stringify(transport)+'),'+JSON.stringify(sha(transport))+');assert.equal(sha('+JSON.stringify(built.worker)+'),'+JSON.stringify(sha(built.worker))+');\n'+
 'require('+JSON.stringify(transport)+').install('+JSON.stringify({workspace:built.workspace,receipt})+');\nrequire('+JSON.stringify(built.worker)+');\n';
 if(fs.existsSync(worker))assert.equal(read(worker),workerSource);else write(worker,workerSource);
 const facade={...built,folder,preparedFile:path.join(folder,'prepared.json'),worker,workerSha256:sha(worker),pins:{...built.pins,...assetPins,[self]:sha(self),[transport]:sha(transport),[built.worker]:sha(built.worker)},originalActualBuild:ref(builtFile),ownedFileTransport:identity};
 if(fs.existsSync(facade.preparedFile))assert.deepEqual(json(facade.preparedFile),facade);else write(facade.preparedFile,JSON.stringify(facade,null,2)+'\n');
 const source=read(original),before='const built = withExpectedSequence(() => buildImageBrowser(id,2)); check(built);',after=`const built = json(${JSON.stringify(facade.preparedFile)}); assert.equal(built.id,id); check(built);`;
 assert.equal(source.split(before).length,2);const changes:any[]=[];let adapted=source.replace(before,after);
 adapted=adapted.replace(/from '(\.\/[^']+)'/g,(match,spec)=>{const replacement="from '"+new URL(spec,pathToFileURL(original)).href+"'";changes.push({match,replacement});return replacement;});
 adapted=adapted.replace(/new URL\('([^']+)',\s*import\.meta\.url\)/g,(match,spec)=>{const replacement='new URL('+JSON.stringify(new URL(spec,pathToFileURL(original)).href)+')';changes.push({match,replacement});return replacement;});
 let reversed=adapted;for(const p of changes)reversed=reversed.replace(p.replacement,p.match);assert.equal(reversed.replace(after,before),source);
 const adapter=path.join(folder,'adapter.ts');if(fs.existsSync(adapter))assert.equal(read(adapter),adapted);else write(adapter,adapted);
 const bridge={status:'SOURCE_BOUND_OWNED_FILE_TRANSPORT_ADAPTER',...identity,facade:ref(facade.preparedFile),worker:ref(worker),adapter:ref(adapter),before,after,changes,reverseExact:true,scope:'Original fallback/canonical callback source and every assertion unchanged. Only source-bound actual build handoff and owned Windows-deep-file path to exact named byte payload transport differ. Declared PB fixtures are staged byte-for-byte from the source-bound native snapshot into their original workspace paths and pinned. External/short paths unchanged; each conversion recorded with exact file/buffer SHA.'};
 const bridgeFile=path.join(folder,'bridge.json');if(fs.existsSync(bridgeFile))assert.deepEqual(json(bridgeFile),bridge);else write(bridgeFile,JSON.stringify(bridge,null,2)+'\n');
 const api=await import(pathToFileURL(adapter).href),result=await api.captureFallback(id);
 for(const[file,digest]of Object.entries(assetPins))assert.equal(sha(file),digest,file);
 const record={...result,actualIncrementalBuild:ref(builtFile),ownedFileTransport:{bridge:ref(bridgeFile),receipt:fs.existsSync(receipt)?ref(receipt):null},scope:bridge.scope};
 const file=path.join(folder,'capture.json');if(fs.existsSync(file))assert.deepEqual(json(file),record);else write(file,JSON.stringify(record,null,2)+'\n');return{...record,record:file};
}
if(process.argv[1]&&path.resolve(process.argv[1])===self){const[id,file]=process.argv.slice(2);assert(id&&file);console.log(JSON.stringify(await captureOwnedFileFallback(id,file),null,2));}
