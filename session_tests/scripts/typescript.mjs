import {readFile} from 'node:fs/promises';
import {register} from 'node:module';
import {isMainThread} from 'node:worker_threads';
import {fileURLToPath} from 'node:url';
import {transformSync} from 'esbuild';

if (isMainThread) register(import.meta.url);

export async function load(url, context, nextLoad) {
    if (!url.endsWith('.ts')) return nextLoad(url, context);
    const source = await readFile(new URL(url), 'utf8');
    const result = transformSync(source, {loader: 'ts', format: 'esm', target: 'es2022',
        sourcefile: fileURLToPath(url), sourcemap: 'inline'});
    return {format: 'module', source: result.code, shortCircuit: true};
}
