const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {chromium} = require(process.env.PLAYWRIGHT_MODULE || 'playwright');

const root = process.argv[2] || 'http://127.0.0.1:8788/docs/';
const output = path.resolve(process.argv[3] || '../session_viewer/target/course-docs-browser');
const journey = path.resolve(__dirname, '../../session_viewer/docs/journey');
const book = JSON.parse(fs.readFileSync(path.join(journey, 'book.json'), 'utf8'));
const lessons = book.chapters.flatMap(chapter => chapter.consolidated || []);
const changed = book.chapters.slice(18).flatMap(chapter => chapter.consolidated || []);
const headings = new Map(book.chapters.flatMap((chapter, c) => (chapter.consolidated || []).map((lesson, l) =>
    [lesson.page.replace(/\.md$/, ''), `${c + 1}.${l + 1} \u00b7 `])));

async function main() {
    fs.mkdirSync(output, {recursive: true});
    assert.equal(book.chapters.length, 22);
    assert.equal(lessons.length, 111);
    const browser = await chromium.launch({headless: true});
    const context = await browser.newContext({viewport: {width: 1400, height: 900}, deviceScaleFactor: 1});
    const page = await context.newPage();
    const errors = [];
    const captures = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('response', response => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
    const visit = async (slug, name) => {
        await page.goto(`${root}#/course${slug ? '/' + slug : ''}`, {waitUntil: 'networkidle'});
        await page.waitForSelector(slug ? '.doc h1' : '.course-index');
        if (slug) {
            const prefix = headings.get(slug) || (slug === 'journey/contents' ? 'Course contents'
                : slug === 'journey/daily-plan' ? 'Plan your coding sessions' : headings.get(changed.at(-1).page.replace(/\.md$/, '')));
            await page.waitForFunction(prefix => document.querySelector('.doc h1')?.textContent.startsWith(prefix), prefix);
        }
        await page.evaluate(() => document.fonts.ready);
        await page.locator('.doc img').evaluateAll(async images => {
            for (const image of images) image.loading = 'eager';
            await Promise.all(images.map(image => image.decode()));
        });
        const overflow = await page.evaluate(() => document.documentElement.scrollWidth > innerWidth + 1);
        assert(!overflow, `${slug}: page overflows horizontally`);
        const file = name + '.png';
        await page.screenshot({path: path.join(output, file)});
        captures.push({slug, file, title: await page.locator(slug ? '.doc h1' : 'h1').first().innerText()});
        console.log(`Checked ${slug || 'course index'}`);
    };
    try {
        await visit('', 'index');
        const intro = await page.locator('.course-index .lead').innerText();
        assert.match(intro, /22 of 22/);
        assert.match(intro, /111 reader-facing lessons/);
        for (const lesson of lessons) {
            assert(await page.locator(`.course-index a[href="#/course/${lesson.page.replace(/\.md$/, '')}"]`).count(), lesson.id);
        }
        for (const slug of ['journey/contents', 'journey/daily-plan']) await visit(slug, path.basename(slug));
        for (const [index, lesson] of changed.entries()) {
            const slug = lesson.page.replace(/\.md$/, '');
            await visit(slug, `lesson-${index + 1}`);
            assert.match(await page.locator('.doc h1').innerText(), /(?:19|20|21|22)\.[1-6]/);
            assert(await page.locator(`[id="checkpoint-${lesson.checkpoint}"]`).count(), `${lesson.id}: endpoint missing`);
            const implementation = page.locator('.doc details').first();
            await implementation.locator('summary').first().click();
            assert(await implementation.getAttribute('open') !== null, `${lesson.id}: implementation does not expand`);
        }
        const last = changed.at(-1);
        await visit('journey/' + last.checkpoint, 'legacy-alias');
        assert(page.url().includes('checkpoint-' + last.checkpoint), 'Legacy URL did not retain its checkpoint anchor');
        await page.setViewportSize({width: 390, height: 844});
        await page.waitForFunction(() => document.querySelector('.main-layout.phone')
            && getComputedStyle(document.querySelector('#site-nav')).visibility === 'hidden');
        await visit(last.page.replace(/\.md$/, ''), 'phone');
        assert.deepEqual(errors, []);
        fs.writeFileSync(path.join(output, 'inspection.json'), JSON.stringify({
            checkedAt: new Date().toISOString(), root, browser: browser.version(),
            chapters: 22, lessons: 111, changedPagesChecked: changed.length,
            errors, captures, visualReview: 'pending individual image inspection',
            scope: 'Documentation UI only; no viewer runtime or historical GPU evidence accepted'
        }, null, 2) + '\n');
    } finally {
        await context.close();
        await browser.close();
    }
}

main().catch(error => { console.error(error); process.exitCode = 1; });
