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
        window, localStorage, sessionStorage, crypto: {randomUUID},
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
    next.viewerDiagnostics.download();
    assert.equal(JSON.parse(await next.downloads[0].text()).failure.message, 'device lost');
    next.listeners.pagehide();
    const again = open(page.localStorage, page.sessionStorage);
    again.viewerDiagnostics.download();
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
