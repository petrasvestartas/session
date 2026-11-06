import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';

const sha = (value: string) => createHash('sha256').update(value).digest('hex');
const source = fileURLToPath(new URL('./image-cap-msaa4.json', import.meta.url));
const id = '96zzc-share-section-addresses-with-outlines-and-ambient';

export function capImageMode(checkpoint: string, lane: string, scale: string, presentation: string, requested = process.env.COURSE_IMAGE_CAP_SAMPLES || '1') {
    assert(['1', '4'].includes(requested), 'Image cap samples must be 1 or 4');
    if (requested === '1' || lane !== 'browser') return false;
    assert.equal(checkpoint, id); assert.equal(lane, 'browser'); assert.equal(scale, '2'); assert.equal(presentation, 'linear-srgb-gpu');
    return true;
}

export function capImageBridge(native: string, profile: any, data?: any) {
    if (profile?.capSamples !== 4) return null;
    data ??= JSON.parse(fs.readFileSync(source, 'utf8'));
    assert.equal(profile.lane, 'browser'); assert.equal(profile.renderScale, 2); assert.equal(profile.presentation, 'linear-srgb-gpu');
    assert.equal(data.id, id); assert.equal(data.samples, 4); assert.equal(data.querySamples, 1);
    const files: Record<string, {code: string; beforeSha256: string | null; afterSha256: string; reversedSha256: string | null}> = {};
    for (const row of data.files) {
        assert(/^src\/(?:renderer|caps_gpu|lib|course_image_[a-z_]+)\.(?:rs|wgsl)$/.test(row.path));
        assert(!Object.hasOwn(files, row.path));
        const target = path.join(native, row.path);
        const before = fs.existsSync(target) ? fs.readFileSync(target, 'utf8') : null;
        assert.equal(before, row.before, 'Exact image-overlay preimage: ' + row.path);
        assert.equal(before === null ? null : sha(before), row.beforeSha256);
        assert.equal(sha(row.after), row.afterSha256);
        files[row.path] = {code: row.after, beforeSha256: row.beforeSha256, afterSha256: row.afterSha256, reversedSha256: row.beforeSha256};
    }
    assert.equal(Object.keys(files).length, 10);
    return {id, samples: 4, querySamples: 1, files, scope: 'Final image only; original single-sample query and callback paths preserved'};
}

export function validateCapCoverage(frame: any, proof: any, prior?: any) {
    assert.deepEqual(proof.internal, frame.internal);
    assert.equal(proof.samples, 4); assert.equal(proof.querySamples, 1);
    assert.deepEqual(proof.attachments, {color: 4, depth: 4, primitive: 4, crossings: 4, selectedCoverage: 4});
    assert.equal(proof.colorResolve, 'same-size'); assert.equal(proof.integerResolve, false); assert.equal(proof.queryTargetsUntouched, true);
    assert.equal(proof.deviceScope, 'owning-Renderer');
    assert.equal(proof.allocationGeneration, 1, 'This fixed image profile does not accept resource replacement');
    const pixels = frame.internal[0] * frame.internal[1];
    assert(Number.isSafeInteger(proof.extraTexelAndBufferBytes));
    assert(proof.extraTexelAndBufferBytes >= pixels * 60 + 448);
    assert(proof.extraTexelAndBufferBytes <= pixels * 68 + 16384, 'Bound image attachments, final presentation and selected mask storage');
    if (prior) assert.deepEqual(proof, prior, 'Two fresh final-image redraws reuse the same attachment allocation');
}

export async function capImageDevices(page: any) {
    return page.evaluate(() => {
        const devices = (window as any).__presentationDevices;
        if (!Array.isArray(devices) || devices.length !== 1) throw Error('One live image device is required');
        const old = (window as any).__courseCapDevice;
        if (old && old !== devices[0]) throw Error('Image mode does not accept device recovery');
        (window as any).__courseCapDevice = devices[0];
        return devices.length;
    });
}

export async function capImageReuse(page: any, frame: any, settled: any, settle: () => Promise<any>) {
    const read = () => page.evaluate(() => JSON.parse(document.querySelector('canvas')!.getAttribute('data-image-cap-coverage')!));
    const first = await read(); validateCapCoverage(frame, first);
    await page.evaluate(async () => {
        window.dispatchEvent(new Event('resize'));
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    });
    const repeated = await settle();
    assert.deepEqual(repeated, settled, 'Same-camera image redraw preserves complete accepted quality and GPU tuples');
    const second = await read(); validateCapCoverage(frame, second, first);
    assert.equal(await capImageDevices(page), 1);
    return {first, repeated: second, deviceCount: 1};
}
