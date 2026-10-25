import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {hash, read} from './model.ts';

export function exactBindgen(lockfile: string) {
    const version = read(lockfile).match(/\[\[package\]\]\s+name = "wasm-bindgen"\s+version = "([^"]+)"/)?.[1];
    assert(version, 'The source lockfile must pin wasm-bindgen');
    const candidates = [process.env.WASM_BINDGEN_EXECUTABLE,
        process.env.LOCALAPPDATA && path.join(process.env.LOCALAPPDATA, 'trunkrs/trunk/cache', 'wasm-bindgen-' + version, 'wasm-bindgen.exe'),
        process.env.USERPROFILE && path.join(process.env.USERPROFILE, '.cargo/bin/wasm-bindgen.exe')].filter(Boolean) as string[];
    for (const executable of candidates) {
        if (!fs.existsSync(executable)) continue;
        const result = spawnSync(executable, ['--version'], {encoding: 'utf8', windowsHide: true});
        if (result.status === 0 && result.stdout.trim() === 'wasm-bindgen ' + version)
            return {executable, version, sha256: hash(fs.readFileSync(executable))};
    }
    throw Error(`No installed wasm-bindgen CLI matches locked ${version}; preserve the lockfile and use its existing Trunk cache version.`);
}
