const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const assert=require('node:assert/strict');
const sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
module.exports.install=({workspace,receipt},runtime={})=>{
    const io=runtime.fs||fs,load=runtime.require||require,cache=runtime.cache||require.cache;
    const root=io.realpathSync(workspace);
    assert(root.replaceAll('\\','/').includes('/target/course-presentation/incremental-browser-builds/cases/'));
    assert(root.endsWith(path.sep+'workspace'));
    const sdk=load('playwright-core'),modules=Object.values(cache);
    const entries=modules.filter(entry=>entry.exports===sdk&&entry.filename&&path.basename(entry.filename)==='index.js');
    assert.equal(entries.length,1,'Identify the actual aliased Playwright root in the CommonJS cache');
    const entry=entries[0],packageFile=path.join(path.dirname(entry.filename),'package.json'),info=JSON.parse(io.readFileSync(packageFile,'utf8'));
    assert.equal(info.name,'playwright-core','Unrecognized active Playwright package');
    assert(typeof info.version==='string'&&info.version.length,'Active Playwright package version is missing');
    const bundles=(entry.children||[]).filter(child=>child.filename&&path.basename(child.filename)==='coreBundle.js'&&modules.includes(child));
    assert.equal(bundles.length,1,'Identify the actual loaded client implementation bundle');
    const implementation={path:bundles[0].filename,sha256:sha(io.readFileSync(bundles[0].filename)),kind:'page-instance-dispatch',
        index:{path:entry.filename,sha256:sha(io.readFileSync(entry.filename))},
        package:{path:packageFile,sha256:sha(io.readFileSync(packageFile)),name:info.name,version:info.version},
        transport:{path:__filename,sha256:sha(io.readFileSync(__filename))}};
    const convert=(file,methodSha256)=>{
        if(typeof file!=='string')return file;
        const absolute=path.resolve(file);
        if(!absolute.startsWith(root+path.sep)||absolute.length<260)return file;
        const real=io.realpathSync(absolute);assert(real.startsWith(root+path.sep));assert(io.statSync(real).isFile());
        const bytes=io.readFileSync(real),digest=sha(bytes),payload={name:path.basename(file),mimeType:'application/octet-stream',buffer:bytes};
        assert.equal(sha(payload.buffer),digest);
        io.appendFileSync(receipt,JSON.stringify({status:'EXACT_OWNED_DEEP_FILE_BYTES_TRANSPORT',file:absolute,pathLength:absolute.length,name:payload.name,mimeType:payload.mimeType,bytes:bytes.length,sha256:digest,bufferSha256:sha(payload.buffer),fileChooserImplementation:{...implementation,setFilesMethodSha256:methodSha256},scope:'Image-only Windows transport; canonical helpers/assertions unchanged. Unknown/external/short paths and original byte payloads pass through unchanged.'})+'\n');
        return payload;
    };
    let active=true;
    const patches=[],contexts=new Set(),pages=new Set(),choosers=new Set(),browsers=new Set();
    const wrap=(object,key,adapter)=>{
        const descriptor=Object.getOwnPropertyDescriptor(object,key),original=object[key];
        assert.equal(typeof original,'function');assert(!original.ownedDeepFileTransport,'A separate immutable image worker installs this transport once');
        const adapted=adapter(original);adapted.ownedDeepFileTransport=true;
        Object.defineProperty(object,key,descriptor&&'value'in descriptor?{...descriptor,value:adapted}:{configurable:true,enumerable:descriptor?.enumerable||false,writable:true,value:adapted});
        patches.push({object,key,descriptor,adapted});
    };
    const attachChooser=chooser=>{
        if(choosers.has(chooser))return;
        wrap(chooser,'setFiles',original=>{
            const methodSha256=sha(Function.prototype.toString.call(original));
            return function(files,...args){
                let converted;
                try{converted=Array.isArray(files)?files.map(file=>convert(file,methodSha256)):convert(files,methodSha256);}
                catch(error){return Promise.reject(error);}
                return Reflect.apply(original,this,[converted,...args]);
            };
        });choosers.add(chooser);
    };
    const attachPage=page=>{
        if(pages.has(page))return;
        wrap(page,'emit',original=>function(event,...args){if(active&&event==='filechooser')attachChooser(args[0]);return Reflect.apply(original,this,[event,...args]);});
        pages.add(page);
    };
    const attachContext=context=>{
        if(contexts.has(context))return;
        const onPage=page=>{if(active)attachPage(page);};
        context.on('page',onPage);contexts.add(context);
        patches.push({context,onPage});
        for(const page of context.pages())attachPage(page);
    };
    const attachBrowser=browser=>{
        if(browsers.has(browser))return;
        wrap(browser,'newContext',original=>function(...args){return Promise.resolve(Reflect.apply(original,this,args)).then(context=>{if(active)attachContext(context);return context;});});
        browsers.add(browser);
        for(const context of browser.contexts())attachContext(context);
    };
    wrap(sdk.chromium,'launch',original=>function(...args){return Promise.resolve(Reflect.apply(original,this,args)).then(browser=>{if(active)attachBrowser(browser);return browser;});});
    return()=>{
        assert(active,'Transport already restored');
        for(const patch of patches)if(patch.object)assert.equal(Object.getOwnPropertyDescriptor(patch.object,patch.key)?.value,patch.adapted,'Transport wrapper changed before restore');
        active=false;
        for(const patch of patches.reverse()){
            if(patch.context)patch.context.off('page',patch.onPage);
            else if(patch.descriptor)Object.defineProperty(patch.object,patch.key,patch.descriptor);
            else delete patch.object[patch.key];
        }
    };
};
