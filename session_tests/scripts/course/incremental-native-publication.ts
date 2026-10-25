import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {bindings} from './presentation.ts';
import {verifyIncremental} from './incremental-native-presentation.ts';
import {assetsFor, expected, filesIn, hash, lockFor, viewer, withExpectedSequence} from './model.ts';

const script = fileURLToPath(import.meta.url);
const absolute = (file: string) => path.isAbsolute(file) ? file : path.resolve(viewer, file);
const sha = (file: string) => hash(fs.readFileSync(absolute(file)));
const ref = (file: string) => ({path: absolute(file), sha256: sha(file)});
const captured = (row: any) => row?.image && ['CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', 'PUBLISHED_ROOT_REVIEWED'].includes(row.status);

function json(file: string) {
    let failure: unknown;
    for (let attempt = 0; attempt < 20; attempt++) {
        try {
            const bytes = fs.readFileSync(absolute(file));
            const result = JSON.parse(bytes.toString(bytes[0] === 255 && bytes[1] === 254 ? 'utf16le' : 'utf8').replace(/^\uFEFF/, ''));
            if (fs.readFileSync(absolute(file)).equals(bytes)) return result;
        } catch (error) { failure = error; }
        Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 20);
    }
    throw Error(`Cannot read a stable JSON snapshot: ${file}: ${failure}`);
}

function atomic(file: string, bytes: string | Buffer) {
    fs.mkdirSync(path.dirname(file), {recursive: true});
    const temporary = file + `.tmp-${process.pid}-${Date.now()}`;
    const descriptor = fs.openSync(temporary, 'wx');
    try { fs.writeFileSync(descriptor, bytes); fs.fsyncSync(descriptor); }
    finally { fs.closeSync(descriptor); }
    fs.renameSync(temporary, file);
}

function context(directory: string) {
    directory = absolute(directory);
    const cohortFile = path.join(directory, 'cohort.json'), cohort = json(cohortFile);
    assert(cohort.inputs && cohort.canonicalCohort, 'Supply an incremental native cohort');
    assert.equal(sha(cohort.canonicalCohort.path), cohort.canonicalCohort.sha256);
    assert.equal(sha(cohort.inventory.path), cohort.inventory.sha256);
    for (const root of cohort.inputDirectories)
        assert.deepEqual(filesIn(root).sort(), Object.keys(cohort.inputs).filter(file => file.startsWith(root + path.sep)).sort());
    for (const [file, input] of Object.entries(cohort.inputs) as [string, any][])
        assert.equal(sha(file), input.sha256, 'Frozen capture input changed: ' + file);
    return {directory, cohort, cohortFile, progress: json(path.join(directory, 'progress.json')),
        ledger: path.join(directory, 'publication')};
}

function verifyCapture(id: string, source: {path: string; sha256: string}, density: number) {
    assert.equal(sha(source.path), source.sha256, 'Capture receipt changed');
    const record = json(source.path);
    assert.equal(record.source.checkpoint, id);
    assert.equal(record.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
    assert.deepEqual(record.source, bindings(id));
    assert.equal(record.density, density);
    if (record.executable && record.inputs) verifyIncremental(absolute(source.path));
    else {
        const kernel = path.relative(record.directory, path.resolve(viewer, '../session_rust')).split(path.sep).join('/');
        for (const [name, text] of Object.entries(expected(id))) {
            const wanted = name === 'Cargo.toml' ? text.replaceAll('../../../session_rust', kernel) : text;
            assert.equal(fs.readFileSync(path.join(record.directory, name), 'utf8'), wanted, name);
        }
        assert.equal(sha(path.join(record.directory, 'Cargo.lock')), sha(lockFor(id)));
        for (const name of assetsFor(id)) assert.equal(sha(path.join(record.directory, name)), sha(path.join(viewer, name)));
        assert.equal(sha(record.driver.path), record.driver.sha256);
        assert.equal(record.command.returncode, 0);
        const resources = json(path.join(viewer, 'target/course-checks', record.command.name + '.resources.json'));
        assert.equal(resources.exitCode, 0); assert.equal(resources.timedOut, false);
        assert.equal(path.resolve(resources.workingDirectory), path.resolve(record.directory));
    }
    const bytes = fs.readFileSync(absolute(record.image.path));
    assert(bytes.subarray(0, 8).equals(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])));
    assert.equal(hash(bytes), record.image.sha256);
    assert.equal(bytes.readUInt32BE(16), record.image.width); assert.equal(bytes.readUInt32BE(20), record.image.height);
    assert.equal(record.image.width, record.source.logicalSize[0] * density);
    assert.equal(record.image.height, record.source.logicalSize[1] * density);
    return record;
}

function destination(file: string) {
    const target = absolute(file), root = path.join(viewer, 'docs/screenshots/journey');
    assert.equal(path.dirname(target), root, 'Only inventory root PNGs may be published');
    assert.equal(path.extname(target), '.png');
    assert.equal(fs.realpathSync(root), root);
    assert(fs.lstatSync(target).isFile() && !fs.lstatSync(target).isSymbolicLink());
    return target;
}

function events(ledger: string) {
    const folder = path.join(ledger, 'events');
    const rows: any[] = []; let previous: string | null = null;
    if (!fs.existsSync(folder)) return rows;
    for (const name of fs.readdirSync(folder).filter(name => name.endsWith('.json')).sort()) {
        const file = path.join(folder, name), row = json(file);
        assert.equal(row.sequence, rows.length + 1); assert.equal(row.previousEventSha256, previous);
        previous = sha(file);
        assert.equal(name, `${String(row.sequence).padStart(6, '0')}-${previous.slice(0, 16)}.json`, 'Publication event changed');
        rows.push({...row, event: {path: file, sha256: previous}});
    }
    return rows;
}

function append(ledger: string, data: any) {
    const previous = events(ledger), sequence = previous.length + 1;
    const row = {sequence, previousEventSha256: previous.at(-1)?.event.sha256 || null,
        createdUtc: new Date().toISOString(), publisher: ref(script), ...data};
    const bytes = JSON.stringify(row, null, 2) + '\n';
    const file = path.join(ledger, 'events', `${String(sequence).padStart(6, '0')}-${hash(bytes).slice(0, 16)}.json`);
    assert(!fs.existsSync(file)); atomic(file, bytes);
    atomic(path.join(ledger, 'index.json'), JSON.stringify({events: sequence, lastEvent: ref(file),
        progressWriter: 'Capture batch only; this publication ledger never updates progress.json.'}, null, 2) + '\n');
    return {...row, event: ref(file)};
}

function verifyManual(reference: {path: string; sha256: string}, item: any, image: any, source: any) {
    assert.equal(sha(reference.path), reference.sha256);
    const receipt = json(reference.path);
    assert.equal(receipt.rootPersonallyViewedOriginalResolution, true);
    const row = receipt.images.find((row: any) => row.id === item.id && absolute(row.destination) === absolute(item.path));
    assert(row); assert.equal(row.image.sha256, image.sha256);
    assert.equal(absolute(row.sourceRecord.path), absolute(source.path)); assert.equal(row.sourceRecord.sha256, source.sha256);
    assert.equal(row.originalSha256, item.originalSha256);
    assert.equal(sha(row.originalBackup), item.originalSha256);
    assert.equal(sha(item.path), image.sha256);
    return {receipt: reference, originalBackup: ref(row.originalBackup)};
}

function inheritedPublication(item: any) {
    assert.equal(item.status, 'PUBLISHED_ROOT_REVIEWED');
    assert.equal(sha(item.rootReview.path), item.rootReview.sha256);
    const review = json(item.rootReview.path), row = review.images.find((row: any) => row.id === item.id);
    assert.equal(review.rootPersonallyViewedOriginalResolution, true);
    assert(row); assert.equal(row.sha256, item.image.sha256);
    const backup = item.originalBackup || ref(path.join(item.retainedFromCohort, 'original-images', path.basename(item.path)));
    assert.equal(sha(backup.path), item.originalSha256);
    return {receipt: item.rootReview, originalBackup: {path: absolute(backup.path), sha256: item.originalSha256}};
}

export function checkPublication(directory: string, reviewFile: string) {
    const c = context(directory), reviewReference = ref(reviewFile), review = json(reviewFile);
    assert.equal(sha(reviewFile), reviewReference.sha256, 'Review changed while being read');
    assert.equal(review.rootPublicationApproval?.approved, true, 'Root publication approval is separate from visual review');
    assert.equal(review.rootPublicationApproval?.agentId, '/root');
    assert(!Object.hasOwn(review, 'rootPersonallyViewedOriginalResolution'), 'Use truthful per-image reviewer identity');
    assert(Array.isArray(review.images) && review.images.length);
    assert.equal(new Set(review.images.map((row: any) => row.id)).size, review.images.length);
    const prior = events(c.ledger);
    const images = withExpectedSequence(() => review.images.map((approved: any) => {
        const item = c.progress.items[approved.id], queued = c.cohort.queue.find((row: any) => row.id === approved.id);
        assert(queued && captured(item), 'Only an actual completed queue capture may be published');
        assert.equal(item.path, queued.path); assert.equal(item.originalSha256, queued.originalSha256);
        const source = {path: item.sourceRecord, sha256: item.sourceRecordSha256};
        assert.equal(absolute(approved.sourceRecord.path), absolute(source.path)); assert.equal(approved.sourceRecord.sha256, source.sha256);
        const capture = verifyCapture(item.id, source, c.cohort.density);
        assert.equal(capture.key, item.sourceKey);
        assert.equal(approved.sha256, capture.image.sha256); assert.equal(item.image.sha256, capture.image.sha256);
        const visual = approved.imageReviewer;
        assert(visual && typeof visual.agentId === 'string' && visual.agentId.length);
        assert.equal(visual.verdict, 'approved');
        assert.equal(visual.originalResolution, true); assert.equal(visual.sha256, capture.image.sha256);
        let representative: any = null;
        if (approved.reviewedRepresentative) {
            representative = approved.reviewedRepresentative;
            assert(c.cohort.queue.some((row: any) => row.id === representative.id));
            const viewed = verifyCapture(representative.id, representative.sourceRecord, c.cohort.density);
            assert.equal(viewed.image.sha256, capture.image.sha256, 'Visual grouping requires exact full PNG byte identity');
            assert.equal(visual.sourceRecordSha256, representative.sourceRecord.sha256);
            assert.equal(approved.duplicateOfReviewedSHA, capture.image.sha256);
        } else assert.equal(visual.sourceRecordSha256, source.sha256);
        const target = destination(item.path), current = sha(target);
        const backup = path.join(c.ledger, 'backups', `${item.id}-${current}.png`);
        const last = prior.findLast((row: any) => row.id === item.id && row.image.sha256 === capture.image.sha256);
        let manual: any = null;
        if (approved.manualPublication) manual = verifyManual(approved.manualPublication, item, capture.image, source);
        else if (approved.inheritPublishedCaptureReview) manual = inheritedPublication(item);
        if (current === capture.image.sha256) {
            assert(last || manual || fs.existsSync(path.join(c.ledger, 'backups', `${item.id}-${item.originalSha256}.png`)),
                'Matching manual publication needs its pinned original backup/receipt');
            if (last) assert.equal(sha(last.originalBackup.path), last.originalBackup.sha256);
        } else assert(current === item.originalSha256 || current === approved.explicitlyReviewedCurrentDestinationSHA,
            'Unknown manual destination change needs explicit exact-SHA replacement approval');
        return {id: item.id, destination: target, originalSha256: item.originalSha256, currentSha256: current,
            image: {...capture.image, path: absolute(capture.image.path)}, sourceRecord: source,
            imageReviewer: visual, reviewedRepresentative: representative, duplicateOfReviewedSHA: approved.duplicateOfReviewedSHA,
            backup, previousPublication: last?.event || null, manual};
    }));
    return {context: c, review: reviewReference, rootPublicationApproval: review.rootPublicationApproval, images};
}

export function publishReviewed(directory: string, reviewFile: string) {
    const ledger = path.join(absolute(directory), 'publication'); fs.mkdirSync(ledger, {recursive: true});
    const lock = path.join(ledger, 'publication.lock'), descriptor = fs.openSync(lock, 'wx');
    try {
        const plan = checkPublication(directory, reviewFile), reviewBytes = fs.readFileSync(plan.review.path);
        assert.equal(hash(reviewBytes), plan.review.sha256);
        const retainedReview = path.join(ledger, 'reviews', plan.review.sha256 + '.json');
        if (!fs.existsSync(retainedReview)) atomic(retainedReview, reviewBytes);
        assert.equal(sha(retainedReview), plan.review.sha256);
        const output = [];
        for (const item of plan.images) {
            assert.equal(sha(item.sourceRecord.path), item.sourceRecord.sha256);
            assert.equal(sha(item.image.path), item.image.sha256); assert.equal(sha(item.destination), item.currentSha256);
            let originalBackup = item.manual?.originalBackup;
            if (item.currentSha256 !== item.image.sha256) {
                if (!fs.existsSync(item.backup)) atomic(item.backup, fs.readFileSync(item.destination));
                assert.equal(sha(item.backup), item.currentSha256); originalBackup = ref(item.backup);
                append(ledger, {status: 'PUBLICATION_INTENT', cohort: ref(plan.context.cohortFile),
                    id: item.id, destination: item.destination, image: item.image, sourceRecord: item.sourceRecord,
                    originalBackup, review: ref(retainedReview), rootPublicationApproval: plan.rootPublicationApproval});
                atomic(item.destination, fs.readFileSync(item.image.path));
            } else if (!originalBackup) {
                const last = events(ledger).findLast((row: any) => row.id === item.id && row.image.sha256 === item.image.sha256);
                originalBackup = last?.originalBackup || ref(path.join(ledger, 'backups', `${item.id}-${item.originalSha256}.png`));
            }
            assert.equal(sha(item.destination), item.image.sha256); assert.equal(sha(originalBackup.path), originalBackup.sha256);
            output.push(append(ledger, {status: 'PUBLISHED_REVIEWED', action: item.currentSha256 === item.image.sha256 ? 'RECONCILED_EXISTING_BYTES' : 'PUBLISHED_NEW_BYTES',
                cohort: ref(plan.context.cohortFile), id: item.id, destination: item.destination, image: item.image,
                sourceRecord: item.sourceRecord, imageReviewer: item.imageReviewer,
                reviewedRepresentative: item.reviewedRepresentative, duplicateOfReviewedSHA: item.duplicateOfReviewedSHA,
                rootPublicationApproval: plan.rootPublicationApproval, review: ref(retainedReview),
                originalBackup, manualPublication: item.manual?.receipt || null}));
        }
        return {status: 'PUBLISHED_IN_SEPARATE_LEDGER', captureProgressUntouched: true, events: output};
    } finally { fs.closeSync(descriptor); fs.unlinkSync(lock); }
}

export function stageGroups(directory: string, ids: string[] = []) {
    const c = context(directory), prior = events(c.ledger), groups = new Map<string, any[]>();
    for (const id of ids) assert(captured(c.progress.items[id]), 'Requested capture is not completed: ' + id);
    withExpectedSequence(() => {
        for (const item of Object.values(c.progress.items) as any[]) {
            if (!captured(item) || ids.length && !ids.includes(item.id)) continue;
            if (!ids.length && (item.status === 'PUBLISHED_ROOT_REVIEWED' || prior.some(row => row.status === 'PUBLISHED_REVIEWED' && row.id === item.id && row.image.sha256 === item.image.sha256))) continue;
            const sourceRecord = {path: item.sourceRecord, sha256: item.sourceRecordSha256};
            const r = verifyCapture(item.id, sourceRecord, c.cohort.density);
            const group = groups.get(r.image.sha256) || [];
            group.push({id: item.id, sourceRecord, blankExpected: item.blankExpected,
                image: {...r.image, path: absolute(r.image.path)}}); groups.set(r.image.sha256, group);
        }
    });
    return {status: 'REVIEW_REQUIRED_NO_PUBLICATION', cohort: ref(c.cohortFile),
        rootPublicationApproval: {approved: false, agentId: '/root'}, groups: [...groups].map(([sha256, members]) => ({sha256, representative: members[0], members})),
        images: [...groups].flatMap(([sha256, members]) => members.map(member => ({id: member.id, sourceRecord: member.sourceRecord, sha256,
            imageReviewer: {agentId: '', sha256, sourceRecordSha256: members[0].sourceRecord.sha256, originalResolution: false, verdict: 'pending'},
            ...(member.id === members[0].id ? {} : {reviewedRepresentative: {id: members[0].id, sourceRecord: members[0].sourceRecord}, duplicateOfReviewedSHA: sha256})}))),
        policy: 'An agent actually views each representative original PNG; root separately approves publication. Each member keeps its own actual source-bound capture.'};
}

export function publicationStatus(directory: string) {
    const c = context(directory), rows = events(c.ledger), published = new Set<string>();
    for (const item of Object.values(c.progress.items) as any[])
        if (item.status === 'PUBLISHED_ROOT_REVIEWED' && sha(item.path) === item.image.sha256) published.add(item.id);
    const changed = [];
    for (const row of new Map(rows.filter(row => row.status === 'PUBLISHED_REVIEWED').map(row => [row.id, row])).values()) {
        if (sha(row.destination) === row.image.sha256) published.add(row.id);
        else { published.delete(row.id); changed.push(row.id); }
    }
    return {captureCompleted: c.progress.completed, captureRemaining: c.progress.remaining,
        published: published.size, publicationEvents: rows.length, destinationChangedSincePublication: changed,
        progressWriter: 'Capture batch', publicationWriter: 'Separate append-only publication ledger'};
}

if (process.argv[1] && path.resolve(process.argv[1]) === script) {
    const args = process.argv.slice(2), mode = args.shift();
    const option = (name: string) => { const index = args.indexOf(name); return index < 0 ? '' : args[index + 1]; };
    const directory = option('--cohort');
    const result = mode === 'check' ? (() => { const p = checkPublication(directory, option('--review')); return {status: 'DRY_CHECK_PASS', images: p.images}; })()
        : mode === 'publish' ? publishReviewed(directory, option('--review'))
        : mode === 'stage' ? stageGroups(directory, option('--ids').split(',').filter(Boolean))
        : mode === 'status' ? publicationStatus(directory) : null;
    assert(result, 'Use stage|status --cohort DIR; check|publish --cohort DIR --review ROOT_APPROVED_JSON');
    if (option('--output')) { const file = absolute(option('--output')); assert(!fs.existsSync(file)); atomic(file, JSON.stringify(result, null, 2) + '\n'); console.log(file); }
    else console.log(JSON.stringify(result, null, 2));
}
