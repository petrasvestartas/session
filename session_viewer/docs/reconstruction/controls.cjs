/** Chapter13 uses serial bounded metadata reads; source-selection assertions stay unchanged. */
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const assert=require('node:assert/strict');
const Module=require('node:module');
const file=path.join(__dirname,'fixtures/streamed-controls.cjs');
const hashes=JSON.parse(fs.readFileSync(path.join(__dirname,'fixtures/test-sources.json'),'utf8'));
let source=fs.readFileSync(file,'utf8');
assert.equal(crypto.createHash('sha256').update(source).digest('hex'),hashes['streamed-controls.cjs']);
/** Fail if the supplied test no longer has exactly the expected metadata assertion. */
function replace(old,replacement){assert.equal(source.split(old).length,2,'unique metadata assertion');source=source.replace(old,replacement);}
replace("assert.equal(metadataReads.length,2,'one color header and one cached window locate all seven LOD arrays and the ID header');", "assert.equal(metadataReads.length,16,'chapter13 serial reader visits all seven LOD arrays and the ID header');");
replace("assert.deepEqual(metadataReads.map(r=>r.length),[16,65536],'metadata does not fetch the full fixed32 ID payload');", "assert(metadataReads.every(r=>r.length<=1024),'small fixture metadata ranges never fetch the full fixed32 ID payload');");
const runner=new Module(file,module);runner.filename=file;runner.paths=Module._nodeModulePaths(path.dirname(file));runner._compile(source,file);
