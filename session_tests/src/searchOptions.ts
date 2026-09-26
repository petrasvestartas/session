// MiniSearch options shared by the build (index) and the page (loadJSON); both sides must match.
// Common English words, numbers and single letters are left out: they match everything and
// would make up a third of the index.
const STOP = new Set(
  'a an and are as at be but by can do does for from has have if in into is it its not of on one or so that the their them then there these this to two was we when where which while will with you your'.split(' '),
);

export const SEARCH_OPTIONS = {
  fields: ['title', 'text'],
  storeFields: ['title', 'page', 'route'],
  processTerm: (term: string) => {
    const t = term.toLowerCase();
    return t.length < 2 || STOP.has(t) || /^\d+$/.test(t) ? null : t;
  },
};
