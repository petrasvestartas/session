import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {completionFiles, completionSubjects} from './completion.ts';
import {viewer} from './model.ts';
import {assertRegistrationExtension} from './registration.ts';

const registrations = new Set(['src/lib.rs', 'src/app/mod.rs', 'src/engine/mod.rs',
    'src/engine/gpu/mod.rs', 'src/engine/gpu/objects.rs', 'src/engine/gpu/hull.rs',
    'src/engine/gpu/instance.rs', 'src/engine/pipelines/mod.rs', 'src/app/stage/mod.rs', 'examples/course/registration.rs']);

function withoutLaterDeclarations(text: string) {
    return text.split('\n').filter(line => {
        if (!line.includes('// register:application-tests') && !line.includes('// register:loading')) return true;
        assert.match(line, /^(?:#\[(?:cfg|path).*\]|(?:pub )?mod \w+;|pub use stream::\{[\w, ]+\};) \/\/ register:(?:application-tests|loading)$/);
        return false;
    }).join('\n');
}

test('cumulative subjects retain every introduced implementation byte for byte', () => {
    let previous = completionFiles('frame');
    for (const subject of completionSubjects.slice(1)) {
        const current = completionFiles(subject);
        for (const [name, text] of Object.entries(previous)) {
            assert.ok(name in current, `Removed: ${name}`);
            if (!registrations.has(name)) assert.equal(withoutLaterDeclarations(current[name]), withoutLaterDeclarations(text), name);
            else {
                assertRegistrationExtension(name, text, current[name]);
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
        assert.equal(text, production, name);
    }
    assert.ok(files['src/app/stage/geometry.rs']);
    assert.ok(files['src/app/stage/net.rs']);
    assert.equal(files['src/app/stage/tests.rs'], fs.readFileSync(path.join(viewer, 'src/app/stage/tests.rs'), 'utf8').replaceAll('\r\n', '\n'));
    assert.ok(files['src/app/stage/application.rs']);
    assert.ok(files['src/app/runtime.rs']);
    for (const subject of completionSubjects.slice(0, -1)) {
        const partial = completionFiles(subject);
        assert.equal(partial['src/app/runtime.rs'], undefined);
        for (const [name, text] of Object.entries(partial)) {
            if (name.startsWith('src/') && !registrations.has(name))
                assert.equal(text, withoutLaterDeclarations(fs.readFileSync(path.join(viewer, name), 'utf8').replaceAll('\r\n', '\n')), name);
        }
    }
});

test('early assemblies omit later entries and keep one unchanged browser startup function', () => {
    for (const [index, subject] of completionSubjects.entries()) {
        const files = completionFiles(subject);
        const registry = files['examples/course/registration.rs'];
        assert.equal(registry.includes('crate::run_web'), subject === 'application');
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

test('application endpoint contains every production source and its exact startup page', () => {
    const files = completionFiles('application');
    const names = fs.readdirSync(path.join(viewer, 'src'), {recursive: true})
        .map(name => `src/${String(name).replaceAll('\\', '/')}`)
        .filter(name => fs.statSync(path.join(viewer, name)).isFile()).sort();
    assert.deepEqual(Object.keys(files).filter(name => name.startsWith('src/')).sort(), names);
    for (const name of names) assert.equal(files[name], fs.readFileSync(path.join(viewer, name), 'utf8').replaceAll('\r\n', '\n'), name);
    assert.equal(files['production.html'], fs.readFileSync(path.join(viewer, 'index.html'), 'utf8').replaceAll('\r\n', '\n'));
    assert.equal(files['index.html'], completionFiles('frame')['index.html']);
    assert(!/session_viewer\s*=/.test(files['Cargo.toml']), 'The endpoint must compile its own source, not link the finished viewer crate');
});


test('registration interfaces reject added implementation and replaced declarations', () => {
    const before = 'mod device;\n';
    assert.doesNotThrow(() => assertRegistrationExtension('src/engine/gpu/mod.rs', before,
        before + 'mod context;\npub use context::Gpu;\n'));
    for (const added of ['fn replacement() {}', 'const HIDDEN: bool = true;',
        'impl Gpu {}', 'pub use context::Gpu; fn hidden() {}']) {
        assert.throws(() => assertRegistrationExtension('src/engine/gpu/mod.rs', before, before + added));
    }
    assert.throws(() => assertRegistrationExtension('src/engine/gpu/mod.rs', before, 'mod substitute;\n'));
    assert.throws(() => assertRegistrationExtension('examples/course/registration.rs', before,
        before + '("camera", other::start),\n'));
});
