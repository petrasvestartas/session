import fs from 'node:fs';
import path from 'node:path';
import {filesIn, hash, fingerprint, viewer} from './model.ts';

export function dependencyInputs(manifest: string, root = viewer) {
    const roots: [string, string][] = [];
    if (/^wood\s*=/m.test(manifest) || /^current_viewer\s*=/m.test(manifest)) {
        roots.push(['wood', path.resolve(root, '../../wood/rust')]);
    }
    if (/^current_viewer\s*=/m.test(manifest)) roots.push(['viewer', root]);
    const inputs: Record<string, string> = {};
    for (const [name, directory] of roots) {
        for (const folder of ['src', 'assets']) {
            const source = path.join(directory, folder);
            if (folder === 'assets' && !fs.existsSync(source)) continue;
            for (const file of filesIn(source)) inputs[`${name}/${path.relative(directory, file)}`] = hash(fs.readFileSync(file));
        }
        for (const file of ['Cargo.toml', 'Cargo.lock', 'build.rs', 'Trunk.toml', 'index.html']) {
            const source = path.join(directory, file);
            if (fs.existsSync(source)) inputs[`${name}/${file}`] = hash(fs.readFileSync(source));
        }
    }
    return roots.length ? {dependencies: fingerprint(inputs), dependency_checker: hash(fs.readFileSync(new URL(import.meta.url)))} : {};
}
