import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {prepareBatch} from './capture-batch.ts';
import {buildBrowser, runBrowser} from './browser-presentation.ts';
import {bindings, prepare, run} from './presentation.ts';
import {runMaintainedPresentation} from './maintained-browser-presentation.ts';
import {runPracticePresentation} from './reference-practice-presentation.ts';
import {course, docs, hash, json, reference, referenceFiles, fingerprint, viewer, write} from './model.ts';

const sha = (file: string) => hash(fs.readFileSync(file));
const end = '97-finish-with-the-maintained-browser-runtime';

function supplementary(image: any, images: any[]) {
    const name = path.basename(image.path), same = images.filter(other => other.kind === 'native' && other.sha256 === image.sha256).map(other => other.checkpoint);
    let route = 'UNMAPPED_REQUIRES_RECIPE', recipe = '';
    if (name === '04-input-light.png') { route = 'native-light-variant'; recipe = 'presentation.prepare/run("04-input-light",2); exact original background toggle.'; }
    else if (name === `${end}-browser-dpr2.png`) { route = 'maintained-current-driver'; recipe = 'Use the same newly verified DPR2 capture as the maintained endpoint; preserve a separate destination receipt.'; }
    else if (name === 'current85ay-visible-two-mm.png') { route = 'measurement-presentation'; recipe = '85ay source; Measure Distance w-1,0,1 w1,0,1; preserve 2 mm annotation and its original framed camera.'; }
    else if (same.length && /^(83zc-|83zd-|96zza-|96zzb-|96zzc-)/.test(name)) {
        route = 'retired-filename-alias-needs-explicit-source-decision';
        recipe = 'Old PNG is byte-identical to current native illustration(s). This proves old image identity only; requires an explicit current-scene alias receipt, never a fabricated retired-checkpoint build.';
    } else if (name.startsWith('command-dock-preview-')) {
        route = 'integration-dock-preview'; recipe = 'Original provenance: docs/screenshots/journey/command-dock-preview.json; integration preview, not a completed typing checkpoint. Recreate actual completion/history UI with matching source.';
    } else if (name.startsWith('current-native-')) {
        route = 'maintained-native-scene'; recipe = 'docs/journey/current-native-scenes.json provides source archive hashes and original camera/settings. New high-resolution maintained native renders need fresh receipts; old timings stay historical.';
    } else if (/^01-frame(?:-grey)?\.png$/.test(name)) {
        route = 'practice-native-readback-with-browser-caption'; recipe = 'docs/capture_practice.cjs frame01 path. First regenerate the actual white/grey native GPU readback at higher physical resolution, then capture its original presentation wrapper at DPR2; do not enlarge old pixels.';
    } else if (name.startsWith('viewer-')) {
        route = 'finished-handwritten-viewer-reference'; recipe = 'docs/capture_viewer.cjs on the actual handwritten maintained source: loaded → Point → Undo → Layers → leaf selection → Arctic → Opacity0.4. Change presentation DPR only in a private clone; preserve original assertions.';
    } else if (name === '37-native.png') {
        route = 'reference37-native'; recipe = 'docs/lessons/37 native source and supplied course_frame example, 900×700 logical frame. Requires a source-bound native presentation driver.';
    }
    return {...image, route, recipe, oldIdenticalNativeCheckpoints: same, status: 'NOT_CAPTURED'};
}

function check(cohort: any, full = false) {
    assert.equal(sha(cohort.inventory.path), cohort.inventory.sha256);
    for (const [file, expected] of Object.entries(cohort.inputs) as [string, any][]) {
        const actual = fs.statSync(file);
        if (full || actual.size !== expected.size || actual.mtimeMs !== expected.mtimeMs)
            assert.equal(sha(file), expected.sha256, `Frozen browser input changed: ${file}`);
    }
}

function progressSummary(progress: any) {
    const rows = Object.values(progress.items) as any[];
    progress.completed = rows.filter(row => ['CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', 'PUBLISHED_ROOT_REVIEWED'].includes(row.status)).length;
    progress.remaining = progress.total - progress.completed;
    progress.published = rows.filter(row => row.status === 'PUBLISHED_ROOT_REVIEWED').length;
    progress.updated = new Date().toISOString(); return progress;
}

export function prepareBrowserBatch(inventoryFile: string, density = 2, earlyReview = '') {
    if (earlyReview) validateEarly(earlyReview);
    const native = prepareBatch(inventoryFile, density), inventory = json(native.inventory.path), steps = course().steps;
    const inputs = {...native.source};
    for (const file of [fileURLToPath(import.meta.url), ...['browser-presentation.ts', 'maintained-browser-presentation.ts', 'reference-practice-presentation.ts', 'private-browser-host.ts', 'exact-bindgen.ts'].map(name => fileURLToPath(new URL(name, import.meta.url))),
        path.join(docs, 'capture_journey.cjs'), path.join(docs, 'capture_current.cjs'), path.join(docs, 'capture_viewer.cjs'),
        path.join(docs, 'typing/manifest.json'), ...reference().external.map((name: string) => path.join(docs, 'lessons/37', name))]) {
        const stat = fs.statSync(file); inputs[file] = {sha256: sha(file), size: stat.size, mtimeMs: stat.mtimeMs};
    }
    const queue = inventory.images.filter((image: any) => image.kind === 'browser').map((image: any) => {
        const step = steps.find(step => step.id === image.checkpoint)!;
        return {id: step.id, path: image.path, originalSha256: image.sha256, primary: image.queue === 'reader-published',
            index: steps.indexOf(step), callback: step.browser_check || null,
            route: step.id === end ? 'maintained-current-driver' : 'full-canonical-callback',
            presentationOnlyRecipeCandidate: !step.browser_check,
            actions: step.browser_actions || []};
    }).sort((a: any, b: any) => a.index - b.index);
    const supplemental = inventory.images.filter((image: any) => ['variant', 'supporting'].includes(image.kind)).map((image: any) => supplementary(image, inventory.images));
    const costGroups: Record<string, number> = {};
    for (const row of queue) costGroups[row.callback || 'generic-only'] = (costGroups[row.callback || 'generic-only'] || 0) + 1;
    const key = hash(JSON.stringify({inventory: native.inventory, density, inputs}));
    const directory = path.join(viewer, 'target/course-presentation/browser-batches', key.slice(0, 20));
    if (fs.existsSync(directory)) { const retained = json(path.join(directory, 'cohort.json')); check(retained, true); return retained; }
    const cohort = {status: 'FROZEN_NOT_EXECUTED', directory, key, density, inventory: native.inventory, inputs, queue,
        total: queue.length, primaryTotal: queue.filter((row: any) => row.primary).length, supplemental, costGroups,
        earlyReview: earlyReview ? {path: path.resolve(earlyReview), sha256: sha(earlyReview)} : null,
        scope: 'Actual browser PNGs only; original full callback assertions remain unchanged. Image density and behavior density are recorded separately.',
        acceleration: 'No automatic callback skipping. A future presentation-only recipe may replace a broad behavior replay only with an explicit source/scene/status recipe receipt; it claims presentation, not repeated behavior acceptance.'};
    write(path.join(directory, 'cohort.json'), JSON.stringify(cohort, null, 2) + '\n');
    write(path.join(directory, 'progress.json'), JSON.stringify(progressSummary({status: 'NOT_STARTED', total: cohort.total, current: null, items: {}}), null, 2) + '\n');
    return cohort;
}

function validateEarly(reviewFile: string) {
    const review = json(reviewFile);
    assert.equal(review.rootPersonallyViewedOriginalResolution, true, 'Root must validate the actual two early DPR captures first');
    for (const id of ['03-triangle', '10-depth']) {
        const entry = review.cases.find((row: any) => row.id === id); assert(entry, id);
        assert.equal(sha(entry.record), entry.sha256);
        const record = json(entry.record);
        assert.equal(record.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
        assert.deepEqual(record.source, bindings(id));
        assert.equal(record.presentation.behaviorDpr, 1); assert.equal(record.presentation.imageDpr, 2);
        assert.equal(sha(record.image.path), record.image.sha256);
        assert.equal(record.image.width, record.presentation.viewport.width * 2);
        assert.equal(record.image.height, record.presentation.viewport.height * 2);
    }
}

export async function runBrowserBatch(directory: string, earlyReview: string, scope = 'primary') {
    assert(['primary', 'remaining', 'all'].includes(scope));
    directory = path.resolve(directory); const cohort = json(path.join(directory, 'cohort.json'));
    check(cohort, true); validateEarly(earlyReview);
    const lock = path.join(directory, 'running.lock');
    fs.closeSync(fs.openSync(lock, 'wx'));
    const progress = json(path.join(directory, 'progress.json'));
    const save = () => write(path.join(directory, 'progress.json'), JSON.stringify(progressSummary(progress), null, 2) + '\n');
    try {
        progress.status = 'RUNNING'; save();
        for (const item of cohort.queue) {
            if (scope === 'primary' && !item.primary || scope === 'remaining' && item.primary) continue;
            check(cohort); progress.current = item.id; save();
            const started = Date.now(), previous = progress.items[item.id];
            const built = item.route === 'full-canonical-callback' ? buildBrowser(item.id, cohort.density) : null;
            const captured = item.route === 'maintained-current-driver' ? await runMaintainedPresentation() : await runBrowser(item.id, cohort.density);
            assert.equal(captured.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
            if (previous?.image) assert.equal(captured.image.sha256, previous.image.sha256);
            check(cohort);
            const sourceRecord = captured.sourceRecord || path.join(captured.folder, 'prepared.json');
            progress.items[item.id] = {...item, status: previous?.status === 'PUBLISHED_ROOT_REVIEWED' ? previous.status : captured.status,
                sourceRecord, sourceRecordSha256: sha(sourceRecord), source: captured.source.files, image: captured.image,
                behaviorDpr: captured.presentation?.behaviorDpr || [1, 2], imageDpr: captured.presentation?.imageDpr || 2,
                elapsedSeconds: previous?.elapsedSeconds ?? (Date.now() - started) / 1000,
                resumed: Boolean(previous || built?.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW')};
            save(); console.log(JSON.stringify({id: item.id, completed: progress.completed, remaining: progress.remaining}));
        }
        check(cohort, true); progress.current = null;
        progress.status = progress.remaining ? 'SCOPE_CAPTURED_REVIEW_PENDING' : 'ALL_BROWSER_CAPTURED_REVIEW_PENDING'; save(); return progress;
    } catch (error) {
        const file = path.join(directory, 'errors', `${Date.now()}-${process.pid}.json`);
        write(file, JSON.stringify({checkpoint: progress.current, error: String((error as Error).stack || error)}, null, 2) + '\n');
        progress.status = 'STOPPED_ON_FIRST_FAILURE'; progress.latestError = file; save(); throw error;
    } finally { fs.unlinkSync(lock); }
}

export async function runSupplementalBatch(directory: string, earlyReview: string) {
    directory = path.resolve(directory); const cohort = json(path.join(directory, 'cohort.json'));
    check(cohort, true); validateEarly(earlyReview);
    const lock = path.join(directory, 'running.lock'); fs.closeSync(fs.openSync(lock, 'wx'));
    const progress = json(path.join(directory, 'progress.json')); progress.supplementalItems ||= {};
    const save = () => write(path.join(directory, 'progress.json'), JSON.stringify(progressSummary(progress), null, 2) + '\n');
    try {
        for (const item of cohort.supplemental) {
            const name = path.basename(item.path); check(cohort); progress.current = item.path; save();
            let captured: any, image: any, record: string;
            if (item.route === 'native-light-variant') {
                prepare('04-input-light', cohort.density); captured = run('04-input-light', cohort.density);
                image = {...captured.image, path: path.join(viewer, captured.image.path)}; record = path.join(captured.directory, 'presentation.json');
            } else if (item.route === 'maintained-current-driver') {
                captured = await runMaintainedPresentation(); image = captured.image; record = captured.sourceRecord;
            } else if (item.route === 'finished-handwritten-viewer-reference') {
                captured = await runPracticePresentation(); image = captured.images.find((row: any) => path.basename(row.destination) === name);
                assert(image); record = captured.sourceRecord;
            } else {
                progress.supplementalItems[item.path] = {...item, status: 'RECIPE_LOCATED_NOT_EXECUTED'}; save(); continue;
            }
            assert.equal(captured.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'); assert.equal(sha(image.path), image.sha256); check(cohort);
            progress.supplementalItems[item.path] = {...item, status: captured.status, image, sourceRecord: record, sourceRecordSha256: sha(record)}; save();
        }
        progress.current = null; progress.supplementalCaptured = Object.values(progress.supplementalItems).filter((row: any) => row.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW').length;
        progress.supplementalRemaining = cohort.supplemental.length - progress.supplementalCaptured; save(); return progress;
    } catch (error) {
        const file = path.join(directory, 'errors', `${Date.now()}-${process.pid}.json`);
        write(file, JSON.stringify({checkpoint: progress.current, error: String((error as Error).stack || error)}, null, 2) + '\n');
        progress.status = 'STOPPED_ON_FIRST_FAILURE'; progress.latestError = file; save(); throw error;
    } finally { fs.unlinkSync(lock); }
}

export function publishBrowserApproved(directory: string, reviewFile: string, rootReviewed: boolean) {
    assert(rootReviewed); directory = path.resolve(directory); reviewFile = path.resolve(reviewFile);
    const cohort = json(path.join(directory, 'cohort.json')), progress = json(path.join(directory, 'progress.json')), review = json(reviewFile);
    assert.equal(review.rootPersonallyViewedOriginalResolution, true); check(cohort, true);
    for (const approved of review.images) {
        const item = progress.items[approved.id]; assert(item);
        assert.equal(item.image.sha256, approved.sha256); assert.equal(sha(item.image.path), approved.sha256);
        assert.equal(sha(item.sourceRecord), item.sourceRecordSha256);
        const record = json(item.sourceRecord); assert.deepEqual(record.source, bindings(approved.id));
        const destination = path.resolve(viewer, item.path), backup = path.join(directory, 'original-images', path.basename(destination));
        assert(destination.startsWith(path.join(docs, 'screenshots/journey') + path.sep));
        if (sha(destination) !== approved.sha256) {
            assert.equal(sha(destination), item.originalSha256); fs.mkdirSync(path.dirname(backup), {recursive: true});
            if (!fs.existsSync(backup)) fs.copyFileSync(destination, backup, fs.constants.COPYFILE_EXCL);
            assert.equal(sha(backup), item.originalSha256); fs.copyFileSync(item.image.path, destination);
        }
        assert.equal(sha(destination), approved.sha256);
        item.status = 'PUBLISHED_ROOT_REVIEWED'; item.rootReview = {path: reviewFile, sha256: sha(reviewFile)};
    }
    write(path.join(directory, 'progress.json'), JSON.stringify(progressSummary(progress), null, 2) + '\n'); return progress;
}

export function publishSupplementalApproved(directory: string, reviewFile: string, rootReviewed: boolean) {
    assert(rootReviewed); directory = path.resolve(directory); reviewFile = path.resolve(reviewFile);
    const cohort = json(path.join(directory, 'cohort.json')), progress = json(path.join(directory, 'progress.json')), review = json(reviewFile);
    assert.equal(review.rootPersonallyViewedOriginalResolution, true); check(cohort, true);
    for (const approved of review.images) {
        const item = progress.supplementalItems?.[approved.destination]; assert(item, 'No actual supplemental capture');
        assert.equal(item.image.sha256, approved.sha256); assert.equal(sha(item.image.path), approved.sha256);
        assert.equal(sha(item.sourceRecord), item.sourceRecordSha256);
        const record = json(item.sourceRecord);
        if (item.route === 'finished-handwritten-viewer-reference') assert.equal(record.source.files, fingerprint(referenceFiles('37')));
        else assert.deepEqual(record.source, bindings(record.source.checkpoint || end));
        const destination = path.resolve(viewer, item.path), relative = path.relative(docs, destination);
        assert(!relative.startsWith('..') && !path.isAbsolute(relative));
        const backup = path.join(directory, 'original-supplemental', path.basename(destination));
        if (sha(destination) !== approved.sha256) {
            assert.equal(sha(destination), item.sha256); fs.mkdirSync(path.dirname(backup), {recursive: true});
            if (!fs.existsSync(backup)) fs.copyFileSync(destination, backup, fs.constants.COPYFILE_EXCL);
            assert.equal(sha(backup), item.sha256); fs.copyFileSync(item.image.path, destination);
        }
        assert.equal(sha(destination), approved.sha256);
        item.status = 'PUBLISHED_ROOT_REVIEWED'; item.rootReview = {path: reviewFile, sha256: sha(reviewFile)};
    }
    write(path.join(directory, 'progress.json'), JSON.stringify(progressSummary(progress), null, 2) + '\n'); return progress;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    const args = process.argv.slice(2), mode = args.shift();
    const option = (name: string, fallback = '') => { const index = args.indexOf(name); return index < 0 ? fallback : args[index + 1]; };
    const result = mode === 'prepare' ? prepareBrowserBatch(option('--inventory'), Number(option('--density', '2')), option('--early-review'))
        : mode === 'run' ? await runBrowserBatch(option('--cohort'), option('--early-review'), option('--scope', 'primary'))
        : mode === 'run-supplemental' ? await runSupplementalBatch(option('--cohort'), option('--early-review'))
        : mode === 'publish' ? publishBrowserApproved(option('--cohort'), option('--review'), args.includes('--root-reviewed-originals'))
        : mode === 'publish-supplemental' ? publishSupplementalApproved(option('--cohort'), option('--review'), args.includes('--root-reviewed-originals')) : null;
    assert(result, 'Usage: browser-capture-batch.ts prepare --inventory FILE | run --cohort DIR --early-review FILE --scope primary|remaining|all | publish --cohort DIR --review FILE --root-reviewed-originals');
    console.log(JSON.stringify({directory: result.directory, status: result.status, total: result.total, primaryTotal: result.primaryTotal,
        completed: result.completed, remaining: result.remaining}, null, 2));
}
