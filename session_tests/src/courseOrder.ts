type Links = { prev?: string; next?: string };

export function latestCodingLesson(pages: Record<string, Links>) {
  if (pages['journey/completion-application']) return 'journey/completion-application';
  const last = Object.keys(pages).find(slug =>
    slug.startsWith('journey/') && pages[slug].prev && !pages[slug].next);
  return last === 'journey/97-finish-with-the-maintained-browser-runtime'
    ? pages[last].prev : last;
}
