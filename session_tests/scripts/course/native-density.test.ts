import test from 'node:test';
import assert from 'node:assert/strict';
import path from 'node:path';
import {nativeDensityBridge} from './presentation.ts';
import {course, docs, expected, read, withExpectedSequence} from './model.ts';

const resize = '    renderer.resize(viewer_journey::viewport::Viewport { width: WIDTH, height: HEIGHT });\n';
const constructor = 'let mut renderer = Renderer::new(device, queue, format, &editor.scene);';
const header = 'const WIDTH: u32 = 2560;\nconst HEIGHT: u32 = 1920;\n' + resize;
const sources = withExpectedSequence(() => ['38a-samples', '79zze-fit-the-original-gordon-surface'].map(id => {
    const step = course().steps.find(row => row.id === id)!;
    return {fixture: read(path.join(docs, step.frame!)), renderer: expected(id)['src/renderer.rs']};
}));

function bridge(fixture: string, renderer = sources[0].renderer) {
    return nativeDensityBridge(header + fixture, 4, fixture, resize, renderer)!;
}

test('early sampling fixture gets physical sizing without inventing a refinement stage', () => {
    const {fixture, renderer} = sources[0], result = bridge(fixture, renderer);
    assert.equal(fixture.includes('math_quality.'), false);
    assert.equal(result.mathematicalRefinementPresent, false);
    assert.equal(result.constructorAdaptedBeforeMath, false);
    assert.deepEqual(result.physical, [2560, 1920]);
    assert(result.code.includes(constructor + '\n    renderer.set_density(4.0);\n' + resize.trimEnd()));
    const change = result.changes[0];
    assert.equal(result.code.replace(change.after, change.before), header + fixture);
    assert.equal(result.code.slice(result.code.indexOf('fn draw(')), fixture.slice(fixture.indexOf('fn draw(')));
    assert.equal(result.drawAndPixelAssertionsPreserved, true);
});

test('later surface fixture still sizes before its one physical refinement boundary', () => {
    const {fixture, renderer} = sources[1], result = bridge(fixture, renderer);
    assert.equal(result.constructorAdaptedBeforeMath, true);
    assert.equal(Object.hasOwn(result, 'mathematicalRefinementPresent'), false);
    assert(result.code.indexOf('renderer.set_density(4.0)') < result.code.indexOf('math_quality.drain_matrix('));
    const change = result.changes[0];
    assert.equal(result.code.replace(change.after, change.before), header + fixture);
    assert.equal(result.code.slice(result.code.indexOf('fn draw(')), fixture.slice(fixture.indexOf('fn draw(')));
    for (const changed of [
        fixture.replace('math_quality.drain_matrix(', 'math_quality.changed_drain('),
        fixture.replace('[WIDTH,HEIGHT]', '[640,480]'),
        fixture.replace('math_quality.drain_matrix(', 'math_quality.drain_matrix(); math_quality.drain_matrix('),
        fixture.replace('math_quality.drain_matrix(', 'renderer.resize(WIDTH, HEIGHT); math_quality.drain_matrix('),
    ]) assert.throws(() => bridge(changed, renderer));
});

test('refinement before construction and an incomplete early math fixture are refused', () => {
    const {fixture} = sources[0];
    assert.throws(() => bridge(fixture.replace(constructor, 'math_quality.drain_matrix([WIDTH,HEIGHT]);\n' + constructor)), /construction must precede/);
    assert.throws(() => bridge(fixture.replace(constructor, constructor + '\nmath_quality.refresh();')), /missing its drain/);
    assert.throws(() => nativeDensityBridge(header + fixture, 4, fixture, resize, ''), /set_density/);
    assert.throws(() => nativeDensityBridge(header + fixture + '\n', 4, fixture, resize, sources[0].renderer), /driver suffix/);
});
