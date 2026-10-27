import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {bindings} from './presentation.ts';
import {filesIn, hash, json, viewer, withExpectedSequence} from './model.ts';

const self=fileURLToPath(import.meta.url),sha=(f:string)=>hash(fs.readFileSync(f));
const ref=(f:string)=>({path:path.resolve(f),sha256:sha(f)});
const save=(f:string,value:any)=>{fs.mkdirSync(path.dirname(f),{recursive:true});fs.writeFileSync(f,JSON.stringify(value,null,2)+'\n',{flag:'wx'});};

export function prepareFinalMetadataSuccessor(cohortFile:string,beforeCourse:string,applicationFile:string){
    cohortFile=path.resolve(cohortFile);beforeCourse=path.resolve(beforeCourse);applicationFile=path.resolve(applicationFile);
    const old=json(cohortFile),priorFile=path.join(path.dirname(cohortFile),'progress.json'),prior=json(priorFile);
    assert.equal(old.total,379);assert.equal(prior.completed,379);assert.equal(prior.remaining,0);
    assert(!Object.values(prior.items).some((r:any)=>r.status==='RUNNING'));
    const courseFile=path.join(viewer,'docs/journey/course.json'),oldPin=old.inputs[courseFile];assert(oldPin);
    assert.equal(sha(beforeCourse),oldPin.sha256);assert.notEqual(sha(courseFile),oldPin.sha256);
    const application=json(applicationFile);assert.equal(application.mode,'APPLY');assert.deepEqual(application.conflicts,[]);
    assert.equal(application.source.sha256,oldPin.sha256);assert.equal(application.result.sha256,sha(courseFile));
    const before=json(beforeCourse),after=json(courseFile),restored=structuredClone(after),changes:any[]=[];
    assert.equal(before.steps.length,after.steps.length);
    for(let i=0;i<before.steps.length;i++){
        const a=before.steps[i],b=after.steps[i],r=restored.steps[i];assert.equal(a.id,b.id);
        if(JSON.stringify(a.image_same_as)!==JSON.stringify(b.image_same_as)){
            changes.push({id:a.id,field:'image_same_as',before:a.image_same_as??null,after:b.image_same_as??null});
            if(Object.hasOwn(a,'image_same_as'))r.image_same_as=a.image_same_as;else delete r.image_same_as;
        }
        if(a.id==='03cf-accept'){
            assert.equal(a.browser_status,'Tab accepts the completion.');assert.equal(b.browser_status,'The field completes known names.');
            changes.push({id:a.id,field:'browser_status',before:a.browser_status,after:b.browser_status});r.browser_status=a.browser_status;
        }
    }
    assert.deepEqual(restored,before,'Only exact03cf startup metadata and image_same_as may change');
    const ordered=(rows:any[])=>rows.map(r=>({id:r.id,field:r.field||r.property,before:r.before,after:r.after})).sort((a,b)=>(a.id+a.field).localeCompare(b.id+b.field));
    assert.deepEqual(ordered(changes),ordered(application.changes),'Receipt must exactly match every actual metadata change');
    const sources=withExpectedSequence(()=>Object.fromEntries(old.queue.map((r:any)=>[r.id,bindings(r.id)])));
    assert.deepEqual(sources,old.sources,'All379 runtime/source/asset/lock/kernel/recipe bindings must remain exact');
    for(const[root,names]of Object.entries(old.sourceFileLists))assert.deepEqual(filesIn(root).sort(),names,root);
    const inputs=structuredClone(old.inputs);
    for(const[f,p]of Object.entries(inputs)as[string,any][]){if(f!==courseFile)assert.equal(sha(f),p.sha256,f);}
    for(const f of[courseFile,self,beforeCourse,applicationFile]){const s=fs.statSync(f);inputs[f]={sha256:sha(f),size:s.size,mtimeMs:s.mtimeMs};}
    const retainedActualRecords:any[]=[];
    for(const row of Object.values(prior.items)as any[]){
        assert(['CAPTURED_AWAITING_ROOT_VISUAL_REVIEW','PUBLISHED_ROOT_REVIEWED'].includes(row.status));
        assert.equal(sha(row.sourceRecord.path),row.sourceRecord.sha256);const actual=json(row.sourceRecord.path);
        assert.deepEqual(actual.source,sources[row.id]);assert.deepEqual(actual.image,row.image);assert.equal(sha(row.image.path),row.image.sha256);
        const resources=actual.resources||actual.host?.resources;assert.equal(resources.exitCode,0);assert.equal(resources.timedOut,false);
        retainedActualRecords.push({id:row.id,sourceRecord:row.sourceRecord,image:row.image});
    }
    assert.equal(retainedActualRecords.length,379);
    const queue=old.queue.map((r:any)=>r.id==='03cf-accept'?{...r,plannedRoute:'fallback',recipeContract:null}:r);
    const qualification={status:'PASS exact metadata-only source relevance; no new capture or behavior execution',oldCohortRecord:ref(cohortFile),oldProgressRecord:ref(priorFile),beforeCourse:ref(beforeCourse),currentCourse:ref(courseFile),application:ref(applicationFile),helper:ref(self),changes,all379RuntimeBindingsEqual:true,completed:379,retainedActualRecords};
    const identity={...old,inputs,queue,finalMetadataQualification:qualification};delete identity.key;delete identity.directory;
    const key=hash(JSON.stringify(identity)),directory=path.join(viewer,'target/course-presentation/incremental-browser-batches',key.slice(0,20));
    const file=path.join(directory,'cohort.json'),proof=path.join(directory,'metadata-qualification.json');assert(!fs.existsSync(directory));
    const c={...identity,key,directory,status:'COMPLETE_SOURCE_EQUIVALENT_METADATA_SUCCESSOR',scope:old.scope+' Final metadata-only successor retains all actual identities; future03cf uses ordinary fallback against corrected canonical startup metadata.'};
    const p={...prior,items:structuredClone(prior.items),queueStatus:'COMPLETE_SOURCE_EQUIVALENT_METADATA_SUCCESSOR',updated:new Date().toISOString()};
    p.items['03cf-accept'].plannedRoute='fallback';p.items['03cf-accept'].recipeContract=null;
    save(file,c);save(path.join(directory,'progress.json'),p);save(proof,{...qualification,newCohortRecord:ref(file)});
    return{newCohortRecord:ref(file),oldCohortRecord:ref(cohortFile),privateQualification:ref(proof),all379RuntimeBindingsEqual:true,completed:379,retainedActualRecords};
}

if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,...args]=process.argv.slice(2);assert.equal(mode,'prepare');assert.equal(args.length,3);console.log(JSON.stringify(prepareFinalMetadataSuccessor(...args as[string,string,string]),null,2));}
