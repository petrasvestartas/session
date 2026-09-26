// Checks a built docs site in headless Chrome: every course page renders, every internal link and
// anchor resolves, images and files load, search answers, the kernel deep link lands, the phone
// layout has no sideways scroll, and nothing logs an error. Prints a summary; exit 1 on failure.
//
//   npx vite build && (serve dist so it answers at <url>)
//   NODE_PATH=<dir with playwright> node scripts/check-site.mjs http://localhost:8795/session/docs/ [shots dir]
import { createRequire } from 'node:module';

// require() honours NODE_PATH, so playwright can live outside this project.
const { chromium } = createRequire(import.meta.url)('playwright');

const root = process.argv[2] || 'http://localhost:8795/session/docs/';
const shots = process.argv[3] || '';
const problems = [];
const consoleErrors = [];

const browser = await chromium.launch({ headless: true, channel: process.env.PW_CHANNEL || 'chrome' });
const ctx = await browser.newContext({ viewport: { width: 1400, height: 900 } });
const page = await ctx.newPage();
page.on('console', (m) => {
  if (m.type() === 'error' || m.type() === 'warning') consoleErrors.push(`${page.url()} ${m.type()}: ${m.text()}`);
});
page.on('pageerror', (e) => consoleErrors.push(`${page.url()} pageerror: ${e.message}`));
page.on('response', (r) => {
  if (r.status() >= 400) consoleErrors.push(`${r.status()} ${r.url()}`);
});

const t0 = Date.now();
await page.goto(root, { waitUntil: 'load' });
await page.waitForSelector('.home');
const paint = await page.evaluate(() => {
  const fcp = performance.getEntriesByName('first-contentful-paint')[0];
  const nav = performance.getEntriesByType('navigation')[0];
  return { fcp: fcp ? Math.round(fcp.startTime) : null, load: nav ? Math.round(nav.loadEventEnd) : null };
});
const cornerHref = await page.getAttribute('.viewer-corner', 'href');

// Course index: the entry list, then a crawl over every page it or a page links to.
await page.goto(root + '#/course');
await page.waitForSelector('.course-index a');
const listed = await page.$$eval('.course-index a', (as) => as.map((a) => ({ slug: a.getAttribute('href').replace(/^#\/course\//, ''), title: a.textContent.trim() })));
const titles = new Map(listed.map((p) => [p.slug, p.title]));
const queue = listed.map((p) => p.slug);
const seen = new Set(queue);
const ids = new Map();
const links = [];
const assets = new Set();

while (queue.length) {
  const slug = queue.shift();
  await page.goto(`${root}#/course/${slug}`);
  try {
    await page.waitForFunction(
      (s) => location.hash.startsWith('#/course/' + s) && document.querySelector('.doc h1, .missing'),
      slug,
    );
    await page.waitForFunction((t) => !t || document.querySelector('.doc h1')?.textContent.replace(/#$/, '').trim() === t, titles.get(slug) || '');
  } catch {
    problems.push(`page did not render: ${slug}`);
    continue;
  }
  const info = await page.evaluate(() => {
    const doc = document.querySelector('.doc');
    return {
      ids: [...doc.querySelectorAll('[id]')].map((e) => e.id),
      hrefs: [...doc.querySelectorAll('a[href]')].map((a) => a.getAttribute('href')),
      imgs: [...doc.querySelectorAll('img')].map((i) => i.getAttribute('src')),
      text: doc.textContent.length,
    };
  });
  if (info.text < 50) problems.push(`page is empty: ${slug}`);
  ids.set(slug, new Set(info.ids));
  for (const h of info.hrefs) {
    links.push({ from: slug, href: h });
    const m = h.match(/^#\/course\/([^#?]+)/);
    if (m && !seen.has(m[1])) {
      seen.add(m[1]);
      queue.push(m[1]);
    }
    if (!h.startsWith('#') && !/^[a-z]+:/i.test(h)) assets.add(h);
  }
  for (const s of info.imgs) assets.add(s);
}

// Links: course routes and anchors, kernel deep links, then every asset over HTTP.
let internal = 0;
for (const { from, href } of links) {
  const c = href.match(/^#\/course\/([^#?]+)(?:#(.+))?$/);
  if (c) {
    internal++;
    if (!ids.has(c[1])) problems.push(`${from}: link to missing page ${href}`);
    else if (c[2] && !ids.get(c[1]).has(decodeURIComponent(c[2]))) problems.push(`${from}: missing anchor ${href}`);
  } else if (href.startsWith('#') && !href.startsWith('#/tests') && !href.startsWith('#/install') && href !== '#/') {
    problems.push(`${from}: unrouted link ${href}`);
  }
}
for (const a of assets) {
  const url = new URL(a, root).href;
  const r = await page.request.get(url);
  if (r.status() !== 200) problems.push(`asset ${r.status()}: ${a}`);
}

// Search: opens with "/", loads the index lazily, finds course prose and a kernel test.
const searchHits = {};
for (const q of ['bind group', process.env.CHECK_TEST || 'Serialization Errors']) {
  await page.goto(root + '#/course');
  await page.waitForSelector('.course-index');
  await page.keyboard.press('/');
  await page.waitForSelector('.search-panel input');
  await page.fill('.search-panel input', q);
  try {
    await page.waitForSelector('.results a', { timeout: 5000 });
  } catch {
    problems.push(`search found nothing for "${q}"`);
    continue;
  }
  const hits = await page.$$eval('.results a', (as) => as.map((a) => a.getAttribute('href') + ' | ' + a.textContent.trim()));
  searchHits[q] = hits.slice(0, 3);
  await page.keyboard.press('Escape');
}

// Kernel deep link: the test row ends up in view and the class line shows.
const deep = await (async () => {
  await page.goto(root + '#/tests?suite=mesh_test&test=' + encodeURIComponent('Protobuf Roundtrip'));
  await page.waitForFunction(() => document.getElementById('test-Protobuf Roundtrip'), null, { timeout: 15000 });
  await page.waitForTimeout(800);
  return page.evaluate(() => {
    const r = document.getElementById('test-Protobuf Roundtrip').getBoundingClientRect();
    return { top: Math.round(r.top), about: document.querySelector('.class-about')?.textContent.trim() };
  });
})().catch((e) => ({ error: String(e) }));
if (deep.error || deep.top < 0 || deep.top > 400 || !deep.about) problems.push(`kernel deep link: ${JSON.stringify(deep)}`);

// Phone: 390 px wide, touch; no sideways scroll on the home, a lesson and the kernel page.
const phone = await browser.newContext({ viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true, deviceScaleFactor: 2 });
const pp = await phone.newPage();
pp.on('pageerror', (e) => consoleErrors.push(`phone pageerror: ${e.message}`));
const phoneWidths = {};
for (const route of ['', '#/course/12-picking', '#/tests?suite=point_test']) {
  await pp.goto(root + route);
  await pp.waitForTimeout(1200);
  phoneWidths[route || 'home'] = await pp.evaluate(() => [document.documentElement.scrollWidth, window.innerWidth]);
  const [sw, iw] = phoneWidths[route || 'home'];
  if (sw > iw) problems.push(`phone: sideways scroll on ${route || 'home'} (${sw} > ${iw})`);
  if (shots) await pp.screenshot({ path: `${shots}/phone-${(route || 'home').replace(/[^a-z0-9]+/gi, '_')}.png` });
}
if (shots) {
  await page.goto(root + '#/course/12-picking');
  await page.waitForSelector('.doc h1');
  await page.waitForTimeout(500);
  await page.screenshot({ path: `${shots}/desktop-12-picking.png` });
}
await browser.close();

console.log(JSON.stringify({
  pages: ids.size,
  listed: listed.length,
  links: links.length,
  internalLinks: internal,
  assets: assets.size,
  firstPaintMs: paint.fcp,
  loadMs: paint.load,
  corner: cornerHref,
  search: searchHits,
  deepLink: deep,
  phone: phoneWidths,
  consoleErrors,
  problems,
  seconds: Math.round((Date.now() - t0) / 1000),
}, null, 1));
process.exit(problems.length || consoleErrors.length ? 1 : 0);
