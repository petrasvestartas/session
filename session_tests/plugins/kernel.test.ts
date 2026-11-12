import test from 'node:test';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {kernelClasses} from './kernel.ts';

test('motion APIs are discoverable under their actual kernel tests', () => {
  const classes = kernelClasses(fileURLToPath(new URL('../../', import.meta.url)));
  for (const [suite, name] of [['vector_test', 'Lerp'], ['color_test', 'Lerp'], ['line_test', 'Lerp'],
    ['polyline_test', 'Morph'], ['xform_test', 'Interpolate']]) {
    assert(classes.find(item => item.suite === suite)?.tests.includes(name), `${suite}: ${name}`);
  }
  const polyline = classes.find(item => item.suite === 'polyline_test')!;
  assert.deepEqual(polyline.aliases, [{name: 'resampled', test: 'Morph'}]);
  assert(!polyline.tests.includes('Resampled'), 'Do not invent a separately executed test');
});
