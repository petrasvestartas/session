import fs from 'node:fs';
import path from 'node:path';
import {docs, viewer, expected, read, write, hash, fingerprint, filesIn, materialize} from './model.ts';
import {runCommand, evidence} from './verify.ts';

/** Build the real production dock in the small lesson-04 host before rewriting its teaching steps. */
export function dockPreview() {
    const source = path.join(docs, 'journey/dock');
    const target = path.join(viewer, 'target/course-real-dock');
    const files = expected('04-input');
    for (const name of ['Cargo.toml', 'index.html']) files[name] = read(path.join(source, name));
    for (const name of ['browser.rs', 'dock.rs']) files['src/' + name] = read(path.join(source, name));
    delete files['src/panel.rs'];
    files['src/lib.rs'] = files['src/lib.rs'].replace(/#\[cfg\(target_arch = "wasm32"\)\]\nmod (command_dock|panel);\n/g, '');
    files['src/lib.rs'] += '\n#[cfg(target_arch = "wasm32")]\nmod command_dock;\n#[cfg(target_arch = "wasm32")]\nmod dock;\n';
    for (const file of filesIn(path.join(viewer, 'src/command_dock'))) {
        files[path.relative(viewer, file)] = read(file);
    }
    fs.rmSync(target, {recursive: true, force: true});
    materialize(files, target, path.join(source, 'Cargo.lock'));
    const fonts: Record<string, string> = {};
    for (const name of ['NotoSans-Regular', 'NotoSansSymbols2-Regular', 'NotoSansSymbols-Regular']) {
        const file = `assets/text/${name}.subset.ttf`;
        const bytes = fs.readFileSync(path.join(viewer, file));
        write(path.join(target, file), bytes);
        fonts[file] = hash(bytes);
    }
    const commands = [
        runCommand('real-dock-wasm', ['cargo', 'build', '--lib', '--locked', '--target', 'wasm32-unknown-unknown', '-j4'], target),
        runCommand('real-dock-web', ['trunk', 'build', '--public-url', './'], target),
    ];
    const bundle = path.join(evidence, 'real-dock/dist');
    fs.rmSync(bundle, {recursive: true, force: true});
    fs.cpSync(path.join(target, 'dist'), bundle, {recursive: true});
    write(path.join(evidence, 'real-dock/build.json'), JSON.stringify({
        source: fingerprint({...files, ...fonts, 'Cargo.lock': read(path.join(source, 'Cargo.lock'))}),
        commands, status: 'integration preview; typing lessons not yet rewritten',
    }, null, 2) + '\n');
    console.log('Built the real command-dock preview. With course serve running: http://127.0.0.1:8781/real-dock/dist/');
}
