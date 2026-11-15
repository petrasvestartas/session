const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');

(async () => {
    const [base, output] = process.argv.slice(2);
    assert(base && output, 'Choose a served documentation URL and a new output directory');
    fs.mkdirSync(output);
    const docs = path.resolve(__dirname, '../../../session_viewer/docs');
    const route = JSON.parse(fs.readFileSync(path.join(docs, 'journey/active-course.json'), 'utf8'));
    const illustrations = JSON.parse(fs.readFileSync(path.join(docs, 'screenshots/completion/evidence.json'), 'utf8'));
    const browser = await chromium.launch({executablePath: process.env.CHROMIUM, headless: true});
    const errors = [], records = [], images = [];
    let stamp;
    try {
        for (const viewport of [{width:1280,height:800},{width:390,height:844}]) {
            const page = await browser.newPage({viewport});
            page.on('pageerror', error => errors.push(String(error)));
            const open = async slug => {
                await page.goto(base + '#/course/' + slug);
                await page.locator('article.doc h1').waitFor();
            };
            const footer = async () => {
                const element = page.locator('.docs-build');
                const time = await element.locator('time').getAttribute('datetime');
                assert(Number.isFinite(Date.parse(time)));
                if (process.env.DOCS_BUILD_STARTED) assert(Date.parse(time) >= Number(process.env.DOCS_BUILD_STARTED) && Date.parse(time) <= Number(process.env.DOCS_BUILD_ENDED));
                if (process.env.DOCS_EXPECTED_STAMP) assert.equal(time, process.env.DOCS_EXPECTED_STAMP);
                if (stamp) assert.equal(time, stamp); else stamp = time;
                const box = await element.boundingBox();
                assert(box.x >= 0 && box.x + box.width <= viewport.width);
                assert(box.y >= viewport.height - 40 && box.y + box.height <= viewport.height);
                await page.locator('#content').evaluate(node => { node.scrollTop = node.scrollHeight; });
                assert.deepEqual(await element.boundingBox(), box);
            };
            await open('journey/active');
            assert.equal(await page.locator('article.doc tbody tr').count(), route.subjects.length);
            await footer();
            await page.locator('article.doc h1').scrollIntoViewIfNeeded();
            await page.screenshot({path:path.join(output, `active-${viewport.width}.png`)});
            await page.locator('article.doc').getByRole('link', {name:route.subjects[0].title, exact:true}).click();
            for (const [index, subject] of route.subjects.entries()) {
                await page.locator('article.doc').getByRole('heading', {name:subject.title, exact:true}).waitFor();
                assert(page.url().includes(subject.page.replace(/\.md$/, '')));
                assert((await page.locator('article.doc').innerText()).includes('Question and answer'));
                await footer();
                records.push({subject:subject.id, viewport, stamp});
                if (subject.id === 'mathematics') {
                    await page.getByRole('heading', {name:'One schema, two hosts', exact:true}).scrollIntoViewIfNeeded();
                    await page.screenshot({path:path.join(output, `mathematics-${viewport.width}.png`)});
                }
                if (index + 1 < route.subjects.length) await page.locator('nav.pager a.next').click();
            }
            for (const lesson of [...new Set(illustrations.assets.map(asset => asset.proposedLesson))]) {
                await open('journey/' + lesson.replace(/\.md$/, ''));
                for (const asset of illustrations.assets.filter(asset => asset.proposedLesson === lesson)) {
                    const img = page.locator('article.doc').getByRole('img', {name:asset.caption, exact:true});
                    await img.scrollIntoViewIfNeeded();
                    await img.evaluate(element => element.decode());
                    const source = await img.getAttribute('src');
                    const response = await page.request.get(new URL(source, page.url()).href);
                    assert.equal(response.status(), 200);
                    assert.equal(crypto.createHash('sha256').update(await response.body()).digest('hex'), asset.sha256);
                    images.push({asset:asset.asset, viewport, sha256:asset.sha256});
                }
                await footer();
                if (lesson === 'completion-text.md') {
                    await page.locator('article.doc').getByRole('img').first().scrollIntoViewIfNeeded();
                    await page.screenshot({path:path.join(output, `text-${viewport.width}.png`)});
                }
            }
            await open('journey/01-canvas');
            await page.getByText('Historical reconstruction: its source, links and checks are preserved.', {exact:false}).waitFor();
            await footer();
            await page.locator('article.doc h1').scrollIntoViewIfNeeded();
            await page.screenshot({path:path.join(output, `historical-${viewport.width}.png`)});
            await page.close();
        }
        assert.deepEqual(errors, []);
        fs.writeFileSync(path.join(output, 'result.json'), JSON.stringify({passed:true, scope:'Active route navigation, historical links, original illustration delivery and fixed build timestamp; no viewer or complete-course acceptance', stamp, records, images, errors}, null, 2));
    } catch (error) {
        fs.writeFileSync(path.join(output, 'failure.json'), JSON.stringify({error:String(error.stack), records, images, errors}, null, 2));
        throw error;
    } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
