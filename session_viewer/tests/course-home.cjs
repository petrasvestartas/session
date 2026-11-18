const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const course = JSON.parse(fs.readFileSync(path.join(__dirname, '../docs/journey/active-course.json'), 'utf8'));

async function verifyCourseHome(page, base) {
    await page.goto(base + '#/course');
    const home = page.locator('.course-index');
    await home.waitFor();
    assert((await home.locator('.lead').innerText()).includes(`${course.subjects.length} cumulative checkpoints`));
    assert.equal(await home.locator(':scope > details').count(), 0);
    assert.equal(await home.locator('.tutorial-list > .tutorial-item').count(), course.subjects.length);
    const totalTutorialLinks = course.subjects.reduce((sum, s) => sum + 1 + s.chapters.length, 0);
    assert.equal(await home.locator('.tutorial-list a.tutorial-link').count(), totalTutorialLinks);

    const geometryItem = home.locator('.tutorial-list > .tutorial-item').nth(6);
    const link = geometryItem.locator('.chapter-list a.chapter-link').filter({hasText: 'Decide which ink is visible'});
    assert((await link.getAttribute('href')).endsWith('/course/journey/completion-visibility'));
    await link.click();
    await page.waitForURL('**/journey/completion-visibility');
    assert((await page.locator('.pager .prev').getAttribute('href')).endsWith('/journey/completion-geometry'));
    assert((await page.locator('.pager .next').getAttribute('href')).endsWith('/journey/completion-decoding'));

    await page.goto(base + '#/course/journey/completion-frame');
    await page.locator('article.doc h1').waitFor();
    assert.equal(await page.locator('.pager .prev').count(), 0);
    assert((await page.locator('.pager .next').getAttribute('href')).endsWith('/journey/completion-painting'));

    await page.goto(base + '#/course');
    return {
        checkpoints: course.subjects.length,
        tutorials: totalTutorialLinks,
        chapterOwner: 'geometry',
        navigation: 'geometry/visibility/decoding',
    };
}

module.exports = {verifyCourseHome};

if (require.main === module) (async () => {
    const {chromium} = require('playwright');
    const browser = await chromium.launch({executablePath: process.env.CHROMIUM, headless: true});
    try {
        for (const width of [1280, 390]) {
            const page = await browser.newPage({viewport: {width, height: 844}});
            console.log(JSON.stringify({width, ...await verifyCourseHome(page, process.argv[2])}));
            await page.close();
        }
    } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
