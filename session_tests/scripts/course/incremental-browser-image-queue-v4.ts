import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {hash,json,viewer,write} from './model.ts';
const self=fileURLToPath(import.meta.url),args=process.argv.slice(2);
const option=(name:string,fallback='')=>{const i=args.indexOf(name);return i<0?fallback:args[i+1];};
const cohort=path.resolve(option('--cohort')),c=json(cohort),chunk=Number(option('--chunk','20'));
assert.equal(c.total,379);assert(Number.isSafeInteger(chunk)&&chunk>=1&&chunk<=20);assert(!args.includes('--limit'));
const pauseFile=path.resolve(option('--pause-file',path.join(c.directory,'pause.request')));assert(pauseFile.startsWith(path.resolve(c.directory)+path.sep));
const forwarded=args.filter((_,i)=>!['--chunk','--pause-file'].includes(args[i])&&!['--chunk','--pause-file'].includes(args[i-1]));
const runner=fileURLToPath(new URL('./incremental-browser-image-batch-v5.ts',import.meta.url)),loader=fileURLToPath(new URL('../typescript.mjs',import.meta.url));
const runDir=path.join(c.directory,'bounded-queue-'+new Date().toISOString().replace(/[:.]/g,'-'));fs.mkdirSync(runDir,{recursive:false});
const sourcePins=Object.fromEntries([self,runner,loader,cohort].map(f=>[f,hash(fs.readFileSync(f))]));
const receipt:any={status:'RUNNING_BOUNDED_IMAGE_QUEUE',sourcePins,cohort,pauseFile,chunk,scope:'Operational serial20-case boundaries only. Each original source/checker/build/GPU/profile assertion remains in the unchanged source-pinned batch helper. No aggregate native/callback acceptance.',chunks:[]};
const save=()=>write(path.join(runDir,'receipt.json'),JSON.stringify(receipt,null,2)+'\n');save();
try{while(true){
 for(const[f,h]of Object.entries(sourcePins))assert.equal(hash(fs.readFileSync(f)),h,f);
 while(fs.existsSync(pauseFile)){receipt.status='AWAITING_EXPLICIT_IMAGE_LANE_RELEASE';save();await new Promise(r=>setTimeout(r,1000));}
 const before=json(path.join(c.directory,'progress.json'));if(before.remaining===0){receipt.status='ALL379_CAPTURED_OR_RETAINED; per-image publication/visual scopes remain separate';save();break;}
 receipt.status='RUNNING_BOUNDED_IMAGE_QUEUE';save();const started=new Date().toISOString();
 const exitCode=await new Promise((resolve,reject)=>{const child=spawn(process.execPath,['--import',pathToFileURL(loader).href,runner,'run',...forwarded,'--limit',String(chunk)],{cwd:viewer,windowsHide:true,stdio:'inherit'});child.once('error',reject);child.once('exit',resolve);});
 const after=json(path.join(c.directory,'progress.json'));receipt.chunks.push({started,finished:new Date().toISOString(),exitCode,capturedBefore:before.completed,capturedAfter:after.completed,remaining:after.remaining});save();assert.equal(exitCode,0,'Original batch failure preserved; explicit diagnosis required');assert(after.completed>before.completed,'No progress; avoid unbounded idle loop');
 receipt.status='SAFE_IMAGE_LANE_BOUNDARY';save();
}}catch(error){receipt.status='STOPPED_WITH_RETAINED_FAILURE';receipt.error=String(error);save();throw error;}
