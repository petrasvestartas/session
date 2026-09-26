declare module 'virtual:course' {
  export const groups: { title: string; slugs: string[] }[];
  export const pages: Record<string, { title: string; prev?: string; next?: string }>;
  export const loaders: Record<string, () => Promise<{ default: CoursePage }>>;
  export interface CoursePage {
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
