const assert = require('node:assert/strict');
const {test} = require('node:test');
const {readFileSync} = require('node:fs');
const {runInNewContext} = require('node:vm');
const {randomUUID} = require('node:crypto');
const source = readFileSync(new URL('../assets/diagnostics.js', `file://${__filename}`), 'utf8');

function storage() {
    const values = {};
    Object.defineProperties(values, {
        getItem: {value: key => values[key] || null},
        setItem: {value: (key, value) => { values[key] = value; }},
        removeItem: {value: key => { delete values[key]; }},
    });
    return values;
}

function open(localStorage = storage(), sessionStorage = storage(), gpu = {}) {
    const elements = {'viewer-diagnostics': {hidden: true}, 'viewer-diagnostics-message': {}};
    const downloads = [], listeners = {};
    let blob;
    const window = {};
    const context = {
        performance: {now: () => 42}, window, localStorage, sessionStorage, crypto: {randomUUID},
        location: {origin: 'https://viewer.test', pathname: '/', search: '?secret=hidden'},
        navigator: {gpu, userAgent: 'test browser'}, isSecureContext: true,
        innerWidth: 100, innerHeight: 80, devicePixelRatio: 1,
        Blob, URL: {createObjectURL: value => { blob = value; return 'blob:test'; }, revokeObjectURL() {}},
        document: {
            getElementById: id => elements[id],
            createElement: () => ({click() { downloads.push(blob); }}),
        },
        setTimeout() {}, setInterval() {},
        addEventListener: (name, listener) => { listeners[name] = listener; },
    };
    runInNewContext(source, context);
    return {...window, elements, downloads, listeners, localStorage, sessionStorage};
}

test('first failure survives later errors, downloads and recovery reloads', async () => {
    const page = open();
    page.viewerDiagnostic('adapter', 'Intel Vulkan');
    page.viewerDiagnostic('fatal', 'device lost');
    for (let i = 0; i < 30; i++) page.viewerDiagnostic('fatal', 'follow-on failure');
    const report = page.viewerDiagnostics.read();
    assert.equal(report.failure.message, 'device lost');
    assert.equal(report.events.length, 24);
    assert.equal(report.adapter, 'Intel Vulkan');
    assert.equal(page.downloads.length, 1);
    assert.equal(JSON.parse(await page.downloads[0].text()).failure.message, 'device lost');
    page.listeners.pagehide();
    const next = open(page.localStorage, page.sessionStorage);
    assert.equal(next.elements['viewer-diagnostics'].hidden, false);
    next.viewerDiagnostics.downloadPrevious();
    assert.equal(JSON.parse(await next.downloads[0].text()).failure.message, 'device lost');
    next.listeners.pagehide();
    const again = open(page.localStorage, page.sessionStorage);
    again.viewerDiagnostics.downloadPrevious();
    assert.equal(JSON.parse(await again.downloads[0].text()).failure.message, 'device lost');
});

test('abrupt interruption is recoverable; clean close and another live tab are not failures', () => {
    const page = open();
    assert.equal(page.viewerDiagnostics.read().page, 'https://viewer.test/');
    const other = open(page.localStorage);
    assert.equal(other.elements['viewer-diagnostics'].hidden, true);
    const restored = open(page.localStorage, page.sessionStorage);
    assert.equal(restored.elements['viewer-diagnostics'].hidden, false);
    const clean = open();
    clean.listeners.pagehide();
    assert.equal(open(clean.localStorage, clean.sessionStorage).elements['viewer-diagnostics'].hidden, true);
    for (let i = 0; i < 10; i++) open(clean.localStorage).listeners.pagehide();
    assert.equal(Object.keys(clean.localStorage).length, 4);
});

test('denied storage still allows a downloadable diagnostic', async () => {
    const denied = new Proxy({}, {get() { throw Error('Storage denied'); }});
    const page = open(denied, denied, undefined);
    page.listeners.unhandledrejection({reason: new Error('GPU initialization failed')});
    assert.equal(page.viewerDiagnostics.read().storage, false);
    assert.equal(page.downloads.length, 1);
    assert.match(await page.downloads[0].text(), /GPU initialization failed/);
});

test('missing WebGPU records a useful startup error', () => {
    const page = open(storage(), storage(), null);
    assert.equal(page.viewerDiagnostics.read().webgpu, false);
    assert.match(page.viewerDiagnostics.read().failure.message, /chrome:\/\/gpu/);
    assert.equal(page.elements['viewer-diagnostics'].hidden, false);
});

test('corrupt and older saved data cannot prevent viewer startup', () => {
    const data = storage();
    data.setItem('session-viewer-report:broken', '{');
    data.setItem('session-viewer-report:old', JSON.stringify({lastSeen: 1, outcome: 'failed'}));
    const page = open(data);
    assert.equal(page.viewerDiagnostics.read().outcome, 'running');
    assert.equal(page.elements['viewer-diagnostics'].hidden, true);
});


test('successful run downloads its own complete phases even after a previous failure', async () => {
    const failed = open();
    failed.viewerDiagnostic('fatal', 'earlier failure');
    const page = open(failed.localStorage, failed.sessionStorage);
    for (let i = 0; i < 40; i++) page.viewerDiagnostic('phase', JSON.stringify({name: 'download', bytes: 8192, durationMs: 12}));
    page.viewerDiagnostic('live-reload', 'initial');
    page.viewerDiagnostic('milestone', 'geometry on screen');
    const report = page.viewerDiagnostics.read();
    assert.equal(report.outcome, 'ready');
    assert.equal(report.phases.length, 40);
    assert.equal(report.liveReloads, 1);
    page.viewerDiagnostics.download();
    assert.equal(JSON.parse(await page.downloads[0].text()).outcome, 'ready');
});

test('failed and interrupted reports from yesterday do not show a stale warning', () => {
    for (const outcome of ['failed', 'running']) {
        const data = storage();
        data.setItem('session-viewer-report:stale', JSON.stringify({version: 1, outcome,
            started: '2020-01-01T00:00:00Z', lastSeen: '2020-01-01T00:00:00Z'}));
        assert.equal(open(data).elements['viewer-diagnostics'].hidden, true);
    }
});

test('cached transitions retain running, ready and failed outcomes', () => {
    for (const outcome of ['running', 'ready', 'failed']) {
        const page = open();
        if (outcome === 'ready') page.viewerDiagnostic('milestone', 'geometry on screen');
        if (outcome === 'failed') page.viewerDiagnostic('fatal', 'original failure');
        const first = page.viewerDiagnostics.read().failure;
        page.listeners.pagehide({persisted: true});
        assert.equal(page.viewerDiagnostics.read().outcome, outcome);
        page.listeners.pageshow({persisted: true});
        assert.equal(page.viewerDiagnostics.read().outcome, outcome);
        assert.deepEqual(page.viewerDiagnostics.read().failure, first);
    }
});

test('late ready observations preserve final closed and failed outcomes', () => {
    for (const outcome of ['closed', 'failed']) {
        const page = open();
        if (outcome === 'failed') page.viewerDiagnostic('fatal', 'original failure');
        else page.listeners.pagehide({persisted: false});
        const first = page.viewerDiagnostics.read().failure;
        page.viewerDiagnostic('milestone', 'geometry on screen');
        assert.equal(page.viewerDiagnostics.read().outcome, outcome);
        assert.deepEqual(page.viewerDiagnostics.read().failure, first);
    }
});

test('a cached ready run does not become an interrupted startup on reload', () => {
    const page = open(); page.viewerDiagnostic('milestone', 'geometry on screen');
    page.listeners.pagehide({persisted: true}); page.listeners.pageshow({persisted: true});
    assert.equal(open(page.localStorage, page.sessionStorage).elements['viewer-diagnostics'].hidden, true);
});
