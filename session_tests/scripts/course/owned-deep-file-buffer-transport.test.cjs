const {EventEmitter}=require('node:events');
const path=require('node:path');
const crypto=require('node:crypto');
const assert=require('node:assert/strict');
const {test}=require('node:test');
const {install}=require('./owned-deep-file-buffer-transport.cjs');
const transportFile=require.resolve('./owned-deep-file-buffer-transport.cjs');
const sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');

function fixture(){
    const workspace=path.resolve('transport-fixture/target/course-presentation/incremental-browser-builds/cases/lesson/workspace');
    const receipt=path.resolve('transport-fixture/receipt.jsonl'),short=path.join(workspace,'short.pb');
    const deep=path.join(workspace,...Array(4).fill('nested-'.repeat(12)),'exact.pb'),external=path.join(workspace+'-external','x'.repeat(200),'external.pb');
    assert(short.length<260);assert(deep.length>=260);
    const files=new Map(),redirects=new Map(),records=[],launchCalls=[],contextCalls=[];
    const controls={receiptError:null,launchError:null,contextError:null,launchPromise:null,contextPromise:null};
    const bytes=Buffer.from([0,1,2,10,255,128,13,0]);files.set(deep,bytes);
    const index=path.resolve('transport-fixture/sdk/index.js'),bundle=path.join(path.dirname(index),'lib/coreBundle.js'),packageFile=path.join(path.dirname(index),'package.json');
    files.set(index,Buffer.from('actual root index fixture'));files.set(bundle,Buffer.from('actual client core bundle fixture'));
    files.set(packageFile,Buffer.from(JSON.stringify({name:'playwright-core',version:'1.61.1-fixture'})));files.set(transportFile,Buffer.from('transport source fixture'));
    const io={
        realpathSync:file=>{file=path.resolve(file);if(file===workspace||files.has(file)||redirects.has(file))return redirects.get(file)||file;throw Error('ENOENT fixture: '+file);},
        statSync:file=>({isFile:()=>files.has(file)}),
        readFileSync:(file,encoding)=>{const value=files.get(path.resolve(file));assert(value,'Missing fake file: '+file);return encoding?value.toString(encoding):value;},
        appendFileSync:(file,text)=>{if(controls.receiptError)throw controls.receiptError;assert.equal(file,receipt);records.push(JSON.parse(text));files.set(file,Buffer.concat([files.get(file)||Buffer.alloc(0),Buffer.from(text)]));},
    };
    class Page extends EventEmitter{
        waitForEvent(event,options={}){return new Promise((resolve,reject)=>{
            const listener=value=>{try{if(options.predicate&&!options.predicate(value))return;this.off(event,listener);resolve(value);}catch(error){this.off(event,listener);reject(error);}};
            this.on(event,listener);
        });}
    }
    class Context extends EventEmitter{
        constructor(pages=[]){super();this.items=pages;}
        pages(){return [...this.items];}
        async newPage(){const page=new Page();this.items.push(page);this.emit('page',page);return page;}
    }
    class Browser{
        constructor(contexts=[]){this.items=contexts;}
        contexts(){return [...this.items];}
        newContext(...args){contextCalls.push({receiver:this,args});if(controls.contextError)throw controls.contextError;
            return controls.contextPromise||Promise.resolve(new Context()).then(context=>{this.items.push(context);return context;});}
        async newPage(...args){return(await this.newContext(...args)).newPage();}
    }
    class Chooser{
        constructor(){this.calls=[];this.promise=Promise.resolve({submitted:true});}
        setFiles(files,...args){this.calls.push({receiver:this,files,args});if(this.error)throw this.error;return this.promise;}
    }
    const existingPage=new Page(),existingContext=new Context([existingPage]),browser=new Browser([existingContext]);
    const sdk={chromium:{launch(...args){launchCalls.push({receiver:this,args});if(controls.launchError)throw controls.launchError;return controls.launchPromise||Promise.resolve(browser);}}};
    const coreModule={filename:bundle,exports:{}},rootModule={filename:index,exports:sdk,children:[coreModule]};
    const runtime={fs:io,require:request=>{assert.equal(request,'playwright-core');return sdk;},cache:{[index]:rootModule,[bundle]:coreModule}};
    return{workspace,receipt,short,deep,external,bytes,files,redirects,records,controls,sdk,browser,existingPage,existingContext,launchCalls,contextCalls,Page,Context,Chooser,
        index,bundle,packageFile,rootModule,runtime,install:()=>install({workspace,receipt},runtime)};
}

test('actual page dispatch adds no filechooser listener and preserves on, once, predicate and waitForEvent behavior',async()=>{
    const f=fixture(),page=f.existingPage,events=[],options={timeout:123};
    const on=function(chooser){events.push({kind:'on',receiver:this,chooser});assert(chooser.setFiles.ownedDeepFileTransport);};
    page.on('filechooser',on);page.once('filechooser',function(chooser){events.push({kind:'once',receiver:this,chooser});});
    const restore=f.install();await f.sdk.chromium.launch();
    assert.equal(page.listenerCount('filechooser'),2);
    let predicatePromise;
    const waited=page.waitForEvent('filechooser',{predicate:chooser=>{predicatePromise=chooser.setFiles(f.deep,options);return true;}});
    assert.equal(page.listenerCount('filechooser'),3);
    const chooser=new f.Chooser(),original=chooser.setFiles;
    assert.equal(page.emit('filechooser',chooser),true);assert.equal(await waited,chooser);await predicatePromise;
    assert.equal(page.listenerCount('filechooser'),1);assert.deepEqual(events.map(row=>row.kind),['on','once']);assert(events.every(row=>row.receiver===page&&row.chooser===chooser));
    assert.equal(chooser.calls[0].files.buffer,f.bytes);assert.equal(chooser.calls[0].args[0],options);
    restore();assert.equal(chooser.setFiles,original);assert.equal(page.listenerCount('filechooser'),1);assert.equal(page.listeners('filechooser')[0],on);
});

test('on/once callbacks and a false wait predicate receive adapted choosers before normal dispatch',async()=>{
    const f=fixture(),page=f.existingPage,first=new f.Chooser(),second=new f.Chooser(),results=[];
    page.on('filechooser',chooser=>results.push(chooser.setFiles(f.deep)));
    page.once('filechooser',chooser=>results.push(chooser.setFiles(f.short)));
    const restore=f.install();await f.sdk.chromium.launch();let resolved=false;
    const waited=page.waitForEvent('filechooser',{predicate:chooser=>{results.push(chooser.setFiles(f.deep));return chooser!==first;}});
    waited.then(()=>{resolved=true;});assert.equal(page.listenerCount('filechooser'),3);
    page.emit('filechooser',first);await Promise.all(results);assert.equal(resolved,false);assert.equal(page.listenerCount('filechooser'),2);
    assert.equal(first.calls.length,3);assert.equal(first.calls[0].files.buffer,f.bytes);assert.equal(first.calls[1].files,f.short);
    page.emit('filechooser',second);assert.equal(await waited,second);await Promise.all(results);assert.equal(page.listenerCount('filechooser'),1);
    restore();assert.equal(page.listenerCount('filechooser'),1);
});

test('owned deep bytes and actual loaded SDK provenance are recorded; payloads, short paths and external paths pass through',async()=>{
    const f=fixture(),restore=f.install();await f.sdk.chromium.launch();const chooser=new f.Chooser();f.existingPage.emit('filechooser',chooser);
    const payload={name:'direct.pb',mimeType:'application/octet-stream',buffer:Buffer.from('direct')},options={timeout:456},extra={extra:true};
    for(const value of [payload,f.short,f.external,Buffer.from('raw'),undefined]){
        assert.equal(chooser.setFiles(value,options,extra),chooser.promise);
        const call=chooser.calls.at(-1);assert.equal(call.receiver,chooser);assert.equal(call.files,value);assert.deepEqual(call.args,[options,extra]);
    }
    const input=[f.deep,payload,f.short,f.external];assert.equal(chooser.setFiles(input,options),chooser.promise);
    const converted=chooser.calls.at(-1).files;assert.notEqual(converted,input);assert.deepEqual(input,[f.deep,payload,f.short,f.external]);
    assert.equal(converted[0].buffer,f.bytes);assert.equal(converted[0].name,path.basename(f.deep));assert.equal(converted[0].mimeType,'application/octet-stream');
    assert.equal(converted[1],payload);assert.equal(converted[2],f.short);assert.equal(converted[3],f.external);
    assert.equal(f.records.length,1);const record=f.records[0];
    assert.equal(record.status,'EXACT_OWNED_DEEP_FILE_BYTES_TRANSPORT');assert.equal(record.file,f.deep);assert.equal(record.bytes,f.bytes.length);
    assert.equal(record.sha256,sha(f.bytes));assert.equal(record.bufferSha256,record.sha256);
    const implementation=record.fileChooserImplementation;
    assert.equal(implementation.path,f.bundle);assert.equal(implementation.sha256,sha(f.files.get(f.bundle)));assert.equal(implementation.kind,'page-instance-dispatch');
    assert.equal(implementation.index.path,f.index);assert.equal(implementation.index.sha256,sha(f.files.get(f.index)));
    assert.equal(implementation.package.path,f.packageFile);assert.equal(implementation.package.name,'playwright-core');assert.equal(implementation.package.version,'1.61.1-fixture');
    assert.equal(implementation.transport.sha256,sha(f.files.get(transportFile)));assert.match(implementation.setFilesMethodSha256,/^[a-f0-9]{64}$/);
    restore();
});

test('existing pages, multiple contexts, browser.newPage and popup pages attach once and restore inherited descriptors',async()=>{
    const f=fixture(),launch=f.sdk.chromium.launch,restore=f.install(),options={headless:true};
    assert.equal(await f.sdk.chromium.launch(options),f.browser);assert.equal(f.launchCalls[0].receiver,f.sdk.chromium);assert.equal(f.launchCalls[0].args[0],options);
    assert.equal(f.existingPage.listenerCount('filechooser'),0);assert.equal(f.existingContext.listenerCount('page'),1);
    const first=await f.browser.newContext(options),second=await f.browser.newContext(options),page=await first.newPage();
    assert.equal(f.contextCalls[0].receiver,f.browser);assert.equal(f.contextCalls[0].args[0],options);
    const emit=page.emit;first.emit('page',page);assert.equal(page.emit,emit);assert.equal(first.listenerCount('page'),1);
    const direct=await f.browser.newPage(options),popup=new f.Page();second.emit('page',popup);
    const pages=[f.existingPage,page,direct,popup],choosers=pages.map(page=>{const chooser=new f.Chooser();assert.equal(page.emit('filechooser',chooser),false);assert(chooser.setFiles.ownedDeepFileTransport);assert.equal(page.listenerCount('filechooser'),0);return chooser;});
    const contexts=f.browser.contexts(),foreign=()=>{};first.on('page',foreign);
    restore();assert.equal(f.sdk.chromium.launch,launch);assert(!Object.hasOwn(f.browser,'newContext'));
    assert(pages.every(page=>!Object.hasOwn(page,'emit')));assert(choosers.every(chooser=>!Object.hasOwn(chooser,'setFiles')));
    assert(contexts.every(context=>context.listenerCount('page')===(context===first?1:0)));assert.equal(first.listeners('page')[0],foreign);
});

test('emit forwards exact receivers, extra arguments and return values and preserves preexisting own descriptors',async()=>{
    const f=fixture(),page=f.existingPage,sentinel={},calls=[];
    const original=function(...args){calls.push({receiver:this,args});return sentinel;};
    Object.defineProperty(page,'emit',{value:original,writable:true,enumerable:true,configurable:true});
    const descriptor=Object.getOwnPropertyDescriptor(page,'emit'),restore=f.install();await f.sdk.chromium.launch();
    const chooser=new f.Chooser(),receiver={},extra={};
    assert.equal(page.emit.call(receiver,'filechooser',chooser,extra),sentinel);assert.equal(calls[0].receiver,receiver);assert.deepEqual(calls[0].args,['filechooser',chooser,extra]);
    assert.equal(page.emit.call(receiver,'unrelated',extra,17),sentinel);assert.deepEqual(calls[1].args,['unrelated',extra,17]);
    restore();assert.deepEqual(Object.getOwnPropertyDescriptor(page,'emit'),descriptor);
});

test('conversion/receipt failures reject before original setFiles; original method failures remain unchanged',async()=>{
    const f=fixture(),restore=f.install();await f.sdk.chromium.launch();const chooser=new f.Chooser();f.existingPage.emit('filechooser',chooser);
    const failure=Error('receipt write failed');f.controls.receiptError=failure;
    await assert.rejects(chooser.setFiles(f.deep),error=>error===failure);assert.equal(chooser.calls.length,0);
    f.controls.receiptError=null;f.redirects.set(f.deep,f.external);
    await assert.rejects(chooser.setFiles(f.deep));assert.equal(chooser.calls.length,0);assert.equal(f.records.length,0);
    f.redirects.clear();const methodError=Error('original setFiles failed');chooser.error=methodError;
    assert.throws(()=>chooser.setFiles(f.short),error=>error===methodError);chooser.error=null;chooser.promise=Promise.reject(methodError);
    const rejected=chooser.setFiles(f.short);assert.equal(rejected,chooser.promise);await assert.rejects(rejected,error=>error===methodError);restore();
});

test('concurrent choosers retain their own promises, buffers, options and restoration',async()=>{
    const f=fixture(),restore=f.install();await f.sdk.chromium.launch();
    const second=path.join(path.dirname(f.deep),'second.pb'),bytes=Buffer.from('second bytes');f.files.set(second,bytes);
    const a=new f.Chooser(),b=new f.Chooser();let finishA,finishB;a.promise=new Promise(resolve=>{finishA=resolve;});b.promise=new Promise(resolve=>{finishB=resolve;});
    f.existingPage.emit('filechooser',a);f.existingPage.emit('filechooser',b);const method=a.setFiles;f.existingPage.emit('filechooser',a);assert.equal(a.setFiles,method);
    const optionsA={},optionsB={},resultA=a.setFiles(f.deep,optionsA),resultB=b.setFiles(second,optionsB);
    assert.equal(resultA,a.promise);assert.equal(resultB,b.promise);assert.equal(a.calls[0].files.buffer,f.bytes);assert.equal(b.calls[0].files.buffer,bytes);
    assert.equal(a.calls[0].args[0],optionsA);assert.equal(b.calls[0].args[0],optionsB);assert.equal(f.records.length,2);
    restore();finishB('B');finishA('A');assert.deepEqual(await Promise.all([resultA,resultB]),['A','B']);
});

test('launch/context errors preserve identity and restoration prevents in-flight operations from installing late wrappers',async()=>{
    const f=fixture(),restore=f.install(),failure=Error('launch failure');f.controls.launchError=failure;
    assert.throws(()=>f.sdk.chromium.launch(),error=>error===failure);f.controls.launchError=null;
    f.controls.launchPromise=Promise.reject(failure);await assert.rejects(f.sdk.chromium.launch(),error=>error===failure);f.controls.launchPromise=null;
    await f.sdk.chromium.launch();f.controls.contextError=failure;
    assert.throws(()=>f.browser.newContext(),error=>error===failure);f.controls.contextError=null;
    f.controls.contextPromise=Promise.reject(failure);await assert.rejects(f.browser.newContext(),error=>error===failure);f.controls.contextPromise=null;
    let finish;f.controls.contextPromise=new Promise(resolve=>{finish=resolve;});const pending=f.browser.newContext();
    restore();const context=new f.Context();finish(context);assert.equal(await pending,context);assert.equal(context.listenerCount('page'),0);
    const page=await context.newPage();assert(!Object.hasOwn(page,'emit'));
});

test('restoring while launch is pending returns the same browser without installing late hooks',async()=>{
    const f=fixture(),launch=f.sdk.chromium.launch,restore=f.install();let finish;
    f.controls.launchPromise=new Promise(resolve=>{finish=resolve;});const pending=f.sdk.chromium.launch();
    restore();finish(f.browser);assert.equal(await pending,f.browser);assert.equal(f.sdk.chromium.launch,launch);
    assert(!Object.hasOwn(f.browser,'newContext'));assert(!Object.hasOwn(f.existingPage,'emit'));assert.equal(f.existingContext.listenerCount('page'),0);
});

test('own chooser descriptors and accessor emit descriptors are restored exactly',async()=>{
    const f=fixture(),page=f.existingPage,emit=page.emit;
    Object.defineProperty(page,'emit',{get:()=>emit,configurable:true,enumerable:true});
    const pageDescriptor=Object.getOwnPropertyDescriptor(page,'emit'),restore=f.install();await f.sdk.chromium.launch();
    const chooser=new f.Chooser();Object.defineProperty(chooser,'setFiles',{value:chooser.setFiles,configurable:false,writable:true,enumerable:false});
    const chooserDescriptor=Object.getOwnPropertyDescriptor(chooser,'setFiles');page.emit('filechooser',chooser);
    restore();assert.deepEqual(Object.getOwnPropertyDescriptor(page,'emit'),pageDescriptor);assert.deepEqual(Object.getOwnPropertyDescriptor(chooser,'setFiles'),chooserDescriptor);
});

test('unknown or ambiguous SDK provenance fails before wrappers and duplicate installation is rejected',()=>{
    const f=fixture(),launch=f.sdk.chromium.launch;
    f.runtime.cache={};assert.throws(f.install,/actual aliased/);assert.equal(f.sdk.chromium.launch,launch);
    f.runtime.cache={[f.index]:f.rootModule,[f.bundle]:f.rootModule.children[0],other:{...f.rootModule,filename:path.resolve('other/index.js')}};
    assert.throws(f.install,/actual aliased/);assert.equal(f.sdk.chromium.launch,launch);
    delete f.runtime.cache.other;const restore=f.install();assert.throws(f.install,/installs this transport once/);restore();
    assert.throws(restore,/already restored/);
});

test('restoration refuses to overwrite another wrapper and leaves all installed hooks intact until repaired',async()=>{
    const f=fixture(),restore=f.install();await f.sdk.chromium.launch();const page=f.existingPage,adapted=page.emit,foreign=function(){};
    page.emit=foreign;assert.throws(restore,/wrapper changed/);assert.equal(page.emit,foreign);assert(f.sdk.chromium.launch.ownedDeepFileTransport);
    page.emit=adapted;restore();assert(!Object.hasOwn(page,'emit'));assert.equal(f.existingContext.listenerCount('page'),0);
});
