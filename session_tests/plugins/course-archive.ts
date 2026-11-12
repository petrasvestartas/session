import fs from 'node:fs';
import path from 'node:path';

export function archivedPage(docs: string, file: string): string {
  const relative = path.relative(docs, file).split(path.sep).join('/');
  if (/^journey\/(?:consolidation\/)?[^/]+\.md$/.test(relative) && !fs.existsSync(file)) {
    const archived = path.join(docs, 'archived', relative.replaceAll('/', '-'));
    if (fs.existsSync(archived)) return archived;
  }
  if (path.dirname(file) !== docs || !/^(?:\d\d[a-z]?-.+|reference-course)\.md$/.test(path.basename(file))) return file;
  const archived = path.join(docs, 'archived', path.basename(file));
  return fs.existsSync(archived) ? archived : file;
}

export function originalPage(docs: string, file: string): string {
  const name = path.basename(file);
  if (path.dirname(file) === path.join(docs, 'archived') && name.startsWith('journey-')) {
    return name.startsWith('journey-consolidation-')
      ? path.join(docs, 'journey/consolidation', name.slice('journey-consolidation-'.length))
      : path.join(docs, 'journey', name.slice('journey-'.length));
  }
  return path.dirname(file) === path.join(docs, 'archived') && /^(?:\d\d[a-z]?-.+|reference-course)\.md$/.test(path.basename(file))
    ? path.join(docs, path.basename(file)) : file;
}
