import assert from 'node:assert/strict';
import test from 'node:test';
import {auditOwnership} from './ownership.ts';

const edit = (path: string, before: string | null, snippet: string) => ({path, before, snippet, why: ''});
const owners = new Map([['first', 'gpu'], ['complete', 'gpu'], ['next', 'math']]);

test('a subject may finish its own files, but a later subject cannot replace them', () => {
    const result = auditOwnership([
        {id: 'first', edits: [edit('src/gpu.rs', null, 'create')]},
        {id: 'complete', edits: [edit('src/gpu.rs', 'old', 'finished')]},
        {id: 'next', edits: [edit('src/gpu.rs', 'finished', 'replacement')]},
    ], owners, name => name);
    assert.deepEqual(result.findings, [{path: 'src/gpu.rs', owner: 'gpu', subject: 'math', checkpoint: 'next', kind: 'rewrite'}]);
});

test('module registration preserves existing code and permits its cfg attribute', () => {
    const result = auditOwnership([
        {id: 'first', edits: [edit('src/lib.rs', null, '')]},
        {id: 'next', edits: [edit('src/lib.rs', 'mod gpu;\n', 'mod gpu;\n#[cfg(test)]\nmod math_tests;\n')]},
    ], owners, code => code);
    assert.equal(result.findings[0].kind, 'registration');
});

test('changing an existing module or hiding a replacement beside a registration remains a rewrite', () => {
    for (const code of ['mod other;\nmod math;\n', 'mod gpu;\nmod math;\nfn replacement() {}\n']) {
        const result = auditOwnership([
            {id: 'first', edits: [edit('src/lib.rs', null, '')]},
            {id: 'next', edits: [edit('src/lib.rs', 'mod gpu;\n', code)]},
        ], owners, source => source);
        assert.equal(result.findings[0].kind, 'rewrite');
    }
});
