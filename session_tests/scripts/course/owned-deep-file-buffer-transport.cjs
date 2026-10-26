const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const assert=require('node:assert/strict');
const sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
module.exports.install=({workspace,receipt})=>{
    const root=fs.realpathSync(workspace);
    assert(root.replaceAll('\\','/').includes('/target/course-presentation/incremental-browser-builds/cases/'));
    assert(root.endsWith(path.sep+'workspace'));
    const implementation=path.resolve(__dirname,'../../../session_viewer/target/course-tools/node_modules/playwright-core/lib/client/fileChooser.js');
    const {FileChooser}=require(implementation),original=FileChooser.prototype.setFiles;
    assert(!original.ownedDeepFileTransport,'A separate immutable image worker installs this transport once');
    const convert=file=>{
        if(typeof file!=='string')return file;
        const absolute=path.resolve(file);
        if(!absolute.startsWith(root+path.sep)||absolute.length<260)return file;
        const real=fs.realpathSync(absolute);assert(real.startsWith(root+path.sep));assert(fs.statSync(real).isFile());
        const bytes=fs.readFileSync(real),digest=sha(bytes),payload={name:path.basename(file),mimeType:'application/octet-stream',buffer:bytes};
        assert.equal(sha(payload.buffer),digest);
        fs.appendFileSync(receipt,JSON.stringify({status:'EXACT_OWNED_DEEP_FILE_BYTES_TRANSPORT',file:absolute,pathLength:absolute.length,name:payload.name,mimeType:payload.mimeType,bytes:bytes.length,sha256:digest,bufferSha256:sha(payload.buffer),fileChooserImplementation:{path:implementation,sha256:sha(fs.readFileSync(implementation))},scope:'Image-only Windows transport; canonical helpers/assertions unchanged. Unknown/external/short paths and original byte payloads pass through unchanged.'})+'\n');
        return payload;
    };
    const adapted=async function(files,options){return original.call(this,Array.isArray(files)?files.map(convert):convert(files),options);};
    adapted.ownedDeepFileTransport=true;FileChooser.prototype.setFiles=adapted;
    return()=>{assert.equal(FileChooser.prototype.setFiles,adapted);FileChooser.prototype.setFiles=original;};
};
