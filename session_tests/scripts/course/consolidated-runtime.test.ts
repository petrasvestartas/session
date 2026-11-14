import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {docs,viewer,expected,filesIn,hash,course} from './model.ts';

const proof=JSON.parse(fs.readFileSync(path.join(docs,'journey/release/history-editor-viewport-windows-20261114.json'),'utf8'));
const normalized=(file: string)=>hash(fs.readFileSync(file,'utf8').replaceAll('\r\n','\n'));

test('consolidated runtime receipts bind the actual checkpoint, kernel, fixtures and assets',()=>{
    const kernel=path.resolve(viewer,'../session_rust');
    assert.deepEqual(Object.fromEntries([...filesIn(path.join(kernel,'src')),path.join(kernel,'Cargo.toml'),path.join(kernel,'build.rs')]
        .map(file=>[path.relative(kernel,file).split(path.sep).join('/'),normalized(file)])),proof.normalizedKernelSources);
    for(const [id,item] of Object.entries(proof.cases) as [string,any][]){
        assert.deepEqual(Object.fromEntries(Object.entries(expected(id)).map(([file,text])=>[file,hash(text.replaceAll('\r\n','\n'))])),item.normalizedSources,id);
        for(const input of [item.frameFixture,item.lock,proof.frameHarness,proof.browserChecker])
            assert.equal(normalized(path.join(docs,input.path)),input.sha256,input.path);
        for(const [file,digest] of Object.entries(item.inputs)) assert.equal(hash(fs.readFileSync(path.join(viewer,file))),digest,file);
        assert.deepEqual(item.native.protectionBefore,item.native.protectionAfter);
        assert.deepEqual(item.native.protectionAfter.failures,[]);
        assert(item.commands.every((command: {returncode: number})=>command.returncode===0));
        assert.equal(item.native.command.args[1],'test');
        assert(item.native.command.args.includes('--lib'));
    }
});

test('six browser captures and three native frames retain their reviewed bytes and viewport scope',()=>{
    assert.equal(proof.preservedImages.checked,429);
    assert.deepEqual(proof.preservedImages.failures,[]);
    for(const [id,item] of Object.entries(proof.cases) as [string,any][]){
        assert.equal(hash(fs.readFileSync(path.join(docs,'screenshots/journey',item.native.image))),item.native.imageSha256);
        const step=course().steps.find(step=>step.id===id)!;
        for(const [shape,width,height] of [['desktop',1280,720],['upright',390,844]] as const){
            const capture=item.captures[shape];
            assert.equal(capture.source,item.source);
            assert.equal(capture.status,step.browser_status);
            assert.deepEqual(capture.initialViewport,{width,height});
            assert.deepEqual(capture.screenshotViewport,{width,height});
            assert.equal(hash(fs.readFileSync(path.join(docs,'screenshots/journey',capture.file))),capture.sha256);
        }
        assert.match(item.captures.upright.visualScope,/cropped/);
    }
});
