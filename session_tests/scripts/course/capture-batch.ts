import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {captureDirectory, prepare, run} from './presentation.ts';
import {course, docs, filesIn, hash, json, read, viewer, withExpectedSequence, write} from './model.ts';

const sha = (file: string) => hash(fs.readFileSync(file));
const root = path.join(viewer, 'target/course-presentation/batches');
const requested = ['41f-input', '51zf-handles', '54p-navigation', '59f-cad', '62o-labels',
    '70za-read-original-sheet-metadata-records', '80za-read-original-projection-planes',
    '96zzzzx-retire-empty-arena-ranges-after-tail-trimming'];

function sourcePaths() {
    const roots = ['docs/journey/code', 'docs/journey/checks', 'src', 'assets'].map(name => path.join(viewer, name));
    roots.push(path.resolve(viewer, '../session_rust/src'), path.resolve(viewer, '../../wood/rust/src'));
    const files = roots.filter(fs.existsSync).flatMap(filesIn);
    files.push(path.join(docs, 'journey/course.json'), path.join(docs, 'journey/frame.rs'));
    for (const name of ['presentation.ts', 'model.ts', 'verify.ts', 'dependency-inputs.ts', 'png.ts', 'capture-batch.ts'])
        files.push(fileURLToPath(new URL(name, import.meta.url)));
    for (const folder of [viewer, path.resolve(viewer, '../session_rust'), path.resolve(viewer, '../../wood/rust')]) {
        for (const name of ['Cargo.toml', 'Cargo.lock', 'build.rs']) {
            const file = path.join(folder, name); if (fs.existsSync(file)) files.push(file);
        }
    }
    files.push(path.resolve(viewer, '../session_tests/scripts/run-guarded.ps1'));
    return [...new Set(files)].sort();
}

function freezeInputs() {
    return Object.fromEntries(sourcePaths().map(file => {
        const stat = fs.statSync(file);
        return [file, {sha256: sha(file), size: stat.size, mtimeMs: stat.mtimeMs}];
    }));
}

function checkFreeze(freeze: any, full = false) {
    assert.deepEqual(sourcePaths(), Object.keys(freeze), 'The frozen source file list changed');
    for (const [file, expected] of Object.entries(freeze) as [string, any][]) {
        const stat = fs.statSync(file);
        if (full || stat.size !== expected.size || stat.mtimeMs !== expected.mtimeMs)
            assert.equal(sha(file), expected.sha256, `Frozen cohort source changed: ${file}`);
    }
}

function summarize(progress: any) {
    const items = Object.values(progress.items) as any[];
    progress.completed = items.filter(item => ['CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', 'PUBLISHED_ROOT_REVIEWED'].includes(item.status)).length;
    progress.published = items.filter(item => item.status === 'PUBLISHED_ROOT_REVIEWED').length;
    progress.remaining = progress.total - progress.completed;
    progress.primaryCompleted = items.filter(item => item.primary && ['CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', 'PUBLISHED_ROOT_REVIEWED'].includes(item.status)).length;
    progress.primaryRemaining = progress.primaryTotal - progress.primaryCompleted;
    progress.updated = new Date().toISOString();
    return progress;
}

export function prepareBatch(inventoryFile: string, density = 2) {
    assert([2, 3].includes(density));
    inventoryFile = path.resolve(inventoryFile);
    const inventory = json(inventoryFile), steps = course().steps;
    const source = freezeInputs();
    const key = hash(JSON.stringify({inventory: sha(inventoryFile), density, source}));
    const directory = path.join(root, key.slice(0, 20));
    if (fs.existsSync(directory)) {
        const existing = json(path.join(directory, 'cohort.json'));
        checkFreeze(existing.source, true); return existing;
    }
    const queue = inventory.images.filter((image: any) => image.kind === 'native').map((image: any) => {
        assert(image.checkpoint && steps.some(step => step.id === image.checkpoint));
        const primary = image.queue === 'reader-published' || requested.includes(image.checkpoint);
        return {id: image.checkpoint, path: image.path, originalSha256: image.sha256, primary,
            index: steps.findIndex(step => step.id === image.checkpoint), blankExpected: image.blankExpected};
    }).sort((a: any, b: any) => a.index - b.index);
    assert.equal(new Set(queue.map((item: any) => item.id)).size, queue.length, 'Native destination must be unique');
    for (const id of requested) assert(queue.some((item: any) => item.id === id), `Requested image missing: ${id}`);
    const cohort = {status: 'FROZEN_NOT_EXECUTED', key, directory, kind: 'native', density,
        inventory: {path: inventoryFile, sha256: sha(inventoryFile)}, source, queue,
        total: queue.length, primaryTotal: queue.filter((item: any) => item.primary).length,
        policy: 'One native presentation job at a time. Original canonical checks/evidence remain separate. No automatic publication.'};
    write(path.join(directory, 'cohort.json'), JSON.stringify(cohort, null, 2) + '\n');
    write(path.join(directory, 'progress.json'), JSON.stringify(summarize({status: 'NOT_STARTED', total: cohort.total,
        primaryTotal: cohort.primaryTotal, current: null, items: {}}), null, 2) + '\n');
    return cohort;
}

export function runBatch(directory: string, scope = 'primary') {
    assert(['primary', 'remaining', 'all'].includes(scope));
    directory = path.resolve(directory);
    const cohort = json(path.join(directory, 'cohort.json'));
    assert.equal(cohort.kind, 'native');
    assert.equal(sha(cohort.inventory.path), cohort.inventory.sha256);
    checkFreeze(cohort.source, true);
    const lock = path.join(directory, 'running.lock');
    const descriptor = fs.openSync(lock, 'wx');
    fs.writeFileSync(descriptor, JSON.stringify({pid: process.pid, started: new Date().toISOString()})); fs.closeSync(descriptor);
    const progress = json(path.join(directory, 'progress.json'));
    const save = () => write(path.join(directory, 'progress.json'), JSON.stringify(summarize(progress), null, 2) + '\n');
    try {
        progress.status = 'RUNNING'; save();
        withExpectedSequence(() => {
            for (const item of cohort.queue) {
                if (scope === 'primary' && !item.primary || scope === 'remaining' && item.primary) continue;
                checkFreeze(cohort.source);
                progress.current = item.id; save();
                const started = Date.now(), prior = progress.items[item.id];
                const prepared = prepare(item.id, cohort.density);
                const result = run(item.id, cohort.density);
                assert.equal(result.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
                if (prior?.image) assert.equal(result.image.sha256, prior.image.sha256, 'Resumed capture differs');
                checkFreeze(cohort.source);
                progress.items[item.id] = {...item, status: prior?.status === 'PUBLISHED_ROOT_REVIEWED' ? prior.status : result.status,
                    sourceKey: result.key, sourceRecord: path.join(prepared.directory, 'presentation.json'),
                    sourceRecordSha256: sha(path.join(prepared.directory, 'presentation.json')), image: result.image,
                    elapsedSeconds: prior?.elapsedSeconds ?? (Date.now() - started) / 1000,
                    resumed: Boolean(prior || prepared.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW')};
                save(); console.log(JSON.stringify({id: item.id, completed: progress.completed, remaining: progress.remaining,
                    primaryCompleted: progress.primaryCompleted, primaryRemaining: progress.primaryRemaining}));
            }
        });
        checkFreeze(cohort.source, true);
        progress.current = null; progress.status = progress.remaining ? 'SCOPE_CAPTURED_REVIEW_PENDING' : 'ALL_NATIVE_CAPTURED_REVIEW_PENDING'; save();
        return progress;
    } catch (error) {
        const failure = {at: new Date().toISOString(), checkpoint: progress.current, error: String((error as Error).stack || error)};
        const file = path.join(directory, 'errors', `${Date.now()}-${process.pid}.json`);
        write(file, JSON.stringify(failure, null, 2) + '\n');
        progress.status = 'STOPPED_ON_FIRST_FAILURE'; progress.latestError = file; save(); throw error;
    } finally { fs.unlinkSync(lock); }
}

export function publishApproved(directory: string, reviewFile: string, rootReviewed: boolean) {
    assert(rootReviewed, 'Publication requires the root-reviewed-originals flag');
    directory = path.resolve(directory); reviewFile = path.resolve(reviewFile);
    const cohort = json(path.join(directory, 'cohort.json')), progress = json(path.join(directory, 'progress.json')), review = json(reviewFile);
    assert.equal(review.rootPersonallyViewedOriginalResolution, true);
    assert(Array.isArray(review.images) && review.images.length > 0);
    checkFreeze(cohort.source, true);
    withExpectedSequence(() => {
        for (const approved of review.images) {
            const item = progress.items[approved.id]; assert(item, `No capture: ${approved.id}`);
            assert.equal(item.image.sha256, approved.sha256, 'Review must identify the exact captured PNG');
            assert.equal(sha(item.sourceRecord), item.sourceRecordSha256);
            const retained = run(approved.id, cohort.density);
            assert.equal(retained.key, item.sourceKey); assert.equal(retained.image.sha256, approved.sha256);
            const destination = path.resolve(viewer, item.path);
            assert(destination.startsWith(path.join(docs, 'screenshots/journey') + path.sep));
            const backup = path.join(directory, 'original-images', path.basename(destination));
            if (sha(destination) !== approved.sha256) {
                assert.equal(sha(destination), item.originalSha256, 'Published destination changed outside this cohort');
                fs.mkdirSync(path.dirname(backup), {recursive: true});
                if (!fs.existsSync(backup)) fs.copyFileSync(destination, backup, fs.constants.COPYFILE_EXCL);
                assert.equal(sha(backup), item.originalSha256);
                fs.copyFileSync(path.join(viewer, retained.image.path), destination);
            }
            assert.equal(sha(destination), approved.sha256);
            item.status = 'PUBLISHED_ROOT_REVIEWED'; item.rootReview = {path: reviewFile, sha256: sha(reviewFile)};
        }
    });
    write(path.join(directory, 'progress.json'), JSON.stringify(summarize(progress), null, 2) + '\n');
    return progress;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    const args = process.argv.slice(2), mode = args.shift();
    const option = (name: string, fallback = '') => { const index = args.indexOf(name); return index < 0 ? fallback : args[index + 1]; };
    assert.equal(option('--kind', 'native'), 'native', 'Browser batches remain separate until the early DPR tests pass');
    const result = mode === 'prepare' ? prepareBatch(option('--inventory'), Number(option('--density', '2')))
        : mode === 'run' ? runBatch(option('--cohort'), option('--scope', 'primary'))
        : mode === 'publish' ? publishApproved(option('--cohort'), option('--review'), args.includes('--root-reviewed-originals')) : null;
    assert(result, 'Usage: capture-batch.ts prepare --inventory FILE | run --cohort DIR --scope primary|remaining|all | publish --cohort DIR --review FILE --root-reviewed-originals');
    console.log(JSON.stringify({directory: result.directory, status: result.status, total: result.total,
        primaryTotal: result.primaryTotal, completed: result.completed, remaining: result.remaining}, null, 2));
}
