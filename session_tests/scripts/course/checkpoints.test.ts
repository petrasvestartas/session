import {changedBlocks, lineContext} from './line-diff.ts';
import {instructions, replaceImplementation} from './render.ts';
import {courseBook, readingBook} from '../../src/courseBook.ts';
import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {createRequire} from 'node:module';
import {save, restore, savedPath} from './checkpoints.ts';
import {dependencyInputs} from './dependency-inputs.ts';
import {removeGenerated} from './verify.ts';
import {expected, edited, typingLoad, fingerprint, withExpectedSequence, compare, safeOutput, hash, referenceFiles, reference, dependencies, lockFor, read, materialize, viewer, assetsFor, course, addedLines} from './model.ts';
const browserFingerprint = createRequire(import.meta.url)(path.join(viewer, 'docs/journey/checks/fingerprint.cjs'));

test('generated cleanup rejects traversal and links outside its owned root', () => {
    const root = fs.mkdtempSync(path.join(viewer, 'target/course-cleanup-test-'));
    const outside = fs.mkdtempSync(path.join(os.tmpdir(), 'viewer-course-cleanup-outside-'));
    const link = path.join(root, 'linked');
    try {
        const child = path.join(root, 'checkpoint');
        fs.mkdirSync(child);
        fs.writeFileSync(path.join(child, 'owned'), 'temporary');
        removeGenerated(child, root);
        assert(!fs.existsSync(child));
        assert.throws(() => removeGenerated(root, root), /inside its generated root/);
        assert.throws(() => removeGenerated(path.join(root, '../other'), root), /inside its generated root/);
        fs.writeFileSync(path.join(outside, 'keep'), 'preserved');
        fs.symlinkSync(outside, link, process.platform === 'win32' ? 'junction' : 'dir');
        assert.throws(() => removeGenerated(link, root), /link outside/);
        assert.throws(() => removeGenerated(path.join(link, 'missing'), root), /link outside/);
        assert.equal(fs.readFileSync(path.join(outside, 'keep'), 'utf8'), 'preserved');
    } finally {
        if (fs.existsSync(link)) fs.unlinkSync(link);
        assert.equal(path.dirname(fs.realpathSync(root)), fs.realpathSync(path.join(viewer, 'target')));
        assert.equal(path.dirname(fs.realpathSync(outside)), fs.realpathSync(os.tmpdir()));
        fs.rmSync(root, {recursive: true, force: true});
        fs.rmSync(outside, {recursive: true, force: true});
    }
});

function fixture(run: (folder: string) => void) {
    const folder = fs.mkdtempSync(path.join(os.tmpdir(), 'viewer-course-'));
    try { run(folder); }
    finally { fs.rmSync(folder, {recursive: true, force: true}); }
}

test('lesson code context shows complete lines and preserves the exact replacement', () => {
    for (const [file, before, after] of [
        ['pub fn make(count:bool)->u32 {\n count as u32\n}\n', 'ount:bool)->u', 'ount:bool,samples:u32)->u'],
        ['first\nsecond\n', 'first\n', ''],
        ['one\nlast', 'ast', 'ater\nnext'],
        ['one\ntwo\nthree\n', 'ne\ntw', 'ther\nchanged'],
    ]) {
        const context = lineContext(file, before, after);
        assert.equal(file.replace(context.before, () => context.after), file.replace(before, () => after));
        const at = file.indexOf(context.before);
        assert(at === 0 || file[at - 1] === '\n');
        assert(at + context.before.length === file.length || context.before.endsWith('\n'));
    }
    assert.equal(lineContext('pub fn make(count:bool)->u32 {\n}', 'ount:bool', 'ount:bool,samples:u32').after,
        'pub fn make(count:bool,samples:u32)->u32 {\n');
    assert.throws(() => lineContext('same same', 'same', 'changed'), /one exact replacement/);
});

test('adjacent lesson edits share one instruction while supplied validation stays separate', () => {
    const files: Record<string, string> = {'src/main.rs': 'let x=1;\nlet y=1;\n'};
    const edits = [
        {path: 'src/main.rs', before: 'x=1', snippet: 'x', why: 'Change x.'},
        {path: 'src/main.rs', before: 'y=1', snippet: 'y', why: 'Change y.'},
        {path: 'src/main.rs', before: 'let y=3;\n', snippet: 'fixture', why: 'Check the result.', copy: true},
        {path: 'src/main.rs', before: 'x=2', snippet: 'last', why: 'Finish x.'},
    ];
    const code: Record<string, string> = {x: 'x=2', y: 'y=3', fixture: 'let y=3;\n// supplied check\n', last: 'x=4'};
    const grouped = instructions(edits, files, edit => code[edit.snippet]);
    assert.equal(grouped.length, 3);
    assert.equal(grouped[0].code, 'let x=2;\nlet y=3;\n');
    assert.equal(grouped[0].inline, true);
    assert.equal(grouped[1].edit.copy, true);
    assert.deepEqual(grouped.flatMap(item => item.references), edits);
    const result = {...files};
    for (const {edit, code} of grouped) result[edit.path] = edited(result[edit.path], edit, code);
    assert.equal(result['src/main.rs'], 'let x=4;\nlet y=3;\n// supplied check\n');
    assert.equal(files['src/main.rs'], 'let x=1;\nlet y=1;\n');
});

test('scoped checkpoint replay preserves snapshots, backward reads and nested scopes', () => {
    const first = expected('01-canvas'), second = expected('02-clear');
    withExpectedSequence(() => {
        const snapshot = expected('01-canvas');
        assert.deepEqual(expected('02-clear'), second);
        assert.deepEqual(snapshot, first);
        snapshot['src/lib.rs'] = 'external edit';
        assert.deepEqual(expected('01-canvas'), first);
        assert.throws(() => withExpectedSequence(() => {
            expected('02-clear');
            throw Error('interrupted');
        }), /interrupted/);
        assert.deepEqual(expected('02-clear'), second);
        assert.throws(() => expected('missing-checkpoint'), /Unknown lesson/);
    });
    assert.deepEqual(expected('01-canvas'), first);
});

test('live viewer and Wood code, shaders and assets invalidate dependent evidence', () => fixture(folder => {
    const root = path.join(folder, 'session/viewer');
    for (const directory of [root, path.join(folder, 'wood/rust')]) {
        fs.mkdirSync(path.join(directory, 'src'), {recursive: true});
        fs.writeFileSync(path.join(directory, 'Cargo.toml'), '[package]\n');
        fs.writeFileSync(path.join(directory, 'src/lib.rs'), 'original');
    }
    const manifest = 'current_viewer = { path = "../.." }';
    const baseline = dependencyInputs(manifest, root);
    fs.writeFileSync(path.join(folder, 'wood/rust/src/lib.rs'), 'changed');
    const wood = dependencyInputs(manifest, root);
    assert.notDeepEqual(wood, baseline);
    fs.writeFileSync(path.join(root, 'src/ink.wgsl'), 'shader');
    const shader = dependencyInputs(manifest, root);
    assert.notDeepEqual(shader, wood);
    fs.mkdirSync(path.join(root, 'assets'));
    fs.writeFileSync(path.join(root, 'assets/font.ttf'), 'font');
    assert.notDeepEqual(dependencyInputs(manifest, root), shader);
    assert.deepEqual(dependencyInputs('[dependencies]\n', root), {});
}));

test('browser evidence includes shared local checks and handles cyclic imports', () => fixture(folder => {
    const entry = path.join(folder, 'entry.cjs'), child = path.join(folder, 'child.cjs');
    fs.writeFileSync(entry, "require('./child.cjs');\n");
    fs.writeFileSync(child, "require('./entry.cjs');\nmodule.exports = 1;\n");
    const before = browserFingerprint('journey/checks/browser/entry.cjs', folder);
    fs.writeFileSync(child, "require('./entry.cjs');\nmodule.exports = 2;\n");
    const after = browserFingerprint('journey/checks/browser/entry.cjs', folder);
    assert.notEqual(after, before, 'Changing a shared checker invalidates its caller');
    fs.writeFileSync(path.join(folder, 'unused.cjs'), 'irrelevant');
    assert.equal(browserFingerprint('journey/checks/browser/entry.cjs', folder), after);
    fs.writeFileSync(child, "require('../outside.cjs');\n");
    assert.throws(() => browserFingerprint('journey/checks/browser/entry.cjs', folder), /inside its check folder/);
}));

test('typing estimates exclude unchanged context and removed lines', () => {
    assert.deepEqual(addedLines('before\nold\nafter\n', 'before\nnew\nafter\n'), ['new\n']);
    assert.deepEqual(addedLines('before\nold\nafter\n', 'before\nafter\n'), []);
    assert.deepEqual(addedLines('', '// Explain ownership\n\nlet value = 1;\n'), ['// Explain ownership\n', '\n', 'let value = 1;\n']);
});

test('restore preserves the unfinished project and does not overwrite a save', () => fixture(folder => {
    const project = path.join(folder, 'journey');
    fs.mkdirSync(project);
    fs.writeFileSync(path.join(project, 'main.rs'), 'working');
    fs.mkdirSync(path.join(project, 'target'));
    fs.writeFileSync(path.join(project, 'target/build'), 'compiled output');
    const checkpoint = save('working', project);
    assert(!fs.existsSync(path.join(checkpoint, 'target')));
    assert.throws(() => save('working', project), /Refusing to overwrite/);
    fs.writeFileSync(path.join(project, 'main.rs'), 'experiment');
    const backup = restore('working', project)!;
    assert.equal(fs.readFileSync(path.join(project, 'main.rs'), 'utf8'), 'working');
    assert.equal(fs.readFileSync(path.join(backup, 'main.rs'), 'utf8'), 'experiment');
    assert(fs.existsSync(path.join(backup, 'target/build')));
    save('22.2.3', project);
    const numbered = path.join(folder, 'numbered');
    restore('22.2.3', numbered);
    assert.equal(fs.readFileSync(path.join(numbered, 'main.rs'), 'utf8'), 'working');
}));

test('unsafe names and references inside the project are rejected, including symlinks', () => fixture(folder => {
    const project = path.join(folder, 'journey');
    fs.mkdirSync(project);
    fs.symlinkSync(project, path.join(folder, 'alias'), 'dir');
    for (const name of ['', '../outside', '/tmp/outside', '..', '1..3', '22.2.3/../outside']) assert.throws(() => savedPath(name, project));
    for (const target of [project, path.join(project, 'answer'), path.join(folder, 'alias/answer')]) {
        assert.throws(() => safeOutput(target, project), /outside/);
    }
    safeOutput(path.join(folder, 'reference'), project);
}));

test('read-only comparison identifies a typo and leaves the project untouched', () => fixture(folder => {
    const files = expected('01-canvas');
    for (const [name, code] of Object.entries(files)) {
        const file = path.join(folder, name);
        fs.mkdirSync(path.dirname(file), {recursive: true});
        fs.writeFileSync(file, code);
    }
    assert.deepEqual(compare(files, folder), []);
    const source = path.join(folder, 'src/lib.rs');
    fs.appendFileSync(source, 'my experiment\n');
    const before = fs.readFileSync(source);
    assert(compare(files, folder)[0].includes('my experiment'));
    assert.deepEqual(fs.readFileSync(source), before);
}));

test('the full reference is reconstructed without Python and matches every final file hash', () => {
    const files = referenceFiles('37');
    for (const [name, digest] of Object.entries(reference().files)) assert.equal(hash(files[name]), digest, name);
    assert.equal(Object.keys(files).length, Object.keys(reference().files).length);
});

test('a dependency update preserves the previous lock and never writes application code', () => fixture(folder => {
    fs.writeFileSync(path.join(folder, 'Cargo.toml'), expected('15-perspective')['Cargo.toml']);
    fs.writeFileSync(path.join(folder, 'Cargo.lock'), read(lockFor('01-canvas')));
    fs.writeFileSync(path.join(folder, 'my-work.rs'), 'my unfinished work');
    dependencies('15-perspective', folder);
    assert.equal(read(path.join(folder, 'Cargo.lock')), read(lockFor('15-perspective')));
    assert.equal(read(path.join(folder, 'Cargo.lock.before-15-perspective')), read(lockFor('01-canvas')));
    dependencies('15-perspective', folder);
    assert.equal(read(path.join(folder, 'my-work.rs')), 'my unfinished work');
    for (const name of assetsFor('15-perspective')) {
        assert(fs.readFileSync(path.join(folder, name)).equals(fs.readFileSync(path.join(viewer, name))));
    }
    const changedFont = path.join(folder, assetsFor('15-perspective')[0]);
    fs.writeFileSync(changedFont, 'my changed font');
    assert.throws(() => dependencies('15-perspective', folder), /Asset differs/);
    assert.equal(read(changedFont), 'my changed font');
    fs.appendFileSync(path.join(folder, 'Cargo.toml'), '# my own dependency experiment\n');
    assert.throws(() => dependencies('15-perspective', folder), /Cargo.toml/);
}));

test('a reference exported at another depth still finds the geometry kernel', () => fixture(folder => {
    const target = path.join(folder, 'reference');
    materialize(expected('15-perspective'), target, lockFor('15-perspective'), assetsFor('15-perspective'));
    const relative = read(path.join(target, 'Cargo.toml')).match(/session_rust = \{ path = "([^"]+)"/)![1];
    assert.equal(path.resolve(target, relative), path.resolve(viewer, '../session_rust'));
}));

test('interactive lessons retain the shared dock and its documented layer-focus adapter', () => withExpectedSequence(() => {
    const steps = course().steps;
    const focusAt = steps.findIndex(step => step.id === '53u-layers');
    const focusEdits = steps[focusAt].edits.filter(edit => edit.snippet.includes('focus-adapter'));
    assert.equal(focusEdits.length, 3);
    for (const step of steps.filter(step => parseInt(step.id) >= 4)) {
        const files = expected(step.id);
        assert(!/<button|<form|id="controls"/.test(files['index.html']), step.id);
        assert((step.browser_actions || []).every(action => !('button' in action)), step.id);
        for (const name of ['mod.rs', 'theme.rs', 'view.rs']) {
            let shared = read(path.join(viewer, 'src/command_dock', name));
            if (name === 'mod.rs' && steps.indexOf(step) >= focusAt) {
                for (const edit of focusEdits) {
                    assert.equal(shared.split(edit.before!).length, 2, 'Focus adapter must match the maintained dock exactly once');
                    shared = shared.replace(edit.before!, () => read(path.join(viewer, 'docs', edit.snippet)));
                }
            }
            assert.equal(files['src/command_dock/' + name], shared, step.id + ': shared dock ' + name);
        }
    }
}));


test('compact teaching blocks reconstruct unique edits with repeated context and deletions', () => {
    const numbered = Array.from({length: 100}, (_, i) => `line ${i}\n`).join('');
    const repeated = 'opening\n' + 'same\n'.repeat(70) + 'closing\n';
    for (const [before, after] of [
        [numbered, numbered.replace('line 5\n', 'new 5\n').replace('line 90\n', '')],
        [numbered, numbered.replace('line 20\n', 'inserted\nline 20\n')],
        [repeated, repeated.replace('opening\nsame\n', 'opening\nchanged\n')],
        ['a\nb\nc', 'a\nx\nc'], ['a\nb\nc\n', 'a\nc\n'], ['same\n', 'same\n'],
    ]) {
        let file = `outer prefix\n${before}outer suffix\n`;
        const wanted = file.replace(before, () => after);
        for (const block of changedBlocks(before, after, file)) {
            assert.equal(file.split(block.before).length, 2);
            file = file.replace(block.before, () => block.after);
        }
        assert.equal(file, wanted);
    }
});

test('compact teaching blocks preserve every canonical edit', () => {
    const files: Record<string, string> = {};
    for (const step of course().steps) for (const edit of step.edits) {
        const before = edit.before || '', code = read(path.join(viewer, 'docs', edit.snippet));
        const original = files[edit.path] || '';
        const wanted = edited(files[edit.path], edit, code);
        if (edit.before && !edit.copy && before.split('\n').length >= 60 && !code.startsWith(before)) {
            let actual = original;
            for (const block of changedBlocks(before, code, original)) {
                assert.equal(actual.split(block.before).length, 2, step.id + ':' + edit.path);
                actual = actual.replace(block.before, () => block.after);
            }
            assert.equal(actual, wanted, step.id + ':' + edit.path);
        }
        files[edit.path] = wanted;
    }
});

test('book chapters preserve the complete prerequisite order and stable lesson routes', () => {
    const data = course();
    const plan = JSON.parse(read(path.join(viewer, 'docs/journey/book.json')));
    const book = courseBook(data.steps, plan);
    const lessons = book.chapters.flatMap(chapter => chapter.sections.flatMap(section => section.lessons));
    assert.deepEqual(lessons.map(lesson => lesson.id), data.steps.filter(step => step.id !== plan.comparison).map(step => step.id));
    assert.equal(book.byId.size, lessons.length);
    assert.equal(new Set(lessons.map(lesson => lesson.number)).size, lessons.length);
    assert(lessons.every(lesson => /^\d+\.\d+\.\d+$/.test(lesson.number) && lesson.page === `journey/${lesson.id}.md`));
    assert.equal(lessons[0].number, '1.1.1');
    assert.equal(book.byId.get('96zzzzzzc-draw-the-construction-grid-as-physical-lines')?.number, '21.5.3');
    assert.equal(book.comparison?.id, plan.comparison);
});

test('book sections reject missing anchors and prerequisite reversals', () => {
    const data = course();
    const plan = JSON.parse(read(path.join(viewer, 'docs/journey/book.json')));
    const missing = structuredClone(plan); missing.chapters[0].sections[0].first = 'missing-lesson';
    assert.throws(() => courseBook(data.steps, missing), /course order/);
    const reversed = structuredClone(plan); reversed.chapters[0].sections.reverse();
    assert.throws(() => courseBook(data.steps, reversed), /course order/);
});

test('consolidation retains source order, endpoints, aliases and legacy numbering', () => {
    const data = course();
    const plan = JSON.parse(read(path.join(viewer, 'docs/journey/book.json')));
    const original = courseBook(data.steps, plan);
    const reading = readingBook(data.steps, plan);
    assert.equal(reading.chapters.length, 22);
    const pilot = reading.chapters[0].sections.flatMap(section => section.lessons);
    assert.deepEqual(pilot.map(lesson => lesson.number), ['1.1', '1.2', '1.3', '1.4']);
    const consolidated = plan.chapters.flatMap((chapter: {consolidated?: {steps: string[]}[]}) => chapter.consolidated || []);
    assert.equal(reading.aliases.size, consolidated.reduce((count: number, lesson: {steps: string[]}) => count + lesson.steps.length, 0));
    assert.equal(reading.aliases.get('01a-gpu')?.page, 'journey/consolidation/01-canvas.md');
    assert.equal(reading.aliases.get('03d-input')?.anchor, 'checkpoint-03d-input');
    assert.equal(original.byId.get('01a-gpu')?.number, '1.1.2');
    for (const lesson of consolidated) {
        const page = read(path.join(viewer, 'docs', lesson.page));
        for (const id of lesson.steps) assert(page.includes(`id="checkpoint-${id}"`));
        assert.equal(lesson.checkpoint, lesson.steps.at(-1));
    }
    const malformed = structuredClone(plan);
    malformed.chapters[0].consolidated[0].steps.pop();
    assert.throws(() => readingBook(data.steps, malformed), /retain every checkpoint/);
    const wrongEndpoint = structuredClone(plan);
    wrongEndpoint.chapters[0].consolidated[0].checkpoint = '01-canvas';
    assert.throws(() => readingBook(data.steps, wrongEndpoint), /runnable checkpoint/);
});


test('implementation refresh preserves prose and ignores closing tags inside code fences', () => {
    const page = '# Existing lesson\n\n<a id="checkpoint-first"></a>\n\n<details>\n<summary>Old implementation</summary>\n\n```rust\nlet text = "</details>";\n```\n\n<details>\n<summary>Nested</summary>\nOld body\n</details>\n\n</details>\n\n## Result\nKeep my explanation.\n';
    const result = replaceImplementation(page, 'first', 'Current implementation', '```rust\nlet current = 1;\n```');
    assert(result.startsWith('# Existing lesson\n\n<a id="checkpoint-first"></a>'));
    assert(result.includes('Implementation first: Current implementation'));
    assert(result.includes('let current = 1;'));
    assert(!result.includes('Old body'));
    assert(result.endsWith('## Result\nKeep my explanation.\n'));
    assert.throws(() => replaceImplementation(page, 'missing', 'Missing', ''), /Missing implementation/);
    assert.throws(() => replaceImplementation('<a id="checkpoint-first"></a>\n<details>\nUnclosed', 'first', 'First', ''), /Unclosed implementation/);
});

test('field renames replace whole references without changing longer names', () => {
    const edit = {path: 'src/renderer.rs', snippet: 'unused', before: 'self.depth', rename_field: true};
    const source = 'self.depth + self.depth.clone() + self.depth_pyramid() + other_self.depth';
    assert.equal(edited(source, edit, 'self.targets.depth'),
        'self.targets.depth + self.targets.depth.clone() + self.depth_pyramid() + other_self.depth');
    assert.throws(() => edited(source, edit, 'self.targets.depth; injected()'), /two field references/);
    assert.throws(() => edited(source, {...edit, copy: true}, 'self.targets.depth'), /must be typed/);
    assert.throws(() => edited('self.depth_pyramid()', edit, 'self.targets.depth'), /not found/);
    assert.throws(() => edited(undefined, {...edit, before: null}, 'self.targets.depth'), /two field references/);
    assert.throws(() => edited(source, edit, 'self.targets.depth\n'), /two field references/);
});

test('the compact target-owner lesson preserves its accepted source apart from scoped Escape and inventory edits', () => {
    const id = '96zzzzzzv-reuse-coherent-frame-targets';
    const record = JSON.parse(read(path.join(viewer, 'docs/journey/visual-review.json'))).steps[id];
    const step = course().steps.find(step => step.id === id)!;
    const fields = step.edits.filter(edit => edit.rename_field);
    assert.equal(fields.length, 2);
    const files = expected(id);
    assert.equal(fingerprint(files), '342f6fe2efc602f6af619374424d0bbf704ee25742d96ac86d5466f3648a2041');
    const escape = course().steps.find(step => step.id === '85as-register-current-names-views-and-command-aliases')!
        .edits.filter(edit => ['src/panel.rs', 'src/browser.rs'].includes(edit.path));
    assert.equal(escape.length, 2);
    for (const edit of escape) {
        const after = read(path.join(viewer, 'docs', edit.snippet));
        files[edit.path] = edited(files[edit.path], {...edit, before: after}, edit.before!);
    }
    const inventory = course().steps.find(step => step.id === '96h-prove-current-source-grammar-and-explicit-differences')!;
    const audit = inventory.edits.filter(edit => edit.path === 'src/source_audit.rs');
    assert.equal(audit.length, 1);
    const after = read(path.join(viewer, 'docs', audit[0].snippet));
    files[audit[0].path] = edited(files[audit[0].path], {...audit[0], before: after}, audit[0].before!);
    const snapshot = inventory.edits.find(edit => edit.path === 'src/current_inventory.json')!;
    const count = (text: string) => JSON.parse(text).files.filter((row: {root: string}) => row.root === 'viewer').length;
    const previousCount = count(snapshot.before!);
    const currentCount = count(read(path.join(viewer, 'docs', snapshot.snippet)));
    assert.deepEqual([previousCount, currentCount], [306, 312]);
    const testPath = 'src/inventory_tests.rs';
    const assertion = (value: number) => `assert_eq!(snapshot.files.iter().filter(|v|v.root=="viewer").count(),${value});`;
    files[testPath] = edited(files[testPath], {path: testPath, snippet: '', why: '', before: assertion(currentCount)}, assertion(previousCount));
    assert.equal(fingerprint(files), record.applicationFingerprint);
    const counted = fields.reduce((sum, edit) => sum + edit.before!.length + read(path.join(viewer, 'docs', edit.snippet)).length, 0);
    assert(counted > 0 && counted < 100);
    assert(typingLoad(step).minutes[1] <= 60);
});
