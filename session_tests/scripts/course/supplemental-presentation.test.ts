import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runMeasurementPresentation} from './supplemental-presentation.ts';
import {captureCompact} from './compact-capture.ts';

const id = '85ay-exercise-current-original-commands-together';

test('standalone measurement rejects unrelated or unbuilt inputs before creating capture outputs', async () => {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'measurement-input-')), prepared = path.join(root, 'prepared.json');
    try {
        fs.writeFileSync(prepared, JSON.stringify({id: '04-input', status: 'BUILT_NOT_CAPTURED'}));
        await assert.rejects(runMeasurementPresentation(prepared), /actual 85ay incremental browser build/);
        fs.writeFileSync(prepared, JSON.stringify({id, status: 'PREPARED_NOT_BUILT'}));
        await assert.rejects(runMeasurementPresentation(prepared), /BUILT_NOT_CAPTURED/);
        assert.deepEqual(fs.readdirSync(root), ['prepared.json']);
    } finally { fs.rmSync(root, {recursive: true}); }
});

test('compact measurement route cannot capture a different checkpoint or use the native lane', async () => {
    await assert.rejects(captureCompact('native', id, 'measurement'), /85ay browser checkpoint/);
    await assert.rejects(captureCompact('browser', '04-input', 'measurement'), /85ay browser checkpoint/);
});
