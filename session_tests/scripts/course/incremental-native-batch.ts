import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {prepare as prepareCanonical} from './presentation.ts';
import {runIncremental, verifyIncremental, capturePolicy} from './incremental-native-presentation.ts';
import {filesIn, hash, json, viewer, withExpectedSequence, write} from './model.ts';

const script = fileURLToPath(import.meta.url);
const sha = (file: string) => hash(fs.readFileSync(file));
const absolute = (file: string) => path.isAbsolute(file) ? file : path.join(viewer, file);
const captured = (row: any) => row?.image && ['CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', 'PUBLISHED_ROOT_REVIEWED'].includes(row.status);

function inputDirectories() {
    return ['docs/journey/code', 'docs/journey/checks', 'src', 'assets'].map(name => path.join(viewer, name))
        .concat(path.resolve(viewer, '../session_rust/src'), path.resolve(viewer, '../../wood/rust/src')).filter(fs.existsSync);
}

function check(cohort: any, full = false) {
    assert.equal(sha(cohort.canonicalCohort.path), cohort.canonicalCohort.sha256);
    assert.equal(sha(cohort.inventory.path), cohort.inventory.sha256);
    for (const root of cohort.inputDirectories) {
        assert.deepEqual(filesIn(root).sort(), Object.keys(cohort.inputs).filter(file => file.startsWith(root + path.sep)).sort(),
            'Frozen source file list changed: ' + root);
    }
    for (const [file, expected] of Object.entries(cohort.inputs) as [string, any][]) {
        const actual = fs.statSync(file);
        if (full || actual.size !== expected.size || actual.mtimeMs !== expected.mtimeMs)
            assert.equal(sha(file), expected.sha256, 'Frozen incremental cohort input changed: ' + file);
    }
}

function canonicalCapture(id: string, row: any, density: number) {
    assert(captured(row), 'A canonical exclusion requires an actual captured result');
    assert.equal(sha(row.sourceRecord), row.sourceRecordSha256);
    const result = prepareCanonical(id, density);
    assert.equal(result.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
    assert.equal(result.image.sha256, row.image.sha256);
    assert.equal(sha(absolute(result.image.path)), result.image.sha256);
    return result;
}

function summary(progress: any) {
    const rows = Object.values(progress.items) as any[];
    progress.canonicalCaptured = rows.filter(row => captured(row) && row.backend === 'canonical-retained').length;
    progress.incrementalCaptured = rows.filter(row => captured(row) && row.backend === 'stable-incremental').length;
    progress.completed = progress.canonicalCaptured + progress.incrementalCaptured;
    progress.remaining = progress.total - progress.completed;
    progress.published = rows.filter(row => row.status === 'PUBLISHED_ROOT_REVIEWED').length;
    progress.updated = new Date().toISOString(); return progress;
}

function refreshCanonical(cohort: any, progress: any) {
    const latest = json(cohort.canonicalProgress);
    withExpectedSequence(() => {
        for (const item of cohort.queue) {
            const row = latest.items[item.id], previous = progress.items[item.id];
            if (!captured(row) || previous?.backend === 'stable-incremental' && captured(previous)) continue;
            canonicalCapture(item.id, row, cohort.density);
            progress.items[item.id] = {...previous, ...row, backend: 'canonical-retained', captureNewInThisBatch: false,
                status: previous?.status === 'PUBLISHED_ROOT_REVIEWED' ? previous.status : row.status,
                rootReview: previous?.rootReview || row.rootReview};
        }
    });
    return summary(progress);
}

export function prepareIncrementalBatch(canonicalDirectory: string) {
    canonicalDirectory = path.resolve(canonicalDirectory);
    const canonicalFile = path.join(canonicalDirectory, 'cohort.json'), canonical = json(canonicalFile);
    assert.equal(canonical.kind, 'native');
    const policy = capturePolicy(), inputs = {...canonical.source};
    for (const file of [script, fileURLToPath(new URL('incremental-native-presentation.ts', import.meta.url)), policy.guard]) {
        const stat = fs.statSync(file); inputs[file] = {sha256: sha(file), size: stat.size, mtimeMs: stat.mtimeMs};
    }
    const key = hash(JSON.stringify({canonical: sha(canonicalFile), inputs, policy}));
    const directory = path.join(viewer, 'target/course-presentation/incremental-batches', key.slice(0, 20));
    if (fs.existsSync(directory)) {
        const retained = json(path.join(directory, 'cohort.json')); check(retained, true);
        const progress = refreshCanonical(retained, json(path.join(directory, 'progress.json')));
        write(path.join(directory, 'progress.json'), JSON.stringify(progress, null, 2) + '\n');
        return {...retained, canonicalCapturedAtPrepare: progress.canonicalCaptured, remainingAtPrepare: progress.remaining};
    }
    const priorFile = path.join(canonicalDirectory, 'progress.json'), priorBytes = fs.readFileSync(priorFile), prior = JSON.parse(priorBytes.toString('utf8'));
    const cohort = {status: 'FROZEN_AWAITING_ACTUAL_BENCHMARK', key, directory, density: canonical.density, capturePolicy: policy,
        canonicalCohort: {path: canonicalFile, sha256: sha(canonicalFile)}, canonicalProgress: priorFile,
        canonicalProgressAtPrepare: {path: path.join(directory, 'canonical-progress-at-prepare.json'), sha256: hash(priorBytes)},
        inventory: canonical.inventory, inputs, inputDirectories: inputDirectories(), queue: canonical.queue, total: canonical.queue.length,
        policy: 'Retain and verify every existing checkpoint capture; compile and execute only missing IDs. Stable workbench adoption requires the actual two-source canonical/incremental benchmark.'};
    check(cohort, true);
    const items: Record<string, any> = {};
    withExpectedSequence(() => {
        for (const item of cohort.queue) {
            const row = prior.items[item.id]; if (!captured(row)) continue;
            canonicalCapture(item.id, row, cohort.density);
            items[item.id] = {...row, backend: 'canonical-retained', captureNewInThisBatch: false};
        }
    });
    write(cohort.canonicalProgressAtPrepare.path, priorBytes);
    write(path.join(directory, 'cohort.json'), JSON.stringify(cohort, null, 2) + '\n');
    const progress = summary({status: 'NOT_EXECUTED_BENCHMARK_REQUIRED', total: cohort.total, current: null, items});
    write(path.join(directory, 'progress.json'), JSON.stringify(progress, null, 2) + '\n');
    return {...cohort, canonicalCapturedAtPrepare: progress.canonicalCaptured, remainingAtPrepare: progress.remaining};
}

export function validateIncrementalBenchmark(proofFile: string) {
    const proof = json(proofFile); assert.equal(proof.rootApprovedAdoption, true);
    const cases = ['42k-input', '43b-view'].map(id => {
        const pair = proof.pairs.find((row: any) => row.id === id); assert(pair, 'Missing actual benchmark: ' + id);
        assert.equal(sha(pair.incremental.path), pair.incremental.sha256);
        assert.equal(sha(pair.canonical.path), pair.canonical.sha256);
        const incremental = verifyIncremental(pair.incremental.path), canonical = json(pair.canonical.path);
        assert.equal(incremental.id, id); assert.equal(canonical.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
        const retained = prepareCanonical(id, 2); assert.equal(retained.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
        assert.deepEqual(incremental.source, canonical.source); assert.deepEqual(canonical.source, retained.source);
        assert.equal(sha(absolute(canonical.image.path)), canonical.image.sha256);
        assert.equal(incremental.image.sha256, canonical.image.sha256, 'Fresh incremental GPU pixels must exactly match this checkpoint canonical capture');
        assert.equal(incremental.image.width, canonical.image.width); assert.equal(incremental.image.height, canonical.image.height);
        assert.equal(incremental.profile.CARGO_INCREMENTAL, '1');
        const policy = capturePolicy();
        if (incremental.profile.capture) assert.deepEqual(incremental.profile.capture, policy);
        else assert.equal(policy.processors, 2, 'The preserved original benchmark supports only the two-CPU policy');
        assert.equal(incremental.compile.resources.processors, policy.processors); assert.equal(incremental.execute.resources.processors, policy.processors);
        assert.equal(incremental.compile.resources.jobCommitLimitBytes, policy.memoryMiB * 1048576);
        assert.equal(incremental.execute.resources.jobCommitLimitBytes, policy.memoryMiB * 1048576);
        assert(incremental.compile.command.args.includes('--offline') && incremental.compile.command.args.includes('--locked'));
        return incremental;
    });
    assert.equal(cases[0].workbench, cases[1].workbench, 'Both benchmark sources must use the same stable compiler path');
    return {path: path.resolve(proofFile), sha256: sha(proofFile), cases: cases.map(row => ({id: row.id, record: row.recordFile,
        imageSha256: row.image.sha256, executableSha256: row.executable.sha256, compileSeconds: row.compile.seconds,
        executeSeconds: row.execute.seconds, changedInputs: row.sync.changed, unchangedInputs: row.sync.unchanged}))};
}

export function runIncrementalBatch(directory: string, benchmarkFile: string) {
    directory = path.resolve(directory); const cohort = json(path.join(directory, 'cohort.json'));
    const canonicalDirectory = path.dirname(cohort.canonicalCohort.path);
    assert(!fs.existsSync(path.join(canonicalDirectory, 'running.lock')), 'Wait until the canonical primary batch releases the heavy lane');
    check(cohort, true); assert.deepEqual(cohort.capturePolicy, capturePolicy()); const benchmark = validateIncrementalBenchmark(benchmarkFile);
    const lock = path.join(directory, 'running.lock'); fs.closeSync(fs.openSync(lock, 'wx'));
    const progress = json(path.join(directory, 'progress.json'));
    const save = () => write(path.join(directory, 'progress.json'), JSON.stringify(summary(progress), null, 2) + '\n');
    try {
        refreshCanonical(cohort, progress);
        progress.status = 'RUNNING_AFTER_VERIFIED_BENCHMARK'; progress.benchmark = benchmark; save();
        const latestCanonical = json(cohort.canonicalProgress);
        withExpectedSequence(() => {
            for (const item of cohort.queue) {
                check(cohort); progress.current = item.id; save(); const started = Date.now();
                const previous = progress.items[item.id], canonical = latestCanonical.items[item.id];
                if (previous?.backend === 'stable-incremental' && captured(previous)) {
                    assert.equal(sha(previous.sourceRecord), previous.sourceRecordSha256);
                    const retained = verifyIncremental(previous.sourceRecord);
                    assert.equal(retained.image.sha256, previous.image.sha256); continue;
                }
                if (captured(canonical)) {
                    const retained = progress.items[item.id];
                    assert.equal(sha(retained.sourceRecord), retained.sourceRecordSha256);
                    assert.equal(sha(absolute(retained.image.path)), retained.image.sha256); continue;
                }
                const snapshot = prepareCanonical(item.id, cohort.density);
                if (snapshot.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW') {
                    const sourceRecord = path.join(snapshot.directory, 'presentation.json');
                    progress.items[item.id] = {...item, status: snapshot.status, backend: 'canonical-retained', captureNewInThisBatch: false,
                        sourceKey: snapshot.key, sourceRecord, sourceRecordSha256: sha(sourceRecord), image: snapshot.image}; save(); continue;
                }
                const result = runIncremental(item.id, cohort.density); assert.equal(result.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
                check(cohort);
                progress.items[item.id] = {...item, status: result.status, backend: 'stable-incremental', captureNewInThisBatch: true,
                    sourceKey: result.key, snapshotKey: result.snapshotKey, sourceRecord: result.recordFile, sourceRecordSha256: sha(result.recordFile),
                    executable: result.executable, image: result.image, compileLogSha256: result.compile.logSha256,
                    executeLogSha256: result.execute.logSha256, elapsedSeconds: (Date.now() - started) / 1000};
                save(); console.log(JSON.stringify({id: item.id, completed: progress.completed, remaining: progress.remaining,
                    canonicalCaptured: progress.canonicalCaptured, incrementalCaptured: progress.incrementalCaptured,
                    compileSeconds: result.compile.seconds, executeSeconds: result.execute.seconds}));
            }
        });
        check(cohort, true); progress.current = null; progress.status = 'ALL_NATIVE_CAPTURED_REVIEW_PENDING'; save(); return progress;
    } catch (error) {
        const file = path.join(directory, 'errors', `${Date.now()}-${process.pid}.json`);
        write(file, JSON.stringify({id: progress.current, error: String((error as Error).stack || error)}, null, 2) + '\n');
        progress.status = 'STOPPED_ON_FIRST_FAILURE'; progress.latestError = file; save(); throw error;
    } finally { fs.unlinkSync(lock); }
}

export function publishIncrementalApproved(directory: string, reviewFile: string, rootReviewed: boolean) {
    assert(rootReviewed); directory = path.resolve(directory); reviewFile = path.resolve(reviewFile);
    const cohort = json(path.join(directory, 'cohort.json')), progress = json(path.join(directory, 'progress.json')), review = json(reviewFile);
    assert.equal(review.rootPersonallyViewedOriginalResolution, true); check(cohort, true);
    withExpectedSequence(() => {
        for (const approved of review.images) {
            const item = progress.items[approved.id]; assert(captured(item));
            assert.equal(item.image.sha256, approved.sha256); assert.equal(sha(item.sourceRecord), item.sourceRecordSha256);
            if (item.backend === 'stable-incremental') {
                const retained = verifyIncremental(item.sourceRecord); assert.equal(retained.image.sha256, approved.sha256);
            } else canonicalCapture(approved.id, item, cohort.density);
            const image = absolute(item.image.path); assert.equal(sha(image), approved.sha256);
            const destination = path.resolve(viewer, item.path);
            assert(destination.startsWith(path.join(viewer, 'docs/screenshots/journey') + path.sep));
            const current = sha(destination), backup = path.join(directory, 'original-images', path.basename(destination));
            if (current !== approved.sha256) {
                assert(current === item.originalSha256 || current === approved.explicitlyReviewedCurrentDestinationSHA,
                    'Preserve a manual publication unless its exact existing SHA is explicitly reviewed');
                fs.mkdirSync(path.dirname(backup), {recursive: true});
                if (!fs.existsSync(backup)) fs.copyFileSync(destination, backup, fs.constants.COPYFILE_EXCL);
                assert.equal(sha(backup), current); fs.copyFileSync(image, destination);
                item.originalBackup = {path: backup, sha256: current};
            } else item.preexistingMatchingPublication = true;
            assert.equal(sha(destination), approved.sha256);
            item.status = 'PUBLISHED_ROOT_REVIEWED'; item.rootReview = {path: reviewFile, sha256: sha(reviewFile)};
        }
    });
    write(path.join(directory, 'progress.json'), JSON.stringify(summary(progress), null, 2) + '\n'); return progress;
}

if (process.argv[1] && path.resolve(process.argv[1]) === script) {
    const args = process.argv.slice(2), mode = args.shift();
    const option = (name: string) => { const index = args.indexOf(name); return index < 0 ? '' : args[index + 1]; };
    const result = mode === 'prepare' ? prepareIncrementalBatch(option('--canonical-cohort'))
        : mode === 'check-benchmark' ? validateIncrementalBenchmark(option('--benchmark'))
        : mode === 'run' ? runIncrementalBatch(option('--cohort'), option('--benchmark'))
        : mode === 'publish' ? publishIncrementalApproved(option('--cohort'), option('--review'), args.includes('--root-reviewed-originals')) : null;
    assert(result, 'Usage: incremental-native-batch.ts prepare --canonical-cohort DIR | check-benchmark --benchmark FILE | run --cohort DIR --benchmark FILE | publish --cohort DIR --review FILE --root-reviewed-originals');
    console.log(JSON.stringify({directory: result.directory, status: result.status, total: result.total,
        canonicalCaptured: result.canonicalCapturedAtPrepare ?? result.canonicalCaptured, remaining: result.remainingAtPrepare ?? result.remaining,
        benchmark: result.cases}, null, 2));
}
