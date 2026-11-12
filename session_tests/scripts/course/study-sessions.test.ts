import assert from 'node:assert/strict';
import test from 'node:test';
import {studySessions} from './render.ts';
import type {Step} from './model.ts';

test('a long subject remains one checkpoint between shorter study sessions', () => {
    const steps = ['frame', 'camera', 'mathematics', 'rows'].map(id => ({id}) as Step);
    const minutes = new Map([['frame', 30], ['camera', 45], ['mathematics', 150], ['rows', 10]]);
    const sessions = studySessions(steps, step => ({lines: 0, characters: 0,
        minutes: [minutes.get(step.id)!, minutes.get(step.id)!]}));
    assert.deepEqual(sessions.map(session => session.steps.map(step => step.id)),
        [['frame'], ['camera'], ['mathematics'], ['rows']]);
    assert.equal(sessions[2].minutes[1], 150);
    assert.deepEqual(sessions.flatMap(session => session.steps), steps);
});

test('short adjacent subjects may share a sitting without merging their checkpoints', () => {
    const steps = ['frame', 'camera'].map(id => ({id}) as Step);
    const sessions = studySessions(steps, () => ({lines: 0, characters: 0, minutes: [15, 25]}));
    assert.equal(sessions.length, 1);
    assert.deepEqual(sessions[0].steps, steps);
    assert.deepEqual(sessions[0].minutes, [30, 50]);
});
