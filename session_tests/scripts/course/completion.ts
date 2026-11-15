import fs from 'node:fs';
import path from 'node:path';
import {viewer, type Files, materializedContent} from './model.ts';

export const completionSubjects = ['frame', 'painting', 'mathematics', 'dock', 'camera', 'rows', 'geometry', 'decoding', 'application'] as const;
const mathematics = ['actions', 'check', 'deck', 'draw', 'eval', 'font', 'geometry', 'layout',
    'matrix', 'math_mode', 'morph', 'net', 'object', 'paint', 'presentation',
    'presentation_tests', 'run', 'tests', 'tex', 'tween', 'view'];

// Only module declarations and this explicit example entry list vary between assemblies.
function declarations(source: string, names: string[]) {
    const lines = source.split('\n');
    return lines.flatMap((line, index) => {
        const match = /^(?:pub(?:\(crate\))? )?mod (\w+);(?:\s*\/\/.*)?\s*$/.exec(line);
        if (!match || !names.includes(match[1])) return [];
        const attributes: string[] = [];
        for (let i = index - 1; i >= 0 && lines[i].startsWith('#['); i--) attributes.unshift(lines[i]);
        return [...attributes, line];
    }).join('\n') + '\n';
}

export function completionFiles(subject: string, read = (name: string) =>
    fs.readFileSync(path.join(viewer, name), 'utf8').replaceAll('\r\n', '\n')): Files {
    const stage = completionSubjects.indexOf(subject as typeof completionSubjects[number]);
    if (stage < 0) throw Error(`Unknown completion subject: ${subject}`);
    const files: Files = {};
    const retain = (name: string) => { files[name] = read(name); };
    for (const name of ['src/engine/gpu/device.rs', 'src/app/launch.rs',
        'examples/course/canvas.rs', 'examples/course/frame.rs', 'examples/course/entry.rs']) retain(name);
    const root = read('src/lib.rs');
    files['src/lib.rs'] = root.slice(0, root.indexOf('mod engine;'))
        + declarations(root, ['engine', 'app', 'course',
            ...(stage >= 3 ? ['command_dock'] : []), ...(stage >= 4 ? ['camera'] : [])]);
    files['src/engine/mod.rs'] = declarations(read('src/engine/mod.rs'), ['gpu', ...(stage >= 5 ? ['pipelines', 'performance', 'diagnostics'] : [])]);
    files['src/app/mod.rs'] = declarations(read('src/app/mod.rs'), ['launch', ...(stage >= 2 ? ['mode', 'stage'] : []), ...(stage >= 6 ? ['walk', 'gizmo', 'knobs', 'interactions'] : []), ...(stage >= 7 ? ['decode', 'validate', 'yielding'] : [])]);
    files['src/engine/gpu/mod.rs'] = declarations(read('src/engine/gpu/mod.rs'), ['device', ...(stage >= 1 ? ['ui'] : []), ...(stage >= 5 ? ['buffers', 'hull', 'instance', 'objects'] : [])]);
    const registrations = read('examples/course/registration.rs');
    const names = ['canvas', ...completionSubjects.slice(0, stage + 1)];
    files['examples/course/registration.rs'] = '// --8<-- [start:course-registration]\n'
        + declarations(registrations, names)
        + '\ntype Start = fn() -> Result<(), wasm_bindgen::JsValue>;\n'
        + 'pub(super) const STARTS: &[(&str, Start)] = &[\n'
        + completionSubjects.slice(0, stage + 1).map(name => `    ("${name}", ${name === 'application' ? 'crate::run_web' : `${name}::start`}),\n`).join('')
        + '];\n// --8<-- [end:course-registration]\n';
    if (stage >= 1) {
        retain('src/engine/gpu/ui.rs');
        retain('examples/course/painting.rs');
    }
    if (stage >= 2) {
        retain('src/app/mode.rs');
        for (const name of mathematics) retain(`src/app/stage/${name}.rs`);
        files['src/app/stage/mod.rs'] = declarations(read('src/app/stage/mod.rs'), mathematics);
        retain('examples/course/mathematics.rs');
        retain('examples/course/index.yaml');
        retain('docs/lessons/03a-math/example.yaml');
    }
    if (stage >= 3) {
        for (const name of ['mod', 'view', 'theme']) retain(`src/command_dock/${name}.rs`);
        retain('examples/course/dock.rs');
    }
    if (stage >= 4) {
        retain('src/camera.rs');
        retain('examples/course/camera.rs');
    }
    if (stage >= 5) {
        for (const name of ['gpu/buffers.rs', 'gpu/hull/geometry.rs', 'gpu/instance/layout.rs',
            'gpu/objects/table.rs', 'pipelines/core.rs', 'pipelines/bindings.rs',
            'pipelines/layouts.rs', 'performance.rs', 'diagnostics.rs']) retain(`src/engine/${name}`);
        files['src/engine/gpu/hull.rs'] = 'mod geometry;\npub use geometry::{Hull, Points, extreme_points, placed_box};\n';
        files['src/engine/gpu/instance.rs'] = 'mod layout;\npub use layout::Instance;\n';
        files['src/engine/gpu/objects.rs'] = 'mod table;\npub use table::{InstanceTable, ObjectRow, ObjectRows, Rebase, world_box};\n';
        files['src/engine/pipelines/mod.rs'] = 'pub(crate) mod bindings;\npub mod layouts;\nmod core;\npub use layouts::Layouts;\npub use core::*;\n';
        for (const name of ['scene', 'clip', 'ink_visibility', 'normals', 'physical', 'projected_triangle']) retain(`src/shaders/${name}.wgsl`);
        retain('examples/course/rows.rs');
        retain('build.rs');
    }
    if (stage >= 6) {
        const laterAdapters = /(?:app\/walk\/cloud\/stream\.rs|application_tests\.rs|gpu\/(?:clip\/(?:tests|editing_tests)\.rs|ssao\/tests\.rs|surface_outline\/tests\.rs|splat\/cloud_witness(?:\.rs|\/)))/;
        for (const folder of ['src/engine', 'src/shaders', 'src/app/walk']) {
            for (const relative of fs.readdirSync(path.join(viewer, folder), {recursive: true})) {
                const name = `${folder}/${String(relative).replaceAll('\\', '/')}`;
                if (!fs.statSync(path.join(viewer, name)).isFile() || laterAdapters.test(name)) continue;
                files[name] = read(name).split('\n').filter(line => !line.includes('// register:application-tests') && !line.includes('// register:loading')).join('\n');
            }
        }
        for (const name of ['gizmo', 'knobs', 'interactions']) retain(`src/app/${name}.rs`);
        retain('examples/course/geometry.rs');
    }
    if (stage >= 7) {
        for (const name of ['decode', 'validate', 'yielding']) retain(`src/app/${name}.rs`);
        retain('examples/course/decoding.rs');
    }
    if (stage >= 8) {
        // The complete source lives in this checkpoint; no dependency on a viewer binary or crate.
        for (const relative of fs.readdirSync(path.join(viewer, 'src'), {recursive: true})) {
            const name = `src/${String(relative).replaceAll('\\', '/')}`;
            if (fs.statSync(path.join(viewer, name)).isFile()) retain(name);
        }
        // Keep the teaching entry page; production startup is a new, independently built entry.
        files['production.html'] = read('index.html');
    }
    files['Cargo.toml'] = read('Cargo.toml').replace('path = "../session_rust"', 'path = "../../../session_rust"');
    files['Cargo.lock'] = read('Cargo.lock');
    files['index.html'] = read('examples/course/index.html');
    return files;
}

export function materializeCompletion(subject: string, output: string) {
    const files = completionFiles(subject);
    const application = completionSubjects.indexOf(subject as typeof completionSubjects[number]) >= 8;
    const builtDocs = path.join(viewer, 'target/docs/vue');
    if (application && !fs.existsSync(builtDocs))
        throw Error('Build the documentation site before materializing the production startup page');
    // mkdir without recursive refuses existing work, including an existing symlink.
    fs.mkdirSync(path.dirname(output), {recursive: true});
    fs.mkdirSync(output);
    for (const [name, content] of Object.entries(files)) {
        const target = path.join(output, name);
        fs.mkdirSync(path.dirname(target), {recursive: true});
        fs.writeFileSync(target, materializedContent(name, content, output));
    }
    // Fonts and their licences remain in the one shared asset directory.
    fs.symlinkSync(path.join(viewer, 'assets'), path.join(output, 'assets'), 'junction');
    if (completionSubjects.indexOf(subject as typeof completionSubjects[number]) >= 2)
        fs.symlinkSync(path.join(viewer, 'examples/stage'), path.join(output, 'examples/stage'), 'junction');
    if (application) {
        fs.mkdirSync(path.join(output, 'target/docs'), {recursive: true});
        fs.symlinkSync(builtDocs, path.join(output, 'target/docs/vue'), 'junction');
        fs.mkdirSync(path.join(output, 'tests'));
        fs.symlinkSync(path.join(viewer, 'tests/fixtures'), path.join(output, 'tests/fixtures'), 'junction');
    }
    return {subject, output, sourceFiles: Object.keys(files).length,
        status: 'candidate; compilation and browser acceptance required'};
}
