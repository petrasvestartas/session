import assert from 'node:assert/strict';
import test from 'node:test';
import {compareEndpoint} from './endpoint-parity.ts';

test('endpoint parity reports missing, retained and changed implementations together', () => {
    assert.deepEqual(compareEndpoint({
        'src/old.rs': 'old', 'src/changed.rs': 'fn value() { 1 }',
        'src/same.rs': '// teaching\r\nfn same() {}\r\n',
    }, {
        'src/new.rs': 'new', 'src/changed.rs': 'fn value() { 2 }',
        'src/same.rs': 'fn same() {}\n',
    }), {missing: ['src/new.rs'], extra: ['src/old.rs'], changed: ['src/changed.rs']});
});

test('comment-like text in shader and Rust literals remains part of parity', () => {
    assert.deepEqual(compareEndpoint({'src/a.rs': 'let a = "// old";'},
        {'src/a.rs': 'let a = "// new";'}).changed, ['src/a.rs']);
    assert.deepEqual(compareEndpoint({'src/a.wgsl': 'fn a() { /* note */ }'},
        {'src/a.wgsl': 'fn a() {  }'}), {missing: [], extra: [], changed: []});
});
