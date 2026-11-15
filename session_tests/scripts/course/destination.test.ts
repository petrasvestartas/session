import assert from 'node:assert/strict';
import {test} from 'node:test';
import {auditDestination, type Destination} from './destination.ts';

const blob = 'a'.repeat(40);
function fixture(): Destination {
    return {
        commit: 'b'.repeat(40), status: 'planned',
        subjects: [{id: 'frame', title: 'First frame', prerequisites: []},
            {id: 'paint', title: 'First painting', prerequisites: ['frame']}],
        files: [{path: 'src/lib.rs', blob, subject: 'frame', teaching: null}],
        inputs: [{path: 'Cargo.lock', blob}],
    };
}
const tree = () => new Map([['src/lib.rs', blob], ['Cargo.lock', blob]]);

test('a matching destination reports planned coverage without accepting a subject', () => {
    const result = auditDestination(fixture(), tree(), []);
    assert.deepEqual(result.errors, []);
    assert.equal(result.acceptedSubjects, 0);
});

test('a new production source cannot disappear from ownership accounting', () => {
    const source = tree(); source.set('src/new.rs', blob);
    assert.deepEqual(auditDestination(fixture(), source, []).errors, ['Unassigned production source: src/new.rs']);
});

test('source and dependency pins are checked against the named commit', () => {
    const source = tree(); source.set('src/lib.rs', 'c'.repeat(40)); source.delete('Cargo.lock');
    assert.equal(auditDestination(fixture(), source, []).errors.length, 2);
});

test('duplicate ownership and unknown subjects fail', () => {
    const plan = fixture(); plan.files.push({...plan.files[0], subject: 'unknown'});
    assert.deepEqual(auditDestination(plan, tree(), []).errors, [
        'Duplicate destination path: src/lib.rs', 'Missing source owner: src/lib.rs',
    ]);
});

test('forward dependencies and self dependencies fail', () => {
    const plan = fixture(); plan.subjects[0].prerequisites = ['paint', 'frame'];
    assert.equal(auditDestination(plan, tree(), []).errors.length, 2);
});

test('working source drift requires explicit reconciliation', () => {
    assert.deepEqual(auditDestination(fixture(), tree(), ['src/lib.rs']).errors,
        ['Production reconciliation required: src/lib.rs']);
});
