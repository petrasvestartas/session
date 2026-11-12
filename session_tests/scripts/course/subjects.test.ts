import assert from 'node:assert/strict';
import test from 'node:test';
import fs from 'node:fs';
import path from 'node:path';
import {collapseSubject} from './subjects.ts';
import {course, docs, expected, snippetSource} from './model.ts';

const edit = (path: string, before: string | null, snippet: string) => ({path, before, snippet, why: ''});
const load = (source: string) => source;

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
    const steps = course().steps.slice(0, 3);
    assert.deepEqual(steps.map(step => step.id), ['01-canvas', '01a-gpu', '02-clear']);
    const result = collapseSubject({}, steps, file => fs.readFileSync(path.join(docs, file), 'utf8'));
    assert.deepEqual(result.files, expected('02-clear'));
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
