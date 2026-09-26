// Lazily load the big pre-generated test data global (testData.js ≈3.5MB) only when a page that
// needs it is opened — instead of on every page via index.html. Keeps the Viewer tab from pulling
// data it never uses.

const base = import.meta.env.BASE_URL || '/';

function injectOnce(file: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const url = base + file;
    if ([...document.scripts].some((s) => s.src.endsWith(file))) { resolve(); return; }
    const s = document.createElement('script');
    s.src = url;
    s.onload = () => resolve();
    s.onerror = () => reject(new Error('failed to load ' + url));
    document.head.appendChild(s);
  });
}

// Missing or broken results leave an empty data set, so the Kernel API page shows its empty state.
export async function ensureTestData(): Promise<unknown> {
  if (typeof (window as any).TEST_DATA === 'undefined') {
    try {
      await injectOnce('testData.js');
    } catch (e) {
      console.warn((e as Error).message);
    }
  }
  if (typeof (window as any).TEST_DATA === 'undefined') (window as any).TEST_DATA = {};
  return (window as any).TEST_DATA;
}
