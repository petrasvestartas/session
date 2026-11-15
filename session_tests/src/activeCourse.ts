export interface ActiveSubject {
  id: string;
  title: string;
  page: string;
  prerequisites: string[];
  chapters: string[];
  acceptance: 'pending';
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
  const seen = new Set<string>(), pages = new Set<string>();
  for (const subject of value.subjects) {
    if (!/^[a-z]+$/.test(subject.id) || seen.has(subject.id) || subject.acceptance !== 'pending')
      throw Error(`Invalid or duplicate subject: ${subject.id}`);
    if (subject.prerequisites.some(id => !seen.has(id))) throw Error(`Unmet prerequisite: ${subject.id}`);
    for (const page of [subject.page, ...subject.chapters]) {
      if (!/^journey\/[a-z-]+\.md$/.test(page) || pages.has(page)) throw Error(`Invalid or duplicate page: ${page}`);
      pages.add(page);
    }
    seen.add(subject.id);
  }
  return value;
}
