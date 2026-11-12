import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {fileURLToPath} from 'node:url';

const viewer = fileURLToPath(new URL('../../../session_viewer/', import.meta.url));
const core = 'actions check deck draw eval font geometry layout matrix morph net object paint run tex tween view'.split(' ');
const lines = (source: string) => source.split(/\r?\n/).filter(line => line.trim() && !line.includes('--8<-- ['));

test('the mathematics checkpoint retains the complete production stage core', () => {
    for (const name of core) {
        const source = fs.readFileSync(path.join(viewer, `src/app/stage/${name}.rs`), 'utf8');
        let lesson = fs.readFileSync(path.join(viewer, `docs/lessons/03a-math/src/app/stage/${name}.rs`), 'utf8');
        for (const logo of ['logo-eth.png', 'logo-brg.png']) {
            lesson = lesson.replace(
                `include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../assets/stage/${logo}"))`,
                `include_bytes!("../../../assets/stage/${logo}")`,
            );
        }
        assert.deepEqual(lines(lesson), lines(source), `${name}: revise the owning subject when production changes`);
    }
});
