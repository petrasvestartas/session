import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {cargoTarget, fileReference, observeExecution, finishExecution, checkWorkbenchOwnership} from './capture-storage.ts';
import {sealCapture, retirePublishedCapture, verifyCaptureLedger} from './capture-ledger.ts';
import {selectAffected} from './compact-capture.ts';

function fixture(transient = false) {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'capture-storage-')), owned = path.join(root, 'owned');
    fs.mkdirSync(owned);
    const workspace = path.join(owned, 'workspace'); fs.mkdirSync(workspace);
    const sourceJson = path.join(workspace, 'course.json'); fs.writeFileSync(sourceJson, '{"source":"exact"}');
    const files = {exe: path.join(owned, 'run.exe'), source: path.join(owned, 'source.rs'), image: path.join(owned, 'image.png'), record: path.join(owned, 'result.json')};
    for (const [name, file] of Object.entries(files)) fs.writeFileSync(file, name);
    const execution = finishExecution(observeExecution([files.exe, files.source]));
    fs.writeFileSync(files.record, JSON.stringify({image: fileReference(files.image), executionIntegrity: execution}));
    const receipt = sealCapture({directory: path.join(root, 'proof', 'case'), ownedRoots: [owned], capture: files.record,
        transientRoots: transient ? [workspace] : [], evidence: [files.source], image: files.image, execution});
    return {root, owned, files, sourceJson, receipt: receipt.path, cleanup: () => fs.rmSync(root, {recursive: true})};
}

test('shared cargo target honors the caller', () => {
    const old = process.env.CARGO_TARGET_DIR;
    try { process.env.CARGO_TARGET_DIR = 'shared-target'; assert.equal(cargoTarget(), path.resolve('shared-target')); }
    finally { if (old === undefined) delete process.env.CARGO_TARGET_DIR; else process.env.CARGO_TARGET_DIR = old; }
});

test('actual execution observation rejects changed bytes', () => {
    const f = fixture();
    try { const before = observeExecution([f.files.exe]); fs.appendFileSync(f.files.exe, 'changed'); assert.throws(() => finishExecution(before)); }
    finally { f.cleanup(); }
});

test('live receipt rejects a replaced binary', () => {
    const f = fixture();
    try { assert.equal(verifyCaptureLedger(f.receipt).status, 'CAPTURED_BINARY_AVAILABLE'); fs.writeFileSync(f.files.exe, 'replacement'); assert.throws(() => verifyCaptureLedger(f.receipt)); }
    finally { f.cleanup(); }
});

test('publication precedes retirement and preserves PNG and immutable proof', () => {
    const f = fixture();
    try {
        const destination = path.join(f.root, 'published.png'), publication = path.join(f.root, 'published.json');
        fs.copyFileSync(f.files.image, destination); fs.writeFileSync(publication, JSON.stringify({image: fileReference(destination)}));
        assert.equal(retirePublishedCapture(f.receipt, destination, publication).status, 'PUBLISHED_BINARY_RETIRED');
        assert(!fs.existsSync(f.files.exe)); assert(!fs.existsSync(f.files.source)); assert(fs.existsSync(f.files.image)); assert(fs.existsSync(f.files.record));
        fs.writeFileSync(f.files.exe, 'different new execution');
        const verified = verifyCaptureLedger(f.receipt); assert.equal(verified.status, 'PUBLISHED_BINARY_RETIRED'); assert.match(verified.execution!, /binary retired/);
    } finally { f.cleanup(); }
});

test('failed publication leaves owned artifacts intact', () => {
    const f = fixture();
    try {
        const destination = path.join(f.root, 'wrong.png'); fs.writeFileSync(destination, 'wrong');
        assert.throws(() => retirePublishedCapture(f.receipt, destination, f.files.record)); assert(fs.existsSync(f.files.exe));
        assert(!fs.existsSync(path.join(path.dirname(f.receipt), 'retirement.json')));
    } finally { f.cleanup(); }
});

test('active lane prevents retirement', () => {
    const f = fixture();
    try {
        const data = JSON.parse(fs.readFileSync(f.receipt, 'utf8')), lock = path.join(f.root, 'active.lock');
        data.locks = [lock]; fs.writeFileSync(f.receipt, JSON.stringify(data)); fs.writeFileSync(lock, 'active');
        assert.throws(() => retirePublishedCapture(f.receipt, f.files.image, f.files.record)); assert(fs.existsSync(f.files.exe));
    } finally { f.cleanup(); }
});

test('retired receipt verifies archived evidence and publication hash', () => {
    const f = fixture();
    try {
        const publication = path.join(f.root, 'publication.json'); fs.writeFileSync(publication, '{}');
        retirePublishedCapture(f.receipt, f.files.image, publication);
        const oldHash = fileReference(publication).sha256;
        fs.appendFileSync(publication, 'later generation'); assert.equal(verifyCaptureLedger(f.receipt).status, 'PUBLISHED_BINARY_RETIRED');
        const receipt = JSON.parse(fs.readFileSync(f.receipt, 'utf8'));
        fs.writeFileSync(path.join(receipt.blobs, oldHash + '.gz'), 'tamper'); assert.throws(() => verifyCaptureLedger(f.receipt));
    } finally { f.cleanup(); }
});

test('source archive corruption is detected', () => {
    const f = fixture();
    try {
        const receipt = JSON.parse(fs.readFileSync(f.receipt, 'utf8'));
        fs.writeFileSync(path.join(receipt.blobs, fileReference(f.files.source).sha256 + '.gz'), 'corrupted');
        assert.throws(() => verifyCaptureLedger(f.receipt));
    } finally { f.cleanup(); }
});

test('interrupted retirement is explicit and resumes only owned matching bytes', () => {
    const f = fixture();
    try {
        const publication = path.join(f.root, 'publication.json'); fs.writeFileSync(publication, '{}');
        retirePublishedCapture(f.receipt, f.files.image, publication);
        fs.unlinkSync(path.join(path.dirname(f.receipt), 'retired.json'));
        assert.equal(verifyCaptureLedger(f.receipt).status, 'RETIREMENT_INCOMPLETE');
        fs.writeFileSync(f.files.exe, 'another job');
        assert.throws(() => retirePublishedCapture(f.receipt, f.files.image, publication));
        assert.equal(fs.readFileSync(f.files.exe, 'utf8'), 'another job');
        fs.writeFileSync(f.files.exe, 'exe');
        assert.equal(retirePublishedCapture(f.receipt, f.files.image, publication).status, 'PUBLISHED_BINARY_RETIRED');
    } finally { f.cleanup(); }
});

test('explicit affected selection excludes unchanged and unknown IDs', () => {
    assert.deepEqual(selectAffected(['38a-samples']), ['38a-samples']);
    assert.throws(() => selectAffected(['01-canvas']));
    assert.throws(() => selectAffected(['unknown']));
    assert.throws(() => selectAffected(['38a-samples', '38a-samples']));
});

test('materialized JSON source retires while the assertion receipt remains', () => {
    const f = fixture(true);
    try {
        const publication = path.join(f.root, 'publication.json'); fs.writeFileSync(publication, '{}');
        retirePublishedCapture(f.receipt, f.files.image, publication);
        assert(!fs.existsSync(f.sourceJson)); assert(fs.existsSync(f.files.record));
        assert.equal(verifyCaptureLedger(f.receipt).status, 'PUBLISHED_BINARY_RETIRED');
    } finally { f.cleanup(); }
});

test('workbench refuses changed and unowned files', () => {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'capture-workbench-'));
    try {
        checkWorkbenchOwnership(root, {});
        const file = path.join(root, 'source.rs'); fs.writeFileSync(file, 'source');
        assert.throws(() => checkWorkbenchOwnership(root, {}));
        const owned = {'source.rs': fileReference(file).sha256}; checkWorkbenchOwnership(root, owned);
        fs.appendFileSync(file, 'external edit'); assert.throws(() => checkWorkbenchOwnership(root, owned));
    } finally { fs.rmSync(root, {recursive: true}); }
});

test('native compiler artifact is linked to the executable actually observed', () => {
    const f = fixture();
    try {
        const record = JSON.parse(fs.readFileSync(f.files.record, 'utf8'));
        const artifact = path.join(f.owned, 'compiler.json'), compilerReportedPath = path.join(f.root, 'compiler-output.exe');
        fs.writeFileSync(artifact, JSON.stringify({reason: 'compiler-artifact', executable: compilerReportedPath}));
        Object.assign(record, {executable: {...fileReference(f.files.exe), compilerReportedPath}, compilerArtifact: fileReference(artifact),
            compile: {resources: {exitCode: 0}}, execute: {resources: {exitCode: 0}}});
        fs.writeFileSync(f.files.record, JSON.stringify(record));
        const options = {ownedRoots: [f.owned], capture: f.files.record, evidence: [], image: f.files.image, execution: record.executionIntegrity};
        const valid = sealCapture({...options, directory: path.join(f.root, 'proof', 'native')});
        assert.equal(verifyCaptureLedger(valid.path).status, 'CAPTURED_BINARY_AVAILABLE');
        record.executable.compilerReportedPath = 'different-compiler-output.exe'; fs.writeFileSync(f.files.record, JSON.stringify(record));
        assert.throws(() => sealCapture({...options, directory: path.join(f.root, 'proof', 'invalid-native')}));
    } finally { f.cleanup(); }
});

test('browser source, Cargo WASM, bindgen, commands and served bundle form one chain', () => {
    const f = fixture();
    try {
        const dist = path.join(f.owned, 'dist'); fs.mkdirSync(dist);
        const wasm = path.join(dist, 'viewer_journey_bg.wasm'), raw = path.join(f.owned, 'raw.wasm');
        fs.writeFileSync(wasm, 'bound wasm'); fs.writeFileSync(raw, 'raw wasm');
        const execution = finishExecution(observeExecution([wasm, f.files.source]));
        const source = {checkpoint: 'fixture'}, bundle = {'viewer_journey_bg.wasm': fileReference(wasm).sha256};
        const compilerReportedPath = path.join(f.root, 'shared', 'viewer_journey.wasm');
        const bindgen = {executable: 'wasm-bindgen.exe', sha256: 'a'.repeat(64)};
        const commands = [['wasm', ['cargo', 'build']], ['bindgen', [bindgen.executable, compilerReportedPath]]].map(([label, args]) => {
            const resourcesFile = path.join(f.owned, label + '.resources.json'), log = path.join(f.owned, label + '.log'), commandFile = path.join(f.owned, label + '.command.json');
            const resources = {exitCode: 0, timedOut: false}; fs.writeFileSync(resourcesFile, JSON.stringify(resources)); fs.writeFileSync(log, 'actual fixture log');
            fs.writeFileSync(commandFile, JSON.stringify({executable: args[0], arguments: args.slice(1)}));
            return {label, args, resourcesFile, resourcesSha256: fileReference(resourcesFile).sha256, resources, log, logSha256: fileReference(log).sha256, commandFile};
        });
        const group = {workbench: path.join(f.root, 'workbench')};
        const build = {source, bundle, group, bindgen, commands, wasm: {...fileReference(raw), compilerReportedPath},
            compilerArtifact: {manifest_path: path.join(group.workbench, 'Cargo.toml'), filenames: [compilerReportedPath]}};
        const buildFile = path.join(f.owned, 'built.json'); fs.writeFileSync(buildFile, JSON.stringify(build));
        const capture = {id: 'fixture', source, bundle, image: fileReference(f.files.image), host: {executionIntegrity: execution,
            resources: {exitCode: 0}, served: {'fixture/dist/viewer_journey_bg.wasm': bundle['viewer_journey_bg.wasm']}}};
        fs.writeFileSync(f.files.record, JSON.stringify(capture));
        const options = {ownedRoots: [f.owned], capture: f.files.record, build: buildFile, evidence: [], image: f.files.image, execution};
        const valid = sealCapture({...options, directory: path.join(f.root, 'proof', 'browser')});
        assert.equal(verifyCaptureLedger(valid.path).status, 'CAPTURED_BINARY_AVAILABLE');
        for (const [index, change] of [
            (b: any) => { b.bundle['viewer_journey_bg.wasm'] = '0'.repeat(64); },
            (b: any) => { b.compilerArtifact.filenames = ['different.wasm']; },
            (b: any) => { b.commands[0].resources.exitCode = 1; },
        ].entries()) {
            const changed = structuredClone(build); change(changed); fs.writeFileSync(buildFile, JSON.stringify(changed));
            assert.throws(() => sealCapture({...options, directory: path.join(f.root, 'proof', 'invalid-browser-' + index)}));
        }
    } finally { f.cleanup(); }
});

test('an old publication remains provable when a new generation replaces the public image', () => {
    const f = fixture();
    try {
        const destination = path.join(f.root, 'public.png'), publication = path.join(f.root, 'public.json');
        fs.copyFileSync(f.files.image, destination); fs.writeFileSync(publication, '{}');
        retirePublishedCapture(f.receipt, destination, publication);
        fs.writeFileSync(destination, 'new generation'); fs.writeFileSync(publication, 'new publication');
        const result = verifyCaptureLedger(f.receipt);
        assert.equal(result.status, 'PUBLISHED_BINARY_RETIRED'); assert.equal(result.currentPublication, false);
    } finally { f.cleanup(); }
});

test('retirement owns the coordinator lock during artifact removal', () => {
    const f = fixture(), remove = fs.unlinkSync;
    try {
        const lock = path.join(f.root, 'active.lock'), data = JSON.parse(fs.readFileSync(f.receipt, 'utf8'));
        data.locks = [lock]; fs.writeFileSync(f.receipt, JSON.stringify(data));
        const publication = path.join(f.root, 'public.json'); fs.writeFileSync(publication, '{}');
        let checked = false;
        fs.unlinkSync = file => {
            if (file === f.files.exe) { assert.throws(() => fs.openSync(lock, 'wx')); checked = true; }
            return remove(file);
        };
        retirePublishedCapture(f.receipt, f.files.image, publication);
        assert(checked); assert(!fs.existsSync(lock));
    } finally { fs.unlinkSync = remove; f.cleanup(); }
});

test('retirement rejects a redirected owned directory without deleting its contents', () => {
    const f = fixture();
    try {
        const moved = path.join(f.root, 'moved'); fs.renameSync(f.owned, moved);
        fs.symlinkSync(moved, f.owned, process.platform === 'win32' ? 'junction' : 'dir');
        const publication = path.join(f.root, 'public.json'); fs.writeFileSync(publication, '{}');
        assert.throws(() => retirePublishedCapture(f.receipt, f.files.image, publication), /redirected/);
        assert(fs.existsSync(path.join(moved, 'run.exe')));
    } finally { f.cleanup(); }
});
