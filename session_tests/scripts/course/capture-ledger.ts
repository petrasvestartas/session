import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {gzipSync, gunzipSync} from 'node:zlib';
import {hash, json, filesIn} from './model.ts';
import {fileReference} from './capture-storage.ts';

const retained = /\.(png|json|jsonl|log)$/i;
const binary = /\.(exe|dll|wasm|rlib|rmeta|pdb|ppm)$/i;
const digest = (file: string) => fileReference(file).sha256;

function exclusive(file: string, value: unknown) {
    fs.mkdirSync(path.dirname(file), {recursive: true});
    const fd = fs.openSync(file, 'wx');
    try {
        const bytes = JSON.stringify(value, null, 2) + '\n';
        fs.writeFileSync(fd, file.endsWith('.gz') ? gzipSync(bytes) : bytes); fs.fsyncSync(fd);
    }
    finally { fs.closeSync(fd); }
    return fileReference(file);
}

export function ledgerReceiptFile(directory: string, name: 'capture' | 'retirement') {
    const files = ['.json', '.json.gz'].map(suffix => path.resolve(directory, name + suffix)).filter(file => fs.existsSync(file));
    assert(files.length <= 1, 'Ambiguous ledger receipt: ' + path.resolve(directory, name));
    return files[0];
}

function owned(root: string, file: string) {
    root = path.resolve(root); file = path.resolve(file);
    assert.equal(fs.realpathSync(root), root, 'Retirement root or an ancestor is redirected');
    const relative = path.relative(root, file);
    assert(relative && !relative.startsWith('..') && !path.isAbsolute(relative), 'Retirement escapes its owned directory');
    let current = root;
    for (const part of ['', ...relative.split(path.sep)]) {
        current = path.join(current, part);
        if (fs.existsSync(current)) assert(!fs.lstatSync(current).isSymbolicLink(), 'Retirement cannot traverse links');
    }
    return file;
}

export function sealCapture(options: {directory: string; ownedRoots: string[]; transientRoots?: string[]; capture: string; build?: string; evidence: string[]; image: string; execution: any; sourceIntegrity?: any; locks?: string[]; activeLock?: string}) {
    const {directory, execution} = options;
    assert(!ledgerReceiptFile(directory, 'capture'), 'Capture receipt already exists');
    assert(execution?.before?.files?.length && execution.after?.files?.length, 'Actual before/after execution observations are required');
    assert.deepEqual(execution.before.files, execution.after.files);
    assert(Date.parse(execution.after.at) >= Date.parse(execution.before.at));
    for (const file of execution.after.files) assert.equal(digest(file.path), file.sha256);
    for (const lock of options.locks || []) {
        if (lock === options.activeLock) assert.equal(json(lock).pid, process.pid, 'Only the sealing process may hold its lane lock');
        else assert(!fs.existsSync(lock), 'Capture process is still active');
    }
    const roots = [...new Set(options.ownedRoots.map(root => path.resolve(root)))];
    const transientRoots = (options.transientRoots || []).map(root => path.resolve(root));
    const candidates = [...new Set(roots.flatMap(root => filesIn(root)))].map(file => {
        const root = roots.find(root => path.relative(root, file) && !path.relative(root, file).startsWith('..'))!;
        owned(root, file);
        const transient = transientRoots.some(directory => file.startsWith(directory + path.sep));
        const keep = /\.png$/i.test(file) || path.basename(file) === 'presentation.json' || (!transient && retained.test(file));
        return {...fileReference(file), root, retire: !keep};
    });
    const evidence = [...new Set([...options.evidence, options.capture, ...(options.build ? [options.build] : []), ...candidates.filter(file => !binary.test(file.path) && !/\.png$/i.test(file.path)).map(file => file.path)])].map(fileReference);
    const blobs = path.join(path.dirname(directory), 'blobs'); fs.mkdirSync(blobs, {recursive: true});
    for (const file of evidence) {
        const blob = path.join(blobs, file.sha256 + '.gz');
        if (!fs.existsSync(blob)) fs.writeFileSync(blob, gzipSync(fs.readFileSync(file.path)), {flag: 'wx'});
        assert.equal(hash(gunzipSync(fs.readFileSync(blob))), file.sha256);
    }
    const receipt = {schema: 1, status: 'CAPTURED_NOT_PUBLISHED', capture: fileReference(options.capture), image: fileReference(options.image),
        build: options.build ? fileReference(options.build) : null,
        execution, sourceIntegrity: options.sourceIntegrity, evidence, blobs, candidates, locks: options.locks || [], scope: 'Immutable execution evidence; fresh build required for re-execution after retirement.'};
    const reference = exclusive(path.join(directory, 'capture.json.gz'), receipt);
    verifyCaptureLedger(reference.path);
    return reference;
}

export function verifyCaptureLedger(file: string) {
    const directory = path.dirname(file);
    ledgerReceiptFile(directory, 'capture');
    const receipt = json(file);
    assert.equal(receipt.schema, 1);
    assert.deepEqual(receipt.execution.before.files, receipt.execution.after.files);
    if (receipt.sourceIntegrity) {
        assert.deepEqual(receipt.sourceIntegrity.before.files, receipt.sourceIntegrity.after.files);
        for (const input of receipt.sourceIntegrity.after.files)
            assert(receipt.evidence.some((item: any) => item.path === input.path && item.sha256 === input.sha256));
    }
    for (const item of receipt.evidence) assert.equal(hash(gunzipSync(fs.readFileSync(path.join(receipt.blobs, item.sha256 + '.gz')))), item.sha256);
    assert(receipt.evidence.some((item: any) => item.path === receipt.capture.path && item.sha256 === receipt.capture.sha256));
    const capture = JSON.parse(gunzipSync(fs.readFileSync(path.join(receipt.blobs, receipt.capture.sha256 + '.gz'))).toString('utf8'));
    assert.equal(capture.image.sha256, receipt.image.sha256);
    assert.deepEqual(capture.executionIntegrity || capture.host?.executionIntegrity, receipt.execution);
    if (capture.executable) {
        assert(receipt.execution.after.files.some((item: any) => item.path === capture.executable.path && item.sha256 === capture.executable.sha256));
        assert(receipt.evidence.some((item: any) => item.path === capture.compilerArtifact.path && item.sha256 === capture.compilerArtifact.sha256));
        const artifact = JSON.parse(gunzipSync(fs.readFileSync(path.join(receipt.blobs, capture.compilerArtifact.sha256 + '.gz'))).toString('utf8'));
        assert.equal(artifact.executable, capture.executable.compilerReportedPath);
        assert.equal(capture.execute.resources.exitCode, 0); assert.equal(capture.compile.resources.exitCode, 0);
    }
    if (capture.bundle) {
        assert(receipt.build, 'Browser capture requires its immutable compiler receipt');
        const archived = (file: string, sha256?: string) => {
            const item = receipt.evidence.find((item: any) => item.path === file && (!sha256 || item.sha256 === sha256));
            assert(item, 'Missing exact archived build evidence: ' + file);
            return gunzipSync(fs.readFileSync(path.join(receipt.blobs, item.sha256 + '.gz'))).toString('utf8');
        };
        const build = JSON.parse(archived(receipt.build.path, receipt.build.sha256));
        assert.deepEqual(build.source, capture.source); assert.deepEqual(build.bundle, capture.bundle);
        assert.equal(path.resolve(build.compilerArtifact.manifest_path), path.join(build.group.workbench, 'Cargo.toml'));
        assert(build.compilerArtifact.filenames.includes(build.wasm.compilerReportedPath));
        assert(receipt.candidates.some((item: any) => item.path === build.wasm.path && item.sha256 === build.wasm.sha256));
        assert(build.commands.some((command: any) => command.label === 'wasm'));
        const bindgen = build.commands.find((command: any) => command.label === 'bindgen');
        assert.equal(bindgen.args[0], build.bindgen.executable); assert.equal(bindgen.args[1], build.wasm.compilerReportedPath);
        assert(/^[a-f0-9]{64}$/.test(build.bindgen.sha256));
        for (const command of build.commands) {
            assert.equal(command.resources.exitCode, 0); assert.equal(command.resources.timedOut, false);
            assert.deepEqual(JSON.parse(archived(command.resourcesFile, command.resourcesSha256)), command.resources);
            archived(command.log, command.logSha256);
            assert.deepEqual(JSON.parse(archived(command.commandFile)), {executable: command.args[0], arguments: command.args.slice(1)});
        }
        for (const [name, sha256] of Object.entries(capture.bundle))
            assert(receipt.execution.after.files.some((item: any) => item.path.endsWith(path.join('dist', name)) && item.sha256 === sha256));
        for (const name of Object.keys(capture.bundle).filter(name => name.endsWith('.wasm')))
            assert.equal((capture.served || capture.host?.served)[capture.id + '/dist/' + name], capture.bundle[name]);
        assert.equal((capture.resources || capture.host?.resources).exitCode, 0);
    }
    assert.equal(digest(receipt.image.path), receipt.image.sha256);
    const retiredFile = ledgerReceiptFile(directory, 'retirement');
    if (!retiredFile) {
        for (const item of receipt.execution.after.files) assert.equal(digest(item.path), item.sha256, 'Live artifact was replaced');
        return {status: 'CAPTURED_BINARY_AVAILABLE', receipt};
    }
    const retired = json(retiredFile), publishedFile = path.join(directory, 'publication.json'), published = json(publishedFile);
    const completedFile = path.join(directory, 'retired.json');
    const completed = fs.existsSync(completedFile) ? json(completedFile) : null;
    if (completed) assert.equal(completed.retirement.sha256, digest(retiredFile));
    assert.equal(retired.capture.sha256, digest(file)); assert.equal(published.capture.sha256, digest(file));
    assert.equal(retired.publication.sha256, digest(publishedFile));
    assert.deepEqual(retired.files, receipt.candidates.filter((item: any) => item.retire));
    if (completed) assert.equal(completed.removed, retired.files.length);
    assert.equal(published.destination.sha256, receipt.image.sha256);
    assert.equal(hash(gunzipSync(fs.readFileSync(path.join(receipt.blobs, published.evidence.sha256 + '.gz')))), published.evidence.sha256);
    for (const item of receipt.candidates.filter((item: any) => !item.retire)) assert.equal(digest(item.path), item.sha256);
    return {status: completed ? 'PUBLISHED_BINARY_RETIRED' : 'RETIREMENT_INCOMPLETE', receipt, publication: published, retirement: retired,
        currentPublication: fs.existsSync(published.destination.path) && digest(published.destination.path) === receipt.image.sha256,
        execution: completed
            ? 'Verified immutable execution chain; binary retired. Current compiler paths are not execution evidence. Re-execution requires a fresh build.'
            : 'Verified immutable execution chain; retirement recorded but deletion is incomplete. Resume retirement with the same publication evidence.'};
}

function retireLocked(file: string, destination: string, publicationEvidence: string) {
    const verified = verifyCaptureLedger(file), receipt = verified.receipt, directory = path.dirname(file);
    assert.equal(digest(destination), receipt.image.sha256, 'Publication must complete before retirement');
    const evidence = fileReference(publicationEvidence);
    const evidenceBlob = path.join(receipt.blobs, evidence.sha256 + '.gz');
    if (!fs.existsSync(evidenceBlob)) fs.writeFileSync(evidenceBlob, gzipSync(fs.readFileSync(evidence.path)), {flag: 'wx'});
    assert.equal(hash(gunzipSync(fs.readFileSync(evidenceBlob))), evidence.sha256);
    if (verified.publication) {
        assert.deepEqual(verified.publication.evidence, evidence);
        assert.deepEqual(verified.publication.destination, fileReference(destination));
        if (verified.status === 'PUBLISHED_BINARY_RETIRED') return verified;
    }
    const files = receipt.candidates.filter((item: any) => item.retire);
    if (!verified.retirement) {
        for (const item of files) { owned(item.root, item.path); assert.equal(digest(item.path), item.sha256); }
        const publicationFile = path.join(directory, 'publication.json');
        const published = {capture: fileReference(file), destination: fileReference(destination), evidence};
        if (fs.existsSync(publicationFile)) assert.deepEqual(json(publicationFile), published);
        else exclusive(publicationFile, published);
        exclusive(path.join(directory, 'retirement.json.gz'), {capture: fileReference(file), publication: fileReference(publicationFile), at: new Date().toISOString(), files});
    }
    for (const item of files) {
        owned(item.root, item.path);
        if (fs.existsSync(item.path)) { assert.equal(digest(item.path), item.sha256); fs.unlinkSync(item.path); }
    }
    exclusive(path.join(directory, 'retired.json'), {retirement: fileReference(ledgerReceiptFile(directory, 'retirement')!), removed: files.length});
    return verifyCaptureLedger(file);
}

export function retirePublishedCapture(file: string, destination: string, publicationEvidence: string) {
    const ownedLocks: string[] = [];
    try {
        for (const lock of json(file).locks) {
            fs.mkdirSync(path.dirname(lock), {recursive: true});
            const fd = fs.openSync(lock, 'wx'); ownedLocks.push(lock);
            try { fs.writeFileSync(fd, JSON.stringify({pid: process.pid, operation: 'retirement'})); }
            finally { fs.closeSync(fd); }
        }
        return retireLocked(file, destination, publicationEvidence);
    } finally {
        for (const lock of ownedLocks.reverse()) {
            assert.equal(json(lock).pid, process.pid); assert.equal(json(lock).operation, 'retirement'); fs.unlinkSync(lock);
        }
    }
}
