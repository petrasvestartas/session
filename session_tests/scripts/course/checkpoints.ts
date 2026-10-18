import fs from 'node:fs';
import path from 'node:path';
import {work} from './model.ts';

export function savedPath(name: string, project = work): string {
    if (!name || !/^(?:[a-zA-Z0-9_-]+|\d+\.\d+\.\d+)$/.test(name)) throw Error('Use a save name with letters, digits, - and _, or a book lesson number such as 22.2.3.');
    return path.join(path.dirname(project), 'journey-saves', name);
}

function snapshot(source: string, destination: string) {
    if (!fs.statSync(source).isDirectory()) throw Error(`No project at ${source}`);
    if (fs.existsSync(destination)) throw Error(`Refusing to overwrite ${destination}`);
    fs.cpSync(source, destination, {
        recursive: true, dereference: false, verbatimSymlinks: true, errorOnExist: true, force: false,
        filter: name => !['target', 'dist', '.git'].includes(path.basename(name)),
    });
}

export function save(name: string, project = work) {
    const destination = savedPath(name, project);
    snapshot(project, destination);
    return destination;
}

export function restore(name: string, project = work): string | undefined {
    const source = savedPath(name, project);
    if (!fs.existsSync(source)) throw Error(`No saved checkpoint named ${name}`);
    let backup: string | undefined;
    if (fs.existsSync(project)) {
        backup = `${project}-before-${Date.now()}-${process.pid}`;
        if (fs.existsSync(backup)) throw Error(`Backup already exists: ${backup}`);
        fs.renameSync(project, backup);
    }
    snapshot(source, project);
    return backup;
}
