import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {fileReference, observeExecution, finishExecution} from './capture-storage.ts';
import {sealCapture, verifyCaptureLedger, retirePublishedCapture} from './capture-ledger.ts';
import {capturePolicy, compiledDrawPolicy, verifyCompiledArtifact, verifyExecutionResources} from './incremental-native-presentation.ts';

test('optional compile limit requires the image lane and a matching actual guard receipt', () => {
    const names = ['COURSE_CAPTURE_PROCESSORS', 'COURSE_CAPTURE_IMAGE_LANE', 'COURSE_NATIVE_COMPILE_MEMORY_MIB'];
    const previous = names.map(name => process.env[name]);
    try {
        process.env.COURSE_CAPTURE_PROCESSORS = '2';
        process.env.COURSE_CAPTURE_IMAGE_LANE = '1';
        delete process.env.COURSE_NATIVE_COMPILE_MEMORY_MIB;
        const original = capturePolicy();
        assert.equal(original.memoryMiB, 6144);
        process.env.COURSE_NATIVE_COMPILE_MEMORY_MIB = '4096';
        const policy = capturePolicy();
        assert.equal(policy.memoryMiB, 4096);
        const resources = {exitCode: 0, timedOut: false, processors: 2, jobCommitLimitBytes: 4096 * 1048576,
            laneMutex: 'Local\\SessionViewerCourseImageJob', minimumAvailableMemoryMiB: 5120, availableMemoryMiBBeforeLaunch: 6000};
        verifyExecutionResources(resources, policy);
        for (const patch of [{jobCommitLimitBytes: 6144 * 1048576}, {minimumAvailableMemoryMiB: 3072}, {availableMemoryMiBBeforeLaunch: 5119}])
            assert.throws(() => verifyExecutionResources({...resources, ...patch}, policy));
        assert.equal(compiledDrawPolicy().memoryMiB, 2048);
        for (const value of ['', '2048', '6144', '8192', '4096x']) {
            process.env.COURSE_NATIVE_COMPILE_MEMORY_MIB = value;
            assert.throws(() => capturePolicy());
        }
        process.env.COURSE_NATIVE_COMPILE_MEMORY_MIB = '4096';
        process.env.COURSE_CAPTURE_IMAGE_LANE = '0';
        assert.throws(() => capturePolicy());
        process.env.COURSE_CAPTURE_IMAGE_LANE = '1';
        process.env.COURSE_CAPTURE_PROCESSORS = '8';
        assert.throws(() => capturePolicy());
    } finally {
        names.forEach((name, i) => { if (previous[i] === undefined) delete process.env[name]; else process.env[name] = previous[i]; });
    }
});

function fixture() {
    const folder = fs.mkdtempSync(path.join(os.tmpdir(), 'compiled-native-'));
    const write = (name: string, value: any) => {
        const file = path.join(folder, name); fs.mkdirSync(path.dirname(file), {recursive: true});
        fs.writeFileSync(file, typeof value === 'string' ? value : JSON.stringify(value)); return file;
    };
    const binary = write('bin/presentation.exe', 'exact compiled artifact'), reported = write('shared/presentation.exe', 'exact compiled artifact');
    const source = write('runtime/examples/presentation.rs', 'original assertions');
    const artifact = {reason: 'compiler-artifact', target: {name: 'presentation', kind: ['example'], src_path: path.join(folder, 'examples/presentation.rs')},
        manifest_path: path.join(folder, 'Cargo.toml'), executable: reported, filenames: [reported]};
    const args = ['cargo', 'build', '--example', 'presentation', '--locked', '--offline', '--target', 'test-host', '-j1', '--message-format=json-render-diagnostics'];
    const resources = {exitCode: 0, timedOut: false, processors: 2, jobCommitLimitBytes: 6144 * 1048576,
        laneMutex: 'Local\\SessionViewerCourseImageJob', arguments: args.slice(1), workingDirectory: folder};
    const artifactFile = write('compiler-artifact.json', artifact);
    const log = write('attempt-build.log', JSON.stringify(artifact) + '\n' + JSON.stringify({reason: 'build-finished', success: true}));
    write('attempt-build.command.json', {executable: 'cargo', arguments: args.slice(1)}); write('attempt-build.resources.json', resources);
    write('attempt-execute.command.json', {executable: binary, arguments: []});
    write('attempt-execute.log', 'Capture admission requires 7168 MiB available; found 7105 MiB.');
    const record: any = {status: 'FAILED_PRESERVED', folder, workbench: folder,
        profile: {host: 'test-host', capture: {processors: 2, jobs: 1, memoryMiB: 6144, imageLane: true}},
        executable: {...fileReference(binary), compilerReportedPath: reported}, compilerArtifact: fileReference(artifactFile),
        compile: {command: {returncode: 0, args}, log, logSha256: fileReference(log).sha256, resources},
        inputs: {'examples/presentation.rs': {sha256: fileReference(source).sha256}}};
    return {folder, record, write, cleanup: () => fs.rmSync(folder, {recursive: true})};
}

test('admission-refused draw retains an exact successful compiler chain', () => {
    const f = fixture();
    try { assert.equal(verifyCompiledArtifact(f.record).length, 5); }
    finally { f.cleanup(); }
});

test('resume rejects a failed compilation, changed source, replaced artifact, or previous draw', () => {
    const mutations = [
        (f: ReturnType<typeof fixture>) => { f.record.status = 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'; },
        (f: ReturnType<typeof fixture>) => { f.record.compile.command.returncode = 1; },
        (f: ReturnType<typeof fixture>) => { f.write('runtime/examples/presentation.rs', 'different assertions'); },
        (f: ReturnType<typeof fixture>) => { f.write('bin/presentation.exe', 'different binary'); },
        (f: ReturnType<typeof fixture>) => { f.write('shared/presentation.exe', 'different compiler artifact'); },
        (f: ReturnType<typeof fixture>) => { f.record.image = {path: 'prior.png'}; },
        (f: ReturnType<typeof fixture>) => { f.write('runtime/frame-adapter.json', {}); },
        (f: ReturnType<typeof fixture>) => { f.write('attempt-execute.resources.json', {exitCode: 0}); },
        (f: ReturnType<typeof fixture>) => { f.write('attempt-execute.log', 'actual renderer failed'); },
        (f: ReturnType<typeof fixture>) => { f.write('attempt-build.resources.json', {...f.record.compile.resources, exitCode: 1}); },
    ];
    for (const mutate of mutations) {
        const f = fixture();
        try { mutate(f); assert.throws(() => verifyCompiledArtifact(f.record)); }
        finally { f.cleanup(); }
    }
});

test('compiler artifact must be the single presentation output from a successful Cargo log', () => {
    for (const replacement of ['failed-finish', 'different-target', 'wrong-manifest', 'duplicate-artifact']) {
        const f = fixture();
        try {
            const artifact = JSON.parse(fs.readFileSync(f.record.compilerArtifact.path, 'utf8'));
            if (replacement === 'different-target') artifact.target.name = 'another-example';
            if (replacement === 'wrong-manifest') artifact.manifest_path = path.join(f.folder, 'unrelated/Cargo.toml');
            f.write('compiler-artifact.json', artifact); f.record.compilerArtifact = fileReference(f.record.compilerArtifact.path);
            const messages = [artifact, ...(replacement === 'duplicate-artifact' ? [artifact] : []), {reason: 'build-finished', success: replacement !== 'failed-finish'}];
            f.write('attempt-build.log', messages.map(message => JSON.stringify(message)).join('\n'));
            f.record.compile.logSha256 = fileReference(f.record.compile.log).sha256;
            assert.throws(() => verifyCompiledArtifact(f.record));
        } finally { f.cleanup(); }
    }
});

test('draw declaration requires actual 2 CPU, 2 GiB, separate image lane and 3 GiB admission', () => {
    const policy = compiledDrawPolicy();
    assert.equal(policy.processors, 2); assert.equal(policy.memoryMiB, 2048); assert.equal(policy.minimumAvailableMemoryMiB, 3072);
    assert.throws(() => compiledDrawPolicy(6144));
    const resources = {exitCode: 0, timedOut: false, processors: 2, jobCommitLimitBytes: 2048 * 1048576,
        laneMutex: 'Local\\SessionViewerCourseImageJob', minimumAvailableMemoryMiB: 3072, availableMemoryMiBBeforeLaunch: 4000};
    verifyExecutionResources(resources, policy);
    for (const patch of [{processors: 8}, {jobCommitLimitBytes: 6144 * 1048576}, {minimumAvailableMemoryMiB: 7168},
        {availableMemoryMiBBeforeLaunch: 3071}, {laneMutex: 'Local\\SessionViewerCourseHeavyJob'}, {timedOut: true}])
        assert.throws(() => verifyExecutionResources({...resources, ...patch}, policy));
});

test('successor uses the original binary through normal sealing and retirement without changing failed proof', () => {
    const f = fixture();
    try {
        const failed = f.write('result.json', f.record), failedReference = fileReference(failed);
        const successor = path.join(f.folder, 'successor'); fs.mkdirSync(successor);
        const image = f.write('successor/frame.png', 'rendered pixels');
        const execution = finishExecution(observeExecution([f.record.executable.path, failed]));
        const result = f.write('successor/result.json', {...f.record, status: 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW',
            compiledOrigin: failedReference, execute: {resources: {exitCode: 0}}, image: fileReference(image), executionIntegrity: execution});
        const receipt = sealCapture({directory: path.join(f.folder, 'ledger'), ownedRoots: [successor, path.join(f.folder, 'bin')],
            capture: result, evidence: [failed, f.record.compilerArtifact.path], image, execution});
        assert.equal(verifyCaptureLedger(receipt.path).status, 'CAPTURED_BINARY_AVAILABLE');
        const published = f.write('published.png', 'rendered pixels'), publication = f.write('publication.json', {review: 'accepted'});
        assert.equal(retirePublishedCapture(receipt.path, published, publication).status, 'PUBLISHED_BINARY_RETIRED');
        assert(!fs.existsSync(f.record.executable.path)); assert.deepEqual(fileReference(failed), failedReference);
        assert.equal(verifyCaptureLedger(receipt.path).status, 'PUBLISHED_BINARY_RETIRED');
    } finally { f.cleanup(); }
});
