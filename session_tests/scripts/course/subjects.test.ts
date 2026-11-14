import assert from 'node:assert/strict';
import test from 'node:test';
import fs from 'node:fs';
import path from 'node:path';
import {collapseSubject} from './subjects.ts';
import {course, docs, expected, fingerprint, hash, snippetSource} from './model.ts';

const edit = (path: string, before: string | null, snippet: string) => ({path, before, snippet, why: ''});
const load = (source: string) => source;

test('consolidated browser checks wait for the runtime startup message', () => {
    for (const id of ['12-identity', '17-solid', '19a-viewport']) {
        const step = course().steps.find(step => step.id === id)!;
        assert(expected(id)['src/browser.rs'].includes(`report(${JSON.stringify(step.browser_status)});`),
            `${id}: acceptance status belongs in the validation prose, not the expected runtime message`);
    }
});

test('a subject introduces the final file without exposing its internal rewrites', () => {
    const result = collapseSubject({}, [
        {id: 'frame', edits: [edit('src/frame.rs', null, 'draft\n')]},
        {id: 'gpu', edits: [edit('src/frame.rs', 'draft', 'finished')]},
    ], load);
    assert.deepEqual(result.edits, [{path: 'src/frame.rs', before: null, code: 'finished\n'}]);
});

test('separated edits to one file collapse while other files retain first-use order', () => {
    const result = collapseSubject({'a.rs': 'top\nold\nbottom\n'}, [
        {id: 'one', edits: [edit('a.rs', 'old', 'draft'), edit('b.rs', null, 'b\n')]},
        {id: 'two', edits: [edit('a.rs', 'draft', 'final')]},
    ], load);
    assert.deepEqual(result.edits, [
        {path: 'a.rs', before: 'old\n', code: 'final\n'},
        {path: 'b.rs', before: null, code: 'b\n'},
    ]);
});

test('insertion and repeated context produce an unambiguous replacement', () => {
    for (const [before, after] of [
        ['same\nsame\n', 'same\nnew\nsame\n'],
        ['a\n', 'a\nb\n'], ['a\n', 'b\na\n'], ['a\nb\n', 'a\n'],
    ]) {
        const result = collapseSubject({'a.rs': before}, [{id: 'subject', edits: [edit('a.rs', before, after)]}], load);
        assert.equal(result.files['a.rs'], after);
        assert.equal(before.split(result.edits[0].before!).length, 2);
    }
});

test('a reverted experiment leaves no learner edit', () => {
    const result = collapseSubject({'a.rs': 'original'}, [
        {id: 'try', edits: [edit('a.rs', 'original', 'experiment')]},
        {id: 'restore', edits: [edit('a.rs', 'experiment', 'original')]},
    ], load);
    assert.deepEqual(result.edits, []);
});

test('field-renaming edits collapse to their complete result', () => {
    const result = collapseSubject({'a.rs': 'self.old + self.old\n'}, [{id: 'subject', edits: [
        {...edit('a.rs', 'self.old', 'self.current'), rename_field: true},
    ]}], load);
    assert.equal(result.files['a.rs'], 'self.current + self.current\n');
    assert.deepEqual(result.edits, [{path: 'a.rs', before: 'self.old + self.old\n', code: 'self.current + self.current\n'}]);
});

test('the first GPU subject replaces three checkpoints with five final files', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/first-frame/checkpoints.json'), 'utf8'));
    const steps = archived.steps;
    assert.deepEqual(steps.map(step => step.id), ['01-canvas', '01a-gpu', '02-clear']);
    const result = collapseSubject({}, steps, file => fs.readFileSync(path.join(docs, file), 'utf8'));
    assert.deepEqual(result.files, archived.endpoint);
    assert.deepEqual(result.files, expected('01-canvas'));
    assert(!course().steps.some(step => ['01a-gpu', '02-clear'].includes(step.id)));
    assert.equal(course().steps.length, 1310);
    assert.equal(result.edits.length, 5);
    assert(result.edits.every(edit => edit.before === null));
    assert.deepEqual(new Set(result.edits.map(edit => edit.path)), new Set(Object.keys(result.files)));
});

test('named first-frame listings reconstruct the same canonical source', () => {
    const plan = JSON.parse(fs.readFileSync(path.join(docs, 'journey/first-frame-subject.json'), 'utf8'));
    const files = Object.fromEntries(plan.edits.map((edit: {path: string; snippet: string}) =>
        [edit.path, snippetSource(edit.snippet)]));
    assert.deepEqual(files, expected('02-clear'));
});

test('selection owns final rollback history while preserving the archived ray-picking endpoint', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/selection-navigation/checkpoints.json'), 'utf8'));
    assert.equal(archived.steps.length, 10);
    assert.deepEqual(archived.before, expected('10-depth'));
    const result = collapseSubject(archived.before, archived.steps, snippetSource);
    assert.deepEqual(result.files, archived.endpoint);
    const editor = JSON.parse(fs.readFileSync(path.join(docs, 'archived/kernel-editor/checkpoints.json'), 'utf8'));
    const history = snippetSource('journey/code/12-history-final.rs');
    assert.equal(history, editor.endpoint['src/history.rs']);
    const completed = {...archived.endpoint, 'src/history.rs': history};
    for (const step of archived.steps) assert.deepEqual(expected(step.id), completed, step.id);
    const subject = course().steps.find(step => step.id === '12-identity')!;
    assert.equal(new Set(subject.edits.map(edit => edit.path)).size, subject.edits.length);
    assert.equal(course().steps.filter(step => archived.steps.some((old: {id: string}) => old.id === step.id)).length, 1);
});

test('kernel geometry and editor actions preserve the final endpoint and five historical routes', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/kernel-editor/checkpoints.json'), 'utf8'));
    const preservation = JSON.parse(fs.readFileSync(path.join(docs, 'archived/kernel-editor/preservation.json'), 'utf8'));
    assert.deepEqual(archived.steps.map(step => step.id), ['17-solid', '18-light', '18a-editor', '18b-actions', '19-actions']);
    assert.deepEqual(expected('12-identity'), {...archived.before, 'src/history.rs': archived.endpoint['src/history.rs']});
    const result = collapseSubject(archived.before, archived.steps, snippetSource);
    assert.deepEqual(result.files, archived.endpoint);
    assert.equal(fingerprint(result.files), preservation.endpoint);
    for (const step of archived.steps) assert.deepEqual(expected(step.id), archived.endpoint, step.id);
    const subject = course().steps.find(step => step.id === '17-solid')!;
    assert.equal(subject.edits.length, 6);
    assert.equal(new Set(subject.edits.map(edit => edit.path)).size, 6);
    assert(!subject.edits.some(edit => edit.path === 'src/history.rs'));
    assert.equal(course().steps.filter(step => preservation.originalIds.includes(step.id)).length, 1);
    assert.equal(fingerprint(expected(course().steps.at(-1)!.id)), preservation.destination);
    assert.equal(preservation.pages.length, 6);
    for (const page of preservation.pages) {
        const source = fs.readFileSync(path.join(docs, page.archive), 'utf8').replaceAll('\r\n', '\n');
        assert.equal(hash(source), page.sha256, page.page);
    }
});

test('viewport and captured input preserve the eight-checkpoint endpoint and archived pages', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/viewport-input/checkpoints.json'), 'utf8'));
    const preservation = JSON.parse(fs.readFileSync(path.join(docs, 'archived/viewport-input/preservation.json'), 'utf8'));
    assert.deepEqual(archived.steps.map(step => step.id), ['19a-viewport', '20-resize', '20a-press', '20b-release', '20c-motion', '21-gestures', '21a-wheel', '22-shortcuts']);
    assert.deepEqual(archived.before, expected('17-solid'));
    const result = collapseSubject(archived.before, archived.steps, snippetSource);
    assert.deepEqual(result.files, archived.endpoint);
    assert.equal(fingerprint(result.files), preservation.endpoint);
    for (const step of archived.steps) assert.deepEqual(expected(step.id), archived.endpoint, step.id);
    const subject = course().steps.find(step => step.id === '19a-viewport')!;
    assert.equal(subject.edits.length, 9);
    assert.equal(new Set(subject.edits.map(edit => edit.path)).size, 9);
    assert.equal(course().steps.filter(step => preservation.originalIds.includes(step.id)).length, 1);
    assert.equal(fingerprint(expected(course().steps.at(-1)!.id)), preservation.destination);
    assert.equal(preservation.pages.length, 9);
    for (const page of preservation.pages)
        assert.equal(hash(fs.readFileSync(path.join(docs, page.archive), 'utf8').replaceAll('\r\n', '\n')), page.sha256, page.page);
});

test('shader, font preparation and painting collapse to the exact runnable endpoint', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/painting/checkpoints.json'), 'utf8'));
    const result = collapseSubject(expected('01-canvas'), archived.steps,
        name => fs.readFileSync(path.join(docs, name), 'utf8'));
    assert.deepEqual(result.files, archived.endpoint);
    assert.deepEqual(expected('03-triangle'), archived.endpoint);
    assert.equal(course().steps.filter(step => ['03-triangle', '03a-fonts', '03a-paint'].includes(step.id)).length, 1);
});

test('the dock owns its complete focus adapter before layers are introduced', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/command-dock/checkpoints.json'), 'utf8'));
    const files = {...archived.result.files};
    assert.equal(archived.steps.length, 22);
    assert.equal(archived.focus.length, 3);
    for (const edit of archived.focus) {
        assert.equal(files[edit.path].split(edit.before).length, 2);
        files[edit.path] = files[edit.path].replace(edit.before, fs.readFileSync(path.join(docs, edit.snippet), 'utf8'));
    }
    assert.deepEqual(expected('03d-input'), files);
    assert.equal(course().steps.filter(step => archived.steps.some((old: {id: string}) => old.id === step.id)).length, 1);
    assert(!course().steps.find(step => step.id === '53u-layers')!.edits.some(edit => edit.path.startsWith('src/command_dock/')));
});

test('shared vertices and indices have one runnable endpoint and preserve the original source', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/shared-mesh/checkpoints.json'), 'utf8'));
    const result = collapseSubject(expected('04-input'), archived.steps, snippetSource);
    assert.deepEqual(result.files, archived.endpoint);
    assert.deepEqual(expected('05-vertices'), archived.endpoint);
    assert.deepEqual(expected('06-indices'), archived.endpoint);
    const subject = course().steps.find(step => step.id === '05-vertices')!;
    assert.equal(new Set(subject.edits.map(edit => edit.path)).size, subject.edits.length);
    assert(!course().steps.some(step => step.id === '06-indices'));
});


test('camera state and matrix upload have one endpoint without an intermediate scalar uniform', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/camera-view/checkpoints.json'), 'utf8'));
    const result = collapseSubject(expected('05-vertices'), archived.steps, snippetSource);
    assert.deepEqual(result.files, archived.endpoint);
    for (const id of ['07-uniforms', '08-camera', '09-matrices']) assert.deepEqual(expected(id), archived.endpoint);
    const subject = course().steps.find(step => step.id === '07-uniforms')!;
    assert.equal(new Set(subject.edits.map(edit => edit.path)).size, subject.edits.length);
    assert(!course().steps.some(step => ['08-camera', '09-matrices'].includes(step.id)));
});


test('depth, mesh validation and stable scene identity share one exact executable endpoint', () => {
    const archived = JSON.parse(fs.readFileSync(path.join(docs, 'archived/depth-scene/checkpoints.json'), 'utf8'));
    const result = collapseSubject(expected('07-uniforms'), archived.steps, snippetSource);
    assert.deepEqual(result.files, archived.endpoint);
    for (const id of ['10-depth', '10a-mesh', '11-scene', '11a-identity']) assert.deepEqual(expected(id), archived.endpoint);
    const subject = course().steps.find(step => step.id === '10-depth')!;
    assert.equal(new Set(subject.edits.map(edit => edit.path)).size, subject.edits.length);
    assert(!course().steps.some(step => ['10a-mesh', '11-scene', '11a-identity'].includes(step.id)));
});
