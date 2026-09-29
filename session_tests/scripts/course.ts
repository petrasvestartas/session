import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';
import {course, docs, viewer, work, expected, referenceFiles, compare, materialize, referenceAssets, safeOutput, lockFor, dependencies} from './course/model.ts';
import {save, restore} from './course/checkpoints.ts';
import {generate} from './course/render.ts';
import {structure, verify} from './course/verify.ts';
import {serve} from './course/serve.ts';
import {verifyReference, parity} from './course/reference.ts';
import {diagrams} from './course/diagrams.ts';
import {foundations} from './course/foundations.ts';
import {dockPreview} from './course/dock.ts';
import {referenceStructure} from './course/reference-checks.ts';

function main() {
    const [action, name, ...args] = process.argv.slice(2);
    const lock = path.join(docs, 'journey/release/Cargo.lock');
    if (action === 'init') {
        fs.mkdirSync(path.dirname(work), {recursive: true});
        fs.mkdirSync(work);
        fs.copyFileSync(lock, path.join(work, 'Cargo.lock'));
        console.log('Created workspace/journey with the dependency lock only. Type the implementation yourself.');
    } else if (action === 'check') {
        const failures = compare(expected(name), work);
        if (failures.length) throw Error(failures.join('\n\n'));
        console.log('Source matches. Build and run the lesson experiment to check behavior.');
    } else if (action === 'dependencies') {
        dependencies(name);
        console.log('Dependency lock is current. Any previous lock was preserved; implementation files were not changed.');
    } else if (action === 'save') {
        console.log(`Saved your files in ${save(name)}`);
    } else if (action === 'restore') {
        const backup = restore(name);
        console.log(`Restored ${name}.${backup ? ` Your previous work is preserved in ${backup}.` : ''}`);
    } else if (action === 'reference') {
        const at = args.indexOf('--output');
        if (at < 0 || !args[at + 1]) throw Error('Choose a new --output directory.');
        const output = path.resolve(process.env.INIT_CWD || process.cwd(), args[at + 1]);
        safeOutput(output);
        materialize(expected(name), output, lockFor(name));
        console.log(`Reference assembled in ${output}. Your project was not changed.`);
    } else if (action === 'reference-check') {
        const failures = compare(referenceFiles(name), path.join(viewer, 'workspace/handwritten'));
        if (failures.length) throw Error(failures.join('\n\n'));
        console.log(`${name}: reference source matches.`);
    } else if (action === 'reference-init') {
        const target = path.join(viewer, 'workspace/handwritten');
        fs.mkdirSync(target);
        referenceAssets(target);
        console.log('Created the reference workspace with external assets only.');
    } else if (action === 'dock-preview') {
        dockPreview();
    } else if (action === 'generate') {
        generate();
    } else if (action === 'structure') {
        structure();
        referenceStructure();
    } else if (action === 'diagrams') {
        diagrams(name === '--check');
    } else if (action === 'foundations') {
        foundations();
    } else if (action === 'parity') {
        parity();
    } else if (action === 'verify-reference') {
        const ids = [name, ...args].filter(Boolean);
        verifyReference(ids.filter(id => id !== '--stored'), ids.includes('--stored'));
    } else if (action === 'verify') {
        const ids = [name, ...args].filter(Boolean);
        const stored = ids.includes('--stored');
        verify(ids.filter(id => id !== '--stored'), stored);
    } else if (action === 'capture') {
        const result = spawnSync(process.execPath, [path.join(docs, 'capture_journey.cjs')], {cwd: viewer, stdio: 'inherit',
            env: {...process.env, NODE_PATH: path.join(viewer, 'target/course-tools/node_modules')}});
        if (result.status !== 0) throw Error('Browser capture failed; see the error above.');
    } else if (action === 'serve') {
        serve(name ? Number(name) : 8781);
    } else if (action === 'list') {
        for (const step of course().steps) console.log(`${step.id}  ${step.title}`);
    } else {
        console.log('npm run course -- init | check ID | dependencies ID | save NAME | restore NAME | reference ID --output NEW_FOLDER');
        console.log('Authoring: dock-preview | generate | structure | verify [ID] [--stored] | verify-reference [ID] [--stored] | parity | foundations | diagrams [--check] | serve [PORT] | capture | list');
    }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    try {
        main();
    } catch (error) {
        console.error(error instanceof Error ? error.message : error);
        process.exitCode = 1;
    }
}
