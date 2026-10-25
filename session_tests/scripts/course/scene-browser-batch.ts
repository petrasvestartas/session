import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {runBrowser} from './browser-presentation.ts';
import {buildImageBrowser} from './image-browser-build.ts';
import {sceneRecipe} from './browser-scene-presentation.ts';
import {prepareImageLaneScene, runImageLaneScene} from './image-lane-browser-scene.ts';
import {runMaintainedPresentation} from './maintained-browser-presentation.ts';
import {prepareStreaming, runStreaming} from './browser-streaming-presentation.ts';
import {additionalImageRecipe, prepareAdditionalScene, runAdditionalScene} from './additional-browser-image-recipes.ts';
import {bindings} from './presentation.ts';
import {course, docs, filesIn, hash, json, viewer, withExpectedSequence, write} from './model.ts';

const self = fileURLToPath(import.meta.url);
const sha = (file: string) => hash(fs.readFileSync(file));
const reference = (file: string) => ({path: path.resolve(file), sha256: sha(path.resolve(file))});
const finalId = '97-finish-with-the-maintained-browser-runtime';
const capturedStatus = 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW';
const earlyFile = path.join(viewer, 'target/course-presentation/early-browser-root-review.json');
const streamingId = '68ac-connect-current-pages-to-the-browser-renderer';
const streamingReviewFile = path.join(viewer, 'target/course-presentation/streaming-feature-0c77fd130514e4789f42/root-review-publication-spec.json');
const sourceOf = (id: string) => withExpectedSequence(() => bindings(id));

function check(cohort: any, full = false) {
    for (const record of [cohort.inventory, cohort.originalCohort, cohort.earlyReview, cohort.benchmarkDecision, cohort.streamingReview, cohort.additionalReview].filter(Boolean)) assert.equal(sha(record.path), record.sha256, record.path);
    for (const [file, expected] of Object.entries(cohort.inputs) as [string, any][]) {
        const stat = fs.statSync(file);
        if (full || stat.size !== expected.size || stat.mtimeMs !== expected.mtimeMs) assert.equal(sha(file), expected.sha256, 'Frozen browser input changed: ' + file);
    }
    for (const [root, names] of Object.entries(cohort.sourceFileLists)) assert.deepEqual(filesIn(root).sort(), names, 'Source file list changed: ' + root);
}

function checkImage(image: any, id: string) {
    const bytes = fs.readFileSync(image.path);
    assert.equal(hash(bytes), image.sha256);
    assert(bytes.subarray(0, 8).equals(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])));
    assert.equal(bytes.readUInt32BE(16), image.width); assert.equal(bytes.readUInt32BE(20), image.height);
    assert.equal(image.width, id === streamingId ? 2200 : 1800); assert.equal(image.height, id === streamingId ? 1600 : 1520);
}

function checkResources(resources: any, imageRecord?: any) {
    assert.equal(resources.exitCode, 0); assert.equal(resources.timedOut, false);
    if (imageRecord && resources.processors === 8) {
        assert.equal(imageRecord.canonicalBehaviorExecuted, false);
        assert.equal(resources.jobCommitLimitBytes, 8589934592); assert.equal(resources.affinityMask, '0xFF00');
        assert.equal(resources.laneMutex, 'Local\\SessionViewerCourseImageJob');
        const guard = fileURLToPath(new URL('parallel-image-guard.ps1', import.meta.url));
        const host = fileURLToPath(new URL('parallel-image-browser-host.ts', import.meta.url));
        assert.equal(imageRecord.pins[guard], sha(guard)); assert.equal(imageRecord.pins[host], sha(host));
        assert.equal(imageRecord.host.inputs[guard], sha(guard));
        if (imageRecord.captureProfile) {
            assert.equal(imageRecord.captureProfile.name, 'parallel-image-only');
            for (const key of ['parent', 'originalWorker', 'guard', 'host']) {
                const file = imageRecord.captureProfile[key]; assert.equal(sha(file.path), file.sha256);
                assert.equal(imageRecord.pins[file.path], file.sha256);
            }
        }
        assert.equal(sha(imageRecord.host.resourcesFile), imageRecord.host.resourcesSha256);
        assert.equal(sha(imageRecord.host.log), imageRecord.host.logSha256);
    } else { assert.equal(resources.processors, 2); assert.equal(resources.jobCommitLimitBytes, 6442450944); }
}

function validateRecord(id: string, file: string, route: string) {
    const record = json(file);
    assert.deepEqual(record.source, sourceOf(id)); checkImage(record.image, id);
    if (route === 'scene-presentation') {
        assert.equal(record.status, 'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW'); assert.equal(record.canonicalBehaviorExecuted, false);
        assert.deepEqual(record.recipe, sceneRecipe(id)); assert(record.semantic?.camera?.every(Number.isFinite));
        assert.equal(record.helperSha256, sha(fileURLToPath(new URL('browser-scene-presentation.ts', import.meta.url))));
        assert.equal(record.state.status, record.terminalStatus); checkResources(record.resources, record);
    } else if (route === 'streaming-feature-presentation') {
        assert.equal(record.status, 'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW');
        assert.equal(record.originalInheritedCallbackExecuted, false); assert.equal(record.unchangedExportedFeatureExecuted, true);
        assert.equal(record.state.status, 'URL scene replaced.'); checkResources(record.host.resources);
    } else if (route === 'additional-scene-presentation') {
        assert.equal(record.status, 'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW'); assert.equal(record.canonicalBehaviorExecuted, false);
        assert.deepEqual(record.recipe, additionalImageRecipe(id)); assert(record.semantic.camera.every(Number.isFinite));
        assert.equal(record.helperSha256, sha(fileURLToPath(new URL('additional-browser-image-recipes.ts', import.meta.url)))); checkResources(record.resources, record);
    } else {
        assert.equal(record.status, capturedStatus);
        if (route === 'maintained-current-driver') {
            assert.deepEqual(record.browser.images.map((image: any) => image.density), [1, 2]); checkResources(record.host.resources);
        } else {
            assert.equal(record.presentation.behaviorDpr, 1); assert.equal(record.presentation.imageDpr, 2); checkResources(record.resources);
        }
    }
    return record;
}

function validateStreamingReview(file: string) {
    const review = json(file); assert.equal(review.rootPersonallyViewedOriginalResolution, true);
    assert.equal(sha(review.record.path), review.record.sha256);
    const record = validateRecord(streamingId, review.record.path, 'streaming-feature-presentation');
    assert.equal(record.image.sha256, review.rootReviewedImageSha256); return review;
}

function validateAdditionalReview(file: string) {
    const review = json(file); assert.equal(review.rootPersonallyViewedOriginalResolution, true);
    assert.equal(review.helperSha256, sha(fileURLToPath(new URL('additional-browser-image-recipes.ts', import.meta.url))));
    assert(review.cases.length > 0); assert(review.approvedIds.length > 0);
    for (const row of review.cases) {
        assert.equal(sha(row.record), row.sha256); const record = validateRecord(row.id, row.record, 'additional-scene-presentation');
        assert.equal(record.image.sha256, row.reviewedImageSha256);
    }
    for (const id of review.approvedIds) assert(additionalImageRecipe(id)); return review;
}

function validateEarly(file: string) {
    const review = json(file); assert.equal(review.rootPersonallyViewedOriginalResolution, true);
    for (const id of ['03-triangle', '10-depth']) {
        const row = review.cases.find((entry: any) => entry.id === id); assert(row, id);
        assert.equal(sha(row.record), row.sha256); validateRecord(id, row.record, 'full-canonical-callback');
    }
    return review;
}

function validateBenchmarks(file: string, queue: any[]) {
    const decision = json(file);
    assert.equal(decision.status, 'ROOT_REVIEWED_SIX_FAMILY_BENCHMARKS'); assert.equal(decision.rootPersonallyViewedOriginalResolution, true);
    assert.equal(decision.sceneHelperSha256, sha(fileURLToPath(new URL('browser-scene-presentation.ts', import.meta.url))));
    const kinds = [...new Set(queue.filter(row => row.route === 'scene-presentation').map(row => row.recipe.kind))].sort();
    assert.deepEqual([...decision.approvedRecipeKinds].sort(), kinds);
    for (const kind of kinds) {
        const row = decision.cases.find((entry: any) => entry.kind === kind); assert(row, 'Actual benchmark required for ' + kind);
        assert.equal(row.rootPersonallyViewedOriginalResolution, true); assert.equal(sha(row.record), row.sha256);
        const item = queue.find(item => item.id === row.id); assert.equal(item?.route, 'scene-presentation');
        const record = validateRecord(row.id, row.record, 'scene-presentation');
        assert.equal(record.recipe.kind, kind); assert.equal(record.image.sha256, row.reviewedImageSha256);
    }
    const comparison = decision.inexpensiveFullCanonicalComparison;
    assert.equal(comparison.sceneEquivalent, true); assert.equal(comparison.rootPersonallyViewedOriginalResolution, true);
    const scene = decision.cases.find((entry: any) => entry.id === comparison.id); assert(scene);
    assert.equal(sha(comparison.fullCallbackRecord), comparison.sha256);
    validateRecord(comparison.id, comparison.fullCallbackRecord, 'full-canonical-callback');
    return decision;
}

function summarize(progress: any) {
    const rows = Object.values(progress.items) as any[];
    const done = (row: any) => [capturedStatus, 'PUBLISHED_ROOT_REVIEWED'].includes(row.status);
    progress.completed = rows.filter(done).length; progress.remaining = progress.total - progress.completed;
    progress.primaryCompleted = rows.filter(row => row.primary && done(row)).length;
    progress.primaryRemaining = progress.primaryTotal - progress.primaryCompleted;
    progress.published = rows.filter(row => row.status === 'PUBLISHED_ROOT_REVIEWED').length;
    progress.updated = new Date().toISOString(); return progress;
}

function artifacts(record: any, built?: any) {
    const files = new Set<string>();
    const add = (file: string) => { assert(fs.existsSync(file), file); files.add(path.resolve(file)); };
    const folder = record.directory || record.folder;
    if (folder && fs.existsSync(path.join(folder, 'resources.json'))) add(path.join(folder, 'resources.json'));
    if (folder && fs.existsSync(path.join(folder, 'browser.log'))) add(path.join(folder, 'browser.log'));
    if (record.host?.log) add(record.host.log);
    if (record.execution) { add(record.execution.log.path); add(record.execution.resources.path); }
    for (const command of built?.commands || record.commands || []) {
        if (command.resourcesFile) {
            assert.equal(command.resources.exitCode, 0); assert.equal(command.resources.timedOut, false);
            assert.equal(sha(command.log), command.logSha256); assert.equal(sha(command.resourcesFile), command.resourcesSha256);
            add(command.log); add(command.resourcesFile); add(command.commandFile); continue;
        }
        assert.equal(command.returncode, 0);
        add(path.join(viewer, 'target/course-checks', command.name + '.log'));
        if (process.platform === 'win32') {
            const resources = path.join(viewer, 'target/course-checks', command.name + '.resources.json');
            assert.equal(json(resources).exitCode, 0); add(resources); add(path.join(viewer, 'target/course-checks', command.name + '.command.json'));
        }
    }
    built ||= record.bundle && record.workspace && record.folder && !record.dist ? record : null;
    if (built) {
        add(path.join(built.folder, 'prepared.json'));
        for (const name of Object.keys(built.bundle)) {
            const file = path.join(built.workspace, 'target/course-checks', built.id, 'dist', name);
            assert.equal(sha(file), built.bundle[name]); add(file);
        }
    }
    if (record.bundle && record.dist) for (const [name, digest] of Object.entries(record.bundle)) {
        const file = path.join(record.dist, name); assert.equal(sha(file), digest); add(file);
    }
    if (record.host?.served) for (const [name, digest] of Object.entries(record.host.served)) {
        const workspace = record.workspace || path.join(record.folder, 'workspace');
        const file = path.join(workspace, 'target/course-checks', name); assert.equal(sha(file), digest); add(file);
    }
    if (record.pins) for (const [file, digest] of Object.entries(record.pins)) { assert.equal(sha(file), digest); add(file); }
    return [...files].map(reference);
}

function completed(item: any, recordFile: string, built?: any, prior?: any) {
    const record = validateRecord(item.id, recordFile, item.route);
    return {...item, status: prior?.status === 'PUBLISHED_ROOT_REVIEWED' ? prior.status : capturedStatus,
        sourceRecord: reference(recordFile), source: record.source, image: record.image,
        artifacts: artifacts(record, built), recipe: record.recipe || item.recipe || null,
        sceneContract: record.sceneContract || null, scene: record.semantic || null,
        behaviorScope: item.route.includes('presentation') ? 'Not rerun; actual final-scene presentation only'
            : item.route === 'maintained-current-driver' ? 'Original maintained DPR1/DPR2 callback and DPR2 presentation'
            : 'Original full callback at DPR1; DPR2 presentation separately recorded',
        canonicalBehaviorExecuted: !item.route.includes('presentation'), imageDpr: 2,
        elapsedSeconds: record.elapsedSeconds ?? record.resources?.elapsedSeconds ?? record.host?.resources?.elapsedSeconds,
        ...(prior?.rootReview ? {rootReview: prior.rootReview} : {})};
}

function verifyCompleted(item: any) {
    assert([capturedStatus, 'PUBLISHED_ROOT_REVIEWED'].includes(item.status));
    assert.equal(sha(item.sourceRecord.path), item.sourceRecord.sha256);
    const record = validateRecord(item.id, item.sourceRecord.path, item.route);
    assert.deepEqual(record.source, item.source); assert.deepEqual(record.image, item.image);
    for (const file of item.artifacts) assert.equal(sha(file.path), file.sha256, file.path);
    return record;
}

export function prepareSceneBatch(oldCohortFile: string, benchmarkFile: string, earlyReview = earlyFile, streamingReview = streamingReviewFile, additionalReview = '') {
    oldCohortFile = path.resolve(oldCohortFile); benchmarkFile = path.resolve(benchmarkFile); earlyReview = path.resolve(earlyReview);
    const original = json(oldCohortFile), steps = course().steps;
    assert.equal(original.density, 2); assert.equal(original.total, 379); assert.equal(original.primaryTotal, 36);
    assert.equal(sha(original.inventory.path), original.inventory.sha256);
    const early = validateEarly(earlyReview);
    const streaming = validateStreamingReview(streamingReview);
    const extra = additionalReview ? validateAdditionalReview(path.resolve(additionalReview)) : null;
    const queue = original.queue.map((row: any) => {
        const step = steps.find(step => step.id === row.id); assert(step); const recipe = sceneRecipe(row.id, step);
        return {...row, route: row.id === finalId ? 'maintained-current-driver' : row.id === streamingId ? 'streaming-feature-presentation'
            : recipe.kind !== 'full-canonical-fallback' ? 'scene-presentation' : extra?.approvedIds.includes(row.id) ? 'additional-scene-presentation' : 'full-canonical-callback', recipe};
    }).sort((a: any, b: any) => Number(b.id.startsWith('68ac-')) - Number(a.id.startsWith('68ac-')) || Number(b.primary) - Number(a.primary) || a.index - b.index);
    assert.equal(new Set(queue.map((row: any) => row.id)).size, 379);
    const decision = validateBenchmarks(benchmarkFile, queue);
    const inputChanges: any[] = [];
    const inputs = Object.fromEntries(Object.entries(original.inputs).map(([file, old]: [string, any]) => {
        const stat = fs.statSync(file), actual = sha(file);
        if (actual !== old.sha256) inputChanges.push({path: file, previousSha256: old.sha256, currentSha256: actual});
        return [file, {sha256: actual, size: stat.size, mtimeMs: stat.mtimeMs}];
    }));
    for (const file of [self, fileURLToPath(new URL('browser-scene-presentation.ts', import.meta.url)), fileURLToPath(new URL('image-browser-build.ts', import.meta.url)),
        fileURLToPath(new URL('capture-guard.ps1', import.meta.url)), fileURLToPath(new URL('browser-streaming-presentation.ts', import.meta.url)),
        ...['image-lane-browser-scene.ts', 'parallel-image-browser-host.ts', 'parallel-image-guard.ps1'].map(name => fileURLToPath(new URL(name, import.meta.url))),
        fileURLToPath(new URL('additional-browser-image-recipes.ts', import.meta.url)), benchmarkFile, earlyReview, streamingReview, additionalReview].filter(Boolean)) {
        const stat = fs.statSync(file); inputs[file] = {sha256: sha(file), size: stat.size, mtimeMs: stat.mtimeMs};
    }
    const roots = ['docs/journey/code', 'docs/journey/checks', 'src', 'assets'].map(name => path.join(viewer, name));
    roots.push(path.resolve(viewer, '../session_rust/src'), path.resolve(viewer, '../../wood/rust/src'));
    const sourceFileLists = Object.fromEntries(roots.filter(fs.existsSync).map(root => [root, filesIn(root).sort()]));
    for (const names of Object.values(sourceFileLists)) for (const file of names) if (!inputs[file]) {
        const stat = fs.statSync(file); inputs[file] = {sha256: sha(file), size: stat.size, mtimeMs: stat.mtimeMs};
        inputChanges.push({path: file, previousSha256: null, currentSha256: inputs[file].sha256});
    }
    const originalCohort = reference(oldCohortFile), benchmarkDecision = reference(benchmarkFile);
    const key = hash(JSON.stringify({originalCohort, benchmarkDecision, inputs, sourceFileLists}));
    const directory = path.join(viewer, 'target/course-presentation/scene-browser-batches', key.slice(0, 20));
    if (fs.existsSync(directory)) { const retained = json(path.join(directory, 'cohort.json')); check(retained, true); return retained; }
    const cohort = {status: 'FROZEN_REVIEWED_RECIPE_BATCH_NOT_EXECUTED', directory, key, density: 2, originalCohort,
        inventory: original.inventory, benchmarkDecision, earlyReview: reference(earlyReview), streamingReview: reference(streamingReview),
        additionalReview: additionalReview ? reference(additionalReview) : null, inputs, sourceFileLists, queue,
        sourceTransition: {previousFreeze: originalCohort, inputChanges,
            scope: 'This successor freezes actual current inputs. Prior freezes remain immutable; reused captures must independently match their current checkpoint bindings.'},
        total: 379, primaryTotal: 36, mapped: queue.filter((row: any) => row.route === 'scene-presentation').length,
        scope: 'Image regeneration only. Each receipt distinguishes presentation-only, original full callback, and maintained callback. No automatic publication.'};
    check(cohort, true);
    const progress: any = {status: 'EARLY_AND_BENCHMARK_CAPTURES_REUSED', total: 379, primaryTotal: 36, current: null, items: {}};
    for (const row of early.cases) {
        const item = queue.find((item: any) => item.id === row.id); assert(item);
        progress.items[item.id] = {...completed(item, row.record), reusedActualCapture: cohort.earlyReview};
    }
    for (const row of decision.cases) {
        const item = queue.find((item: any) => item.id === row.id); assert(item);
        progress.items[item.id] = {...completed(item, row.record), reusedActualCapture: benchmarkDecision};
    }
    const streamItem = queue.find((item: any) => item.id === streamingId);
    progress.items[streamingId] = {...completed(streamItem, streaming.record.path), reusedActualCapture: cohort.streamingReview};
    for (const row of extra?.cases || []) {
        const item = queue.find((item: any) => item.id === row.id);
        if (item?.route === 'additional-scene-presentation') progress.items[item.id] = {...completed(item, row.record), reusedActualCapture: cohort.additionalReview};
    }
    write(path.join(directory, 'cohort.json'), JSON.stringify(cohort, null, 2) + '\n');
    write(path.join(directory, 'progress.json'), JSON.stringify(summarize(progress), null, 2) + '\n'); return cohort;
}

export async function runSceneBatch(directory: string, scope = 'primary', onlyId = '', presentationOnly = false) {
    assert(['primary', 'remaining', 'all'].includes(scope)); directory = path.resolve(directory);
    const cohort = json(path.join(directory, 'cohort.json')); check(cohort, true);
    validateEarly(cohort.earlyReview.path); validateBenchmarks(cohort.benchmarkDecision.path, cohort.queue);
    validateStreamingReview(cohort.streamingReview.path); if (cohort.additionalReview) validateAdditionalReview(cohort.additionalReview.path);
    const lock = path.join(directory, 'running.lock'); fs.writeFileSync(lock, JSON.stringify({pid: process.pid, started: new Date().toISOString()}), {flag: 'wx'});
    const progress = json(path.join(directory, 'progress.json'));
    const save = () => write(path.join(directory, 'progress.json'), JSON.stringify(summarize(progress), null, 2) + '\n');
    try {
        progress.status = 'RUNNING'; save();
        if (onlyId) assert(cohort.queue.some((row: any) => row.id === onlyId));
        for (const item of cohort.queue) {
            if (onlyId ? item.id !== onlyId : scope === 'primary' ? !item.primary : scope === 'remaining' && item.primary) continue;
            if (presentationOnly && !item.route.includes('presentation')) continue;
            check(cohort); const prior = progress.items[item.id];
            if (prior && [capturedStatus, 'PUBLISHED_ROOT_REVIEWED'].includes(prior.status)) { verifyCompleted(prior); continue; }
            progress.current = item.id; save(); const started = Date.now();
            let recordFile: string, built: any = null;
            if (item.route === 'maintained-current-driver') {
                const record = await runMaintainedPresentation(); recordFile = record.sourceRecord;
            } else {
                built = withExpectedSequence(() => buildImageBrowser(item.id, 2));
                progress.items[item.id] = {...item, status: 'BUILT_WAITING_CAPTURE', buildRecord: reference(path.join(built.folder, 'prepared.json')),
                    ...(prior?.scenePrepared ? {scenePrepared: prior.scenePrepared} : {})}; save();
                if (['scene-presentation', 'streaming-feature-presentation', 'additional-scene-presentation'].includes(item.route)) {
                    let scenePrepared = prior?.scenePrepared;
                    if (scenePrepared) assert.equal(sha(scenePrepared.path), scenePrepared.sha256);
                    else {
                        const scene: any = withExpectedSequence(() => item.route === 'scene-presentation' ? prepareImageLaneScene(item.id, path.join(built.folder, 'prepared.json'))
                            : item.route === 'streaming-feature-presentation' ? prepareStreaming(path.join(built.folder, 'prepared.json'))
                            : prepareAdditionalScene(item.id, path.join(built.folder, 'prepared.json')));
                        scenePrepared = reference(path.join(scene.directory || scene.folder, 'prepared.json'));
                        progress.items[item.id].scenePrepared = scenePrepared; save();
                    }
                    recordFile = path.join(path.dirname(scenePrepared.path), item.route === 'additional-scene-presentation' ? 'capture.json' : 'result.json');
                    if (!fs.existsSync(recordFile)) {
                        if (item.route === 'scene-presentation') await runImageLaneScene(scenePrepared.path);
                        else if (item.route === 'streaming-feature-presentation') await runStreaming(scenePrepared.path);
                        else await runAdditionalScene(scenePrepared.path);
                    }
                } else {
                    const record = await runBrowser(item.id, 2); recordFile = path.join(record.folder, 'prepared.json');
                }
            }
            check(cohort);
            progress.items[item.id] = {...completed(item, recordFile!, built), captureAndBuildElapsedSeconds: (Date.now() - started) / 1000}; save();
            console.log(JSON.stringify({id: item.id, route: item.route, completed: progress.completed, remaining: progress.remaining}));
        }
        check(cohort, true); progress.current = null; progress.status = progress.remaining ? 'SCOPE_CAPTURED_REVIEW_PENDING' : 'ALL_BROWSER_CAPTURED_REVIEW_PENDING'; save(); return progress;
    } catch (error) {
        const file = path.join(directory, 'errors', `${Date.now()}-${process.pid}.json`);
        write(file, JSON.stringify({id: progress.current, error: String((error as Error).stack || error)}, null, 2));
        progress.status = 'STOPPED_ON_FIRST_FAILURE'; progress.latestError = reference(file); save(); throw error;
    } finally { fs.unlinkSync(lock); }
}

export function publishSceneBatch(directory: string, reviewFile: string, rootReviewed: boolean) {
    assert(rootReviewed); directory = path.resolve(directory); reviewFile = path.resolve(reviewFile);
    const cohort = json(path.join(directory, 'cohort.json')), progress = json(path.join(directory, 'progress.json')), review = json(reviewFile);
    assert.equal(review.rootPersonallyViewedOriginalResolution, true); check(cohort, true);
    const approved = review.images.map((row: any) => {
        const item = progress.items[row.id]; assert(item); verifyCompleted(item); assert.equal(row.sha256, item.image.sha256);
        assert.equal(row.sourceRecordSha256, item.sourceRecord.sha256, 'Root review must name the actual per-image scope receipt');
        const destination = path.resolve(viewer, item.path);
        assert.equal(path.dirname(destination), path.join(docs, 'screenshots/journey'));
        const current = sha(destination); assert([item.originalSha256, item.image.sha256].includes(current), 'Preserve outside image edits: ' + destination);
        return {item, destination, current};
    });
    for (const {item, destination, current} of approved) {
        if (current !== item.image.sha256) {
            const backup = path.join(directory, 'original-images', path.basename(destination)); fs.mkdirSync(path.dirname(backup), {recursive: true});
            if (!fs.existsSync(backup)) fs.copyFileSync(destination, backup, fs.constants.COPYFILE_EXCL);
            assert.equal(sha(backup), item.originalSha256); fs.copyFileSync(item.image.path, destination);
        }
        assert.equal(sha(destination), item.image.sha256); item.status = 'PUBLISHED_ROOT_REVIEWED'; item.rootReview = reference(reviewFile);
    }
    check(cohort, true); write(path.join(directory, 'progress.json'), JSON.stringify(summarize(progress), null, 2) + '\n'); return progress;
}

export function benchmarkSchema() {
    return {status: 'ROOT_REVIEWED_SIX_FAMILY_BENCHMARKS', rootPersonallyViewedOriginalResolution: true, sceneHelperSha256: 'ACTUAL_HELPER_SHA',
        approvedRecipeKinds: ['documented-example-line', 'selected-original-cloud', 'two-original-nurbs-surfaces', 'retained-four-brep', 'ordinary-and-instance-slots', 'original-contact-panel'],
        cases: [{kind: 'ONE_ACTUAL_CASE_PER_KIND', id: 'ACTUAL_QUEUE_ID', record: 'ACTUAL_SCENE_RESULT_JSON', sha256: 'ACTUAL_RECORD_SHA',
            reviewedImageSha256: 'ACTUAL_IMAGE_SHA', rootPersonallyViewedOriginalResolution: true}],
        inexpensiveFullCanonicalComparison: {id: 'ONE_ACTUAL_CASE_ID', fullCallbackRecord: 'ACTUAL_CURRENT_FULL_BROWSER_PREPARED_JSON', sha256: 'ACTUAL_RECORD_SHA',
            sceneEquivalent: true, rootPersonallyViewedOriginalResolution: true}};
}

if (process.argv[1] && path.resolve(process.argv[1]) === self) {
    const [mode, ...args] = process.argv.slice(2);
    const option = (name: string, fallback = '') => { const at = args.indexOf(name); return at < 0 ? fallback : args[at + 1]; };
    const result = mode === 'schema' ? benchmarkSchema()
        : mode === 'prepare' ? prepareSceneBatch(option('--original-cohort'), option('--benchmark'), option('--early-review', earlyFile),
            option('--streaming-review', streamingReviewFile), option('--additional-review'))
        : mode === 'run' ? await runSceneBatch(option('--cohort'), option('--scope', 'primary'), option('--id'), args.includes('--presentation-only'))
        : mode === 'publish' ? publishSceneBatch(option('--cohort'), option('--review'), args.includes('--root-reviewed-originals')) : null;
    assert(result, 'Usage: scene-browser-batch.ts schema | prepare --original-cohort FILE --benchmark FILE [--early-review FILE] | run --cohort DIR [--scope primary|remaining|all] [--id ID] | publish --cohort DIR --review FILE --root-reviewed-originals');
    console.log(JSON.stringify(result, null, 2));
}
