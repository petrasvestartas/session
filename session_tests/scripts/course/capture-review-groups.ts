import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {hash, json, viewer, write} from './model.ts';

const sha = (file: string) => hash(fs.readFileSync(file));
const absolute = (file: string) => path.isAbsolute(file) ? file : path.join(viewer, file);

export function groupCaptures(nativeDirectory = '', browserDirectory = '') {
    const members: any[] = [], snapshots: any[] = [];
    for (const [kind, directory] of [['native', nativeDirectory], ['browser', browserDirectory]]) {
        if (!directory) continue;
        const progressFile = path.resolve(directory, 'progress.json'), progress = json(progressFile);
        snapshots.push({kind, path: progressFile, sha256: sha(progressFile)});
        for (const [section, rows] of [['main', Object.values(progress.items)], ['supplemental', Object.values(progress.supplementalItems || {})]] as [string, any[]][]) {
            for (const row of rows) {
                if (!row.image || !['CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', 'PUBLISHED_ROOT_REVIEWED'].includes(row.status)) continue;
                const image = absolute(row.image.path), bytes = fs.readFileSync(image);
                assert.equal(hash(bytes), row.image.sha256); assert.equal(sha(row.sourceRecord), row.sourceRecordSha256);
                const width = bytes.readUInt32BE(16), height = bytes.readUInt32BE(20);
                assert.equal(width, row.image.width); assert.equal(height, row.image.height);
                members.push({kind: section === 'supplemental' ? 'supplemental' : kind, id: row.id || row.checkpoint || row.path,
                    destination: row.path, sha256: row.image.sha256, width, height, image,
                    sourceRecord: row.sourceRecord, sourceRecordSha256: row.sourceRecordSha256, sourceKey: row.sourceKey || row.source || null});
            }
        }
    }
    const grouped = new Map<string, any>();
    for (const member of members) {
        const group = grouped.get(member.sha256) || {sha256: member.sha256, width: member.width, height: member.height, representativeID: member.id,
            representativeImage: member.image, members: []};
        assert.equal(group.width, member.width); assert.equal(group.height, member.height);
        group.members.push(member); grouped.set(member.sha256, group);
    }
    const groups = [...grouped.values()];
    const manifest = {status: 'AWAITING_ROOT_ORIGINAL_RESOLUTION_VIEWS', snapshots, capturedImages: members.length, uniquePngs: groups.length,
        scope: 'Every member was separately source-bound and captured. PNG byte identity permits one original-resolution visual review per SHA; no capture reuse or behavior-test claim.', groups};
    const key = hash(JSON.stringify(manifest)).slice(0, 20), directory = path.join(viewer, 'target/course-presentation/review-groups', key);
    const manifestFile = path.join(directory, 'groups.json'), template = path.join(directory, 'root-review-template.json');
    if (!fs.existsSync(manifestFile)) write(manifestFile, JSON.stringify(manifest, null, 2) + '\n');
    if (!fs.existsSync(template)) write(template, JSON.stringify({rootPersonallyViewedOriginalResolution: false,
        reviewed: groups.map(group => ({sha256: group.sha256, representativeID: group.representativeID, rootPersonallyReviewed: false}))}, null, 2) + '\n');
    return {manifestFile, template, capturedImages: members.length, uniquePngs: groups.length};
}

export function approveGroups(manifestFile: string, rootReviewFile: string) {
    const manifest = json(manifestFile), review = json(rootReviewFile); assert.equal(review.rootPersonallyViewedOriginalResolution, true);
    const outputs: Record<string, any[]> = {native: [], browser: [], supplemental: []};
    for (const reviewed of review.reviewed) {
        assert.equal(reviewed.rootPersonallyReviewed, true);
        const group = manifest.groups.find((row: any) => row.sha256 === reviewed.sha256); assert(group, 'Reviewed SHA absent from the captured group manifest');
        assert.equal(reviewed.representativeID, group.representativeID); assert.equal(sha(group.representativeImage), group.sha256);
        for (const member of group.members) {
            assert.equal(sha(member.image), group.sha256); assert.equal(sha(member.sourceRecord), member.sourceRecordSha256);
            outputs[member.kind].push({id: member.id, destination: member.destination, sha256: group.sha256,
                representativeID: group.representativeID, duplicateOfReviewedSHA: member.id === group.representativeID ? null : group.sha256,
                visualReviewScope: 'Exact PNG byte identity with the personally reviewed original; this member retains its own source-bound capture receipt.'});
        }
    }
    const directory = path.join(path.dirname(path.resolve(manifestFile)), 'approved-' + sha(rootReviewFile).slice(0, 16));
    const paths: Record<string, string> = {};
    for (const [kind, images] of Object.entries(outputs)) {
        const file = path.join(directory, kind + '-review.json');
        const record = {rootPersonallyViewedOriginalResolution: true, rootReview: {path: path.resolve(rootReviewFile), sha256: sha(rootReviewFile)},
            grouping: {path: path.resolve(manifestFile), sha256: sha(manifestFile)}, images};
        const text = JSON.stringify(record, null, 2) + '\n';
        if (fs.existsSync(file)) assert.equal(fs.readFileSync(file, 'utf8'), text); else write(file, text);
        paths[kind] = file;
    }
    return paths;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    const args = process.argv.slice(2), mode = args.shift();
    const option = (name: string) => { const at = args.indexOf(name); return at < 0 ? '' : args[at + 1]; };
    const result = mode === 'group' ? groupCaptures(option('--native'), option('--browser'))
        : mode === 'approve' ? approveGroups(option('--manifest'), option('--review')) : null;
    assert(result, 'Usage: capture-review-groups.ts group --native DIR [--browser DIR] | approve --manifest FILE --review ROOT_REVIEW_FILE');
    console.log(JSON.stringify(result, null, 2));
}
