import test from 'node:test';
import assert from 'node:assert/strict';
import {assertRepeatedImage} from './verify.ts';
import type {Step} from './model.ts';

test('a shared physical filename does not replace proof of the declared parent bytes', () => {
    const steps = [{id: 'parent'}, {id: 'other'}, {id: 'child', image_same_as: 'parent'}] as Step[];
    const file = '/canonical/one.png', seen = new Map([['parent', new Set(['same'])], ['other', new Set(['different'])]]);
    assert.doesNotThrow(() => assertRepeatedImage(steps[2], file, file, steps, seen, 'same'));
    assert.throws(() => assertRepeatedImage(steps[2], file, file, steps, seen, 'different'), /Undeclared repeated/);
    assert.throws(() => assertRepeatedImage(steps[1], file, file, steps, seen, 'same'), /Undeclared repeated/);
});

test('several children can retain one original image owner after intervening children', () => {
    const steps = [{id: '03-triangle'}, {id: '03a-fonts', image_same_as: '03-triangle'},
        {id: '03a-paint', image_same_as: '03-triangle'}] as Step[];
    const seen = new Map([['03-triangle', new Set(['triangle'])], ['03a-fonts', new Set(['triangle'])]]);
    assert.doesNotThrow(() => assertRepeatedImage(steps[2], '/canonical/03-triangle.png', '/canonical/03-triangle.png', steps, seen, 'triangle'));
});

test('repeated-image parent must still exist and precede the child', () => {
    for (const parent of ['child', 'future', 'missing']) {
        const steps = [{id: 'child', image_same_as: parent}, {id: 'future'}] as Step[];
        assert.throws(() => assertRepeatedImage(steps[0], '/canonical/one.png', undefined, steps, new Map(), 'same'), /parent must precede/);
    }
});

test('first-clear exception remains restricted to the original browser checkpoint pair', () => {
    const steps = [{id: '01-canvas'}, {id: '02-clear'}, {id: '03-triangle'}] as Step[];
    const file = '/canonical/01-canvas-browser.png', seen = new Map([['01-canvas', new Set(['same'])]]);
    assert.doesNotThrow(() => assertRepeatedImage(steps[1], file, file, steps, seen, 'same'));
    assert.throws(() => assertRepeatedImage(steps[1], '/canonical/native.png', file, steps, seen, 'same'), /Undeclared repeated/);
    assert.throws(() => assertRepeatedImage(steps[2], file, file, steps, seen, 'same'), /Undeclared repeated/);
});
