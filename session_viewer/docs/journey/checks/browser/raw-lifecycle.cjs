// Keep these targets outside Playwright's focus/visibility emulation.
module.exports = async original => {
    const root = await original.context().browser().newBrowserCDPSession();
    const {browserContextId} = await root.send('Target.createBrowserContext');
    const pending = new Map(), errors = []; let next = 0;
    root.on('Target.receivedMessageFromTarget', event => {
        const message = JSON.parse(event.message), slot = pending.get(message.id);
        if (message.method === 'Runtime.exceptionThrown') errors.push(message.params.exceptionDetails);
        if (!slot) return;
        pending.delete(message.id); clearTimeout(slot.timer);
        message.error ? slot.reject(Error(message.error.message)) : slot.resolve(message.result);
    });
    const page = async () => {
        const {targetId} = await root.send('Target.createTarget', {url: 'about:blank', browserContextId});
        const {sessionId} = await root.send('Target.attachToTarget', {targetId, flatten: false});
        const send = (method, params = {}) => new Promise((resolve, reject) => {
            const id = ++next;
            const timer = setTimeout(() => { pending.delete(id); reject(Error(`${method} timed out`)); }, 20000);
            pending.set(id, {resolve, reject, timer});
            root.send('Target.sendMessageToTarget', {sessionId, message: JSON.stringify({id, method, params})}).catch(error => {
                pending.delete(id); clearTimeout(timer); reject(error);
            });
        });
        await send('Page.enable'); await send('Runtime.enable');
        return {send,
            activate: () => root.send('Target.activateTarget', {targetId}),
            async evaluate(fn) {
                const result = await send('Runtime.evaluate', {expression: `(${fn.toString()})()`, returnByValue: true});
                if (result.exceptionDetails) throw Error(JSON.stringify(result.exceptionDetails));
                return result.result.value;
            },
            addInitScript: fn => send('Page.addScriptToEvaluateOnNewDocument', {source: `(${fn.toString()})();`}),
        };
    };
    return {page, errors, async close() {
        await root.send('Target.disposeBrowserContext', {browserContextId}); await root.detach();
        await original.bringToFront();
    }};
};
