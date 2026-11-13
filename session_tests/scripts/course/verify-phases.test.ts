import assert from 'node:assert/strict';
import test from 'node:test';
import {webEvidence, verify} from './verify.ts';

const web = () => ({source: 'checked source', commands: [
    {name: 'lesson-wasm', args: ['cargo', 'build'], returncode: 0},
    {name: 'lesson-web', args: ['trunk', 'build', '--release'], returncode: 0},
]});

test('native continuation requires successful wasm and optimized web evidence for the same source', () => {
    assert.throws(() => webEvidence('lesson', undefined, 'checked source'), /stale or missing/);
    assert.throws(() => webEvidence('lesson', web(), 'changed source'), /stale or missing/);
    for (const index of [0, 1]) {
        const missing = web();
        missing.commands.splice(index, 1);
        assert.throws(() => webEvidence('lesson', missing, 'checked source'));
        const failed = web();
        failed.commands[index].returncode = 1;
        assert.throws(() => webEvidence('lesson', failed, 'checked source'));
    }
    const debug = web();
    debug.commands[1].args.pop();
    assert.throws(() => webEvidence('lesson', debug, 'checked source'), /optimized/);
});

test('continuing native checks cannot mutate the saved web command list', () => {
    const saved = web();
    const resumed = webEvidence('lesson', saved, 'checked source');
    resumed.commands.push({name: 'lesson-frame', args: [], returncode: 0});
    assert.equal(saved.commands.length, 2);
});

test('partial phases cannot be used as stored acceptance', () => {
    for (const phase of ['web', 'native'] as const)
        assert.throws(() => verify([], true, false, phase), /require all phases/);
});
