declare module 'virtual:course' {
  export const checkpointCount: number;
  export const tutorials: CourseTutorial[];
  export const groups: { title: string; slugs: string[]; sections?: { title: string; slugs: string[] }[] }[];
  export const pages: Record<string, { title: string; prev?: string; next?: string }>;
  export const loaders: Record<string, () => Promise<{ default: CoursePage }>>;
  export interface CourseChapter {
    number: string;
    title: string;
    slug: string;
    heroImg?: string;
    summary?: string;
    fileCount?: number;
    stageQuery?: string;
  }
  export interface CourseTutorial {
    number: string;
    id: string;
    title: string;
    slug: string;
    heroImg?: string;
    summary?: string;
    fileCount?: number;
    stageQuery?: string;
    chapters: CourseChapter[];
  }
  export interface CoursePage {
    redirect?: string;
    title: string;
    html: string;
    toc: { level: number; id: string; text: string }[];
  }
}
declare module 'virtual:search-index' {
  const json: string;
  export default json;
}
declare module 'virtual:kernel' {
  const classes: Record<string, { label: string; description: string }>;
  export default classes;
}
