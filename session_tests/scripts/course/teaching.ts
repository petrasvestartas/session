import fs from 'node:fs';
import path from 'node:path';
import {docs, hash, viewer, type Files} from './model.ts';
import {completionFiles, completionSubjects} from './completion.ts';

export interface Teaching {
    path: string;
    owner: string;
    sourceSha256: string;
    explanation?: {page: string; heading: string; paragraph: string};
}

export function auditTeaching(entries: Teaching[], sources: Files, pages: Files, owners: Map<string, string>) {
    const errors: string[] = [], missing: string[] = [];
    const seen = new Set<string>();
    let explained = 0;
    for (const entry of entries) {
        const name = entry.path;
        if (seen.has(name)) errors.push(`Duplicate teaching entry: ${name}`);
        seen.add(name);
        if (!(name in sources)) { errors.push(`Unknown production source: ${name}`); continue; }
        if (!owners.has(name) || entry.owner !== owners.get(name)) errors.push(`Wrong introduction owner: ${name}`);
        if (hash(sources[name]) !== entry.sourceSha256) errors.push(`Review changed source: ${name}`);
        const explanation = entry.explanation;
        if (!explanation) { missing.push(name); continue; }
        const page = pages[explanation.page];
        if (!page) { errors.push(`Missing teaching page: ${name}`); continue; }
        // Listings and links cannot satisfy the prose witness.
        const proseOnly = (text: string) => text.replace(/```[^]*?```/g, '').replace(/\[[^\]]*\]\([^)]*\)/g, '');
        const prose = proseOnly(page), paragraph = proseOnly(explanation.paragraph);
        const heading = /^(#{2,6}) /m.exec(explanation.heading);
        const start = prose.indexOf(explanation.heading + '\n');
        const rest = start < 0 ? '' : prose.slice(start + explanation.heading.length + 1);
        const next = heading ? rest.search(new RegExp(`^#{1,${heading[1].length}} `, 'm')) : 0;
        const section = next < 0 ? rest : rest.slice(0, next);
        if (!heading || start < 0 || paragraph.length < 100 || !section.includes(paragraph))
            errors.push(`Missing substantive explanation in named section: ${name}`);
        else explained++;
    }
    for (const name of Object.keys(sources)) if (!seen.has(name)) errors.push(`Unaccounted production source: ${name}`);
    return {sources: Object.keys(sources).length, explained, missing, errors,
        scope: 'Prose witnesses and introduction ownership only; human teaching review and executable acceptance remain separate.'};
}

export const supportOwners = new Map([
    ['Cargo.toml', 'frame'], ['Cargo.lock', 'frame'], ['viewer-core/Cargo.toml', 'frame'],
    ['build.rs', 'rows'],
    ['Trunk.toml', 'application'], ['index.html', 'application'],
    ['assets/diagnostics.js', 'application'], ['assets/text-quality.html', 'application'],
    ['assets/view_local.yaml', 'application'],
]);

export function productionSupportFiles(root: string) {
    const files = fs.readdirSync(root).filter(name => /\.(rs|html|toml|lock)$/.test(name));
    if (fs.existsSync(path.join(root, 'viewer-core/Cargo.toml'))) files.push('viewer-core/Cargo.toml');
    for (const relative of fs.readdirSync(path.join(root, 'assets'), {recursive: true})) {
        const name = String(relative).replaceAll('\\', '/');
        if (name.startsWith('pb/')) continue;
        if (/\.(js|html|yaml|yml)$/.test(name)) files.push(`assets/${name}`);
    }
    return files.sort();
}

export function teachingInputs() {
    const owners = new Map<string, string>();
    for (const subject of completionSubjects)
        for (const name of Object.keys(completionFiles(subject)))
            if ((name.startsWith('src/') || name.startsWith('viewer-core/src/')) && !owners.has(name))
                owners.set(name, subject);
    const sources: Files = {}, pages: Files = {};
    for (const [name, owner] of supportOwners) owners.set(name, owner);
    for (const name of productionSupportFiles(viewer))
        sources[name] = fs.readFileSync(path.join(viewer, name), 'utf8').replaceAll('\r\n', '\n');
    for (const folder of ['src', 'viewer-core/src']) {
        for (const relative of fs.readdirSync(path.join(viewer, folder), {recursive: true})) {
            const name = `${folder}/${String(relative).replaceAll('\\', '/')}`;
            if (fs.statSync(path.join(viewer, name)).isFile())
                sources[name] = fs.readFileSync(path.join(viewer, name), 'utf8').replaceAll('\r\n', '\n');
        }
    }
    for (const name of fs.readdirSync(path.join(docs, 'journey')))
        if (name.endsWith('.md')) pages[`journey/${name}`] = fs.readFileSync(path.join(docs, 'journey', name), 'utf8').replaceAll('\r\n', '\n');
    return {sources, pages, owners};
}

export function teaching() {
    const entries: Teaching[] = JSON.parse(fs.readFileSync(path.join(docs, 'journey/completion-teaching.json'), 'utf8'));
    const {sources, pages, owners} = teachingInputs();
    return auditTeaching(entries, sources, pages, owners);
}
