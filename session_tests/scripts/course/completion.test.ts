import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {completionFiles, completionSubjects} from './completion.ts';
import {viewer} from './model.ts';

const registrations = new Set(['src/lib.rs', 'src/app/mod.rs', 'src/engine/mod.rs',
    'src/engine/gpu/mod.rs', 'src/engine/gpu/objects.rs', 'src/engine/gpu/hull.rs',
    'src/engine/gpu/instance.rs', 'src/engine/pipelines/mod.rs', 'examples/course/registration.rs']);

test('cumulative subjects retain every introduced implementation byte for byte', () => {
    let previous = completionFiles('frame');
    for (const subject of completionSubjects.slice(1)) {
        const current = completionFiles(subject);
        for (const [name, text] of Object.entries(previous)) {
            assert.ok(name in current, `Removed: ${name}`);
            if (!registrations.has(name)) assert.equal(current[name], text, name);
            else {
                const lines = text.split('\n').filter(line => line.trim());
                let cursor = 0;
                for (const line of current[name].split('\n')) if (line === lines[cursor]) cursor++;
                assert.equal(cursor, lines.length, `Registration changed instead of appending: ${name}`);
            }
        }
        previous = current;
    }
});

test('retained production implementations are actual source, without substitute geometry or UI', () => {
    const files = completionFiles(completionSubjects.at(-1)!);
    for (const [name, text] of Object.entries(files)) {
        if (!name.startsWith('src/') || registrations.has(name) || name === 'src/app/stage/mod.rs') continue;
        const production = fs.readFileSync(path.join(viewer, name), 'utf8').replaceAll('\r\n', '\n');
        const retained = production.split('\n').filter(line => {
            if (!line.includes('// register:application-tests') && !line.includes('// register:loading')) return true;
            assert.match(line, /^(?:#\[(?:cfg|path).*\]|(?:pub )?mod \w+;|pub use stream::\{[\w, ]+\};) \/\/ register:(?:application-tests|loading)$/);
            return false;
        }).join('\n');
        assert.equal(text, retained, name);
    }
    assert.ok(files['src/app/stage/geometry.rs']);
    assert.ok(files['src/app/stage/net.rs']);
    assert.equal(files['src/app/stage/tests.rs'], fs.readFileSync(path.join(viewer, 'src/app/stage/tests.rs'), 'utf8').replaceAll('\r\n', '\n'));
    assert.equal(files['src/app/stage/application.rs'], undefined);
    assert.equal(files['src/app/runtime.rs'], undefined);
});

test('early assemblies omit later entries and keep one unchanged browser startup function', () => {
    for (const [index, subject] of completionSubjects.entries()) {
        const files = completionFiles(subject);
        const registry = files['examples/course/registration.rs'];
        assert.ok(!registry.includes('crate::run_web'));
        assert.match(files['src/lib.rs'], /feature = "tutorial"/);
        for (const later of completionSubjects.slice(index + 1)) {
            assert.ok(!registry.includes(`mod ${later};`));
            assert.equal(files[`examples/course/${later}.rs`], undefined);
        }
    }
    assert.throws(() => completionFiles('viewer'), /Unknown completion subject/);
});


test('shader checkpoints own the production build script and every transitive include', () => {
    for (const subject of completionSubjects) {
        const files = completionFiles(subject);
        const shaders = Object.keys(files).filter(name => name.endsWith('.wgsl'));
        if (!shaders.length) continue;
        assert.equal(files['build.rs'], fs.readFileSync(path.join(viewer, 'build.rs'), 'utf8').replaceAll('\r\n', '\n'));
        for (const name of shaders) {
            for (const match of files[name].matchAll(/^\s*#include "([^"]+)"/gm)) {
                assert.ok(files[`src/shaders/${match[1]}`], `${name} includes missing ${match[1]}`);
            }
        }
    }
});
