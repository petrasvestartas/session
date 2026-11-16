export interface ActiveSubject {
  id: string;
  title: string;
  page: string;
  prerequisites: string[];
  chapters: string[];
  acceptance: 'pending' | 'accepted';
  review?: {path: string; sha256: string};
}
export interface ActiveCourse {
  version: 1;
  status: 'incomplete';
  historicalManifest: string;
  subjects: ActiveSubject[];
}

export function activeCourse(value: ActiveCourse): ActiveCourse {
  if (value.version !== 1 || value.status !== 'incomplete' || !value.subjects.length)
    throw Error('Invalid active course');
  const seen = new Set<string>(), pages = new Set<string>(), accepted = new Set<string>();
  for (const subject of value.subjects) {
    if (!/^[a-z]+$/.test(subject.id) || seen.has(subject.id)
        || !['pending', 'accepted'].includes(subject.acceptance))
      throw Error(`Invalid or duplicate subject: ${subject.id}`);
    if (subject.prerequisites.some(id => !seen.has(id))) throw Error(`Unmet prerequisite: ${subject.id}`);
    if (subject.acceptance === 'accepted') {
      if (!subject.review || !/^journey\/release\/[a-z0-9-]+\.json$/.test(subject.review.path)
          || !/^[a-f0-9]{64}$/.test(subject.review.sha256)
          || subject.prerequisites.some(id => !accepted.has(id)))
        throw Error(`Missing review or accepted prerequisite: ${subject.id}`);
      accepted.add(subject.id);
    } else if (subject.review) throw Error(`Pending subject has an acceptance review: ${subject.id}`);
    for (const page of [subject.page, ...subject.chapters]) {
      if (!/^journey\/[a-z-]+\.md$/.test(page) || pages.has(page)) throw Error(`Invalid or duplicate page: ${page}`);
      pages.add(page);
    }
    seen.add(subject.id);
  }
  return value;
}
