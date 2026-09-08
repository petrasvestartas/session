/** Chapter12 verifies the object/edge portion of the supplied interaction gate. */
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const assert=require('node:assert/strict');
const Module=require('node:module');
const file=path.join(__dirname,'fixtures/interaction.cjs');
const hashes=JSON.parse(fs.readFileSync(path.join(__dirname,'fixtures/test-sources.json'),'utf8'));
let source=fs.readFileSync(file,'utf8');
assert.equal(crypto.createHash('sha256').update(source).digest('hex'),hashes['interaction.cjs']);
/** Fail explicitly when the supplied phase boundary changes. */
function unique(value){assert.equal(source.split(value).length,2,'unique interaction phase anchor');return source.indexOf(value);}
const start=unique("      await key(page,'F10');state=await snapshot(page);\n      assert.equal(state.selection.Controls?.parent,parent,");
const end=unique('      history.push({kind:spec.kind,state});');
source=source.slice(0,start)+source.slice(end);
// Without F10, Escape retains an edge-mode parent but clears ordinary object selection.
source=source.replace('      history.push({kind:spec.kind,state});',
  '      const escapeParent = state.selection.Edge ? parent : null;\n      history.push({kind:spec.kind,state});');
source=source.replace("assert.equal(state.selection,'Object');assert.equal(state.selected,parent);",
  "assert.equal(state.selection,'Object');assert.equal(state.selected,escapeParent);");
source=source.replace("await click(page,project(state,mesh.edge),true);await key(page,'F10');", "await click(page,project(state,mesh.edge),true);");
source=source.replace("if(state.selected_guid)assert.equal(state.selected_guid,spec.guid);", "assert.equal(state.identity[1],spec.guid,'source GUID survives object picking');");
source=source.replace('object/edge/controls, identity, repeated F10, Esc, parent replacement and CSS tolerance','object/edge, source GUID, Esc, parent replacement and CSS tolerance');
const runner=new Module(file,module);runner.filename=file;runner.paths=Module._nodeModulePaths(path.dirname(file));runner._compile(source,file);
