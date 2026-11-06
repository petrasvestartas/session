import {imageCourseSnapshot, imageCourseManifestInputs} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {course, docs, expected, filesIn, json, viewer, withExpectedSequence, woodDependency} from './model.ts';
import {fileReference, observeExecution, finishExecution} from './capture-storage.ts';
import {sealCapture, verifyCaptureLedger, retirePublishedCapture, ledgerReceiptFile} from './capture-ledger.ts';
import {imageProfile} from './image-render-scale.ts';
import {nativeAdapterProfile, nativeCaptureDensity} from './presentation.ts';
import {cleanCompilerTransition} from './compiler-cache.ts';

const self = fileURLToPath(import.meta.url);
const root = path.join(viewer, 'target/course-compact');
const laneLock = path.join(root, 'active.lock');

function sourceEvidence(id: string, lane: string) {
    const step = course().steps.find(step => step.id === id)!;
    const manifest = withExpectedSequence(() => expected(id)['Cargo.toml']);
    const files = [...filesIn(path.dirname(self)).filter(file => /\.(ts|cjs|ps1)$/.test(file)),
        path.join(docs, 'journey/course.json'), path.join(docs, 'journey/frame.rs'), path.join(docs, 'capture_journey.cjs'),
        ...['typescript.mjs', 'run-guarded.cjs', 'run-guarded.ps1'].map(name => fileURLToPath(new URL('../' + name, import.meta.url)))];
    for (const relative of [step.frame, step.browser_check]) if (relative) files.push(path.join(docs, relative));
    if (id === '97-finish-with-the-maintained-browser-runtime') files.push(path.join(docs, 'capture_current.cjs'));
    const roots = manifest.includes('session_rust') ? [path.resolve(viewer, '../session_rust')] : [];
    const wood = woodDependency(manifest);
    if (wood) roots.push(wood);
    if (/^current_viewer\s*=/m.test(manifest)) roots.push(path.resolve(viewer, '../../wood/rust'));
    if (/^current_viewer\s*=/m.test(manifest)) roots.push(viewer);
    for (const directory of roots) {
        files.push(...filesIn(path.join(directory, 'src')));
        for (const name of ['Cargo.toml', 'Cargo.lock', 'build.rs']) if (fs.existsSync(path.join(directory, name))) files.push(path.join(directory, name));
    }
    const shared=lane==='browser'?imageCourseSnapshot():null;if(shared)files.push(shared.path);
    return [...new Set(files)];
}

export function selectAffected(ids: string[], firstChanged = '38a-samples') {
    const steps = course().steps, boundary = steps.findIndex(step => step.id === firstChanged);
    assert(boundary >= 0, 'Unknown changed boundary');
    assert(ids.length && new Set(ids).size === ids.length, 'Supply explicit unique affected IDs');
    for (const id of ids) assert(steps.findIndex(step => step.id === id) >= boundary, 'Unknown or unchanged-prefix ID: ' + id);
    return ids;
}

async function browserCapture(id: string, route: string, built: any) {
    const file = built.preparedFile;
    if (route === 'measurement') {
        const module = await import('./supplemental-presentation.ts');
        return (await module.runMeasurementPresentation(file)).sourceRecord;
    }
    if (route === 'maintained') {
        const module = await import('./maintained-browser-presentation.ts');
        return (await module.runMaintainedPresentation(file)).sourceRecord;
    }
    if (route === 'fallback') {
        const module = await import('./owned-file-browser-fallback.ts');
        return (await module.captureOwnedFileFallback(id, file)).record;
    }
    if (route === 'historical') {
        const module = await import('./owned-file-action-browser-recipes.ts');
        const prepared = await module.prepareOwnedActionScene('historical', id, file);
        return (await module.runOwnedActionScene('historical', path.join(prepared.folder, 'prepared.json'))).record;
    }
    if (route === 'source-status') {
        const module = await import('./source-startup-status-browser-presentation-v2.ts');
        return (await module.captureSourceStartupStatus(id, file)).file;
    }
    if (route === 'streaming') {
        const module = await import('./browser-streaming-presentation.ts'), p = module.prepareStreaming(file);
        await module.runStreaming(path.join(p.directory, 'prepared.json'));
        return path.join(p.directory, 'result.json');
    }
    if (route === 'generic') {
        const module = await import('./image-lane-browser-scene.ts'), p = module.prepareImageLaneScene(id, file);
        await module.runImageLaneScene(path.join(p.directory, 'prepared.json'));
        return path.join(p.directory, 'result.json');
    }
    const recipes: Record<string, [string, string, string, string, string]> = {
        additional: ['additional-browser-image-recipes.ts', 'prepareAdditionalScene', 'runAdditionalScene', 'prepared.json', 'capture.json'],
        exported: ['exported-browser-image-recipes.ts', 'prepareExportedScene', 'runExportedScene', 'prepared.json', 'capture.json'],
        terminal: ['terminal-browser-image-recipes-v2.ts', 'prepareTerminalScene', 'runTerminalScene', 'prepared.json', 'terminal-capture.json'],
        'child-context': ['child-context-browser-image-recipes.ts', 'prepareChildContextScene', 'runChildContextScene', 'child-context-prepared.json', 'child-context-capture.json'],
        'early-default': ['early-default-browser-presentation.ts', 'prepareEarlyDefaultScene', 'runEarlyDefaultScene', 'prepared.json', 'early-default-capture.json'],
        'main-page': ['main-page-browser-image-recipes.ts', 'prepareMainPageScene', 'runMainPageScene', 'prepared.json', 'main-page-capture.json'],
    };
    assert(recipes[route], 'Supply an explicit supported recipe; full callback fallback requires its original qualified helper');
    const [name, prepare, run, prepared, result] = recipes[route], module = await import('./' + name);
    const p = await module[prepare](id, file);
    await module[run](path.join(p.folder, prepared));
    return path.join(p.folder, result);
}

export async function captureCompact(lane: string, id: string, route = 'generic', nativeDensity = 2,
    options: {compiledRecord?: string; drawMemoryMiB?: number} = {}) {
    nativeCaptureDensity(lane, nativeDensity);
    assert(['native', 'browser'].includes(lane));
    assert(lane === 'native' || (!options.compiledRecord && options.drawMemoryMiB === undefined), 'Compiled draw options require the native lane');
    assert(course().steps.some(step => step.id === id), 'Unknown checkpoint');
    assert(route !== 'measurement' || (lane === 'browser' && id === '85ay-exercise-current-original-commands-together'), 'Measurement route requires the 85ay browser checkpoint');
    assert(route !== 'maintained' || (lane === 'browser' && id === '97-finish-with-the-maintained-browser-runtime'), 'Maintained route requires the final browser checkpoint');
    assert(lane !== 'browser' || id !== '97-finish-with-the-maintained-browser-runtime' || route === 'maintained', 'Final browser checkpoint requires its maintained assertions');
    const captureImageProfile = imageProfile(id, lane, undefined, docs);
    if (lane === 'browser' && captureImageProfile) assert.equal(route, captureImageProfile.route || (id.startsWith('88ad-') ? 'exported' : 'main-page'), 'Image profile requires its exact image recipe');
    fs.mkdirSync(root, {recursive: true});
    const adapterProfile = lane === 'native' ? nativeAdapterProfile() : null;
    const inputs = [...sourceEvidence(id, lane), ...(captureImageProfile?.files.map(file => file.path) || []), ...(adapterProfile ? [adapterProfile.metadata.path, adapterProfile.enumerator.path] : [])], sourceBefore = observeExecution(inputs);
    const descriptor = fs.openSync(laneLock, 'wx');
    fs.writeFileSync(descriptor, JSON.stringify({pid: process.pid, lane, id})); fs.closeSync(descriptor);
    let file: string, buildFile: string | undefined, ownedRoots: string[], transientRoots: string[], execution: any, evidence: string[];
    let sourceIntegrity: any, sharedCourseIntegrity: any;
    const pendingFile = path.join(root, lane + '-pending.json');
    let ownsPending = false;
    try {
        assert(!fs.existsSync(pendingFile), 'Unsealed attempt requires explicit diagnosis and owned-file cleanup before retry: ' + pendingFile);
        const priorCaptures: string[] = [];
        for (const entry of fs.readdirSync(root, {withFileTypes: true})) {
            if (!entry.isDirectory() || !entry.name.startsWith(lane + '-')) continue;
            const receipt = ledgerReceiptFile(path.join(root, entry.name), 'capture');
            if (receipt) {
                assert(fs.existsSync(path.join(path.dirname(receipt), 'retired.json')), 'Publish and retire the prior lane capture before starting another; keep only one terminal bundle');
                priorCaptures.push(receipt);
            }
        }
        fs.writeFileSync(pendingFile, JSON.stringify({pid: process.pid, id, lane, attempt: process.env.COURSE_CAPTURE_ATTEMPT || null,
            started: new Date().toISOString(), source: sourceBefore, status: 'ACTIVE_UNSEALED_ATTEMPT'}, null, 2) + '\n', {flag: 'wx'});
        ownsPending = true;
        if (lane === 'native') {
            const module = await import('./incremental-native-presentation.ts');
            if (options.compiledRecord) {
                const compiled = json(options.compiledRecord);
                assert.equal(compiled.id, id); assert.equal(compiled.density, nativeDensity);
            }
            const record = withExpectedSequence(() => options.compiledRecord
                ? module.resumeCompiledNative(options.compiledRecord, options.drawMemoryMiB)
                : module.runIncremental(id, nativeDensity, options.drawMemoryMiB));
            assert.equal(record.density, nativeDensity);
            file = record.recordFile; ownedRoots = [record.folder, record.snapshotDirectory]; execution = record.executionIntegrity;
            transientRoots = [record.snapshotDirectory, path.join(record.folder, 'runtime')];
            evidence = [record.compile.log, record.execute.log].flatMap(log => [log, log.replace(/\.log$/, '.command.json'), log.replace(/\.log$/, '.resources.json')]);
            if (record.compiledOrigin) {
                const origin = json(record.compiledOrigin.path);
                ownedRoots.push(origin.folder); transientRoots.push(path.join(origin.folder, 'runtime'));
                evidence.push(record.compiledOrigin.path, record.compiledOrigin.helper.path, ...record.compiledOrigin.evidence.map((item: any) => item.path));
            }
            if (record.nativeDensityBridge) evidence.push(record.nativeDensityBridge.record.path, record.nativeDensityBridge.before.path);
            if (record.renderScaleReceipt) evidence.push(record.renderScaleReceipt.path, record.renderScaleBridge.record.path, record.renderScaleBridge.before.path);
            if (record.adapterReceipt) evidence.push(record.adapterReceipt.path, record.adapterBridge.record.path, record.adapterBridge.before.path);
        } else {
            const module = await import('./incremental-browser-image-build.ts');
            const built = withExpectedSequence(() => module.buildIncrementalImageBrowser(id, 2, {
                beforeIdentityChange: (previous, prepared) => cleanCompilerTransition({previous, prepared, captures: priorCaptures, viewer}),
            }));
            const sharedBefore=observeExecution(imageCourseManifestInputs(built.workspace));
            for(const input of sharedBefore.files)assert.equal(built.pins[input.path],input.sha256,'Prepared shared course input changed');
            file = await browserCapture(id, route, built);
            const sharedIntegrity=finishExecution(sharedBefore);
            const record = json(file), baseline = json(built.baseline.path);
            ownedRoots = [built.folder, path.dirname(file), path.dirname(built.baseline.path), baseline.native];
            transientRoots = [built.native, built.workspace, baseline.native, baseline.workspace];
            if (record.folder) ownedRoots.push(record.folder);
            if (record.host?.log) ownedRoots.push(path.dirname(record.host.log));
            execution = record.executionIntegrity || record.host?.executionIntegrity;
            if(sharedBefore.files.length)sharedCourseIntegrity={...sharedIntegrity,prepared:fileReference(built.preparedFile)};
            evidence = [built.imageBuild.path, built.preparedFile, ...imageCourseManifestInputs(built.workspace)];
            if (record.ownedActionSourceBridge) {
                const reference = record.ownedActionSourceBridge;
                assert.equal(fileReference(reference.path).sha256, reference.sha256);
                const bridge = json(reference.path);
                assert.equal(fileReference(bridge.adapted.path).sha256, bridge.adapted.sha256);
                evidence.push(reference.path, bridge.adapted.path);
            }
            buildFile = built.imageBuild.path;
        }
        sourceIntegrity = finishExecution(sourceBefore);
        const record = json(file!), image = path.isAbsolute(record.image.path) ? record.image.path : path.resolve(viewer, record.image.path);
        assert.equal(fileReference(image).sha256, record.image.sha256);
        if (lane === 'browser' && captureImageProfile) assert.deepEqual(record.renderScale?.profile, captureImageProfile, 'The executed image must qualify the requested scale');
        for (const directory of ownedRoots!) {
            const relative = path.relative(path.join(viewer, 'target'), path.resolve(directory));
            assert(relative && !relative.startsWith('..') && !path.isAbsolute(relative));
            assert(relative.startsWith('course-presentation' + path.sep) || /^course-presentation-[^\\/]+$/.test(relative), 'Only generated capture directories are retired');
        }
        const receipt = sealCapture({directory: path.join(root, `${lane}-${id}-${fileReference(file!).sha256.slice(0, 20)}`),
            ownedRoots: ownedRoots!, transientRoots: transientRoots!, capture: file!, build: buildFile, evidence: [...evidence!, ...inputs], image, execution, sourceIntegrity, sharedCourseIntegrity,
            activeLock: laneLock,
            locks: [laneLock, path.join(viewer, 'target/course-presentation/incremental-native-running.lock'), path.join(viewer, 'target/course-ib/compact-browser-v1/workbench.lock')]});
        fs.unlinkSync(pendingFile); ownsPending = false;
        return {receipt, image: fileReference(image), status: 'CAPTURED_NOT_PUBLISHED'};
    } catch (error) {
        if (ownsPending) fs.writeFileSync(pendingFile + '.failed-' + Date.now() + '.json', JSON.stringify({pid: process.pid, id, lane, finished: new Date().toISOString(),
            error: String((error as Error).stack || error), status: 'FAILED_UNSEALED_PRESERVED_NO_ACCEPTANCE'}, null, 2) + '\n', {flag: 'wx'});
        throw error;
    } finally { fs.unlinkSync(laneLock); }
}

if (process.argv[1] && path.resolve(process.argv[1]) === self) {
    const [mode, ...args] = process.argv.slice(2);
    const result = mode === 'capture' ? await captureCompact(args[0], args[1], args[2], args[3] === undefined ? 2 : Number(args[3]))
        : mode === 'resume-native' ? await captureCompact('native', args[0], 'generic', args[2] === undefined ? 2 : Number(args[2]), {compiledRecord: path.resolve(args[1]), drawMemoryMiB: 2048})
        : mode === 'select' ? selectAffected(args)
        : mode === 'verify' ? verifyCaptureLedger(path.resolve(args[0]))
        : mode === 'retire' ? retirePublishedCapture(path.resolve(args[0]), path.resolve(args[1]), path.resolve(args[2])) : null;
    assert(result, 'capture native|browser ID [RECIPE] [NATIVE_DENSITY] | resume-native ID COMPILED_RECORD [NATIVE_DENSITY] | select ID... | verify RECEIPT | retire RECEIPT PUBLISHED_PNG PUBLICATION_EVIDENCE');
    console.log(JSON.stringify(result, null, 2));
}
