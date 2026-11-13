import path from 'node:path';
import {course, expected, filesIn, read, viewer, type Files} from './model.ts';
import {rustCode} from './reference.ts';

export function compareEndpoint(actual: Files, production: Files) {
    const missing = Object.keys(production).filter(name => !(name in actual)).sort();
    const extra = Object.keys(actual).filter(name => !(name in production)).sort();
    const changed = Object.keys(production).filter(name => name in actual
        && rustCode(actual[name]) !== rustCode(production[name])).sort();
    return {missing, extra, changed};
}

export function endpointParity(details = false) {
    const checkpoint = course().steps.at(-1)!.id;
    const files = expected(checkpoint);
    const actual = Object.fromEntries(Object.entries(files).filter(([name]) => name.startsWith('src/')));
    const production = Object.fromEntries(filesIn(path.join(viewer, 'src')).map(file =>
        [path.relative(viewer, file).split(path.sep).join('/'), read(file)]));
    const differences = compareEndpoint(actual, production);
    const passed = Object.values(differences).every(names => names.length === 0);
    console.log(JSON.stringify({checkpoint, scope: 'Reconstructed cumulative source; no runtime acceptance',
        productionFiles: Object.keys(production).length, checkpointFiles: Object.keys(actual).length,
        passed, ...Object.fromEntries(Object.entries(differences).map(([key, names]) =>
            [key, details ? names : names.length]))}, null, 2));
    if (!passed) throw Error('The cumulative endpoint differs from production. No baseline was changed.');
}
