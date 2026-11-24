const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const course = JSON.parse(fs.readFileSync(path.join(__dirname, '../docs/journey/active-course.json'), 'utf8'));

async function verifyCourseHome(page, base) {
    const items = course.tutorials || course.subjects;
    await page.goto(base + '#/course');
    const home = page.locator('.course-index');
    await home.waitFor();
    assert((await home.locator('.lead').innerText()).includes(`${course.subjects.length} cumulative checkpoints`));
    assert.equal(await home.locator(':scope > details').count(), 0);
    assert.equal(await home.locator('.tutorial-list > .tutorial-item').count(), items.length);
    const totalTutorialLinks = items.reduce((sum, s) => sum + 1 + s.chapters.length, 0);
    assert.equal(await home.locator('.tutorial-list a.tutorial-link').count(), totalTutorialLinks);

    const renderingItem = home.locator('.tutorial-list > .tutorial-item').nth(2);
    const link = renderingItem.locator('.chapter-list a.chapter-link').filter({hasText: '2D mathematics'});
    assert((await link.getAttribute('href')).endsWith('/course/journey/tutorial-three-mathematics'));
    await link.click();
    await page.waitForURL('**/journey/tutorial-three-mathematics');
    assert((await page.locator('.pager .prev').getAttribute('href')).endsWith('/journey/tutorial-three-painting'));
    assert((await page.locator('.pager .next').getAttribute('href')).endsWith('/journey/tutorial-four-geometry'));

    await page.goto(base + '#/course/journey/tutorial-one-startup');
    await page.locator('article.doc h1').waitFor();
    assert.equal(await page.locator('.pager .prev').count(), 0);
    assert((await page.locator('.pager .next').getAttribute('href')).endsWith('/journey/tutorial-two-wgpu'));

    await page.goto(base + '#/course');
    return {
        checkpoints: course.subjects.length,
        tutorials: totalTutorialLinks,
        chapterOwner: '03-rendering',
        navigation: 'painting/mathematics/geometry',
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
