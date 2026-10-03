module.exports = async page => page.addInitScript(() => {
    if (typeof GPUQueue === 'undefined') return;
    const records = [], metadata = new Set(), lost = new WeakSet(), owners = new WeakMap();
    const probe = window.__life = {devices: [], lostCalls: [], queueCalls: 0, enabled: false, ticks: 0, deltas: [], intervals: []};
    const kind = target => target === window ? 'window' : target === document ? 'document' : target === document.querySelector('canvas') ? 'canvas' : null;
    const add = EventTarget.prototype.addEventListener, remove = EventTarget.prototype.removeEventListener;
    const deny = sessionStorage.getItem('__denyReportLifecycle') === '1'; sessionStorage.removeItem('__denyReportLifecycle');
    EventTarget.prototype.addEventListener = function (name, callback, options) {
        if (this === window && name === 'pageshow') metadata.add(callback);
        if (deny && this === window && name === 'pageshow') throw new DOMException('Denied lifecycle registration', 'SecurityError');
        const result = add.call(this, name, callback, options), target = kind(this);
        if (target) records.push({target, name, callback, removed: 0});
        return result;
    };
    EventTarget.prototype.removeEventListener = function (name, callback, options) {
        const target = kind(this), row = records.find(row => row.target === target && row.name === name && row.callback === callback && !row.removed);
        if (row) row.removed++;
        return remove.call(this, name, callback, options);
    };
    probe.summary = () => ({activeMetadata: records.filter(row => metadata.has(row.callback) && !row.removed).length,
        removedMetadata: records.filter(row => metadata.has(row.callback)).reduce((sum, row) => sum + row.removed, 0),
        removedRuntime: records.filter(row => !metadata.has(row.callback)).reduce((sum, row) => sum + row.removed, 0),
        intervals: probe.intervals.map(row => ({...row})), ticks: probe.ticks, deltas: [...probe.deltas], lostCalls: [...probe.lostCalls]});
    const request = GPUAdapter.prototype.requestDevice;
    GPUAdapter.prototype.requestDevice = async function (...args) {
        const device = await request.apply(this, args); owners.set(device.queue, device); probe.devices.push(device);
        device.lost.then(() => lost.add(device)); return device;
    };
    for (const [prototype, methods] of [
        [GPUQueue.prototype, ['writeBuffer', 'writeTexture', 'submit']],
        [GPUDevice.prototype, ['createBuffer', 'createTexture', 'createCommandEncoder', 'createBindGroup', 'createRenderPipeline', 'createShaderModule']],
        [GPUCanvasContext.prototype, ['configure', 'getCurrentTexture']],
    ]) for (const method of methods) {
        const original = prototype[method];
        prototype[method] = function (...args) {
            const device = owners.get(this) || args[0]?.device || this;
            if (method === 'configure') owners.set(this, device);
            if (this instanceof GPUQueue) probe.queueCalls++;
            if (lost.has(device)) probe.lostCalls.push(method);
            return original.apply(this, args);
        };
    }
    const interval = window.setInterval.bind(window), clear = window.clearInterval.bind(window);
    window.setInterval = function (callback, period, ...args) {
        if (period !== 15000) return interval(callback, period, ...args);
        const row = {id: 0, period, cleared: 0};
        row.id = interval(() => {
            if (!probe.enabled) return;
            const before = probe.queueCalls; probe.ticks++; callback(...args); probe.deltas.push(probe.queueCalls - before);
        }, 20); probe.intervals.push(row); return row.id;
    };
    window.clearInterval = function (id) {
        const row = probe.intervals.find(row => row.id === id); if (row) row.cleared++;
        return clear(id);
    };
});
