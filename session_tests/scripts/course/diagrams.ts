import fs from 'node:fs';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
import {docs, viewer, read, write} from './model.ts';

export function diagrams(check: boolean) {
    const binary = path.join(viewer, 'target/tools/d2-v0.9.0/bin/d2');
    if (!fs.existsSync(binary)) throw Error('Install D2 0.9.0 in target/tools/d2-v0.9.0/bin/d2 before regenerating diagrams.');
    const out = path.join(viewer, 'target/course-diagrams');
    fs.mkdirSync(out, {recursive: true});
    const changed: string[] = [];
    for (const name of fs.readdirSync(path.join(docs, 'diagrams')).filter(name => name.endsWith('.d2'))) {
        const source = path.join(docs, 'diagrams', name);
        const rendered = path.join(out, name.replace(/\.d2$/, '.svg'));
        const result = spawnSync(binary, [source, rendered], {encoding: 'utf8', env: {...process.env, D2_THEME: '0'}});
        if (result.status !== 0) throw Error(result.stdout + result.stderr);
        let svg = read(rendered);
        svg = svg.replace(/<svg\b[^>]*>/, tag => {
            const box = tag.match(/viewBox="0 0 ([\d.]+) ([\d.]+)"/);
            return box && !/\bwidth=/.test(tag) ? tag.slice(0, -1) + ` width="${box[1]}" height="${box[2]}">` : tag;
        });
        const target = path.join(docs, 'illustrations', path.basename(rendered));
        if (!fs.existsSync(target) || read(target) !== svg) {
            changed.push(name);
            if (!check) write(target, svg);
        }
    }
    if (check && changed.length) throw Error(`Stale diagrams: ${changed.join(', ')}`);
    console.log(check ? 'D2 diagrams reproduce exactly.' : `Updated ${changed.length} diagrams.`);
}
