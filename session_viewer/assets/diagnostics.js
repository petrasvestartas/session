(() => {
    const prefix = 'session-viewer-report:';
    const now = () => new Date().toISOString();
    const read = key => {
        try { return JSON.parse(localStorage.getItem(key)); }
        catch { return null; }
    };
    let tab;
    try {
        tab = sessionStorage.getItem(prefix) || crypto.randomUUID();
        sessionStorage.setItem(prefix, tab);
    } catch { tab = crypto.randomUUID(); }

    const key = prefix + crypto.randomUUID();
    let reports = [];
    try {
        reports = Object.keys(localStorage).filter(key => key.startsWith(prefix))
            .map(key => ({key, report: read(key)}))
            .filter(({report}) => report?.version === 1 && typeof report.started === 'string'
                && typeof report.lastSeen === 'string')
            .sort((a, b) => b.report.lastSeen.localeCompare(a.report.lastSeen));
        for (const item of reports.slice(3)) localStorage.removeItem(item.key);
    } catch { /* Private browsing may deny storage; downloads still work. */ }

    // A stale heartbeat means an interruption, not proof of a browser crash.
    const previous = reports.find(item => Date.now() - Date.parse(item.report.lastSeen) < 2 * 60 * 60 * 1000
        && (item.report.outcome === 'failed'
        || (item.report.outcome === 'running' && (item.report.tab === tab
            || Date.now() - Date.parse(item.report.lastSeen) > 120000))))?.report;
    const report = {
        version: 1, tab, started: now(), lastSeen: now(), outcome: 'running',
        page: location.origin + location.pathname, browser: navigator.userAgent,
        secureContext: isSecureContext, webgpu: !!navigator.gpu,
        viewport: [innerWidth, innerHeight], devicePixelRatio,
        storage: true, events: [], phases: [], resources: [], liveReloads: 0,
    };
    let downloaded = false;

    function save() {
        report.lastSeen = now();
        const canvas = document.getElementById('canvas');
        if (canvas) report.canvas = [canvas.width, canvas.height];
        report.viewport = [innerWidth, innerHeight];
        report.devicePixelRatio = devicePixelRatio;
        try { localStorage.setItem(key, JSON.stringify(report)); }
        catch { report.storage = false; }
    }

    function download(selected = report) {
        const blob = new Blob([JSON.stringify(selected, null, 2)], {type: 'application/json'});
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = `session-viewer-${selected.started.replace(/[:.]/g, '-')}.json`;
        link.click();
        setTimeout(() => URL.revokeObjectURL(url), 10000);
    }

    function show(message) {
        const panel = document.getElementById('viewer-diagnostics');
        if (!panel) return;
        document.getElementById('viewer-diagnostics-message').textContent = message;
        panel.hidden = false;
    }

    window.viewerDiagnostic = (kind, message) => {
        const event = {time: now(), elapsedMs: performance.now(), kind, message: String(message).slice(0, 4096)};
        report.events.push(event);
        report.events = report.events.slice(-24);
        if (kind === 'adapter') report.adapter = event.message;
        if (kind === 'phase') {
            try { report.phases.push({...JSON.parse(message), elapsedMs: event.elapsedMs}); } catch {}
            report.phases = report.phases.slice(-256);
        }
        if (kind === 'live-reload') report.liveReloads++;
        if (kind === 'milestone' && message === 'geometry on screen') report.outcome = 'ready';
        if (kind === 'fatal') {
            report.failure ||= event;
            report.outcome = 'failed';
        }
        save();
        if (kind === 'fatal') {
            show(report.storage ? 'A viewer error was saved. Download the report before sharing it.'
                : 'Storage is unavailable. Download the report before closing this tab.');
            if (!downloaded) {
                downloaded = true;
                // Browsers may block automatic downloads; the button remains available.
                try { download(); } catch { /* Keep the saved report. */ }
            }
        }
    };
    window.viewerDiagnostics = {download, downloadPrevious: () => previous && download(previous), read: () => JSON.parse(JSON.stringify(report))};
    addEventListener('error', event => {
        if (event.message) window.viewerDiagnostic('fatal', event.error?.stack || event.message);
    });
    addEventListener('unhandledrejection', event => {
        window.viewerDiagnostic('fatal', event.reason?.stack || event.reason);
    });
    addEventListener('pagehide', () => {
        if (report.outcome !== 'failed') report.outcome = 'closed';
        save();
    });
    addEventListener('pageshow', event => {
        if (event.persisted && report.outcome === 'closed') report.outcome = 'running';
        save();
    });
    document.addEventListener?.('visibilitychange', () => window.viewerDiagnostic('visibility', document.visibilityState));
    document.addEventListener?.('freeze', () => window.viewerDiagnostic('freeze', 'page frozen'));
    document.addEventListener?.('resume', () => window.viewerDiagnostic('resume', 'page resumed'));
    if (typeof PerformanceObserver !== 'undefined') {
        const observer = new PerformanceObserver(list => {
            for (const entry of list.getEntries()) {
                if (entry.initiatorType !== 'fetch' && !/\.wasm$/.test(entry.name)) continue;
                report.resources.push({url: entry.name.split('?')[0], startMs: entry.startTime,
                    durationMs: entry.duration, transferBytes: entry.transferSize,
                    encodedBytes: entry.encodedBodySize, decodedBytes: entry.decodedBodySize});
            }
            report.resources = report.resources.slice(-128);
            save();
        });
        observer.observe({type: 'resource', buffered: true});
    }
    // Use the viewer's adapter request; wgpu currently drops architecture/vendor on WebGPU.
    if (navigator.gpu?.requestAdapter) {
        const request = navigator.gpu.requestAdapter.bind(navigator.gpu);
        navigator.gpu.requestAdapter = async options => {
            const adapter = await request(options);
            if (adapter?.info) {
                const info = adapter.info;
                report.gpuAdapterInfo = {vendor: info.vendor, architecture: info.architecture,
                    device: info.device, description: info.description};
                save();
            }
            return adapter;
        };
    }
    setInterval(save, 15000);
    save();

    if (previous) show('A previous viewer run failed or was interrupted. Its report is available.');
    if (!navigator.gpu) {
        window.viewerDiagnostic('fatal', isSecureContext
            ? 'WebGPU is unavailable. Check chrome://gpu and the browser setup in the documentation.'
            : 'WebGPU requires HTTPS or localhost. Open the viewer on a secure origin.');
    }
})();
