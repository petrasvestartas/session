import fs from'node:fs';import path from'node:path';import assert from'node:assert/strict';import crypto from'node:crypto';import{fileURLToPath,pathToFileURL}from'node:url';
const repo=process.cwd(),viewer=path.join(repo,'session_viewer'),self=fileURLToPath(import.meta.url),sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex'),ref=p=>({path:p,sha256:sha(p)}),json=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const{supportCommand}=await import(pathToFileURL(path.join(repo,'session_tests/scripts/course/supplemental-private-image.ts')).href);
const{png,ppm}=await import(pathToFileURL(path.join(repo,'session_tests/scripts/course/png.ts')).href);
export async function captureFourSampleScenes(oldFile=path.join(viewer,'target/course-presentation/private-scenes-551f30129d3b22b2681d/qualified-three-scene-result.json')){
oldFile=path.resolve(oldFile);const old=json(oldFile);
assert.equal(old.images.length,3);assert.equal(sha(old.executable.path),old.executable.sha256);
const key=crypto.createHash('sha256').update(JSON.stringify({self:ref(self),old:ref(oldFile),override:{VIEWER_MSAA:'4'}})).digest('hex');
const folder=path.join(viewer,'target/course-presentation/current-scenes-four-sample-'+key.slice(0,20));assert(!fs.existsSync(folder));fs.mkdirSync(folder,{recursive:true});
const images=[];for(const row of old.images){
    assert.equal(sha(row.input.path),row.input.sha256);assert.equal(sha(old.executable.path),old.executable.sha256);
    const name=path.basename(row.destination,'.png').replace('current-native-',''),frame=path.join(folder,'current-native-'+name+'.ppm');
    assert.equal(row.settingsFromUnchangedRecipe.VIEWER_MSAA,'1');const settings={...row.settingsFromUnchangedRecipe,VIEWER_MSAA:'4',VIEWER_W:'1280',VIEWER_H:'960',VIEWER_DPR:'2',VIEWER_ADAPTER:'Microsoft'};
    const execution=supportCommand('current-scenes-four-sample-'+key.slice(0,12)+'-'+name,[old.executable.path,frame,row.input.path],viewer,settings);
    assert.equal(sha(row.input.path),row.input.sha256);assert.equal(sha(old.executable.path),old.executable.sha256);
    const decoded=ppm(frame);assert.equal(decoded.width,1280);assert.equal(decoded.height,960);const image=frame.replace('.ppm','.png');fs.writeFileSync(image,png(frame));
    images.push({path:image,destination:row.destination,sha256:sha(image),width:1280,height:960,raw:ref(frame),input:row.input,settings,execution,originalOneSampleImage:{path:row.path,sha256:row.sha256},scope:'Same original maintained scene/PB/pose/eightframe recipe at1280x960/logical640x480; ONLY intendedqualitysetting change:forcedMSAA1to4. No geometry smoothing or source edit.'});
    fs.writeFileSync(path.join(folder,'progress.json'),JSON.stringify({status:'PARTIAL actualcaptures',images,old:ref(oldFile),executable:old.executable,runner:ref(self)},null,2)+'\n');
}
fs.writeFileSync(path.join(folder,'capture.json'),JSON.stringify({status:'THREE_ACTUAL_FOUR_SAMPLE_PRESENTATION_IMAGES_AWAITING_VISUAL_REVIEW',canonicalBehaviorExecuted:false,canonicalValidationProfile:false,images,old:ref(oldFile),executable:old.executable,runner:ref(self),scope:'Explicitimagequalitysuccessor; old1ximages andoriginalsourcepostflightfailure retained. Floor-output ungenerated/unaccepted.'},null,2)+'\n');console.log(path.join(folder,'capture.json'));
return path.join(folder,'capture.json');
}
if(process.argv[1]&&path.resolve(process.argv[1])===self)await captureFourSampleScenes(process.argv[2]);
