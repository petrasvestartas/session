import fs from 'node:fs';
import path from 'node:path';
import {docs, viewer, hash, write} from './model.ts';
import {runCommand} from './verify.ts';

export function foundations() {
    const out = path.join(viewer, 'target/course-foundations');
    fs.mkdirSync(out, {recursive: true});
    for (const name of fs.readdirSync(path.join(docs, 'foundations/code')).filter(name => name.endsWith('.rs')).sort()) {
        const source = path.join(docs, 'foundations/code', name);
        const id = path.basename(name, '.rs');
        const executable = path.join(out, id);
        const commands = [runCommand(`foundation-${id}-compile`, ['rustc', '--edition', '2024', source, '-o', executable], viewer),
            runCommand(`foundation-${id}-run`, [executable], viewer)];
        write(path.join(out, `${id}.json`), JSON.stringify({source: hash(fs.readFileSync(source)), commands}, null, 2) + '\n');
        console.log(`${id}: Rust assertions passed`);
    }
}
