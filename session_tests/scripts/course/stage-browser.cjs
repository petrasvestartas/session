const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');

const [current, baseline, deck, chapterText, orientation, destination] = process.argv.slice(2);
const chapter = Number(chapterText);
const baselineCommit = process.env.STAGE_BASELINE_COMMIT;
assert(/^[a-f0-9]{40}$/.test(baselineCommit || ''), 'Pin the deployed baseline with STAGE_BASELINE_COMMIT');
assert(current && baseline && /^[\w-]+$/.test(deck) && Number.isInteger(chapter) && chapter > 0
    && ['desktop', 'upright'].includes(orientation) && destination,
    'Choose CURRENT_URL BASELINE_URL DECK CHAPTER desktop|upright OUTPUT_DIRECTORY');
const output = path.resolve(destination);
const hash = data => crypto.createHash('sha256').update(data).digest('hex');
const viewport = orientation === 'desktop' ? {width:1280,height:720} : {width:390,height:844};
const args = JSON.parse(process.env.STAGE_BROWSER_ARGS || '["--enable-unsafe-webgpu","--ignore-gpu-blocklist","--use-angle=d3d11"]');
assert(Array.isArray(args) && args.every(x => typeof x === 'string'));
const fixedClock = process.env.STAGE_FIXED_CLOCK === '1';
const saveAll = process.env.STAGE_SAVE_ALL === '1';
const checker = hash(fs.readFileSync(__filename));

async function main() {
    fs.mkdirSync(output, {recursive:true});
    const browser = await chromium.launch({executablePath:process.env.CHROMIUM || 'C:/Program Files/Google/Chrome/Application/chrome.exe', headless:true, args});
    const context = await browser.newContext({viewport, deviceScaleFactor:1, hasTouch:orientation === 'upright'});
    const pages = [], errors = [], frames = [], bundles = [], downloads = [], adapters = [], roundTripFailures = [];
    let comparisonComplete = false;
    function roundTrip(actual,expected,message) {
        try { assert.equal(actual,expected,message); }
        catch (error) { roundTripFailures.push({message,actual,expected}); }
    }
    const state = page => page.locator('#canvas').getAttribute('data-stage').then(JSON.parse);
    async function wait(page, step) {
        await page.bringToFront();
        if (fixedClock) {
            const deadline = Date.now() + 60000;
            while (Date.now() < deadline) {
                const settled = await page.evaluate(async ({chapter,step}) => {
                    window.__stageNow += 16;
                    await new Promise(requestAnimationFrame);
                    await Promise.all(window.__stageDevices.map(device => device.queue.onSubmittedWorkDone()));
                    await new Promise(requestAnimationFrame);
                    const text = document.querySelector('#canvas')?.getAttribute('data-stage');
                    if (!text) return false;
                    const s = JSON.parse(text);
                    return s.chapter === chapter - 1 && s.step === step - 1 && !s.animating;
                }, {chapter,step});
                if (settled) break;
            }
        }
        await page.waitForFunction(({chapter, step}) => {
            const text = document.querySelector('#canvas')?.getAttribute('data-stage');
            if (!text) return false;
            const s = JSON.parse(text);
            return s.chapter === chapter - 1 && s.step === step - 1 && !s.animating;
        }, {chapter, step}, {timeout:60000});
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
        const s = await state(page);
        assert.deepEqual(s.errors, [], 'Deck reports contextual errors');
        assert.deepEqual(s.overlaps, [], 'Settled deck objects overlap');
        s.items.sort((a,b) => a.id.localeCompare(b.id));
        return s;
    }
    async function picture(page) {
        const bytes = await page.locator('#canvas').screenshot();
        const png = PNG.sync.read(bytes);
        assert.equal(png.width, viewport.width);
        assert.equal(png.height, viewport.height);
        let ink = 0;
        for (let i=0;i<png.data.length;i+=4)
            if (Math.max(...[0,1,2].map(c => Math.abs(png.data[i+c]-png.data[c]))) > 30) ink++;
        assert(ink > 500, `Blank stage: ${ink} non-background pixels`);
        return {bytes, pixels:hash(png.data), sha256:hash(bytes), ink};
    }
    async function compare(label, step, save=false) {
        const pair = [];
        for (const entry of pages) {
            const s = await wait(entry.page, step);
            const image = await picture(entry.page);
            if (save || saveAll) fs.writeFileSync(path.join(output, `${label}-${entry.name}.png`), image.bytes);
            pair.push({state:s, pixels:image.pixels, sha256:image.sha256, ink:image.ink});
        }
        frames.push({label, step, pair});
        assert.deepEqual(pair[1].state, pair[0].state, `${label}: published stage state differs`);
        assert.equal(pair[1].pixels, pair[0].pixels, `${label}: rendered pixels differ from the published baseline`);
        return pair[1].pixels;
    }
    async function key(value, step) {
        for (const {page} of pages) {
            await page.bringToFront();
            await page.keyboard.press(value);
            if (step !== undefined) await wait(page,step);
        }
    }
    try {
        for (const [name, base] of [['baseline',baseline],['current',current]]) {
            const page = await context.newPage();
            page.on('pageerror', e => errors.push({name,message:e.message}));
            page.on('console', message => {if(message.type()==='error') errors.push({name,message:message.text()});});
            page.on('response', response => {
                if (/\.(wasm|js)(\?|$)/.test(response.url())) downloads.push(response.body().then(bytes =>
                    bundles.push({name,url:response.url(),bytes:bytes.length,sha256:hash(bytes)})));
            });
            await page.addInitScript(fixedClock => {
                window.__stageAdapters = [];
                window.__stageGpuErrors = [];
                window.__stageDevices = [];
                window.__stageNow = 1000;
                if (fixedClock) performance.now = () => window.__stageNow;
                const requestDevice = GPUAdapter.prototype.requestDevice;
                GPUAdapter.prototype.requestDevice = async function(...args) {
                    const device = await requestDevice.apply(this,args);
                    window.__stageDevices.push(device);
                    device.addEventListener('uncapturederror', event => window.__stageGpuErrors.push(event.error.message));
                    device.lost.then(info => window.__stageGpuErrors.push(`Device lost: ${info.reason}: ${info.message}`));
                    return device;
                };
                const request = navigator.gpu.requestAdapter.bind(navigator.gpu);
                navigator.gpu.requestAdapter = async options => {
                    const adapter = await request(options);
                    if (adapter) {
                        const {vendor,architecture,device,description} = adapter.info;
                        window.__stageAdapters.push({options:options || {},vendor,architecture,device,description});
                    }
                    return adapter;
                };
            },fixedClock);
            const url = new URL(base);
            url.search = new URLSearchParams({stage:`decks/${deck}/index.yaml`,data:'off',chapter:String(chapter),step:'1'}).toString();
            pages.push({name,page,url:String(url)});
            await page.goto(String(url));
            await wait(page,1);
        }
        const total = (await state(pages[1].page)).steps;
        assert(total > 0);
        const first = await compare('first',1,true);
        for (let step=2;step<=total;step++) {
            await key('ArrowRight',step);
            await compare(`step-${step}`,step,step===total);
            console.log(`${deck}/${chapter}/${orientation}: ${step}/${total}`);
        }
        if (total > 1) {
            await key('ArrowLeft',total-1);
            const previous = await compare('previous',total-1,true);
            roundTrip(previous, frames[total-2].pair[1].pixels, 'Backward navigation does not reconstruct the same settled frame');
        }
        const selected = Math.max(1,total-1);
        const beforeTheme = frames.at(-1).pair[1].pixels;
        await key('t');
        const dark = await compare('dark',selected,true);
        assert.notEqual(dark,beforeTheme,'Theme has no visible effect');
        await key('t');
        roundTrip(await compare('light-restored',selected,true),beforeTheme,'Theme round trip changes content');
        if (total > 1) {
            for (const {page} of pages) {
                await page.bringToFront();
                if (orientation === 'upright') await page.touchscreen.tap(viewport.width * 0.8,viewport.height * 0.8);
                else await page.mouse.click(viewport.width * 0.8,viewport.height * 0.5);
                await wait(page,total);
            }
            const final = await compare(orientation === 'upright' ? 'touch-step' : 'mouse-step',total,true);
            roundTrip(final,frames[total-1].pair[1].pixels,'Pointer stepping differs from keyboard stepping');
        }
        if (deck === 'force-density-matrices' && chapter === 7) {
            const original = frames.at(-1).pair[1].pixels;
            const x = viewport.width * (orientation === 'upright' ? 0.5 : 0.25);
            const y = viewport.height * (orientation === 'upright' ? 0.22 : 0.5);
            for (const {page} of pages) {
                await page.bringToFront();
                await page.mouse.move(x,y); await page.mouse.down();
                await page.mouse.move(x+40,y+20,{steps:5}); await page.mouse.up();
            }
            assert.notEqual(await compare('orbit',total,true),original,'Scene orbit has no visible effect');
            await key('r');
            roundTrip(await compare('camera-restored',total,true),original,'Authored camera reset changes the original frame');
        }
        await Promise.all(downloads);
        assert.deepEqual(errors,[]);
        for (const {name,page,url} of pages) {
            assert.deepEqual(await page.evaluate(() => window.__stageGpuErrors),[], `${name}: WebGPU errors`);
            const requests = await page.evaluate(() => window.__stageAdapters);
            assert(requests.length > 0, `${name}: no actual WebGPU adapter recorded`);
            adapters.push({name,url,requests});
        }
        comparisonComplete = true;
        assert.deepEqual(roundTripFailures,[], 'Exact history-frame assertions failed; production comparison is recorded separately');
        fs.writeFileSync(path.join(output,'result.json'),JSON.stringify({status:'passed',deck,chapter,orientation,viewport,
            browser:browser.version(),args,scope:'Published production baseline versus local production; emulated upright viewport, not physical-phone acceptance',
            checker,baselineCommit,fixedClock,
            first,frames,adapters,bundles,errors},null,2)+'\n');
    } catch(error) {
        await Promise.allSettled(downloads);
        for (const {name,page} of pages) {
            await page.bringToFront().catch(()=>{});
            await page.screenshot({path:path.join(output,`failure-${name}.png`)}).catch(()=>{});
        }
        fs.writeFileSync(path.join(output,'failure.json'),JSON.stringify({status:'failed',error:String(error),checker,
            baselineCommit,fixedClock,comparisonComplete,roundTripFailures,frames,bundles,adapters,errors},null,2)+'\n');
        throw error;
    } finally { await browser.close(); }
}
main().catch(error => { console.error(error); process.exitCode=1; });
