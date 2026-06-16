// The Install tab's content: one entry per install_sections/NN-title.md (files starting "_", e.g. the
// template, are ignored). Shared by InstallView (renders a section) and MainLayout (lists them in the
// left sidebar). Bundled as raw text at build time. Same mechanism as viewerSections.ts.
export interface InstallSection { id: string; title: string; raw: string; order: number }

const mods = import.meta.glob('../install_sections/*.md', { query: '?raw', import: 'default', eager: true }) as Record<string, string>;

export const sections: InstallSection[] = Object.entries(mods)
  .filter(([path]) => !path.split('/').pop()!.startsWith('_'))
  .map(([path, raw]) => {
    const file = path.split('/').pop()!.replace('.md', '');
    const heading = (raw.match(/^#\s+(.+)$/m) || [])[1];
    return { id: file, title: heading || file, raw, order: parseInt(file, 10) || 0 };
  })
  .sort((a, b) => a.order - b.order);
