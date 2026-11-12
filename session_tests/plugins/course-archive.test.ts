import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {archivedPage, originalPage} from './course-archive.ts';

test('historical pages resolve into the archive without moving live guides or snippet fixtures', () => {
  const docs = fs.mkdtempSync(path.join(os.tmpdir(), 'course-archive-'));
  try {
    fs.mkdirSync(path.join(docs, 'archived'));
    for (const name of ['04b-strokes.md', 'reference-course.md', 'map.md']) {
      fs.writeFileSync(path.join(docs, 'archived', name), 'retained');
    }
    for (const name of ['04b-strokes.md', 'reference-course.md']) {
      const original = path.join(docs, name);
      const archived = path.join(docs, 'archived', name);
      assert.equal(archivedPage(docs, original), archived);
      assert.equal(originalPage(docs, archived), original);
    }
    for (const name of ['map.md', '01-missing.md', 'typing/code/01-001.rs', 'journey/01-canvas.md']) {
      const file = path.join(docs, name);
      assert.equal(archivedPage(docs, file), file);
      assert.equal(originalPage(docs, file), file);
    }
    assert.equal(originalPage(docs, path.join(docs, 'archived/README.md')), path.join(docs, 'archived/README.md'));
    for (const name of ['journey/01-canvas.md', 'journey/consolidation/01-frame.md']) {
      const live = path.join(docs, name);
      const archived = path.join(docs, 'archived', name.replaceAll('/', '-'));
      fs.writeFileSync(archived, 'retained journey');
      assert.equal(archivedPage(docs, live), archived);
      assert.equal(originalPage(docs, archived), live);
      fs.mkdirSync(path.dirname(live), {recursive: true});
      fs.writeFileSync(live, 'current subject');
      assert.equal(archivedPage(docs, live), live);
    }
  } finally {
    fs.rmSync(docs, {recursive: true, force: true});
  }
});
