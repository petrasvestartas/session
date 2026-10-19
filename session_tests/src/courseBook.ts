type Step = { id: string; title: string; page: string };
export type ReaderLesson = Step & { steps: string[]; checkpoint: string };
export type BookPlan = { chapters: { title: string; sections: { title: string; first: string }[]; consolidated?: ReaderLesson[] }[]; comparison: string };
export type BookLesson = Step & { number: string; chapter: string; section: string };
export type BookSection = { number: string; title: string; lessons: BookLesson[] };
export type BookChapter = { number: number; title: string; sections: BookSection[] };

export function courseBook(steps: Step[], plan: BookPlan) {
  const lessons = steps.filter(step => step.id !== plan.comparison);
  const byId = new Map<string, BookLesson>();
  const sections = plan.chapters.flatMap(chapter => chapter.sections);
  const starts = sections.map(section => lessons.findIndex(step => step.id === section.first));
  if (starts[0] !== 0 || starts.some((at, i) => at < 0 || (i > 0 && at <= starts[i - 1]))) {
    throw Error('Book sections must start at distinct lessons in course order');
  }
  let sectionIndex = 0;
  const chapters: BookChapter[] = plan.chapters.map((chapter, c) => ({
    number: c + 1, title: chapter.title,
    sections: chapter.sections.map((section, s) => {
      const at = sectionIndex++;
      const number = `${c + 1}.${s + 1}`;
      return {number, title: section.title, lessons: lessons.slice(starts[at], starts[at + 1] ?? lessons.length).map((step, l) => {
        const lesson = {...step, number: `${number}.${l + 1}`, chapter: `${c + 1} · ${chapter.title}`, section: `${number} · ${section.title}`};
        byId.set(step.id, lesson);
        return lesson;
      })};
    }),
  }));
  return {chapters, byId, comparison: steps.find(step => step.id === plan.comparison)};
}

export function readingBook(steps: Step[], plan: BookPlan) {
  const book = courseBook(steps, plan);
  const aliases = new Map<string, {page: string; anchor: string}>();
  for (const [index, chapter] of book.chapters.entries()) {
    const merged = plan.chapters[index].consolidated;
    if (!merged) continue;
    const original = chapter.sections.flatMap(section => section.lessons.map(lesson => lesson.id));
    const retained = merged.flatMap(lesson => lesson.steps);
    if (JSON.stringify(original) !== JSON.stringify(retained)
        || merged.some(lesson => lesson.id !== lesson.steps[0] || lesson.checkpoint !== lesson.steps.at(-1))) {
      throw Error('Consolidated lessons must retain every checkpoint once in prerequisite order and end at their runnable checkpoint');
    }
    chapter.sections = merged.map((lesson, i) => {
      const number = `${chapter.number}.${i + 1}`;
      for (const id of lesson.steps) aliases.set(id, {page: lesson.page, anchor: `checkpoint-${id}`});
      return {number, title: lesson.title, lessons: [{...lesson, number,
        chapter: `${chapter.number} · ${chapter.title}`, section: `${number} · ${lesson.title}`} ]};
    });
  }
  const lessons = book.chapters.flatMap(chapter => chapter.sections.flatMap(section => section.lessons));
  return {...book, byId: new Map(lessons.map(lesson => [lesson.id, lesson])), aliases};
}
