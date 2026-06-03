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

export async function ensureTestData(): Promise<unknown> {
  if (typeof (window as any).TEST_DATA === 'undefined') await injectOnce('testData.js');
  return (window as any).TEST_DATA;
}
